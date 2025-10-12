use thiserror::Error;
use crate::types::coordinate::Coordinate;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum InterpreterError {
    #[error("Initialization error: {0}")]
    Initialization(#[from] InitError),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecError),

    #[error("System error: {0}")]
    System(#[from] SystemError),
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum InitError {
    #[error("No start symbol (@) found in program")]
    NoStartSymbol,

    #[error("Multiple start symbols (@) found in program")]
    MultipleStartSymbols,

    #[error("Invalid character '{0}' at position {1}")]
    InvalidCharacter(char, Coordinate),

    #[error("Grid size {0}x{1} exceeds maximum supported size of 1000x1000")]
    GridSizeExceeded(usize, usize),
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ExecError {
    #[error("Stack underflow: attempted to pop from empty stack")]
    StackUnderflow,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Modulo by zero")]
    ModuloByZero,

    #[error("Invalid memory access at {0}")]
    InvalidMemoryAccess(Coordinate),

    #[error("Subroutine underflow: attempted to return with empty call stack")]
    SubroutineUnderflow,

    #[error("Droplet collision at {0}")]
    DropletCollision(Coordinate),

    #[error("Execution timeout: exceeded maximum tick limit of {0}")]
    ExecutionTimeout(u64),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Invalid operation '{0}'")]
    InvalidOperation(char),
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SystemError {
    #[error("Out of memory")]
    OutOfMemory,

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<std::io::Error> for InterpreterError {
    fn from(error: std::io::Error) -> Self {
        InterpreterError::System(SystemError::IoError(error.to_string()))
    }
}

pub type Result<T> = std::result::Result<T, InterpreterError>;