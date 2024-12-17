# Last Man Standing

This is a very basic game I made for a game jam about 6 months ago. I'm finally getting it together so that I can do some more work on it. It uses Bevy and builds to web assembly for a web-based game.

## Requirements

+ Rust version 1.83
+ wasm32-unknown-unknown target

Currently this is on the stable toolchain. It uses Trunk to manage its WebAssembly build: https://trunkrs.dev/

### Using Trunk

1) Install trunk (`cargo install trunk --locked`)
1) Install wasm-bindgen (because I am an apple M1 user, this must be done manually)
1) `trunk serve` will serve the game and hotreload on any changes

## Deployment

Deploying to Github pages. TBD
