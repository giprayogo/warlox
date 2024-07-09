use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::expr::{Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::{Token, TokenType, Value};

pub struct Interpreter {
    environment: Environment,
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

impl Interpreter {
    // TODO: Re-consider this "visitor" pattern; it becomes weird.
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        self.visit_expr(expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.visit_stmt(stmt)
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => {}
                Err(e) => eprintln!("{e}"),
            }
            // }.map_err(|e| eprintln!("{e}"));
        }
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
        match stmt {
            Stmt::Expr { expression } => self.evaluate(expression).map(|_| {}),
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)?
                } else {
                    Value::Null
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Literal { value } => Ok(value.clone()), // TODO: Refactor to not clone.
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => check_number_operand(operator, right).map(Value::Number),
                    TokenType::Bang => Ok(Value::Boolean(is_truthy(right))),
                    _ => unreachable!(), // TODO: Can this be enforced by the type?
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Greater => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left > right)),
                    TokenType::GreaterEqual => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left >= right)),
                    TokenType::Less => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left < right)),
                    TokenType::LessEqual => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Boolean(left <= right)),
                    TokenType::Minus => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Number(left - right)),
                    TokenType::Plus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left + right))
                        }
                        (Value::String(left), Value::String(right)) => {
                            Ok(Value::String(left + &right))
                        }
                        // Reason: Chapter 7 Challenge 2
                        (Value::String(left), Value::Number(right)) => {
                            Ok(Value::String(left + &right.to_string()))
                        }
                        (Value::Number(left), Value::String(right)) => {
                            Ok(Value::String(left.to_string() + &right))
                        }
                        _ => Err(RuntimeError::OperandsNotNumbersOrStrings(operator.line)),
                    },
                    TokenType::Slash => {
                        match check_number_operands(operator, left, right)
                            .map(|(left, right)| left / right)
                        {
                            Ok(v) => {
                                // Reason: Chapter 7 Challenge 3
                                if v.is_infinite() {
                                    Err(RuntimeError::DivideByZero(operator.line))
                                } else {
                                    Ok(Value::Number(v))
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Star => check_number_operands(operator, left, right)
                        .map(|(left, right)| Value::Number(left * right)),
                    TokenType::BangEqual => Ok(Value::Boolean(!is_equal(left, right))),
                    TokenType::EqualEqual => Ok(Value::Boolean(is_equal(left, right))),
                    _ => unreachable!(), // TODO: Can this be expressed by the type instead?
                }
            }
            Expr::Comma { left, right } => {
                self.evaluate(left)?;
                self.evaluate(right)
            }
            Expr::Ternary {
                condition,
                left,
                right,
            } => {
                if is_truthy(self.evaluate(condition)?) {
                    self.evaluate(left)
                } else {
                    self.evaluate(right)
                }
            }
            Expr::Variable { name } => self.environment.get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, value)
            }
        }
    }
}
