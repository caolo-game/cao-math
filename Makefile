.PHONY: docs build test

test:
	wasm-pack test --firefox --headless
	wasm-pack test --chrome --headless
	wasm-pack test --node

build:
	wasm-pack build --scope caolo-game -d out/pkg

pack: build
	wasm-pack pack out/pkg

publish: pack
	cd out/pkg && npm publish --access=public

docs: build
	cd docs && npx typedoc --inputFiles ../out/pkg/cao_math.d.ts
