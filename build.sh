set -e
pushd client
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown
popd
wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/debug/client.wasm
