//
// error.rs - Error types for sscript
//

use std::fmt;

#[derive(Debug, Clone)]
pub enum SscriptError {
    ProgramExit,
    InvalidChar(char),
    InvalidInstruction(String),
    StackUnderflow,
    ValueError(String),
    InvalidOperands(String),
    ProgramPanic(String),
    IOError(String),
    FileNotFound(String),
    FileNotCreatable(String),
    RoundParenthesisError,
    SquaredParenthesisError,
    StringQuotingError,
    CurlyParenthesisError,
    InvalidNameDefine(String),
    InjectError(Vec<SscriptError>),
}

impl fmt::Display for SscriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SscriptError::ProgramExit => write!(f, "Program exit"),
            SscriptError::InvalidChar(c) => write!(f, "Invalid character: '{}'", c),
            SscriptError::InvalidInstruction(s) => write!(f, "Invalid instruction: {}", s),
            SscriptError::StackUnderflow => write!(f, "Stack underflow"),
            SscriptError::ValueError(s) => write!(f, "Value error: {}", s),
            SscriptError::InvalidOperands(s) => write!(f, "Invalid operands: {}", s),
            SscriptError::ProgramPanic(s) => write!(f, "Program panic: {}", s),
            SscriptError::IOError(s) => write!(f, "I/O error: {}", s),
            SscriptError::FileNotFound(s) => write!(f, "File not found: {}", s),
            SscriptError::FileNotCreatable(s) => write!(f, "File not creatable: {}", s),
            SscriptError::RoundParenthesisError => write!(f, "Round parenthesis mismatch"),
            SscriptError::SquaredParenthesisError => write!(f, "Squared parenthesis mismatch"),
            SscriptError::StringQuotingError => write!(f, "String quoting marks mismatch"),
            SscriptError::CurlyParenthesisError => write!(f, "Curly parenthesis mismatch"),
            SscriptError::InvalidNameDefine(s) => write!(f, "Invalid name for define: {}", s),
            SscriptError::InjectError(errors) => {
                writeln!(f, "Inject failed:")?;
                for (i, e) in errors.iter().enumerate() {
                    writeln!(f, "  Stack {}: {}", i, e)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SscriptError {}

pub type Result<T> = std::result::Result<T, SscriptError>;
