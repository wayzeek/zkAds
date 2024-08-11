generate_wasm:
	@echo "Generating wasm file from the rust program"
	cargo build --target wasm32-wasi --release --manifest-path ./app/program/Cargo.toml

generate_proof: ARGS = 1.0 1.0
generate_proof:
	@echo "Generating proof for running the wasm program with private inputs"
	RUST_LOG=debug cargo +nightly run --release --manifest-path ./local/Cargo.toml $(ARGS)