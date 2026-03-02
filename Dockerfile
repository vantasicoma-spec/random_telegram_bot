FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim as runtime

WORKDIR /app

COPY --from=builder /app/target/release/random_roll_bot /app/random_roll_bot

CMD ["./random_roll_bot"]
