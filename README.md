# Learning Birds

![screenshot](res/app-screenshot.png)

## Description
A simple genetic algorithm woven together with a neural network based upon a [tutorial](https://pwy.io/en/posts/learning-to-fly-pt1/) by [Patryk27](https://github.com/Patryk27/shorelark). Some major changes were made, including changing the backend neural network implementation to matrix designs and rebuilding the frontend in SolidJS.

## Dependencies
- [Rust](https://www.rust-lang.org/) for building & testing rust
  - Use Rust nightly for planned dependency on [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon). Although this ought to be set by the `rust-toolchain.toml` file.
- [Cargo Trunk](https://trunkrs.dev/) to serve the yew site on the front-end.

## Usage
Assuming you have the cargo toolchain installed and wasm-pack, running `npm run build` from the `./app` directory will build the project. Using `npm start` will bring the project up on `localhost:3000`. 

## TODO:
- [ ] Add RustDocs
- [ ] Integrate Rayon throw wasm-bindgen-rayon
- [ ] Add more tests to Rust and Typescript code
- [x] Handle resize event
- [ ] Refactor observers on the front-end to own their elements
- [x] Swap to Yew for testing