FROM rust:1.30.0

WORKDIR /app

RUN rustup override set nightly && \
    cargo install diesel_cli --no-default-features --features postgres && \
    cargo install cargo-watch
