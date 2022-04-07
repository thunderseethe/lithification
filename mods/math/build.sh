set -e
cargo build --release --target wasm32-unknown-unknown
mkdir -p out
wasm-opt -all -O4 -o out/math.opt.wasm target/wasm32-unknown-unknown/release/math.wasm
wasm2wat --enable-all --generate-names -o out/math.opt.wat out/math.opt.wasm