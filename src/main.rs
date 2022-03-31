use axum::{
    Router, 
    error_handling::HandleError,
    extract::{ws::{WebSocket, WebSocketUpgrade}, connect_info::IntoMakeServiceWithConnectInfo},
    routing::{get, get_service},
    response::IntoResponse, Server,
};
use hyper::server::conn::AddrIncoming;
use std::{
    net::SocketAddr,
};
use tower_http::services::ServeDir;
use tokio::sync::mpsc;
use std::sync::Arc;
use parking_lot::Mutex;

use common::{Block, Chunk};
use wasmtime::*;

mod ecs;

fn server(outgoing_receiver: Arc<Mutex<mpsc::Receiver<common::Message>>>, incoming_sender: mpsc::Sender<common::Message>) -> Server<AddrIncoming, IntoMakeServiceWithConnectInfo<Router, std::net::SocketAddr>> {
    let serve_generated_dir = get_service(ServeDir::new("generated"));
    let app = Router::new()
        .route("/chunk", get(move |ws| ws_chunk(ws, outgoing_receiver, incoming_sender)))
        .fallback(HandleError::new(serve_generated_dir, err_handle));

    let socket_addr = "192.168.0.163:3030".parse().unwrap();
    log::info!("Launched Server..."); 
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
}

fn handle_msg(msg: common::Message, player_position: &mut common::PlayerPosition, outgoing_message: &mut Vec<common::Message>) {
    if msg.tag == "sphere_chunk" {
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
            
        outgoing_message.push(common::Message {
            tag: "chunk".to_owned(),
            bytes: chunk.to_bytes(),
        });
    }
    else if msg.tag == "player_backward" {
        player_position.add(0.0, 0.0, 1.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
    else if msg.tag == "player_forward" {
        player_position.add(0.0, 0.0, -1.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
    else if msg.tag == "player_up" {
        player_position.add(0.0, 1.0, 0.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
    else if msg.tag == "player_down" {
        player_position.add(0.0, -1.0, 0.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
    else if msg.tag == "player_right" {
        player_position.add(0.0, 1.0, 0.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
    else if msg.tag == "player_left" {
        player_position.add(-1.0, 0.0, 0.0);
        outgoing_message.push(common::Message {
            tag: "player_position".to_owned(),
            bytes: player_position.to_bytes(),
        })
    }
}

async fn initialize_wasm() -> Result<wasmer::> {

}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    initialize_wasm().await.expect("Wasm Error");

    let (outgoing_sender, outgoing_receiver) = mpsc::channel::<common::Message>(100);
    let (incoming_sender, mut incoming_receiver) = mpsc::channel::<common::Message>(100);
    let outgoing_receiver = Arc::new(Mutex::new(outgoing_receiver));

    let server_handle = server(outgoing_receiver, incoming_sender);

    //let engine = Engine::new(
    //    Config::new()
    //        .wasm_module_linking(true)
    //        .wasm_multi_memory(true)).expect("Failed to initialize wasm");



    let mut player_position = common::PlayerPosition::new();
    let game_loop = tokio::spawn(async move {
        log::info!("Started game loop...");
        let mut outgoing_message: Vec<common::Message> = Vec::with_capacity(9);

        loop {
            let msg = match incoming_receiver.recv().await {
                Some(msg) => {
                    log::info!("Received msg: {:?}", msg);
                    msg
                },
                // We should technically still do some frame stuff here even if we don't receive a message
                None => continue
            };

            // Clear out any lingering messages from last loop
            outgoing_message.clear();
            
            handle_msg(msg, &mut player_position, &mut outgoing_message);

            for msg in outgoing_message.drain(..) {
                if let Err(e) = outgoing_sender.send(msg).await {
                    log::error!("Failed to enqueue outgoing message: {:?}", e);
                }
            }
        }
    });

    server_handle.await.unwrap();
    log::info!("Server stopped");
    game_loop.await.unwrap();
    log::info!("Gmae loop stopped");
}

/*async fn initialize_wasm() -> anyhow::Result<()> {
    use tokio::fs::read;
 
    let engine = Engine::new(
        Config::new()
            .wasm_module_linking(true)
            .wasm_multi_memory(true)).expect("Failed to initialize wasm");

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
    linker.func_wrap("env", "abort", |mut caller: Caller<'_, ()>, msg: i32, _: i32, line: i32, col: i32| -> () {
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
}*/

async fn ws_chunk(ws: WebSocketUpgrade, outgoing_receiver: Arc<Mutex<mpsc::Receiver<common::Message>>>, incoming_sender: mpsc::Sender<common::Message>) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket, outgoing_receiver: Arc<Mutex<mpsc::Receiver<common::Message>>>, incoming_sender: mpsc::Sender<common::Message>) {
        // TODO This is too rudimentary and should halt on disconnect
        loop {
            if let Some(msg_res) = socket.recv().await {
                match msg_res {
                    Ok(ws_msg) => {
                        use axum::extract::ws::Message::Binary;
                        if let Binary(bytes) = ws_msg {
                            let res = common::Message::from_bytes(&bytes).and_then(|msg| {
                                incoming_sender.try_send(msg).map_err(|_| anyhow::anyhow!(""))
                            });
                            if let Err(e) = res {
                                log::error!("Dropped a message: {:?}", e)
                            }
                        }
                    },
                    // TODO: Handle disconnect
                    Err(err) => log::error!("Failed to receive message: {:?}", err),
                }
            }

            let msg_res = outgoing_receiver.lock().try_recv();
            if let Ok(out_msg) = msg_res  {
                if let Err(e) = socket.send(axum::extract::ws::Message::Binary(out_msg.to_bytes())).await {
                    log::error!("Failed to send message: {:?}", e);
                }
            }
        }
    }
    ws.on_upgrade(move |ws| handle_socket(ws, outgoing_receiver, incoming_sender))
}

async fn err_handle(_err: std::io::Error) -> impl IntoResponse {
    "I don't know what do"
}
