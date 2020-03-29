test:
	cargo test
	wasm-pack test --firefox --headless

build:
	wasm-pack build --scope caolo-game

pack:
	wasm-pack pack

publish: build
	cd pkg && npm publish --access=public

