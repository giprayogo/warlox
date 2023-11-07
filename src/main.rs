use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fmt::Display;
use std::io::stdin;
use std::process::exit;
use std::{env, fs};
mod token_type;
use token_type::TokenType;

type ErrorVec = Vec<Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut lox = Lox::new();
    match args.len().cmp(&1) {
        Greater => {
            println!("Usage: warlox [script]");
            exit(64);
        }
        Equal => {
            lox.run_file(&args[0])?;
        }
        Less => {
            lox.run_prompt()?;
        }
    }
    Ok(())
}

struct Lox {
    had_error: bool,  // TODO: deprecate
    errors: ErrorVec, // TODO: If eventually shared, consider wrapping in Cell, RefCell, Mutex, etc.
}

impl Lox {
    fn new() -> Self {
        Self {
            had_error: false,
            errors: Vec::new(),
        }
    }
    fn run_file(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
        let string = fs::read_to_string(path)?;
        self.run(&string);
        Ok(())
    }
    fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        let mut line = String::new();
        loop {
            println!("> ");
            match stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => return Err(Box::new(e)),
            };
            self.run(&line);
        }
        Ok(())
    }
    fn run(&mut self, source: &String) {
        let mut scanner = Scanner::new(source, &mut self.errors);
        scanner.scan_tokens();

        for token in scanner {
            println!("{token}");
        }
    }
    fn error(&mut self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }
    fn report(&mut self, line: i32, wherein: String, message: String) {
        eprintln!("[line {line}] Error {wherein}: {message}");
        self.had_error = true.into();
    }
}

#[derive(Debug, Clone)]
struct UnexpectedCharacterError {
    line: i32,
}
impl Error for UnexpectedCharacterError {}
impl Display for UnexpectedCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character.")
    }
}

#[derive(Debug, Clone)]
struct UnterminatedStringError {
    line: i32,
}
impl Error for UnterminatedStringError {}
impl Display for UnterminatedStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character.")
    }
}

struct Scanner<'a> {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    errors: &'a mut ErrorVec,
}

impl<'a> IntoIterator for Scanner<'a> {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl<'a> Scanner<'a> {
    fn new(source: &String, errors: &'a mut ErrorVec) -> Self {
        println!("{source}");
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors,
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
            _ => {
                self.errors
                    .push(Box::new(UnexpectedCharacterError { line: self.line }));
            }
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
