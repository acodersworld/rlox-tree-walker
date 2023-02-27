mod environment;
mod eval_value;
mod expr;
mod interpreter;
mod resolver;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token;

use std::{env, vec::Vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    lox::lox_main(&args[1..]);
}
