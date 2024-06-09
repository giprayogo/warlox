use std::fmt::Display;

/// Struct for the Lox tokens.
// TODO: I don't like having all fields public...
#[derive(Debug, Clone)]
pub struct Token {
    // TODO: Maybe better as a trait.
    /// Type of current token.
    pub token_type: TokenType,
    /// Source string of the current token.
    pub lexeme: String,
    // TODO: Part of type? Why is this even here?
    /// Holds dynamic value in the interpreter.
    pub literal: Value,
    /// Line number of the current token in the source code.
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Value, line: i32) -> Self {
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
                Value::Null => "".into(),
                v => format!(" {v:?}"),
            },
            self.line,
            self.lexeme,
        )
    }
}

/// Values in Lox.
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Number(v) => write!(f, "{v}"),
            String(v) => write!(f, "{v}"),
            Boolean(v) => write!(f, "{v}"),
            Null => write!(f, "null"),
        }
    }
}

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    QuestionMark,
    Colon,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EoF,
}

// TODO: Learn more about this
#[derive(Debug)]
pub struct ParseTokenTypeError;

impl FromStr for TokenType {
    type Err = ParseTokenTypeError;

    // TODO: Literals should be possible here too!
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(Self::And),
            "class" => Ok(Self::Class),
            "else" => Ok(Self::Else),
            "false" => Ok(Self::False),
            "for" => Ok(Self::For),
            "fun" => Ok(Self::Fun),
            "if" => Ok(Self::If),
            "nil" => Ok(Self::Nil),
            "or" => Ok(Self::Or),
            "print" => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "this" => Ok(Self::This),
            "true" => Ok(Self::True),
            "var" => Ok(Self::Var),
            "while" => Ok(Self::While),
            _ => Err(ParseTokenTypeError),
        }
    }
}
