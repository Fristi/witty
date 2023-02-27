# witty

## Prerequisites

You need to install the following libraries:

- [Rust](https://rustup.rs/)

Don't forget to install `wasm32-unknown-unknown` target for rust.

## Running this demo 

```bash 
# build wasm component in 'app'
cargo b --release --target wasm32-unknown-unknown -p app

# load the component in 'runner'
cargo r -p runner
# note that `./target/component.wasm` was saved in runner.

```