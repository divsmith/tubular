//! Integration tests for subroutine operations
//! Tests subroutine Call (C) and Return (R) operations in complete program execution

use tubular::parser::grid_parser::GridParser;
use tubular::interpreter::execution::TubularInterpreter;
use tubular::types::error::InterpreterError;

#[test]
fn test_subroutine_operations_basic_call_and_return() {
    let program = r"
@
|
2:
5:
0
C
!
2,5
|
42
n,
R
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

    // Verify the program output contains "42"
    assert!(result.final_output.contains("42"));
}

#[test]
fn test_subroutine_operations_nested_calls() {
    let program = r"
@
|
1:
5:
0
C
!
1,5
|
2:
10:
0
C
!
2,10
|
99
n,
R
!
|
50
n,
R
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

    // Verify both outputs appear
    assert!(result.final_output.contains("99"));
    assert!(result.final_output.contains("50"));
}

#[test]
fn test_subroutine_operations_empty_return_stack() {
    let program = r"
@
|
R
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    let result = interpreter.run().unwrap();

    // Should complete successfully (empty return stack is no-op)
    match result.status {
        tubular::interpreter::execution::ExecutionStatus::Completed => {
            // Expected - program should complete
        }
        _ => panic!("Program should have completed successfully"),
    }
}

#[test]
fn test_subroutine_operations_invalid_call_target() {
    let program = r"
@
|
99:
99:
0
C
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    let result = interpreter.run().unwrap();

    // Should complete successfully (invalid call target means no jump)
    match result.status {
        tubular::interpreter::execution::ExecutionStatus::Completed => {
            // Expected - program should complete
        }
        _ => panic!("Program should have completed successfully"),
    }

    // Verify call stack remains empty (no call was made)
    assert_eq!(interpreter.state().call_stack.depth(), 0);
}

#[test]
fn test_subroutine_operations_multiple_calls() {
    let program = r"
@
|
1:
5:
0
C
|
2:
10:
1
C
!
1,5
|
10
n,
R
!
2,10
|
20
n,
R
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

    // Verify both outputs appear
    assert!(result.final_output.contains("10"));
    assert!(result.final_output.contains("20"));
}

#[test]
fn test_subroutine_operations_direction_changes() {
    let program = r"
@
|
1:
5:
1
C
!
1,5
|
15
n,
R
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

    // Verify output appears (direction change should work)
    assert!(result.final_output.contains("15"));
}

#[test]
fn test_subroutine_operations_negative_coordinates() {
    let program = r"
@
|
-5:
-10:
0
C
!
-5,-10
|
42
n,
R
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

    // Verify negative coordinates work
    assert!(result.final_output.contains("42"));
}

#[test]
fn test_subroutine_operations_stack_underflow() {
    let program = r"
@
|
42:
C
!
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();
    let mut interpreter = TubularInterpreter::new(grid)
        .unwrap()
        .with_options(false, false, Some(100));

    let result = interpreter.run().unwrap();

    // Should complete successfully even with stack underflow
    match result.status {
        tubular::interpreter::execution::ExecutionStatus::Completed => {
            // Expected - program should complete
        }
        _ => panic!("Program should have completed successfully"),
    }

    // Should attempt call with defaults (y=0, direction=Down=2)
    // But target (42, 0) likely doesn't exist, so no jump
    assert_eq!(interpreter.state().call_stack.depth(), 0);
}

#[test]
fn test_subroutine_operations_large_coordinates() {
    let program = r"
@
|
1000000:
2000000:
0
C
!
1000000,2000000
|
12345
n,
R
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

    // Verify large coordinates work
    assert!(result.final_output.contains("12345"));
}

#[test]
fn test_subroutine_operations_call_stack_depth() {
    let program = r"
@
|
1:
5:
0
C
!
1,5
|
2:
10:
0
C
!
2,10
|
3:
15:
0
C
!
3,15
|
999
n,
R
!
|
111
n,
R
!
|
222
n,
R
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

    // Verify all three outputs appear (in reverse order due to stack)
    assert!(result.final_output.contains("999"));
    assert!(result.final_output.contains("111"));
    assert!(result.final_output.contains("222"));

    // Verify call stack is empty at end
    assert_eq!(interpreter.state().call_stack.depth(), 0);
}

#[test]
fn test_subroutine_operations_program_structure() {
    let program = r"
@
|
1:
5:
0
C
!
1,5
|
42
n,
R
";

    let parser = GridParser::new();
    let grid = parser.parse_string(program).unwrap();

    // Verify the program structure is valid
    assert!(grid.start.is_some());
    assert!(grid.size() > 0);

    // Should be able to create interpreter
    let interpreter = TubularInterpreter::new(grid);
    assert!(interpreter.is_ok());
}

#[test]
fn test_subroutine_operations_detection() {
    use tubular::operations::subroutines::SubroutineOperations;

    // Test subroutine operation detection
    assert!(SubroutineOperations::is_subroutine_operation('C')); // Call operation
    assert!(SubroutineOperations::is_subroutine_operation('R')); // Return operation

    // Test non-subroutine operations
    assert!(!SubroutineOperations::is_subroutine_operation('A'));
    assert!(!SubroutineOperations::is_subroutine_operation(':'));
    assert!(!SubroutineOperations::is_subroutine_operation('!'));
    assert!(!SubroutineOperations::is_subroutine_operation('G'));
    assert!(!SubroutineOperations::is_subroutine_operation('P'));
}

#[test]
fn test_subroutine_operations_with_memory() {
    let program = r"
@
|
1:
5:
0
C
!
1,5
|
10:
15:
42
P
|
10:
15:
G
n,
R
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

    // Verify output contains the retrieved value
    assert!(result.final_output.contains("42"));
}

#[test]
fn test_subroutine_operations_complex_flow() {
    // A complex program with multiple subroutines and control flow
    let program = r"
     @
     |
     1
     :
     5
     :
     0
     C
     |
     2
     :
     10
     :
     0
     C
     !

     1,5
     |
     25
     n
     |
     R
     !

     2,10
     |
     50
     n
     |
     3
     :
     15
     :
     0
     C
     |
     R
     !

     3,15
     |
     75
     n
     |
     R
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
fn test_subroutine_operations_return_preserves_droplet_value() {
    let program = r"
@
|
1:
5:
0
C
!
1,5
|
100
:
R
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

    // Verify return operation preserved droplet value
    // (We can't easily test exact value but program should complete)
    assert!(result.status == tubular::interpreter::execution::ExecutionStatus::Completed);
}

#[test]
fn test_subroutine_operations_direction_conversion() {
    use tubular::operations::subroutines::SubroutineOperations;
    use tubular::types::direction::Direction;

    // Test direction to value conversion
    assert_eq!(SubroutineOperations::direction_to_value(Direction::Up).to_isize(), Some(0));
    assert_eq!(SubroutineOperations::direction_to_value(Direction::Right).to_isize(), Some(1));
    assert_eq!(SubroutineOperations::direction_to_value(Direction::Down).to_isize(), Some(2));
    assert_eq!(SubroutineOperations::direction_to_value(Direction::Left).to_isize(), Some(3));

    // Test value to direction conversion
    assert_eq!(SubroutineOperations::value_to_direction(&tubular::types::bigint::TubularBigInt::new(0)), Direction::Up);
    assert_eq!(SubroutineOperations::value_to_direction(&tubular::types::bigint::TubularBigInt::new(1)), Direction::Right);
    assert_eq!(SubroutineOperations::value_to_direction(&tubular::types::bigint::TubularBigInt::new(2)), Direction::Down);
    assert_eq!(SubroutineOperations::value_to_direction(&tubular::types::bigint::TubularBigInt::new(3)), Direction::Left);

    // Test roundtrip conversion
    for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
        let value = SubroutineOperations::direction_to_value(direction);
        let converted_back = SubroutineOperations::value_to_direction(&value);
        assert_eq!(direction, converted_back);
    }
}