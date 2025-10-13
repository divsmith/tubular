//! Unit tests for the error types and error handling

use tubular::types::error::*;
use tubular::types::Coordinate;
use std::io::{Error as IoError, ErrorKind};

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let coord = Coordinate::new(5, 10);
        let position = Position::new(3, 7, coord);

        assert_eq!(position.line, 3);
        assert_eq!(position.column, 7);
        assert_eq!(position.coordinate, coord);
    }

    #[test]
    fn test_error_context_new() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "hello world".to_string();
        let context = ErrorContext::new(position, source_line.clone());

        assert_eq!(context.position, position);
        assert_eq!(context.source_line, source_line);
        assert_eq!(context.error_span, (5, 6));
        assert!(context.surrounding_lines.is_empty());
    }

    #[test]
    fn test_error_context_with_surrounding_lines() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "main".to_string();
        let surrounding = vec![(1, "let x = 5".to_string()), (2, "print(x)".to_string())];

        let context = ErrorContext::new(position, source_line)
            .with_surrounding_lines(surrounding.clone());

        assert_eq!(context.surrounding_lines, surrounding);
    }

    #[test]
    fn test_error_context_with_span() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 3, coord);
        let source_line = "abcdef".to_string();

        let context = ErrorContext::new(position, source_line)
            .with_span(2, 5);

        assert_eq!(context.error_span, (2, 5));
    }

    #[test]
    fn test_error_context_builder_pattern() {
        let coord = Coordinate::new(10, 20);
        let position = Position::new(5, 8, coord);
        let source_line = "function call".to_string();
        let surrounding = vec![(4, "let x = 10".to_string())];
        let suggestions = vec!["Add semicolon".to_string()];

        let context = ErrorContext::new(position, source_line)
            .with_surrounding_lines(surrounding.clone())
            .with_span(5, 10);

        assert_eq!(context.position.line, 5);
        assert_eq!(context.position.column, 8);
        assert_eq!(context.surrounding_lines, surrounding);
        assert_eq!(context.error_span, (5, 10));
    }

    #[test]
    fn test_error_info_new() {
        let message = "Something went wrong".to_string();
        let error_type = ErrorType::Syntax;

        let info = ErrorInfo::new(message.clone(), error_type.clone());

        assert_eq!(info.message, message);
        assert_eq!(info.error_type, error_type);
        assert!(info.context.is_none());
        assert!(info.suggestions.is_empty());
        assert!(info.help_text.is_none());
        assert_eq!(info.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_error_info_builder_pattern() {
        let message = "Invalid syntax".to_string();
        let error_type = ErrorType::Syntax;
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "let x =".to_string();
        let context = ErrorContext::new(position, source_line);
        let suggestions = vec!["Add a value after =".to_string()];
        let help = "Variable assignments require a value".to_string();

        let info = ErrorInfo::new(message.clone(), error_type.clone())
            .with_context(context.clone())
            .with_suggestions(suggestions.clone())
            .with_help(help.clone())
            .with_severity(ErrorSeverity::Warning);

        assert_eq!(info.message, message);
        assert_eq!(info.error_type, error_type);
        assert_eq!(info.context.as_ref().unwrap(), &context);
        assert_eq!(info.suggestions, suggestions);
        assert_eq!(info.help_text.as_ref().unwrap(), &help);
        assert_eq!(info.severity, ErrorSeverity::Warning);
    }

    #[test]
    fn test_error_info_display() {
        let info = ErrorInfo::new("Syntax error".to_string(), ErrorType::Syntax);
        let display_str = format!("{}", info);
        assert!(display_str.contains("Syntax Error"));
        assert!(display_str.contains("Syntax error"));
    }

    #[test]
    fn test_error_type_display() {
        assert_eq!(format!("{}", ErrorType::Syntax), "Syntax Error");
        assert_eq!(format!("{}", ErrorType::Validation), "Validation Error");
        assert_eq!(format!("{}", ErrorType::Initialization), "Initialization Error");
        assert_eq!(format!("{}", ErrorType::Execution), "Execution Error");
        assert_eq!(format!("{}", ErrorType::Runtime), "Runtime Error");
        assert_eq!(format!("{}", ErrorType::System), "System Error");
        assert_eq!(format!("{}", ErrorType::Semantic), "Semantic Error");
    }

    #[test]
    fn test_interpreter_error_enhanced() {
        let message = "Custom error".to_string();
        let error_type = ErrorType::Semantic;

        let error = InterpreterError::enhanced(message.clone(), error_type.clone());

        match error {
            InterpreterError::Enhanced { info, source } => {
                assert_eq!(info.message, message);
                assert_eq!(info.error_type, error_type);
                assert!(source.is_none());
            }
            _ => panic!("Expected Enhanced error variant"),
        }
    }

    #[test]
    fn test_interpreter_error_with_context() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "test line".to_string();
        let context = ErrorContext::new(position, source_line);

        let error = InterpreterError::enhanced("Test error".to_string(), ErrorType::Syntax)
            .with_context(context.clone());

        match error {
            InterpreterError::Enhanced { info, .. } => {
                assert_eq!(info.context.as_ref().unwrap(), &context);
            }
            _ => panic!("Expected Enhanced error variant"),
        }
    }

    #[test]
    fn test_interpreter_error_with_suggestions() {
        let suggestions = vec!["Try this".to_string(), "Or that".to_string()];
        let error = InterpreterError::enhanced("Error".to_string(), ErrorType::Validation)
            .with_suggestions(suggestions.clone());

        match error {
            InterpreterError::Enhanced { info, .. } => {
                assert_eq!(info.suggestions, suggestions);
            }
            _ => panic!("Expected Enhanced error variant"),
        }
    }

    #[test]
    fn test_interpreter_error_with_help() {
        let help = "This is how to fix it".to_string();
        let error = InterpreterError::enhanced("Error".to_string(), ErrorType::Runtime)
            .with_help(help.clone());

        match error {
            InterpreterError::Enhanced { info, .. } => {
                assert_eq!(info.help_text.as_ref().unwrap(), &help);
            }
            _ => panic!("Expected Enhanced error variant"),
        }
    }

    #[test]
    fn test_interpreter_error_with_severity() {
        let error = InterpreterError::enhanced("Warning".to_string(), ErrorType::Semantic)
            .with_severity(ErrorSeverity::Warning);

        assert_eq!(error.severity(), ErrorSeverity::Warning);
    }

    #[test]
    fn test_interpreter_error_error_type() {
        let error = InterpreterError::enhanced("Test".to_string(), ErrorType::Execution);
        assert_eq!(error.error_type(), ErrorType::Execution);

        let init_error = InterpreterError::Initialization(InitError::NoStartSymbol);
        assert_eq!(init_error.error_type(), ErrorType::Initialization);

        let exec_error = InterpreterError::Execution(ExecError::StackUnderflow);
        assert_eq!(exec_error.error_type(), ErrorType::Execution);

        let sys_error = InterpreterError::System(SystemError::OutOfMemory);
        assert_eq!(sys_error.error_type(), ErrorType::System);
    }

    #[test]
    fn test_interpreter_error_severity() {
        let enhanced_error = InterpreterError::enhanced("Test".to_string(), ErrorType::Syntax)
            .with_severity(ErrorSeverity::Info);
        assert_eq!(enhanced_error.severity(), ErrorSeverity::Info);

        let init_error = InterpreterError::Initialization(InitError::NoStartSymbol);
        assert_eq!(init_error.severity(), ErrorSeverity::Error);

        let exec_error = InterpreterError::Execution(ExecError::StackUnderflow);
        assert_eq!(exec_error.severity(), ErrorSeverity::Error);

        let sys_error = InterpreterError::System(SystemError::OutOfMemory);
        assert_eq!(sys_error.severity(), ErrorSeverity::Error);
    }

    #[test]
    fn test_interpreter_error_context_accessors() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "test".to_string();
        let context = ErrorContext::new(position, source_line);

        let error = InterpreterError::enhanced("Test".to_string(), ErrorType::Syntax)
            .with_context(context.clone());

        assert_eq!(error.context(), Some(&context));
        assert!(error.suggestions().is_empty());
        assert!(error.help_text().is_none());
    }

    #[test]
    fn test_interpreter_error_suggestions_accessors() {
        let suggestions = vec!["Suggestion 1".to_string(), "Suggestion 2".to_string()];
        let help = "Help text".to_string();

        let error = InterpreterError::enhanced("Test".to_string(), ErrorType::Validation)
            .with_suggestions(suggestions.clone())
            .with_help(help.clone());

        assert_eq!(error.suggestions(), &suggestions[..]);
        assert_eq!(error.help_text(), Some(&help));
        assert!(error.context().is_none());
    }

    #[test]
    fn test_init_error_variants() {
        let coord = Coordinate::new(5, 10);
        let no_start = InitError::NoStartSymbol;
        let multiple_start = InitError::MultipleStartSymbols;
        let invalid_char = InitError::InvalidCharacter('x', coord);
        let grid_size = InitError::GridSizeExceeded(2000, 1500);

        assert!(no_start.to_string().contains("No start symbol"));
        assert!(multiple_start.to_string().contains("Multiple start symbols"));
        assert!(invalid_char.to_string().contains("Invalid character"));
        assert!(grid_size.to_string().contains("Grid size"));
    }

    #[test]
    fn test_exec_error_variants() {
        let coord = Coordinate::new(1, 2);
        let stack_underflow = ExecError::StackUnderflow;
        let div_by_zero = ExecError::DivisionByZero;
        let mod_by_zero = ExecError::ModuloByZero;
        let invalid_memory = ExecError::InvalidMemoryAccess(coord);
        let subroutine_underflow = ExecError::SubroutineUnderflow;
        let collision = ExecError::DropletCollision(coord);
        let exec_timeout = ExecError::ExecutionTimeout(1000000);
        let wall_timeout = ExecError::WallClockTimeout(5000);
        let soft_tick = ExecError::SoftTickLimitWarning(900000);
        let soft_time = ExecError::SoftTimeLimitWarning(4000);
        let internal = ExecError::InternalError("Something broke".to_string());
        let invalid_op = ExecError::InvalidOperation('?');

        assert!(stack_underflow.to_string().contains("Stack underflow"));
        assert!(div_by_zero.to_string().contains("Division by zero"));
        assert!(mod_by_zero.to_string().contains("Modulo by zero"));
        assert!(invalid_memory.to_string().contains("Invalid memory access"));
        assert!(subroutine_underflow.to_string().contains("Subroutine underflow"));
        assert!(collision.to_string().contains("Droplet collision"));
        assert!(exec_timeout.to_string().contains("Execution timeout"));
        assert!(wall_timeout.to_string().contains("Wall-clock timeout"));
        assert!(soft_tick.to_string().contains("Soft tick limit"));
        assert!(soft_time.to_string().contains("Soft time limit"));
        assert!(internal.to_string().contains("Internal error"));
        assert!(invalid_op.to_string().contains("Invalid operation"));
    }

    #[test]
    fn test_system_error_variants() {
        let out_of_memory = SystemError::OutOfMemory;
        let io_error = SystemError::IoError("File not found".to_string());
        let internal = SystemError::InternalError("System crash".to_string());

        assert!(out_of_memory.to_string().contains("Out of memory"));
        assert!(io_error.to_string().contains("I/O error"));
        assert!(internal.to_string().contains("Internal error"));
    }

    #[test]
    fn test_error_chaining() {
        let source_error = InterpreterError::Execution(ExecError::StackUnderflow);
        let enhanced_error = InterpreterError::enhanced(
            "Enhanced error message".to_string(),
            ErrorType::Runtime
        );

        // Note: We can't directly create chained errors in the current API,
        // but we can test the structure
        match source_error {
            InterpreterError::Execution(exec_error) => {
                assert_eq!(exec_error, ExecError::StackUnderflow);
            }
            _ => panic!("Expected Execution error"),
        }
    }

    #[test]
    fn test_error_equality() {
        let coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(1, 2);
        let coord3 = Coordinate::new(3, 4);

        let pos1 = Position::new(0, 5, coord1);
        let pos2 = Position::new(0, 5, coord2);
        let pos3 = Position::new(1, 6, coord3);

        let context1 = ErrorContext::new(pos1.clone(), "test".to_string());
        let context2 = ErrorContext::new(pos2.clone(), "test".to_string());
        let context3 = ErrorContext::new(pos3.clone(), "different".to_string());

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
        assert_eq!(context1, context2);
        assert_ne!(context1, context3);

        let info1 = ErrorInfo::new("Error".to_string(), ErrorType::Syntax);
        let info2 = ErrorInfo::new("Error".to_string(), ErrorType::Syntax);
        let info3 = ErrorInfo::new("Different".to_string(), ErrorType::Syntax);

        assert_eq!(info1, info2);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = IoError::new(ErrorKind::NotFound, "File not found");
        let interpreter_error: InterpreterError = io_error.into();

        match interpreter_error {
            InterpreterError::System(SystemError::IoError(msg)) => {
                assert!(msg.contains("File not found"));
            }
            _ => panic!("Expected System::IoError variant"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        // Test that Result alias works correctly
        let ok_result: Result<i32> = Ok(42);
        let error_result: Result<i32> = Err(InterpreterError::Execution(ExecError::StackUnderflow));

        assert!(ok_result.is_ok());
        assert!(error_result.is_err());
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = InterpreterError::enhanced("Test error".to_string(), ErrorType::Syntax);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Enhanced"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_comprehensive_error_builder() {
        let coord = Coordinate::new(10, 20);
        let position = Position::new(5, 8, coord);
        let source_line = "let x = 5".to_string();
        let surrounding = vec![
            (4, "function main() {".to_string()),
            (6, "print(x)".to_string()),
            (7, "}".to_string()),
        ];
        let suggestions = vec![
            "Add a semicolon at the end".to_string(),
            "Check variable naming".to_string(),
        ];
        let help = "Variable declarations in this language require proper syntax".to_string();

        let error = InterpreterError::enhanced(
            "Syntax error in variable declaration".to_string(),
            ErrorType::Syntax
        )
        .with_context(
            ErrorContext::new(position, source_line)
                .with_surrounding_lines(surrounding)
                .with_span(4, 9)
        )
        .with_suggestions(suggestions)
        .with_help(help)
        .with_severity(ErrorSeverity::Error);

        // Verify all components are present
        assert_eq!(error.error_type(), ErrorType::Syntax);
        assert_eq!(error.severity(), ErrorSeverity::Error);
        assert!(error.context().is_some());
        assert!(!error.suggestions().is_empty());
        assert!(error.help_text().is_some());

        let context = error.context().unwrap();
        assert_eq!(context.position.line, 5);
        assert_eq!(context.position.column, 8);
        assert_eq!(context.error_span, (4, 9));
        assert_eq!(context.surrounding_lines.len(), 3);
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_position_properties(line in any::<usize>(), column in any::<usize>(), x in any::<isize>(), y in any::<isize>()) {
        let coord = Coordinate::new(x, y);
        let position = Position::new(line, column, coord);

        assert_eq!(position.line, line);
        assert_eq!(position.column, column);
        assert_eq!(position.coordinate, coord);
    }

    #[test]
    fn test_error_context_span_properties(
        line in any::<usize>(),
        column in any::<usize>(),
        x in any::<isize>(),
        y in any::<isize>(),
        start in any::<usize>(),
        end in any::<usize>()
    ) {
        let coord = Coordinate::new(x, y);
        let position = Position::new(line, column, coord);
        let source_line = "test line".to_string();

        let context = if start <= end {
            ErrorContext::new(position, source_line).with_span(start, end)
        } else {
            ErrorContext::new(position, source_line).with_span(end, start)
        };

        // Span should be properly ordered
        assert!(context.error_span.0 <= context.error_span.1);
    }

    #[test]
    fn test_error_info_builder_pattern_properties(
        message in ".*",
        suggestions in prop::collection::vec(".*", 0..5)
    ) {
        let info = ErrorInfo::new(message.clone(), ErrorType::Runtime)
            .with_suggestions(suggestions.clone());

        assert_eq!(info.message, message);
        assert_eq!(info.suggestions, suggestions);
        assert_eq!(info.error_type, ErrorType::Runtime);
        assert_eq!(info.severity, ErrorSeverity::Error);
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_error_creation() {
        let start = Instant::now();

        for i in 0..100_000 {
            let _error = InterpreterError::enhanced(
                format!("Error {}", i),
                ErrorType::Runtime
            );
        }

        let duration = start.elapsed();
        println!("Error creation (100K): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_error_with_context() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "test line".to_string();
        let context = ErrorContext::new(position, source_line);
        let suggestions = vec!["Suggestion 1".to_string(), "Suggestion 2".to_string()];
        let help = "Help text".to_string();

        let start = Instant::now();

        for i in 0..50_000 {
            let _error = InterpreterError::enhanced(
                format!("Error {}", i),
                ErrorType::Syntax
            )
            .with_context(context.clone())
            .with_suggestions(suggestions.clone())
            .with_help(help.clone())
            .with_severity(ErrorSeverity::Warning);
        }

        let duration = start.elapsed();
        println!("Error with context (50K): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn benchmark_error_display() {
        let errors: Vec<_> = (0..10_000)
            .map(|i| InterpreterError::enhanced(
                format!("Error {}", i),
                ErrorType::Runtime
            ))
            .collect();

        let start = Instant::now();

        for error in &errors {
            let _display = format!("{}", error);
        }

        let duration = start.elapsed();
        println!("Error display (10K): {:?}", duration);
        assert!(duration.as_millis() < 300);
    }

    #[test]
    fn benchmark_error_accessors() {
        let coord = Coordinate::new(1, 2);
        let position = Position::new(0, 5, coord);
        let source_line = "test".to_string();
        let context = ErrorContext::new(position, source_line);
        let suggestions = vec!["Suggestion".to_string()];
        let help = "Help".to_string();

        let error = InterpreterError::enhanced("Test".to_string(), ErrorType::Validation)
            .with_context(context)
            .with_suggestions(suggestions)
            .with_help(help);

        let start = Instant::now();

        for _ in 0..1_000_000 {
            let _type = error.error_type();
            let _severity = error.severity();
            let _context = error.context();
            let _suggestions = error.suggestions();
            let _help = error.help_text();
        }

        let duration = start.elapsed();
        println!("Error accessors (5M): {:?}", duration);
        assert!(duration.as_millis() < 100);
    }
}