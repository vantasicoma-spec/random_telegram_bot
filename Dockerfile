FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim as runtime

WORKDIR /app

# Устанавливаем необходимые библиотеки
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/random_roll_bot /app/random_roll_bot

# Добавляем скрипт для отладки
RUN echo '#!/bin/bash\n\
    echo "Starting bot..."\n\
    echo "Environment:"\n\
    printenv | grep -E "(TELEGRAM|RUST_LOG)"\n\
    echo "Current directory: $(pwd)"\n\
    echo "Files in current directory:"\n\
    ls -la\n\
    echo "Running binary..."\n\
    exec ./random_roll_bot' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

CMD ["/app/entrypoint.sh"]
