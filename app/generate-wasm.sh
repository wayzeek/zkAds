#!/bin/bash
cd program/
cargo build --release --target wasm32-wasi
cd ..