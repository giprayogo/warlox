use core::fmt;

use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType, Value},
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, Copy)]
enum ParseErrorType {
    Colon,
    Expression,
    RightParen,
    LeftHandOperand,
    SemicolonAfterExpresssion,
    SemicolonAfterVarDeclaration,
    VarName,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    parse_error_type: ParseErrorType,
    token: Token,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::EoF {
            write!(f, "{} at end {}", self.token.line, self.parse_error_type)
        } else {
            write!(
                f,
                "{} at '{}' {}",
                self.token.line, self.token.lexeme, self.parse_error_type
            )
        }
    }
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            // TODO: Review variants; kinda repetitive.
            match self {
                ParseErrorType::Colon => "Expect ':' after expression.".to_string(),
                ParseErrorType::SemicolonAfterExpresssion =>
                    "Expect ';' after expression.".to_string(),
                ParseErrorType::SemicolonAfterVarDeclaration =>
                    "Expect ';' after variable declaration.".to_string(),
                ParseErrorType::Expression => "Expect expression.".to_string(),
                ParseErrorType::RightParen => "Expect ')' after expression.".to_string(),
                ParseErrorType::LeftHandOperand =>
                    "Missing binary operator left hand operand.".to_string(),
                ParseErrorType::VarName => "Expect variable name.".to_string(),
            }
        )
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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(v) = self.declaration() {
                statements.push(v);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let statement = if self.match_token_type(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match statement {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("{e}");
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, ParseErrorType::VarName)?
            .clone();

        let initializer = if self.match_token_type(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            ParseErrorType::SemicolonAfterVarDeclaration,
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_token_type(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorType::SemicolonAfterExpresssion,
        )?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorType::SemicolonAfterExpresssion,
        )?;
        Ok(Stmt::Expr { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr> {
        let mut expr = self.ternary()?;

        while self.match_token_type(&[TokenType::Comma]) {
            let right = self.ternary()?;
            expr = Expr::Comma {
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        if self.match_token_type(&[TokenType::QuestionMark]) {
            let left = self.ternary()?;
            self.consume(TokenType::Colon, ParseErrorType::Colon)?;
            let right = self.ternary()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let equality_tokens = [TokenType::BangEqual, TokenType::EqualEqual];
        if self.match_token_type(&equality_tokens) {
            self.comparison()?;
            return Err(ParseError {
                parse_error_type: ParseErrorType::LeftHandOperand,
                token: self.peek().clone(),
            });
        }

        let mut expr = self.comparison()?;

        while self.match_token_type(&equality_tokens) {
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
        let token_types = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        if self.match_token_type(&token_types) {
            self.comparison()?;
            return Err(ParseError {
                parse_error_type: ParseErrorType::LeftHandOperand,
                token: self.peek().clone(),
            });
        }

        let mut expr = self.term()?;

        while self.match_token_type(&token_types) {
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
        let token_types = [TokenType::Minus, TokenType::Plus];
        if self.match_token_type(&token_types) {
            self.comparison()?;
            return Err(ParseError {
                parse_error_type: ParseErrorType::LeftHandOperand,
                token: self.peek().clone(),
            });
        }

        let mut expr = self.factor()?;

        while self.match_token_type(&token_types) {
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
        let token_types = [TokenType::Slash, TokenType::Star];
        if self.match_token_type(&token_types) {
            self.comparison()?;
            return Err(ParseError {
                parse_error_type: ParseErrorType::LeftHandOperand,
                token: self.peek().clone(),
            });
        }

        let mut expr = self.unary()?;

        while self.match_token_type(&token_types) {
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
        } else if self.match_token_type(&[Identifier]) {
            Ok(Expr::Variable {
                name: self.previous().clone(),
            })
        } else if self.match_token_type(&[LeftParen]) {
            let expr = self.expression()?;
            // TODO: I don't like how this is written
            self.consume(RightParen, ParseErrorType::RightParen)?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(ParseError {
                parse_error_type: ParseErrorType::Expression,
                token: self.peek().clone(),
            })
        }
    }

    /// Check if the current token matches one of the token types, consuming it if true.
    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    // * NOTE: equivalent to peek() in PeekableIterator trait.
    /// Check if the current token matchees token type without advancing the iterator.
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    // * NOTE: equivalent to next() in Iterator trait.
    /// Advance the iterator, returning the current token.
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

    /// Get the next token without consuming it.
    fn peek(&self) -> &Token {
        // TODO: A bit risky?
        &self.tokens[self.current]
    }

    // * NOTE: the only reason why previous is required is because
    // * match_token_type advances the iterator... refactor!
    /// Get the previous token.
    fn previous(&self) -> &Token {
        // TODO: A bit risky?
        &self.tokens[self.current - 1]
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        parse_error_type: ParseErrorType,
    ) -> Result<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                parse_error_type,
                token: self.peek().clone(),
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
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
