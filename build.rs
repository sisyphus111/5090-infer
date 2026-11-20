fn main() {
    println!("cargo::rerun-if-changed=kernels");

    cc::Build::new()
        .cuda(true)
        .flag("-arch=sm_120")
        .file("kernels/hello.cu")
        .compile("infer_kernels");
}
