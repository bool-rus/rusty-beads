cargo build --lib -r --target wasm32-unknown-unknown
cp -r site target/
wasm-bindgen target/wasm32-unknown-unknown/release/rusty_beads.wasm --out-dir target/site --no-modules --no-typescript
