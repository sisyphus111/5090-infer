FROM rust:1.100.0-slim-bookworm AS rust

FROM nvidia/cuda:13.0.3-devel-ubuntu24.04 AS dev
COPY --from=rust /usr/local/cargo /usr/local/cargo
COPY --from=rust /usr/local/rustup /usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH="/usr/local/cargo/bin:${PATH}"

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends gdb \
    && rm -rf /var/lib/apt/lists/*
# Allow Cargo to create its package cache when the shell runs as the host user.
RUN chmod 1777 "$CARGO_HOME"
WORKDIR /app
CMD ["bash"]

FROM dev AS build
COPY Cargo.toml Cargo.lock build.rs ./
COPY kernels/ kernels/
COPY src/ src/
RUN cargo build --release --locked

FROM nvidia/cuda:13.0.3-base-ubuntu24.04 AS runtime
COPY --from=build /app/target/release/infer5090 /usr/local/bin/infer5090
CMD ["/usr/local/bin/infer5090"]
