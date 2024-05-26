use crate::expr::{Expr, ExprVisitor};
use crate::token::{Token, TokenType::*, Value};

pub struct Interpreter;

type LineNumber = i32;

// TODO: Should they actually be grouped? Really?
pub enum RuntimeError {
    /// Unary operator taking non-number operand
    OperandNotNumber(LineNumber),
    /// Binary operator taking non-number operand
    OperandsNotNumbers(LineNumber),
    /// Plus operator taking non-number or string operand
    OperandsNotNumbersOrStrings(LineNumber),
}

impl Interpreter {
    // TODO: Re-consider this "visitor" pattern
    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        self.visit(expr)
    }

    pub fn interpret(expr: Expr) {}
}

// TODO: Directly return Value?
/// Lox definition of "truthy" value
fn is_truthy(literal: Value) -> bool {
    match literal {
        Value::Null => false,
        Value::Boolean(bool) => bool,
        Value::String(_) => true,
        Value::Number(_) => true,
    }
}

// TODO: Directly return Value?
/// Lox definition of "equal" value
fn is_equal(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Null, _) => false,
        (_, Value::Null) => false,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        _ => false,
    }
}

// TODO: Can I express that this expect an unary expression in the function signature?
/// Check if an unary operator's operand is number
fn check_number_operand(operator: &Token, operand: Value) -> Result<f64, RuntimeError> {
    match operand {
        Value::Number(v) => Ok(v),
        _ => Err(RuntimeError::OperandNotNumber(operator.line)),
    }
}

// TODO: Can I express that this expect a binary expression in the function signature?
/// Check if a binary operator's operands are numbers
fn check_number_operands(
    operator: &Token,
    left: Value,
    right: Value,
) -> Result<(f64, f64), RuntimeError> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok((left, right)),
        _ => Err(RuntimeError::OperandsNotNumbers(operator.line)),
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit(&self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Literal { value } => Ok(value.clone()), // TODO: consider not cloning
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                // Propagate error
                let right = match self.evaluate(right) {
                    Ok(v) => v,
                    e => return e,
                };

                match operator.token_type {
                    Minus => check_number_operand(operator, right).map(Value::Number),
                    Bang => Ok(Value::Boolean(is_truthy(right))),
                    _ => unreachable!(), // TODO: Can this be expressed by the type instead?
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // Propagate error
                let left = match self.evaluate(left) {
                    Ok(v) => v,
                    e => return e,
                };
                let right = match self.evaluate(right) {
                    Ok(v) => v,
                    e => return e,
                };

                match operator.token_type {
                    Greater => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left > right)),
                    GreaterEqual => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left >= right)),
                    Less => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left < right)),
                    LessEqual => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left <= right)),
                    Minus => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Number(left - right)),
                    Plus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left + right))
                        }
                        (Value::String(left), Value::String(right)) => {
                            Ok(Value::String(left + &right))
                        }
                        _ => Err(RuntimeError::OperandsNotNumbersOrStrings(operator.line)),
                    },
                    Slash => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Number(left / right)),
                    Star => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Number(left * right)),
                    BangEqual => Ok(Value::Boolean(is_equal(left, right))),
                    EqualEqual => Ok(Value::Boolean(!is_equal(left, right))),
                    _ => unreachable!(), // TODO: Can this be expressed by the type instead?
                }
            }
        }
    }
}
