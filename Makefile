test:
	wasm-pack test --firefox --headless
	wasm-pack test --chrome --headless

build:
	wasm-pack build --scope caolo-game --target web

pack: build
	wasm-pack pack

publish: pack
	cd pkg && npm publish --access=public

