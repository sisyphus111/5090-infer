FROM rust:1.100.0-slim-bookworm AS rust

FROM nvidia/cuda:13.0.3-devel-ubuntu24.04
COPY --from=rust /usr/local/cargo /usr/local/cargo
COPY --from=rust /usr/local/rustup /usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH="/usr/local/cargo/bin:${PATH}"

WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs ./
COPY kernels/ kernels/
COPY src/ src/
RUN cargo build --release --locked

CMD ["./target/release/infer5090"]
