FROM rust:1.100.0-slim-bookworm
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release --locked

CMD ["./target/release/infer5090"]
