use axum::{
    Router, 
    error_handling::HandleError,
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    routing::{get, get_service},
    response::IntoResponse,
};
use std::{
    net::SocketAddr,
};
use tower_http::services::ServeDir;

use common::{Block, Chunk};
use wasmtime::*;

mod ecs;

#[tokio::main]
async fn main() {
    initialize_wasm().await.expect("Wasm Error");

    let serve_generated_dir = get_service(ServeDir::new("generated"));
    let app = Router::new()
        .route("/chunk", get(ws_chunk))
        .fallback(HandleError::new(serve_generated_dir, err_handle));

    let socket_addr = "192.168.0.163:3030".parse().unwrap();
    println!("Launched Server..."); 
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
        .await
        .unwrap();
}

async fn initialize_wasm() -> anyhow::Result<()> {
    use tokio::fs::read;

    let engine = Engine::new(
        Config::new()
            .wasm_module_linking(true))?;  
 
    let wasm = read("mods/player_movement/build/optimized.wasm").await?;
    let module = Module::new(&engine, wasm).unwrap();

    let mut store = Store::new(&engine, ());
    fn get_string_impl(bytes: &[u8], ptr: usize) -> String {
        let mut len = [0u8; 4];
        len.copy_from_slice(&bytes[(ptr - 4)..ptr]);
        let len: u32 = unsafe { std::mem::transmute(len) };
        let str: Vec<u16> = (&bytes[ptr..(ptr + (len as usize))]).chunks(2)
            .map(|s| unsafe { std::mem::transmute([s[0], s[1]]) })
            .collect::<Vec<_>>();
        String::from_utf16_lossy(&str)
    }

    let mut linker: Linker<()> = Linker::new(&engine);
    linker.func_wrap("env", "abort", |mut caller: Caller<'_, ()>, msg: i32, filename: i32, line: i32, col: i32| -> () {
        let mem = caller.get_export("memory")
            .and_then(|m| m.into_memory())
            .unwrap();
        let data = mem.data(&caller);
        let msg = get_string_impl(data, msg as usize);

        println!("Abort: {:?} ({}, {})", msg, line, col);
    })?;

    let instance = linker.instantiate(&mut store, &module)?;
    let run = instance.get_typed_func::<(i32,), i32, _>(&mut store, "run")?;
    
    let output = run.call(&mut store, (0,))? as usize;
    let mod_mem = instance.get_memory(&mut store, "memory").expect("Expected module to export memory");

    let pos = &mod_mem.data(&store)[output..(output+12)];
    let (x, y, z):(u32, u32, u32) = (read_u32(&pos[0..4]), read_u32(&pos[4..8]), read_u32(&pos[8..12]));
    println!("({}, {}, {})", x, y, z);
    Ok(())
}

fn read_u32(bytes: &[u8]) -> u32 {
    let mut dst = [0u8; 4];
    dst.copy_from_slice(bytes);
    unsafe { std::mem::transmute(dst) }
}

async fn ws_chunk(ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket) {
        let chunk_half_dim = (Chunk::DIMENSION/2) as i16;
        let chunk_sphere_threshold = chunk_half_dim * chunk_half_dim;
        let chunk = Chunk::from_fn(|x: u8, y: u8, z:u8| {
            let x2 = (x as i16) - chunk_half_dim;
            let x2 = x2 * x2;
            let y2 = (y as i16) - chunk_half_dim;
            let y2 = y2 * y2;
            let z2 = (z as i16) - chunk_half_dim;
            let z2 = z2 * z2;

            if x2 + y2 + z2 < chunk_sphere_threshold {
                Block::new(1)
            } else {
                Block::new(0)
            }
        });

        while let Some(msg) = socket.recv().await {
            if let Err(_) = msg {
                return;
            }


            let resp = common::Message {
                tag: "chunk".to_owned(),
                bytes: chunk.to_bytes(),
            };
            if socket.send(Message::Binary(resp.to_bytes())).await.is_err() {
                return;
            }
        }
    }
    ws.on_upgrade(handle_socket)
}

async fn err_handle(_err: std::io::Error) -> impl IntoResponse {
    "I don't know what do"
}
