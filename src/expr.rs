use crate::token::{Token, Value};

pub trait ExprVisitor {
    type Output;

    fn visit_expr(&self, expr: &Expr) -> Self::Output {
        println!("{expr:?}");
        unimplemented!()
    }
}

// TODO: add new() implementation? I don't like specifying Box again and again.
// TODO: Does not need to be a box? The expression doesn't have to own the subexpressions, right?
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Comma {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
