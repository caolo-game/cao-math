build:
	wasm-pack build

install: build
	wasm-pack build --release
	cd www && yarn install --force

test:
	cargo test
	wasm-pack test --firefox --headless
