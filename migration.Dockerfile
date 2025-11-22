FROM rust:1.87-bullseye

WORKDIR /app

COPY ./migrations migrations

RUN cargo install --locked sqlx-cli --no-default-features --features postgres

CMD ["cargo", "sqlx", "migrate", "run"]
