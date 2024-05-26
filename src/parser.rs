use core::fmt;

use crate::{
    expr::Expr,
    token::{Token, TokenType, Value},
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
    token: Token, // TODO: Not ideal...
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::EoF {
            write!(f, "{} at end {}", self.token.line, self.message)
        } else {
            write!(
                f,
                "{} at '{}' {}",
                self.token.line, self.token.lexeme, self.message
            )
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_token_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        // TODO: Refactor so that it can use normal Rust match
        // TODO: I think this TokenType actually makes thing confusing...
        use TokenType::*;

        if self.match_token_type(&[False]) {
            Ok(Expr::Literal {
                value: Value::Boolean(false),
            })
        } else if self.match_token_type(&[True]) {
            Ok(Expr::Literal {
                value: Value::Boolean(true),
            })
        } else if self.match_token_type(&[Nil]) {
            Ok(Expr::Literal { value: Value::Null })
        } else if self.match_token_type(&[Number, String]) {
            Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            })
        } else if self.match_token_type(&[LeftParen]) {
            let expr = self.expression()?;
            // TODO: I don't like how this is written
            self.consume(RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(ParseError {
                token: self.peek().clone(),
                message: "Expect expression.".to_string(),
            })
        }
    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    // NOTE: First target for refactor
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    // TODO: There should be a trait for these group of methods?
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EoF)
    }

    fn peek(&self) -> &Token {
        // TODO: A bit risky?
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        // TODO: A bit risky?
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.peek().clone(),
                message: message.to_string(),
            })
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            };

            use TokenType::*;
            match self.peek().token_type {
                Class => unimplemented!(),
                Fun => unimplemented!(),
                Var => unimplemented!(),
                For => unimplemented!(),
                If => unimplemented!(),
                While => unimplemented!(),
                Print => unimplemented!(),
                Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    // TODO: I'm sure there's some equivalent method with standard iterator trait
    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }
}
