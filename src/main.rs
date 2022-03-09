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

#[tokio::main]
async fn main() {
    println!("Launched Server...");

    let serve_generated_dir = get_service(ServeDir::new("generated"));
    let app = Router::new()
        .route("/chunk", get(ws_chunk))
        .fallback(HandleError::new(serve_generated_dir, err_handle));

    let socket_addr = "192.168.0.163:3030".parse().unwrap();
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
        .await
        .unwrap();
}

async fn ws_chunk(ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket) {
        let chunk = Chunk::from_fn(|x: u8, y: u8, z:u8| {
            let x2 = (x as i16) - 16;
            let x2 = x2 * x2;
            let y2 = (y as i16) - 16;
            let y2 = y2 * y2;
            let z2 = (z as i16) - 16;
            let z2 = z2 * z2;

            if x2 + y2 + z2 <= 16i16*16i16 {
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
