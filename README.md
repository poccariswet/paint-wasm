# paint-wasm

this repo is sample for mouse event listener on wasm

https://paint-wasm-sample.netlify.com/


## Usage

- generate

```
$  cargo generate --git https://github.com/poccariswet/rust-wasm-template-without-bundler
```

- build

```
$ make build
```
or
```
$ wasm-pack build --target web
```

- run serve
```
$ make start
```

- for hosting

```
$ make build-hosting
```
