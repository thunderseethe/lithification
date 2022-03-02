{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  #nativeBuildInputs = with pkgs; [ rustc cargo gcc ];
  buildInputs = with pkgs; [ 
    wasm-bindgen-cli
    shaderc
    spirv-tools
  ];
} 
