# Shooter-rust-wasm

This is a port of [shooter-rust](https://github.com/msakuta/shooter-rust) to WebAssembly.

Try it now in your browser! https://msakuta.github.io/shooter-rust-wasm/

## Screenshots

![image](screenshots/screenshot01.jpg)


## Controls

* Arrow keys, W, A, S, D - move
* Z, X - select weapon
* Space - shoot weapon
* P - toggle pause game


# Building

## Prerequisites

This game uses JavaScript and WebAssembly (Wasm), so you need a browser with WebAssembly support.
Most modern browser support it nowadays.



## How to build and run

Install

* Cargo >1.40
* npm >7.0.2

Install npm packages

    npm i

### Launch development server

    npm start

It will start webpack-dev-server, launch a browser and show http://localhost:8080 automatically.

### Launch production distribution

    npm run build

# History

I originally wrote this game in C back in 2007 with Windows API.

About a decade later, I re-implmented it in Rust and [Piston](https://github.com/PistonDevelopers/piston).

Now finally I could bring it to the web without converting
the codebase into JavaScript or TypeScript.


# Libraries

* wasm-bindgen

I use WebGL API directly, without any graphics libraries.
