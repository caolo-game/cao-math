test:
	wasm-pack test --firefox --headless
	wasm-pack test --chrome --headless

build:
	wasm-pack build --scope caolo-game

pack:
	wasm-pack pack

publish:
	cd pkg && npm publish --access=public

