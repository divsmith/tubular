use thiserror::Error;
use crate::types::coordinate::Coordinate;

/// Position information for error context
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub coordinate: Coordinate,
}

impl Position {
    pub fn new(line: usize, column: usize, coordinate: Coordinate) -> Self {
        Self { line, column, coordinate }
    }
}

/// Context information for errors
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorContext {
    pub position: Position,
    pub source_line: String,
    pub surrounding_lines: Vec<(usize, String)>,
    pub error_span: (usize, usize), // start and end column indices
}

impl ErrorContext {
    pub fn new(position: Position, source_line: String) -> Self {
        Self {
            position: position.clone(),
            source_line,
            surrounding_lines: Vec::new(),
            error_span: (position.column, position.column + 1),
        }
    }

    pub fn with_surrounding_lines(mut self, lines: Vec<(usize, String)>) -> Self {
        self.surrounding_lines = lines;
        self
    }

    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.error_span = (start, end);
        self
    }
}

/// Enhanced error information with context and suggestions
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorInfo {
    pub message: String,
    pub error_type: ErrorType,
    pub context: Option<ErrorContext>,
    pub suggestions: Vec<String>,
    pub help_text: Option<String>,
    pub severity: ErrorSeverity,
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl ErrorInfo {
    pub fn new(message: String, error_type: ErrorType) -> Self {
        Self {
            message,
            error_type,
            context: None,
            suggestions: Vec::new(),
            help_text: None,
            severity: ErrorSeverity::Error,
        }
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help_text = Some(help);
        self
    }

    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
}

/// Categories of errors for better organization
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Syntax,
    Validation,
    Initialization,
    Execution,
    Runtime,
    System,
    Semantic,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Syntax => write!(f, "Syntax Error"),
            ErrorType::Validation => write!(f, "Validation Error"),
            ErrorType::Initialization => write!(f, "Initialization Error"),
            ErrorType::Execution => write!(f, "Execution Error"),
            ErrorType::Runtime => write!(f, "Runtime Error"),
            ErrorType::System => write!(f, "System Error"),
            ErrorType::Semantic => write!(f, "Semantic Error"),
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum InterpreterError {
    #[error("Initialization error: {0}")]
    Initialization(#[from] InitError),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecError),

    #[error("System error: {0}")]
    System(#[from] SystemError),

    /// Enhanced error with context and suggestions
    #[error("{info}")]
    Enhanced {
        info: ErrorInfo,
        #[source]
        source: Option<Box<InterpreterError>>,
    },
}

impl InterpreterError {
    pub fn enhanced(message: String, error_type: ErrorType) -> Self {
        Self::Enhanced {
            info: ErrorInfo::new(message, error_type),
            source: None,
        }
    }

    pub fn with_context(self, context: ErrorContext) -> Self {
        match self {
            Self::Enhanced { mut info, source } => {
                info.context = Some(context);
                Self::Enhanced { info, source }
            }
            other => other,
        }
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        match &mut self {
            Self::Enhanced { info, .. } => {
                info.suggestions = suggestions;
            }
            _ => {}
        }
        self
    }

    pub fn with_help(mut self, help: String) -> Self {
        match &mut self {
            Self::Enhanced { info, .. } => {
                info.help_text = Some(help);
            }
            _ => {}
        }
        self
    }

    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        match &mut self {
            Self::Enhanced { info, .. } => {
                info.severity = severity;
            }
            _ => {}
        }
        self
    }

    pub fn error_type(&self) -> ErrorType {
        match self {
            Self::Initialization(_) => ErrorType::Initialization,
            Self::Execution(_) => ErrorType::Execution,
            Self::System(_) => ErrorType::System,
            Self::Enhanced { info, .. } => info.error_type.clone(),
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Enhanced { info, .. } => info.severity.clone(),
            _ => ErrorSeverity::Error,
        }
    }

    pub fn context(&self) -> Option<&ErrorContext> {
        match self {
            Self::Enhanced { info, .. } => info.context.as_ref(),
            _ => None,
        }
    }

    pub fn suggestions(&self) -> &[String] {
        match self {
            Self::Enhanced { info, .. } => &info.suggestions,
            _ => &[],
        }
    }

    pub fn help_text(&self) -> Option<&String> {
        match self {
            Self::Enhanced { info, .. } => info.help_text.as_ref(),
            _ => None,
        }
    }
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

    #[error("Wall-clock timeout: exceeded maximum execution time of {0}ms")]
    WallClockTimeout(u64),

    #[error("Soft tick limit warning: approaching maximum tick limit of {0}")]
    SoftTickLimitWarning(u64),

    #[error("Soft time limit warning: approaching maximum execution time of {0}ms")]
    SoftTimeLimitWarning(u64),

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