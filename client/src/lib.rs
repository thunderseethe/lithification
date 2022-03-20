use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::Window,
};

use common::{Chunk, Message};

mod fetch;
mod render;
use render::State;

pub async fn run(event_loop: EventLoop<Message>, window: Window) {
    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        //*control_flow = ControlFlow::Wait;
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
                    let chunk = Chunk::from_bytes(&msg.bytes).expect("Failed to parse chunk out of message payload");
                    log::info!("Received Chunk {:?}", chunk); 
                    state.set_chunk(chunk);
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}