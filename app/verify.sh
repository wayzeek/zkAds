#!/bin/bash
cd verifier/
RUST_LOG=debug cargo +nightly run --release
cd ..