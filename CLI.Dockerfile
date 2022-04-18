FROM rust:1.59.0

WORKDIR /usr/src/app

## Copy the dummy and the cargo files
RUN echo "fn main() {}" > dummy.rs
COPY ["Cargo.toml", "Cargo.lock", "./"]

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

## RUN cargo build --release

## docker-compose run rust
ENTRYPOINT cargo run --release