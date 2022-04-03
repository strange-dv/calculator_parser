mod error;
mod expression;
mod operator;
mod parser;
mod token;
mod lexer;

use std::error::Error;
use std::io::Write;

fn eval(line: String) -> Result<(), Box<dyn Error>> {
    let tokens = lexer::lex(line)?;
    let mut token_iter = tokens.iter().peekable();
    let mut parser = parser::Parser::new(&mut token_iter);
    let result = parser.parse();
    match result {
        Ok(mut ast) => println!("{}", ast.eval()),
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

fn get_line() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_s) => {}
        Err(_e) => {}
    };

    input.trim().to_string()
}

fn run_repl() -> Result<(), Box<dyn Error>> {
    loop {
        let line = get_line();

        if line.eq("quit") {
            return Ok(());
        }
        if let Err(e) = eval(line) {
            println!("Error: {}", e);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    run_repl()
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}
