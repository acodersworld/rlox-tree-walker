use std::io::Read;

use crate::interpreter::Interpreter;
use crate::parser;
use crate::scanner;
use crate::stmt;

pub fn lox_main(args: &[String]) {
    if args.len() > 1 {
        println!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), std::vec::Vec<String>> {
    let tokens = scanner::scan(source)?;
    let stmts = parser::parse(&tokens)?;

    for st in stmts {
        match st {
            stmt::Stmt::Expr(expr) => println!("{:#?}", interpreter.evaluate_expr(&expr)),
            stmt::Stmt::Print(exprs) => {
                for expr in exprs {
                    match interpreter.evaluate_expr(&expr) {
                        Ok(value) => print!("{} ", value),
                        Err(e) => { return Err(vec![e]) }
                    }
                }
                println!("");
            }
        }
    }
    Ok(())
}

fn run_file(filename: &str) {
    if let Ok(mut file) = std::fs::File::open(filename) {
        let mut buf = String::new();
        if let Err(e) = file.read_to_string(&mut buf) {
            eprintln!("Failed to read from file: {}", e);
            return
        }

        let mut interpreter = Interpreter::new();
        if let Err(e) = run(&mut interpreter, &buf) {
            eprintln!("Error: {}", e[0]);
        }
    }
    else {
        eprintln!("Failed to open file '{}'", filename);
    }

    //    run();
}

fn run_prompt() {
    let mut interpreter = Interpreter::new();

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
