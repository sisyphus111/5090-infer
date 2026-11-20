#include <cuda_runtime.h>
#include <stdio.h>

__global__ void hello_kernel() {
    printf("hello from GPU\n");
}

extern "C" int launch_hello() {
    hello_kernel<<<1, 1>>>();
    const cudaError_t status = cudaGetLastError();
    if (status != cudaSuccess) {
        return status;
    }
    // Wait for device printf to flush before the Rust process exits.
    return cudaDeviceSynchronize();
}
