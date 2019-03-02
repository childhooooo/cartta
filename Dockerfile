FROM rustlang/rust:nightly

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install cargo-watch