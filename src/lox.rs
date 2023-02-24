use std::io::Read;

use crate::environment::Environment;
use crate::interpreter::InterpreterContext;
use crate::parser;
use crate::scanner;

pub fn lox_main(args: &[String]) {
    if args.len() > 1 {
        println!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run(interpreter: &mut InterpreterContext, source: &str) -> Result<(), std::vec::Vec<String>> {
    let tokens = scanner::scan(source)?;
    let stmts = parser::parse(&tokens)?;

    if let Err(e) = interpreter.interpret(&stmts) {
        return Err(vec![e]);
    }
    Ok(())
}

fn run_file(filename: &str) {
    if let Ok(mut file) = std::fs::File::open(filename) {
        let mut buf = String::new();
        if let Err(e) = file.read_to_string(&mut buf) {
            eprintln!("Failed to read from file: {}", e);
            return;
        }

        let mut global_environment = Environment::new();
        let mut interpreter = InterpreterContext::new(&mut global_environment);
        if let Err(e) = run(&mut interpreter, &buf) {
            eprintln!("Error: {}", e[0]);
        }
    } else {
        eprintln!("Failed to open file '{}'", filename);
    }

    //    run();
}

fn run_prompt() {
    let mut global_environment = Environment::new();
    let mut interpreter = InterpreterContext::new(&mut global_environment);

    let mut line = String::new();
    loop {
        eprint!(":> ");
        if let Err(_) = std::io::stdin().read_line(&mut line) {
            return;
        }

        if let Err(e) = run(&mut interpreter, &line) {
            eprintln!("Error: {}", e[0]);
        }

        line.clear();
    }
}
