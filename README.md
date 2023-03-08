# YogaMat

This is a reincarnation of an iPhone app I built in 2010.
I rewrote it in Rust using the Bevy game engine and built for wasm to deploy
on my GitHub Pages.

## Build locally
```bash
cargo run --release
```
## Build for WASM
### Add the target to the environment. This is a one time thing per rust installation.
```bash
rustup target add wasm32-unknown-unknown
```
### Build the wasm-release profile defined in the Cargo.toml.
```bash
cargo build --profile wasm-release --target wasm32-unknown-unknown
```
### Use wasm-bindgen to build the webapp directory for serving on the web.
[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
#### Install
```bash
cargo install -f wasm-bindgen-cli
```
#### Run
```bash
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript target/wasm32-unknown-unknown/wasm-release/yogamat.wasm
```
### Right click
After running the above wasm-bindgen command:
Either you have to disable the right click context menu in the browser to use the default key bindings or change the bindings in camera.rs.
To disable right click this needs to be put in yogamat.js at the beginning of the init function.
```javascript
// async function init(input) {
    document.addEventListener("contextmenu", function (e){
        e.preventDefault();
    }, false);
    // ...
```
### Optionally run [wasm-opt](https://crates.io/crates/wasm-opt) to optimize the wasm file for size
[wasm-opt](https://crates.io/crates/wasm-opt)

Rust bindings for [Binaryen's c++ toolkit's](https://github.com/WebAssembly/binaryen) wasm-opt
to optimize the wasm even further past any optimizations in Cargo.toml `[profile]`
#### install
```bash
cargo install wasm-opt --locked
```
#### run
In the webapp dir:
(This takes my wasm size from 12M to 8.7M.)
```bash
wasm-opt -Oz -o yogamat_bg.wasm yogamat_bg.wasm
```
## Run WASM
### If you don't have a server
[simple-http-server](https://crates.io/crates/simple-http-server)
```bash
cargo install simple-http-server
```
#### python server won't work
python3 -m http.server

### Serve it
In the webapp dir:
```bash
basic-http-server .
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
