use client::{start_websocket, run};
use winit::{
    event_loop::EventLoop,
};
use common::Message;

fn main() {
    let event_loop: EventLoop<Message> = EventLoop::with_user_event();
    let window = winit::window::Window::new(&event_loop).unwrap();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    use winit::platform::web::WindowExtWebSys;
    // On wasm, append the canvas to the document body
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("couldn't append canvas to document body");

    let ws = start_websocket(event_loop.create_proxy()).expect("WebSocket to start successfully");

    wasm_bindgen_futures::spawn_local(run(event_loop, ws, window));
}
