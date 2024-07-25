use core::fmt;

use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType, Value},
};

// TODO: Revise to something simpler.
type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, Copy)]
enum ParseErrorType {
    ExpectColon,
    ExpectExpression,
    ExpectRightParen,
    MissingLeftHandOperand,
    ExpectSemicolonAfterExpresssion,
    ExpectSemicolonAfterVarDeclaration,
    ExpectVarName,
    InvalidAssignment,
    ExpectRightBraceAfterBlock,
    ExpectLeftParenAfterIf,
    ExpectRightParenAfterIfCondition,
    ExpectLeftParenAfterWhile,
    ExpectRightParenAfterCondition,
    ExpectLeftParenAfterFor,
    ExpectSemicolonAfterLoopCondition,
    ExpectRightParenAfterForClauses,
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
        use ParseErrorType::*;
        write!(
            f,
            "{}",
            // TODO: Review variants; kinda repetitive.
            match self {
                ExpectColon => "Expect ':' after expression.".to_string(),
                ExpectSemicolonAfterExpresssion => "Expect ';' after expression.".to_string(),
                ExpectSemicolonAfterVarDeclaration =>
                    "Expect ';' after variable declaration.".to_string(),
                ExpectExpression => "Expect expression.".to_string(),
                ExpectRightParen => "Expect ')' after expression.".to_string(),
                MissingLeftHandOperand => "Missing binary operator left hand operand.".to_string(),
                ExpectVarName => "Expect variable name.".to_string(),
                InvalidAssignment => "Invalid assignment target.".to_string(),
                ExpectRightBraceAfterBlock => "Expect '}' after block.".to_string(),
                ExpectLeftParenAfterIf => "Expect '(' after if.".to_string(),
                ExpectRightParenAfterIfCondition => "Expect ')' after if condition.".to_string(),
                ExpectLeftParenAfterWhile => "Expect '(' after while.".to_string(),
                ExpectRightParenAfterCondition => "Expect ')' after condition.".to_string(),
                ExpectLeftParenAfterFor => "Expect '(' after for.".to_string(),
                ExpectSemicolonAfterLoopCondition => "Expect ';' after loop condition.".to_string(),
                ExpectRightParenAfterForClauses => "Expect ')' after for clauses.".to_string(),
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
        // TODO: Personally I'd rather not catch this within the parser; throw to main?
        match statement {
            Ok(v) => Some(v),
            Err(e) => {
                // Not at parse(), to allow block continue with invalid statements?
                // For what purpose though?
                eprintln!("{e}");
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, ParseErrorType::ExpectVarName)?
            .clone();

        let initializer = if self.match_token_type(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            ParseErrorType::ExpectSemicolonAfterVarDeclaration,
        )?;
        Ok(Stmt::VarDecl { name, initializer })
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(
            TokenType::LeftParen,
            ParseErrorType::ExpectLeftParenAfterWhile,
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParseErrorType::ExpectRightParenAfterCondition,
        )?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn statement(&mut self) -> Result<Stmt> {
        use TokenType::*;
        if self.match_token_type(&[For]) {
            self.for_statement()
        } else if self.match_token_type(&[If]) {
            self.if_statement()
        } else if self.match_token_type(&[Print]) {
            self.print_statement()
        } else if self.match_token_type(&[While]) {
            self.while_statement()
        } else if self.match_token_type(&[LeftBrace]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    /// Add support for for statement as syntax sugar at parser level.
    fn for_statement(&mut self) -> Result<Stmt> {
        use TokenType::*;
        self.consume(LeftParen, ParseErrorType::ExpectLeftParenAfterFor)?;

        let initializer = if self.match_token_type(&[Semicolon]) {
            None
        } else if self.match_token_type(&[Var]) {
            Some(self.var_declaration()?) // This consumes semicolon
        } else {
            Some(self.expression_statement()?) // This too
        };

        // Don't miss the NOT
        // Also check does NOT advance the iterator
        // No condition == while true == infinite loop
        let condition = if !self.check(&Semicolon) {
            self.expression()?
        } else {
            Expr::Literal {
                value: Value::Boolean(true),
            }
        };
        // NOTE: Rather than this, shouldn't I be able to match expression statement?
        self.consume(Semicolon, ParseErrorType::ExpectSemicolonAfterLoopCondition)?;

        // Don't miss the NOT
        // Also check does NOT advance the iterator
        let increment = if !self.check(&RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(RightParen, ParseErrorType::ExpectRightParenAfterForClauses)?;
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            // Append increment to the body statement.
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            }
        }

        // Desugar as a while loop.
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            }
        }

        Ok(body)
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut statements = Vec::new();

        use TokenType::*;
        while !self.check(&RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.declaration() {
                statements.push(statement);
            }
        }

        self.consume(RightBrace, ParseErrorType::ExpectRightBraceAfterBlock)?;
        Ok(Stmt::Block { statements })
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        use TokenType::*;
        self.consume(LeftParen, ParseErrorType::ExpectLeftParenAfterIf)?;
        let expr = self.expression()?;
        self.consume(RightParen, ParseErrorType::ExpectRightParenAfterIfCondition)?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token_type(&[Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition: expr,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorType::ExpectSemicolonAfterExpresssion,
        )?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorType::ExpectSemicolonAfterExpresssion,
        )?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr> {
        let mut expr = self.assignment()?;

        while self.match_token_type(&[TokenType::Comma]) {
            let right = self.assignment()?;
            expr = Expr::Comma {
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.ternary()?;

        if self.match_token_type(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            // Right associative
            let value = self.ternary()?;

            use Expr::*;
            match expr {
                Variable { token: name } => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => Err(ParseError {
                    parse_error_type: ParseErrorType::InvalidAssignment,
                    token: equals,
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn ternary(&mut self) -> Result<Expr> {
        let mut expr = self.or()?;

        if self.match_token_type(&[TokenType::QuestionMark]) {
            let left = self.expression()?;
            self.consume(TokenType::Colon, ParseErrorType::ExpectColon)?;
            let right = self.ternary()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.match_token_type(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = Box::new(self.and()?);
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_token_type(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        use TokenType::*;
        while self.match_token_type(&[BangEqual, EqualEqual]) {
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

        use TokenType::*;
        while self.match_token_type(&[Greater, GreaterEqual, Less, LessEqual]) {
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

        use TokenType::*;
        while self.match_token_type(&[Minus, Plus]) {
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

        use TokenType::*;
        while self.match_token_type(&[Slash, Star]) {
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
                token: self.previous().clone(),
            })
        } else if self.match_token_type(&[LeftParen]) {
            let expr = self.expression()?;
            // TODO: I don't like how this is written
            self.consume(RightParen, ParseErrorType::ExpectRightParen)?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else if self.match_token_type(&[
            Bang,
            Minus,
            Slash,
            Star,
            Minus,
            Plus,
            Greater,
            GreaterEqual,
            Less,
            LessEqual,
            BangEqual,
            EqualEqual,
        ]) {
            let operator = self.previous().clone();
            // Continue parsing right hand side.
            self.equality()?;
            Err(ParseError {
                parse_error_type: ParseErrorType::MissingLeftHandOperand,
                token: operator,
            })
        } else {
            Err(ParseError {
                parse_error_type: ParseErrorType::ExpectExpression,
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
