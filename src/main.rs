use infer5090::request::token_count;

fn main() {
    let tokens = vec![101, 42];
    println!("{}", token_count(&tokens));
}
