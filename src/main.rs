use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fmt::Display;
use std::io::stdout;
use std::io::{stdin, Write};
use std::process::exit;
use std::str::FromStr;
use std::sync::Mutex;
use std::{env, fs};
mod token_type;
use token_type::TokenType;

// TODO: Perhaps error should be its own module
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
    scanner.scan_tokens();
    for token in scanner {
        println!("{token}");
    }
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

/// Struct for the source and current state of the scanner
/// TODO: I really want to use just standard iterator for this. perhaps later.
struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

/// TODO: Eventually more extensive
impl IntoIterator for Scanner {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Scanner {
    /// Constructor
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Read whole file or line (interactive) into list of tokens
    /// TODO: I don't want to unnecessarily eat memory?
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
    }

    /// If we are at the end of token. TODO: Should use standard safe structure?
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Consume one or more characters to output a single token. TODO: Iterator interface
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
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                };
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                };
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                };
            }
            '/' => {
                if self.match_char('/') {
                    // TODO: Use builtin
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            // TODO: instead match general whitespace characters
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            // TODO: Later support full unicode as identifier??
            c if c.is_ascii_alphanumeric() => self.identifier(),
            _ => error(self.line, "Unexpected character.".into()),
        }
    }

    /// Advance character iterator returning a string. TODO: Actually make into an iterator
    fn advance(&mut self) -> char {
        // TODO: causes panic with unterminated string literal
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    /// Push a single token into the internal collection. TODO: I would like a dedicated collection type for tokens which take care of this?
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    /// Push a token with literal value
    fn add_token_literal(&mut self, token_type: TokenType, literal_value: Option<Literals>) {
        // TODO: I think better integration with iterator type is possible
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal_value, self.line))
    }

    /// Test whether the next character matches given one, conditionally advancing the iterator if so.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Peek the next character without advancing the iterator. TODO: There's a standard Rust facility for this
    fn peek(&self) -> char {
        // TODO: Use builtin
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Consume a string of characters producing a string literal token
    fn string(&mut self) {
        // TODO: I should be able to consume and move at the same time with iterator?
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        // TODO: Check the indexing
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        // TODO: I don't think I need these encapsulations
        self.add_token_literal(TokenType::String, Some(Literals::String(value)))
    }

    /// Consume a string of characters producing a number literal token
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            // Consume digits after period
            // TODO: Can be generalized with above??/
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token_literal(
            TokenType::Number,
            // TODO: Ugly
            Some(Literals::Double(
                f64::from_str(
                    &(self.source[self.start..self.current]
                        .iter()
                        .collect::<String>()),
                )
                .unwrap(),
            )),
        )
    }

    /// Two characters lookahead
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// Consume a string of characters producing identifier or reserver keywords
    fn identifier(&mut self) {
        // TODO: consider supporting full unicode
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        // TODO: This pattern appear several time; fx-ize?
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = match TokenType::from_str(&text) {
            Ok(v) => v,
            Err(_) => TokenType::Identifier,
        };
        self.add_token(token_type);
    }
}

/// Struct for the Lox tokens. TODO: given Rust's enum it should be possible to join them
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literals>,
    line: i32,
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{}, ln {} \"{}\"",
            self.token_type,
            match &self.literal {
                Some(v) => format!(" {v:?}"),
                None => "".into(),
            },
            self.line,
            self.lexeme,
        )
    }
}

/// Struct with container for the literal token's.
// TODO: Utilize Rust's enum fully for these. i.e. can be joined with TokenType
#[derive(Debug)]
enum Literals {
    String(String),
    Double(f64),
}
