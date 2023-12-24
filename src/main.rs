use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::io::stdout;
use std::io::{stdin, Write};
use std::process::exit;
use std::sync::Mutex;
use std::{env, fs};
mod expr;
mod parser;
mod scanner;
mod token;
use expr::{AstPrinter, Expr};
use parser::Parser;
use scanner::Scanner;
use token::{LoxLiteral, Token, TokenType};

// TODO: Perhaps error should be its own module
static HAD_ERROR: Mutex<bool> = Mutex::new(false);

fn main() -> Result<(), Box<dyn Error>> {
    // Test expression printer
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal {
                value: Some(LoxLiteral::Double(123.0)),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(LoxLiteral::Double(45.67)),
            }),
        }),
    };
    println!("{}", AstPrinter.print(&expression));

    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.len().cmp(&1) {
        Greater => {
            println!("Usage: warlox [script]");
            exit(64);
        }
        Equal => {
            run_file(&args[0])?;
        }
        Less => {
            run_prompt()?;
        }
    }
    Ok(())
}

/// Load and interpret a Lox source code file
fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;
    run(&string);
    Ok(())
}

/// Run interactive prompt for the Lox interpreter
fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush()?;
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        run(&line);
        line.clear();
    }
    Ok(())
}

/// Token scanner loop for a single file or line (interactive)
fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = match parser.parse() {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!("{}", AstPrinter.print(&expression));
}

/// Rudimentary error reporting mechanism
fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

/// Rudimentary error reporting mechanism (actual printing part)
fn report(line: i32, wherein: String, message: String) {
    eprintln!("[line {line}] Error {wherein}: {message}");
    let mut had_error = HAD_ERROR.lock().expect("Unexpected mutex error");
    *had_error = false;
}
