use axum::{
    Router, 
    error_handling::HandleError,
    extract::{ws::{WebSocket, WebSocketUpgrade}, connect_info::IntoMakeServiceWithConnectInfo},
    routing::{get, get_service},
    response::IntoResponse, Server,
};
use hyper::{server::conn::AddrIncoming};
use std::{
    net::SocketAddr,
};
use tower_http::{services::ServeDir, compression::CompressionLayer};
use tokio::sync::mpsc;
use std::sync::Arc;
use parking_lot::Mutex;

use wasmtime::*;

//mod ecs;
mod event;
mod resources;
mod world;

fn server(binary_blob: Vec<u8>, outgoing_receiver: Arc<Mutex<mpsc::Receiver<common::Message>>>, incoming_sender: mpsc::Sender<common::Message>) -> Server<AddrIncoming, IntoMakeServiceWithConnectInfo<Router, std::net::SocketAddr>> {
    let serve_generated_dir = get_service(ServeDir::new("generated"));
    let app = Router::new()
        .route("/chunk", get(move |ws| ws_chunk(ws, outgoing_receiver, incoming_sender)))
        .route("/blob", get(move || async { binary_blob }).layer(CompressionLayer::new()))
        .fallback(HandleError::new(serve_generated_dir, err_handle));

    let socket_addr = "192.168.0.163:3030".parse().unwrap();
    log::info!("Launched Server..."); 
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
}


struct Env {
    event: event::Event,
}
impl Env {
    fn new() -> Self {
        Self {
            event: event::Event::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let (outgoing_sender, outgoing_receiver) = mpsc::channel::<common::Message>(100);
    let (incoming_sender, mut incoming_receiver) = mpsc::channel::<common::Message>(100);
    let outgoing_receiver = Arc::new(Mutex::new(outgoing_receiver));

    /*let env = Env::default();

    let mut cranelift = Cranelift::new();
    cranelift.canonicalize_nans(true);

    let mut features = Features::new();
    features.module_linking(true)
        .multi_value(true)
        .tail_call(true)
        .threads(true);

    let store = Store::new(&UniversalEngine::new(
        Cranelift::compiler(Box::new(cranelift)),
        Target::new(wasmer::HOST, CpuFeature::for_host()),
        features,
    ));

    fn env_abort(_msg: u32, _file_name: u32, _line: u32, _column: u32) {
        // pass for now
        log::error!("wasm abort");
    }
 
    let math_mod = Module::from_file(&store, "mods/math/out/math.opt.wasm").expect("Could not create math module");
    let math_instance = Instance::new(&math_mod, &imports!{}).expect("Could not create math instance");
    
    let mut imports = imports! {
        "env" => {
            "abort" => Function::new_native(&store, env_abort),
        },
        "host" => {
            "is_event" => Function::new_native_with_env(&store, env.clone(), host::is_event),
            "message_empty" => Function::new_native_with_env(&store, env.clone(), host::message_empty),
            "intern_string" => Function::new_native_with_env(&store, env.clone(), host::intern_string), 
            "register_resource_inner" => Function::new_native_with_env(&store, env.clone(), host::register_resource_inner),
            "MESSAGE_SIZE" => Global::new(&store, Value::I32(std::mem::size_of::<common::Message> as i32)),
        },
    };
    imports.register("math", math_instance.exports);

    let player_movement_mod = Module::from_file(&store, "mods/player_movement/build/optimized.wasm").expect("Could not create math module");
    let player_move_instance = Instance::new(&player_movement_mod, &imports).expect("Could not instantiate player_movement module");*/
    let mut config = Config::new();
    config.wasm_module_linking(true)
          .wasm_multi_memory(true);
    let engine = Engine::new(&config).expect("Failed to initialize engine");
    let mut store = Store::new(&engine, Env::new());

    let mut linker: Linker<Env> = Linker::new(&engine);
    event::event::add_to_linker(&mut linker, |ctx| -> &mut event::Event {
        &mut ctx.event
    }).expect("Failed to add event to linker");

    let math_mod = Module::from_file(&engine, "./mods/math/target/wasm32-unknown-unknown/release/math.wasm")
        .expect("Failed to create math module");

    linker.module(&mut store, "math", &math_mod).expect("Failed to link module");

    let player_movement_mod = Module::from_file(&engine, "./mods/player_movement/target/wasm32-unknown-unknown/release/player_movement.wasm")
        .expect("Failed to create player_movement module");
    
    let pm_mod = linker.instantiate(&mut store, &player_movement_mod).expect("Failed to pre-initialize player_movement mod");
    let init_fn = pm_mod.get_func(&mut store, "init").expect("Modules must export an init method");
    init_fn.call(&mut store, &[], &mut []).expect("Failed to init() module");

    let binary_blob = store.data().event.serialize_symbols().expect("Failed to serialize interner");

    let server_handle = server(binary_blob, outgoing_receiver, incoming_sender);

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
            
            let run_func = pm_mod.get_func(&mut store, "run").expect("Module must export a `run` method");
            let mem = pm_mod.get_memory(&mut store, "memory").expect("Module must export `memory`");
            let heap_base: u32 = pm_mod.get_global(&mut store, "__heap_base")
                .and_then(|g| g.get(&mut store).i32())
                .map(|i| i as u32)
                .expect("Module must export an i32 valued global `__heap_base`");

            mem.write(&mut store, 0, &[]);

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
    "I don't know what to do"
}
