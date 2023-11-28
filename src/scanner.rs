use crate::error;
use crate::token::{Literals, Token, TokenType};
use std::str::FromStr;

/// Struct for the source and current state of the scanner
/// TODO: I really want to use just standard iterator for this. perhaps later.
pub struct Scanner {
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
    pub fn new(source: &str) -> Self {
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
    pub fn scan_tokens(&mut self) {
        while !Scanner::is_at_end(self) {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens
            .push(Token::new(TokenType::EoF, "".to_string(), None, self.line));
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
            c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),
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
