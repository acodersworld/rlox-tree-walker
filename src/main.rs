mod scanner;
mod token;
mod expr;
mod parser;
mod lox;

use std::env;

fn main() {
    let args: std::vec::Vec<String> = env::args().collect();
    lox::lox_main(&args[1..]);
}
