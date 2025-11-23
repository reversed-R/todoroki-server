# builder
FROM rust:1.87-bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --bin todoroki-presentation --release

# runner
FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

COPY --from=builder /app/target/release/todoroki-presentation ./server

EXPOSE 8080

CMD ["./server"]
