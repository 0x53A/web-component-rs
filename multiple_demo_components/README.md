This Example demonstrates that it's possible, and trivial, to load multiple independent rust web-components, from multiple wasm blobs.

### Build

In each folder, run

```sh
wasm-pack build --target web
```

### Run

nix-shell -p python3
python3 -m http.server 8000

### All together

```sh
cd first_component
wasm-pack build --target web
cd ../second_component
wasm-pack build --target web
cd ..
nix-shell -p python3 --run "python3 -m http.server 8000"
```