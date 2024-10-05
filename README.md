Note that you can test this with wasm-server-runner, which means changing values in your home directory.

See here:

https://github.com/jakobhellermann/wasm-server-runner

The command is cargo run --target wasm32-unknown-unknown. Then browse to http://localhost:1334/out/. For now anyway.

I also made a very basic output version in the out directory following the directions here:

https://bevy-cheatbook.github.io/platforms/wasm/webpage.html

I'll make this into a Makefile shortly.
The Makefile needs to run the complicated build command, and copy over the assets and html to the out directory.
