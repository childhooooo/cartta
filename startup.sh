diesel setup && \
export ROCKET_DATABASES="{postgres_database={url=\"${DATABASE_URL}\"}}" && \
export ROCKET_PORT=${PORT} && \
export ROCKET_ENV=production && \
cargo run --release