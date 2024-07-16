use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    /// Unary operator taking non-number operand
    OperandNotNumber(i32),
    /// Binary operator taking non-number operand
    OperandsNotNumbers(i32),
    /// Plus operator taking non-number or string operand
    OperandsNotNumbersOrStrings(i32),
    /// Division by zero
    DivideByZero(i32),
    /// Variable is not defined
    UndefinedVariable(String, i32),
    /// Variable is not initialized
    UninitializedVariable(String, i32),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperandNotNumber(line_number) => {
                write!(f, "Operand must be a number.\n[line {}]", line_number)
            }
            Self::OperandsNotNumbers(line_number) => {
                write!(f, "Operands must be numbers.\n[line {}]", line_number)
            }
            Self::OperandsNotNumbersOrStrings(line_number) => {
                write!(
                    f,
                    "Operands must be two numbers or two strings.\n[line {}]",
                    line_number
                )
            }
            Self::DivideByZero(line_number) => {
                write!(f, "Division by zero\n[line {}]", line_number)
            }
            Self::UndefinedVariable(name, line_number) => {
                write!(f, "Undefined variable {}.\n[line {}]", name, line_number)
            }
            Self::UninitializedVariable(name, line_number) => {
                write!(
                    f,
                    "Variable {name} has not been initialized.\n[line {line_number}]`"
                )
            }
        }
    }
}

impl Error for RuntimeError {}
