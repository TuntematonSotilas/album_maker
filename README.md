# album_maker

> Album Maker

## Install / check required tools

Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [cargo-make](https://sagiegurari.github.io/cargo-make/)

## Run

1. Open a new terminal and run: `cargo make serve`
1. Open a second terminal run: `cargo make watch`

## Lint

Run `cargo make verify` in your terminal to format and lint the code.

## Docker

1. Build : `docker build . -t tuntematonsotilas/gbt:amaker`
1. Run : `docker run -p 8080:80 tuntematonsotilas/gbt:amaker`
1. Test : http://localhost:8080
1. Push `docker push tuntematonsotilas/gbt:amaker`

## Deploy to Koyeb 
Initialize the App
```sh
koyeb app init amaker --docker "tuntematonsotilas/gbt:amaker" --ports 80:http --routes /:80 --docker-private-registry-secret docker-hub-credentials
```
Update Service
```sh
koyeb services list
koyeb services update 1b68cac4 --docker "tuntematonsotilas/gbt:amaker" --docker-private-registry-secret docker-hub-credentials
```

Koyeb CLI needs an authentication for pivate DockerHub : [koyeb doc](https://www.koyeb.com/docs/apps/private-container-registry-secrets#dockerhub)