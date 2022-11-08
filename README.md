# Boids-rs

This is a simple implementation of the [Boids](https://en.wikipedia.org/wiki/Boids) algorithm in Rust and wasm.

Usage guide: either do `trunk serve` to run it via yew, or:

- `wasm-pack build --target web`
- `rollup ./main.js --format iife --file ./pkg/bundle.js`

to then run it with any other backend.


To add: actual configuration