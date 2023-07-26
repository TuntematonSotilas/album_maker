FROM rust:1.70.0 as builder
WORKDIR /app
COPY Cargo.toml ./
COPY Makefile.toml ./ 
COPY src/ src/
COPY public/ public/
#COPY .env .env
RUN cargo install cargo-make
RUN cargo install wasm-pack
RUN cargo make build_release

FROM nginx:1.23.1 as runner
WORKDIR /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
COPY index.html ./
COPY --from=builder /app/pkg/ pkg/ 
COPY public/ public/ 