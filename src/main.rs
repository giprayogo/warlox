use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fmt::Display;
use std::io::stdin;
use std::process::exit;
use std::sync::Mutex;
use std::{env, fs};
mod token_type;
use token_type::TokenType;

static HAD_ERROR: Mutex<bool> = Mutex::new(false);

fn main() -> Result<(), Box<dyn Error>> {
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

fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;
    run(&string);
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
        run(&line);
    }
    Ok(())
}

fn run(source: &String) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    for token in scanner {
        println!("{token}");
    }
}

fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, wherein: String, message: String) {
    eprintln!("[line {line}] Error {wherein}: {message}");
    let mut had_error = HAD_ERROR.lock().expect("Unexpected mutex error");
    *had_error = false;
}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl IntoIterator for Scanner {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Scanner {
    fn new(source: &String) -> Self {
        println!("{source}");
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) {
        while !Scanner::is_at_end(self) {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(Token {
            token_type: TokenType::EoF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        // vec![Token::new(TokenType::EoF, "".to_string(), None, 0)]
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() // TODO: probably not efficient
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => error(self.line, "Unexpected character.".into()),
        }
    }
    // TODO: I can use iterator?
    fn advance(&mut self) -> char {
        // TODO: Not safe, find alternative
        let char = self.source[self.current];
        self.current += 1;
        char
    }
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current]
            .iter()
            .collect::<String>(); // TODO: check
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal: None,
            line: self.line,
        })
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literals>,
    line: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix printing of literal
        write!(f, "{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Literals>, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug)]
enum Literals {
    String(String),
    Int(i32),
}

impl Display for Literals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Literal (Not implemented)")
    }
}
