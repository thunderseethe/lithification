use std::path::Path;
use web_sys::{Request, Response, RequestInit, RequestMode};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;

pub async fn fetch_bytes<P: AsRef<Path>>(url: P) -> anyhow::Result<Vec<u8>> {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors); 
    let url = url.as_ref().to_str().ok_or(anyhow::anyhow!("Path contained invalid utf8"))?;

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|_| anyhow::Error::msg("Failed to construct request"))?;
    request
        .headers();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| anyhow::Error::msg("Failed to fetch url"))?;
    let resp: Response = resp_value.dyn_into().unwrap();
    log::info!("received response {:?}", resp);

    // Convert this other `Promise` into a rust `Future`.
    let array_buffer_value = JsFuture::from(
        resp.array_buffer().map_err(|_| anyhow::Error::msg("Failed to convert response to array buffer"))?
    ).await.map_err(|_| anyhow::Error::msg("Failed to await response"))?;
    let array_buffer = array_buffer_value.dyn_into().unwrap();

    // Actually do the thing here
    Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
}