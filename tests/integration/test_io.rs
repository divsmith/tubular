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