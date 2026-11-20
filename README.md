A from-scratch LLM inference engine, tuned for the RTX 5090.

## Quick start

Build the image from the repository root. Running it on an RTX 5090 requires a
CUDA 13.0-compatible NVIDIA driver and Docker configured with the NVIDIA
Container Toolkit.

```sh
docker build -f docker/Dockerfile -t 5090-infer:local .
docker run --rm --gpus all 5090-infer:local
```

During `cargo build`, `build.rs` compiles `kernels/hello.cu` with `nvcc` for
`sm_120` and links the resulting static library into `infer5090`. At runtime,
Rust prints the token count `2`, then a single GPU thread prints `hello from GPU`.
