build:
	wasm-pack build

install: build
	cd www && yarn install --force

test:
	wasm-pack test --firefox --headless
