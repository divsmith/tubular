//! Integration tests for I/O operations
//! Tests character input, numeric input, and interactive functionality

use tubular::parser::grid_parser::GridParser;
use tubular::interpreter::execution::TubularInterpreter;
use tubular::types::error::InterpreterError;

#[test]
fn test_character_input_basic() {
    let program = r"
@
|
?
,
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    // This test would require mocking stdin, so we test the parsing
    // and validation instead
    assert!(grid.start.is_some());
    assert!(grid.size() > 0);
}

#[test]
fn test_numeric_input_basic() {
    let program = r"
@
|
?
?
,
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    // This test would require mocking stdin, so we test the parsing
    // and validation instead
    assert!(grid.start.is_some());
    assert!(grid.size() > 0);
}

#[test]
fn test_calculator_program_structure() {
    let program = r"
@
|
?
:
?
:
A,
n,
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify the program structure is valid
    assert!(grid.start.is_some());
    assert_eq!(grid.size(), 9); // 9 cells including whitespace

    // Verify we can create an interpreter
    let interpreter = TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_character_output_with_input() {
    let program = r"
@
|
7,
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    let result = interpreter.run().unwrap();

    // Should complete successfully
    match result.status {
        tubular::interpreter::execution::ExecutionStatus::Completed => {
            // Expected - program should complete
        }
        _ => panic!("Program should have completed successfully"),
    }
}

#[test]
fn test_numeric_output_with_input() {
    let program = r"
@
|
42
n,
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    let result = interpreter.run().unwrap();

    // Should complete successfully
    match result.status {
        tubular::interpreter::execution::ExecutionStatus::Completed => {
            // Expected - program should complete
        }
        _ => panic!("Program should have completed successfully"),
    }
}

#[test]
fn test_io_operation_detection() {
    use tubular::operations::io::IoOperations;

    // Test I/O operation detection
    assert!(IoOperations::is_io_operation(',')); // character output
    assert!(IoOperations::is_io_operation('n')); // numeric output
    assert!(IoOperations::is_io_operation('!')); // sink
    assert!(IoOperations::is_io_operation('?')); // character input

    // Test non-I/O operations
    assert!(!IoOperations::is_io_operation('5'));
    assert!(!IoOperations::is_io_operation('|'));
    assert!(!IoOperations::is_io_operation('A'));
}

#[test]
fn test_data_source_detection() {
    use tubular::operations::io::IoOperations;

    // Test data source (input) detection
    assert!(IoOperations::is_data_source('?'));

    // Test non-data-source operations
    assert!(!IoOperations::is_data_source(','));
    assert!(!IoOperations::is_data_source('n'));
    assert!(!IoOperations::is_data_source('!'));
    assert!(!IoOperations::is_data_source('5'));
}

#[test]
fn test_data_sink_detection() {
    use tubular::operations::io::IoOperations;

    // Test data sink (output) detection
    assert!(IoOperations::is_data_sink(',')); // character output
    assert!(IoOperations::is_data_sink('n')); // numeric output
    assert!(IoOperations::is_data_sink('!')); // sink

    // Test non-data-sink operations
    assert!(!IoOperations::is_data_sink('?'));
    assert!(!IoOperations::is_data_sink('5'));
    assert!(!IoOperations::is_data_sink('|'));
}

#[test]
fn test_complex_io_program() {
    // A more complex program that uses multiple I/O operations
    let program = r"
     @
     |
     5
     |
     ?
     A,
     |
     n,
     !
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify program structure
    assert!(grid.start.is_some());
    assert!(grid.size() > 5);

    // Should be able to create interpreter
    let interpreter = TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_input_parsing_validation() {
    use tubular::operations::io::IoOperations;

    // Test that input parsing validation works correctly
    // These tests focus on the validation logic without requiring actual stdin

    // Test that all I/O symbols are correctly identified
    let io_symbols = [',', 'n', '!', '?'];
    for &symbol in &io_symbols {
        assert!(IoOperations::is_io_operation(symbol),
               "Symbol '{}' should be recognized as I/O operation", symbol);
    }

    // Test that non-I/O symbols are correctly identified
    let non_io_symbols = ['|', '-', '/', '\\', '@', '5', 'A', ':', ';', 'd'];
    for &symbol in &non_io_symbols {
        assert!(!IoOperations::is_io_operation(symbol),
               "Symbol '{}' should NOT be recognized as I/O operation", symbol);
    }
}

#[test]
fn test_input_buffer_creation() {
    use tubular::operations::io::{InputBuffer, ValidationMode};

    // Test basic buffer creation
    let buffer = InputBuffer::new();
    assert_eq!(buffer.validation_mode(), ValidationMode::Lenient);

    // Test buffer with predefined input
    let buffer_with_input = InputBuffer::with_input("hello\nworld".to_string());
    assert_eq!(buffer_with_input.validation_mode(), ValidationMode::Lenient);
}

#[test]
fn test_validation_modes() {
    use tubular::operations::io::ValidationMode;

    // Test that validation modes are correctly implemented
    assert_eq!(ValidationMode::Lenient, ValidationMode::Lenient);
    assert_ne!(ValidationMode::Strict, ValidationMode::Lenient);
    assert_ne!(ValidationMode::Permissive, ValidationMode::Strict);
    assert_ne!(ValidationMode::Permissive, ValidationMode::Lenient);
}

#[test]
fn test_numeric_validation_functions() {
    use tubular::operations::io::IoOperations;

    // Test integer validation
    assert!(IoOperations::is_valid_integer("123"));
    assert!(IoOperations::is_valid_integer("-456"));
    assert!(IoOperations::is_valid_integer("0"));
    assert!(!IoOperations::is_valid_integer(""));  // Empty string
    assert!(!IoOperations::is_valid_integer("-"));  // Just minus
    assert!(!IoOperations::is_valid_integer("12a3"));  // Contains letter
    assert!(!IoOperations::is_valid_integer("12.3"));  // Contains decimal
    assert!(!IoOperations::is_valid_integer("abc"));  // Only letters
}

#[test]
fn test_number_extraction() {
    use tubular::operations::io::IoOperations;

    // Test number extraction from strings
    assert_eq!(IoOperations::extract_number("123"), Some(123));
    assert_eq!(IoOperations::extract_number("-456"), Some(-456));
    assert_eq!(IoOperations::extract_number("abc123def"), Some(123));
    assert_eq!(IoOperations::extract_number("abc-123def"), Some(-123));
    assert_eq!(IoOperations::extract_number("abc"), None);
    assert_eq!(IoOperations::extract_number(""), None);
    assert_eq!(IoOperations::extract_number("-"), None);
    assert_eq!(IoOperations::extract_number("123abc456"), Some(123)); // First number
}

#[test]
fn test_intelligent_parsing() {
    use tubular::operations::io::IoOperations;

    // Test direct numbers
    assert_eq!(IoOperations::parse_intelligently("123"), Some(123));
    assert_eq!(IoOperations::parse_intelligently("-456"), Some(-456));

    // Test number extraction
    assert_eq!(IoOperations::parse_intelligently("abc123def"), Some(123));

    // Test word parsing
    assert_eq!(IoOperations::parse_intelligently("zero"), Some(0));
    assert_eq!(IoOperations::parse_intelligently("one"), Some(1));
    assert_eq!(IoOperations::parse_intelligently("five"), Some(5));
    assert_eq!(IoOperations::parse_intelligently("ten"), Some(10));
    assert_eq!(IoOperations::parse_intelligently("ZERO"), Some(0)); // Case insensitive

    // Test invalid input
    assert_eq!(IoOperations::parse_intelligently("abc"), None);
    assert_eq!(IoOperations::parse_intelligently(""), None);
}

#[test]
fn test_numeric_validation_modes() {
    use tubular::operations::io::{IoOperations, ValidationMode};

    // Test lenient mode
    let result = IoOperations::validate_and_parse_numeric("123", ValidationMode::Lenient).unwrap();
    assert_eq!(result, "123");

    let result = IoOperations::validate_and_parse_numeric("abc", ValidationMode::Lenient).unwrap();
    assert_eq!(result, "0"); // Falls back to 0

    let result = IoOperations::validate_and_parse_numeric("", ValidationMode::Lenient).unwrap();
    assert_eq!(result, "0"); // Empty input becomes 0

    // Test strict mode
    let result = IoOperations::validate_and_parse_numeric("123", ValidationMode::Strict).unwrap();
    assert_eq!(result, "123");

    let result = IoOperations::validate_and_parse_numeric("abc", ValidationMode::Strict);
    assert!(result.is_err()); // Should error on invalid input

    // Test permissive mode
    let result = IoOperations::validate_and_parse_numeric("five", ValidationMode::Permissive).unwrap();
    assert_eq!(result, "5"); // Parses word to number

    let result = IoOperations::validate_and_parse_numeric("abc123def", ValidationMode::Permissive).unwrap();
    assert_eq!(result, "123"); // Extracts number

    let result = IoOperations::validate_and_parse_numeric("xyz", ValidationMode::Permissive).unwrap();
    assert_eq!(result, "0"); // Falls back to 0
}

#[test]
fn test_enhanced_calculator_program() {
    // Test the enhanced calculator program structure
    let program = r"
@
|
??
:
??
:
A
n,
!
";

    let parser = tubular::parser::grid_parser::GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify the program structure is valid
    assert!(grid.start.is_some());
    assert!(grid.size() > 0);

    // Verify we can create an interpreter
    let interpreter = tubular::interpreter::execution::TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_character_input_program() {
    // Test character input program structure
    let program = r"
@
?
,
!
";

    let parser = tubular::parser::grid_parser::GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify the program structure is valid
    assert!(grid.start.is_some());
    assert!(grid.size() > 0);

    // Verify we can create an interpreter
    let interpreter = tubular::interpreter::execution::TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_mixed_io_program() {
    // Test a program that uses multiple types of I/O
    let program = r"
@
|
7
|
??
A
n,
?
,
!
";

    let parser = tubular::parser::grid_parser::GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify the program structure is valid
    assert!(grid.start.is_some());
    assert!(grid.size() > 5);

    // Should be able to create interpreter
    let interpreter = tubular::interpreter::execution::TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_io_error_handling() {
    use tubular::operations::io::{IoOperations, ValidationMode};

    // Test that strict validation properly handles errors
    let invalid_inputs = ["abc", "12.3", "1e5", "NaN", "inf"];

    for input in invalid_inputs.iter() {
        let result = IoOperations::validate_and_parse_numeric(input, ValidationMode::Strict);
        assert!(result.is_err(), "Input '{}' should cause validation error", input);
    }
}

#[test]
fn test_io_buffer_with_multiple_lines() {
    use tubular::operations::io::InputBuffer;

    // Test buffer with multiple lines of input
    let input = "5\n3\n+\n7".to_string();
    let buffer = InputBuffer::with_input(input);

    // We can't easily test the actual reading without mocking stdin,
    // but we can verify the buffer was created correctly
    assert_eq!(buffer.validation_mode(), tubular::operations::io::ValidationMode::Lenient);
}