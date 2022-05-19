FROM rust:1.60 as builder
WORKDIR /app
COPY Cargo.toml ./
COPY Makefile.toml ./ 
COPY src/ src/
COPY public/ public/
RUN cargo install cargo-make
RUN cargo install wasm-pack --version 0.10.2
RUN cargo make build_release

FROM nginx:1.21.6 as runner
WORKDIR /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
COPY index.html ./
COPY pkg/ pkg/ 
COPY public/ public/ 