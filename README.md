# album_maker

![Preview](public/preview.png)

## Install / check required tools

Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [cargo-make](https://sagiegurari.github.io/cargo-make/)

Add WASM Target : `rustup target add wasm32-unknown-unknown`

## Configure

Configure environment variables :

Copy the file `.env.example` to a new file named `.env` 
And set your variables in this file

## Run

1. Open a new terminal and run: `cargo make serve`
1. Open a second terminal run: `cargo make watch`

## Lint

Run `cargo make verify` in your terminal to format and lint the code.

## Docker

1. Build : `docker build . -t amaker` 
1. Run : `docker run -p 8080:80 amaker`
1. Test : http://localhost:8080
