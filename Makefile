build:
	wasm-pack build

install: build
	cd www && yarn install --force

