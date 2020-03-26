build:
	wasm-pack build

test:
	cargo test
	wasm-pack test --firefox --headless
