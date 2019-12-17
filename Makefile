build:
	wasm-pack build --target web

build-hosting:
	wasm-pack build --target web
	rm -rf dist
	mkdir dist
	cp index.html dist
	mkdir dist/pkg
	cp pkg/paint-wasm.js dist/pkg
	cp pkg/paint-wasm.wasm dist/pkg

start:
	python3 -m http.server
