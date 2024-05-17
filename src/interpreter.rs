use crate::expr::{Expr, ExprVisitor};
use crate::token::{LoxValue, TokenType, TokenType::*};

pub struct Interpreter;

impl Interpreter {
    // TODO: Re-consider this visitor trait
    fn evaluate(&self, expr: &Expr) -> Option<LoxValue> {
        self.visit(expr)
    }

    fn is_truthy(&self, literal: Option<LoxValue>) -> bool {
        match literal {
            Some(literal) => match literal {
                LoxValue::Boolean(bool) => bool,
                LoxValue::String(_) => true,
                LoxValue::Double(_) => true,
            },
            None => false,
        }
    }

    fn is_equal(&self, a: Option<LoxValue>, b: Option<LoxValue>) -> bool {
        match (a, b) {
            (None, None) => true,
            (None, _) => false,
            (_, None) => false,
            (Some(LoxValue::Boolean(a)), Some(LoxValue::Boolean(b))) => a == b,
            (Some(LoxValue::String(a)), Some(LoxValue::String(b))) => a == b,
            (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => a == b,
            _ => false,
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Option<LoxValue>;

    fn visit(&self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Literal { value } => value.clone(), // TODO: consider not cloning
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

                match operator.token_type {
                    Minus => right.map(|x| match x {
                        LoxValue::Double(v) => LoxValue::Double(-v),
                        _ => unimplemented!(),
                    }),
                    Bang => Some(LoxValue::Boolean(self.is_truthy(right))),
                    _ => panic!("Unexpected expression: {expr:?}"),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // let left = self
                //     .evaluate(left)
                //     .expect("Unexpected nil value in binary expression.");
                // let right = self
                //     .evaluate(right)
                //     .expect("Unexpected nil value in binary expression.");
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                // Rust forces me to not blindly cast things!
                // TODO: generic-ize
                match operator.token_type {
                    Greater => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Boolean(a > b))
                        }
                        _ => panic!("Greater operator only works on Number values."),
                    },
                    GreaterEqual => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Boolean(a >= b))
                        }
                        _ => panic!("GreaterEqual operator only works on Number values."),
                    },
                    Less => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Boolean(a < b))
                        }
                        _ => panic!("Less operator only works on Number values."),
                    },
                    LessEqual => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Boolean(a <= b))
                        }
                        _ => panic!("LessEqual operator only works on Number values."),
                    },
                    Minus => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Double(a - b))
                        }
                        _ => panic!("Minus operator only works on Number values."),
                    },
                    Plus => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Double(a + b))
                        }
                        (Some(LoxValue::String(a)), Some(LoxValue::String(b))) => {
                            Some(LoxValue::String(a + &b))
                        }
                        _ => panic!("Plus operator only works on Number or String values."),
                    },
                    Slash => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Double(a / b))
                        }
                        _ => panic!("Slash operator only works on Number values."),
                    },
                    Star => match (left, right) {
                        (Some(LoxValue::Double(a)), Some(LoxValue::Double(b))) => {
                            Some(LoxValue::Double(a * b))
                        }
                        _ => panic!("Star operator only works on Number values."),
                    },
                    BangEqual => Some(LoxValue::Boolean(self.is_equal(left, right))),
                    EqualEqual => Some(LoxValue::Boolean(!self.is_equal(left, right))),
                    _ => panic!("Unexpected binary operator."),
                }
            }
        }
    }
}
