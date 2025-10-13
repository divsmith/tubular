//! Integration tests for memory operations
//! Tests reservoir Get (G) and Put (P) operations in complete program execution

use tubular::parser::grid_parser::GridParser;
use tubular::interpreter::execution::TubularInterpreter;
use tubular::types::error::InterpreterError;

#[test]
fn test_memory_operations_basic() {
    let program = r"
@
|
5:
10:
42
P
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

    // Verify the reservoir contains the stored value at (5, 10)
    let reservoir = &interpreter.state().reservoir;
    let coord = tubular::interpreter::memory::ReservoirCoordinate::new(5, 10);
    assert_eq!(reservoir.get(coord), tubular::types::bigint::TubularBigInt::new(42));
}

#[test]
fn test_memory_operations_get() {
    let program = r"
@
|
5:
10:
P
|
5:
10:
G
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

    // Verify the final droplet value is 42 (retrieved from memory)
    let droplets = &interpreter.state().droplets;
    assert!(!droplets.is_empty(), "Should have at least one droplet");
    let last_droplet = &droplets[droplets.len() - 1];
    assert_eq!(last_droplet.value, tubular::types::bigint::TubularBigInt::new(42));
}

#[test]
fn test_memory_operations_uninitialized() {
    let program = r"
@
|
5:
10:
G
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

    // Verify the final droplet value is 0 (uninitialized memory)
    let droplets = &interpreter.state().droplets;
    assert!(!droplets.is_empty(), "Should have at least one droplet");
    let last_droplet = &droplets[droplets.len() - 1];
    assert_eq!(last_droplet.value, tubular::types::bigint::TubularBigInt::zero());
}

#[test]
fn test_memory_operations_overwrite() {
    let program = r"
@
|
5:
10:
42
P
|
5:
10:
99
P
|
5:
10:
G
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

    // Verify the final value is 99 (overwritten value)
    let droplets = &interpreter.state().droplets;
    assert!(!droplets.is_empty(), "Should have at least one droplet");
    let last_droplet = &droplets[droplets.len() - 1];
    assert_eq!(last_droplet.value, tubular::types::bigint::TubularBigInt::new(99));

    // Verify reservoir contains the overwritten value
    let reservoir = &interpreter.state().reservoir;
    let coord = tubular::interpreter::memory::ReservoirCoordinate::new(5, 10);
    assert_eq!(reservoir.get(coord), tubular::types::bigint::TubularBigInt::new(99));
}

#[test]
fn test_memory_operations_negative_coordinates() {
    let program = r"
@
|
-5:
-10:
42
P
|
-5:
-10:
G
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

    // Verify negative coordinates work
    let droplets = &interpreter.state().droplets;
    assert!(!droplets.is_empty(), "Should have at least one droplet");
    let last_droplet = &droplets[droplets.len() - 1];
    assert_eq!(last_droplet.value, tubular::types::bigint::TubularBigInt::new(42));

    // Verify reservoir contains the value at negative coordinates
    let reservoir = &interpreter.state().reservoir;
    let coord = tubular::interpreter::memory::ReservoirCoordinate::new(-5, -10);
    assert_eq!(reservoir.get(coord), tubular::types::bigint::TubularBigInt::new(42));
}

#[test]
fn test_memory_operations_large_coordinates() {
    let program = r"
@
|
1000000:
2000000:
12345
P
|
1000000:
2000000:
G
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

    // Verify large coordinates work
    let droplets = &interpreter.state().droplets;
    assert!(!droplets.is_empty(), "Should have at least one droplet");
    let last_droplet = &droplets[droplets.len() - 1];
    assert_eq!(last_droplet.value, tubular::types::bigint::TubularBigInt::new(12345));
}

#[test]
fn test_memory_operations_multiple_locations() {
    let program = r"
@
|
5:
10:
42
P
|
15:
20:
99
P
|
5:
10:
G
:
15:
20:
G
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

    // Verify reservoir contains both values
    let reservoir = &interpreter.state().reservoir;

    let coord1 = tubular::interpreter::memory::ReservoirCoordinate::new(5, 10);
    assert_eq!(reservoir.get(coord1), tubular::types::bigint::TubularBigInt::new(42));

    let coord2 = tubular::interpreter::memory::ReservoirCoordinate::new(15, 20);
    assert_eq!(reservoir.get(coord2), tubular::types::bigint::TubularBigInt::new(99));
}

#[test]
fn test_memory_operations_stack_underflow() {
    let program = r"
@
|
42
P
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

    // Verify the value was stored at (42, 0) due to stack underflow (y=0)
    let reservoir = &interpreter.state().reservoir;
    let coord = tubular::interpreter::memory::ReservoirCoordinate::new(42, 0);
    assert_eq!(reservoir.get(coord), tubular::types::bigint::TubularBigInt::new(42));
}

#[test]
fn test_memory_operations_program_structure() {
    let program = r"
@
|
7:
15:
42
P
|
7:
15:
G
n,
!
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
fn test_memory_operations_detection() {
    use tubular::operations::memory::MemoryOperations;

    // Test memory operation detection
    assert!(MemoryOperations::is_memory_operation('G')); // Get operation
    assert!(MemoryOperations::is_memory_operation('P')); // Put operation

    // Test non-memory operations
    assert!(!MemoryOperations::is_memory_operation('A'));
    assert!(!MemoryOperations::is_memory_operation(':'));
    assert!(!MemoryOperations::is_memory_operation('!'));
    assert!(!MemoryOperations::is_memory_operation('|'));
    assert!(!MemoryOperations::is_memory_operation('C'));
}

#[test]
fn test_memory_operations_complex_program() {
    // A more complex program that uses multiple memory operations
    let program = r"
     @
     |
     1
     :
     2
     :
     10
     P
     |
     3
     :
     4
     :
     20
     P
     |
     1
     :
     2
     :
     5
     P
     |
     1
     :
     2
     G
     :
     3
     :
     4
     G
     :
     1
     :
     2
     G
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
fn test_memory_operations_with_arithmetic() {
    let program = r"
@
|
5:
10:
:
P
|
3:
4:
:
P
|
5:
10:
G
:
3:
4:
G
:
A,
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

    // Note: We can't easily test the exact output without running the program
    // but we can verify it completes successfully
    assert!(result.final_output.contains("13")); // 10 + 3 = 13
}