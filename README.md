A from-scratch LLM inference engine, tuned for the RTX 5090.

## Quick start

Build the default `runtime` image from the repository root. Running it on an RTX
5090 requires a CUDA 13.0-compatible NVIDIA driver and Docker configured with
the NVIDIA Container Toolkit.

```sh
docker build -f docker/Dockerfile -t 5090-infer:local .
docker run --rm --gpus all 5090-infer:local
```

During `cargo build`, `build.rs` compiles `kernels/hello.cu` with `nvcc` for
`sm_120` and links the resulting static library into `infer5090`. At runtime,
Rust prints the token count `2`, then a single GPU thread prints `hello from GPU`.

## Develop with Docker

Build the `dev` target and open a shell with your checkout mounted at `/app`.
The container uses your user and group IDs when writing to the checkout.

```sh
docker build -f docker/Dockerfile --target dev -t 5090-infer:dev .
docker run --rm -it --gpus all --user "$(id -u):$(id -g)" \
  --mount "type=bind,source=$PWD,target=/app" 5090-infer:dev
```

Run `cargo run --locked` in the shell after editing Rust or CUDA sources on the
host. Cargo writes the debug binary to `target/debug/infer5090`.

Use `rust-gdb target/debug/infer5090` to debug Rust code. To stop in the GPU
kernel, open `cuda-gdb target/debug/infer5090`, then enter `break hello_kernel`
and `run`.
