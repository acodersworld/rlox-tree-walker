mod eval_value;
mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod token;
mod stmt;

use std::env;

fn main() {
    let args: std::vec::Vec<String> = env::args().collect();
    lox::lox_main(&args[1..]);
}
