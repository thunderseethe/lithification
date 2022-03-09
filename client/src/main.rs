use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::Window,
};

use renderer::State;
use common::{Chunk, Message};

async fn run(event_loop: EventLoop<Message>, window: Window) {
    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {event, window_id} if window_id == window.id() => 
                if !state.input(&event) {
                    match event {
                        WindowEvent::Resized(size) => 
                            state.resize(size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => 
                            state.resize(*new_inner_size),
                        WindowEvent::CloseRequested 
                        | WindowEvent::KeyboardInput {
                                input: KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {},
                    };
                },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::UserEvent(msg) => {
                if msg.tag == "chunk" {
                    log::info!("Received Chunk {:?}", Chunk::from_bytes(&msg.bytes));
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

pub fn start_websocket(event_loop: EventLoopProxy<Message>) -> Result<(), JsValue> {
    let ws = WebSocket::new("ws://192.168.0.163:3030/chunk")?;

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_cb = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            log::info!("{:?}", array.to_vec());
            let msg = Message::from_bytes(&array.to_vec()).expect("Could not parse message from received bytes");
            event_loop.send_event(msg).expect("Could not send user event");
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
    onmessage_cb.forget();

    let onerror_cb = Closure::wrap(Box::new(move |e: ErrorEvent| {
        log::error!("error event: {:#?}", e.error());
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
    onerror_cb.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        log::info!("socket opened");
        match cloned_ws.send_with_str("give me chunk") {
            Ok(_) => {},
            Err(err) => log::error!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

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

    start_websocket(event_loop.create_proxy()).expect("WebSocket to start successfully");

    wasm_bindgen_futures::spawn_local(run(event_loop, window));
}
