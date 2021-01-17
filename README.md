# Shooter-rust-wasm

This is a port of [shooter-rust](https://github.com/msakuta/shooter-rust) to WebAssembly.

## Screenshots

![image](screenshots/screenshot01.jpg)


## Controls

* W, A, S, D - move
* Z, X - select weapon
* Space - shoot weapon


## History

I originally wrote this game in C back in 2007 with Windows API.

About a decade later, I re-implmented it in Rust and [Piston](https://github.com/PistonDevelopers/piston).

Now finally I could bring it to the web without converting
the codebase into JavaScript or TypeScript.


## Libraries

* wasm-bindgen

I use WebGL API directly, without any graphics libraries.
