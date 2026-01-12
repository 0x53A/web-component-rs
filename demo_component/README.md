### Build

wasm-pack build --target web

### Run

nix-shell -p python3

python3 -m http.server 8000

### All together

```sh
wasm-pack build --target web && nix-shell -p python3 --run "python3 -m http.server 8000"
```