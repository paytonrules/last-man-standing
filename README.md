Note that you can test this with wasm-server-runner, which means changing values in your home directory.

See here:

https://github.com/jakobhellermann/wasm-server-runner

The command is cargo run --target wasm32-unknown-unknown. Then browse to http://localhost:1334/out/. For now anyway.

I also made a very basic output version in the out directory following the directions here:

https://bevy-cheatbook.github.io/platforms/wasm/webpage.html

I'll make this into a Makefile shortly.
The Makefile needs to run the complicated build command, and copy over the assets and html to the out directory.

TODO:
Let's try using trunk instead of using wasm-server-runner.
Follow https://bevy-cheatbook.github.io/platforms/wasm/gh-pages.html to deploy to github pages
- Just use `out` as the root directory
Figure out why you have 'target' and 'out'. Write that up here.
Write a little automation to do this, maybe in Make, maybe in rust/bin
Update this

# Using Trunk

1 - install trunk (`cargo install trunk --locked`)
2 - install wasm-bindgen (because I am an apple M1 user)
