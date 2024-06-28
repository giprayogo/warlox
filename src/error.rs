use std::error::Error;
use std::fmt;

type LineNumber = i32;

#[derive(Debug)]
pub enum RuntimeError {
    /// Unary operator taking non-number operand
    OperandNotNumber(LineNumber),
    /// Binary operator taking non-number operand
    OperandsNotNumbers(LineNumber),
    /// Plus operator taking non-number or string operand
    OperandsNotNumbersOrStrings(LineNumber),
    /// Division by zero
    DivideByZero(LineNumber),
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
                write!(f, "Division by zero [line {}]", line_number)
            }
        }
    }
}

impl Error for RuntimeError {}
