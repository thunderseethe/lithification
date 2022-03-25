set -e
MODE=""
DIR="debug"

while getopts 'r' flag; do
  case "${flag}" in
    r) MODE="--release"; DIR="release" ;;
  esac
done

pushd client
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown $MODE
popd
wasm-bindgen --out-dir generated --target no-modules --debug --keep-debug "target/wasm32-unknown-unknown/$DIR/client.wasm"
