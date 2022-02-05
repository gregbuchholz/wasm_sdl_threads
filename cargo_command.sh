#!/usr/bin/sh

#using "stock" nightly rustc from back in December that is know to compile
cargo +nightly-2021-12-06 build --target=wasm32-unknown-emscripten --release -Z build-std=panic_abort,std
