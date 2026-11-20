use infer5090::{hello_gpu, request::token_count};

fn main() -> Result<(), String> {
    let tokens = vec![101, 42];
    println!("{}", token_count(&tokens));
    hello_gpu().map_err(|status| format!("hello kernel failed with CUDA error {status}"))
}
