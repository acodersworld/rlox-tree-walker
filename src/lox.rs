use crate::interpreter::Interpreter;
use crate::parser;
use crate::scanner;
use std::io;

pub fn lox_main(args: &[String]) {
    if args.len() > 0 {
        println!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), std::vec::Vec<String>> {
    let tokens = scanner::scan(source)?;
    let expr = parser::parse(&tokens)?;

    println!("{:#?}", interpreter.evaluate_expr(&expr));
    Ok(())
}

fn run_file(filename: &str) {
    //    run();
}

fn run_prompt() {
    let mut interpreter = Interpreter::new();

    let mut line = String::new();
    loop {
        eprint!(":> ");
        if let Err(_) = io::stdin().read_line(&mut line) {
            return;
        }

        if let Err(e) = run(&mut interpreter, &line) {
            eprintln!("Error: {}", e[0]);
        }

        line.clear();
    }
}
