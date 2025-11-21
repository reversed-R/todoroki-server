# builder
FROM rust:1.87-bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --bin todoroki-presentation --release

# runner
FROM rust:1.87-bullseye

WORKDIR /app

RUN cargo install --locked sqlx-cli --no-default-features --features postgres

COPY --from=builder /app/target/release/todoroki-presentation ./server

CMD cargo sqlx migrate run && ./server
