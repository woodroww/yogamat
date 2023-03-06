# YogaMat

This is a reincarnation of an iPhone app I built in 2010.
I rewrote it in Rust using the Bevy game engine and built for wasm to deploy
on my GitHub Pages.

## build locally
```bash
cargo run --release
```
## build for wasm
### build the wasm-release profile defined in the Cargo.toml
```bash
cargo build --profile wasm-release --target wasm32-unknown-unknown
```
### use wasm-bindgen to build the webapp directory for serving on the web
```bash
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript ../target/wasm32-unknown-unknown/wasm-release/yoga_matt.wasm
```
### optionally run [wasm-opt](https://crates.io/crates/wasm-opt) to optimize the wasm file for size
In the webapp dir:
```bash
wasm-opt -Oz -o yoga_matt_bg.wasm yoga_matt_bg.wasm
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
