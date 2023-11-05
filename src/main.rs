use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fmt::Display;
use std::io::stdin;
use std::process::exit;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match args.len().cmp(&1) {
        Greater => {
            println!("Usage: warlox [script]");
            exit(64);
        }
        Equal => {
            Lox::run_file(&args[0])?;
        }
        Less => {
            Lox::run_prompt()?;
        }
    }
    Ok(())
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
        let string = fs::read_to_string(path)?;
        Lox::run(&string);
        Ok(())
    }
    fn run_prompt() -> Result<(), Box<dyn Error>> {
        let mut line = String::new();
        loop {
            println!("> ");
            match stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => return Err(Box::new(e)),
            };
            Lox::run(&line);
        }
        Ok(())
    }
    fn run(source: &String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token}");
        }
    }
    fn error(&mut self, line: i32, message: String) {
        Lox::report(self, line, "".to_string(), message);
    }
    fn report(&mut self, line: i32, wherein: String, message: String) {
        eprintln!("[line {line}] Error {wherein}: {message}");
        self.had_error = true;
    }
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn new(source: &String) -> Self {
        println!("{source}");
        return Self {
            source: source.clone(),
        };
    }
    fn scan_tokens(&self) -> Vec<Token> {
        vec![Token]
    }
}

struct Token;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not implemented")
    }
}
