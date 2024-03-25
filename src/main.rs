use std::env;
mod runtime;
mod lexer;
fn main() {
    let args: Vec<String> = env::args().collect();
    runtime::start_lovelace(&args);
}