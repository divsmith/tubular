# Tubular Unit Test Suite

This directory contains comprehensive unit tests for the Tubular interpreter.

## Structure

```
tests/unit/
├── mod.rs                    # Main test module entry point
├── types/                    # Type system tests
│   ├── mod.rs
│   ├── coordinate.rs         # Coordinate struct tests
│   ├── direction.rs          # Direction enum tests
│   ├── bigint.rs             # TubularBigInt wrapper tests
│   └── error.rs              # Error handling tests
├── interpreter/              # Interpreter core tests
│   ├── mod.rs
│   ├── droplet.rs            # Droplet state and movement tests
│   ├── grid.rs               # Program grid and cell tests
│   ├── stack.rs              # Data stack tests
│   ├── memory.rs             # Memory reservoir tests
│   ├── subroutines.rs        # Call stack and subroutines tests
│   ├── execution.rs          # Execution engine tests (placeholder)
│   └── collision.rs          # Collision detection tests (placeholder)
├── operations/               # Operation tests
│   ├── mod.rs
│   ├── flow_control.rs       # Flow control operations (placeholder)
│   ├── arithmetic.rs         # Arithmetic operations (placeholder)
│   ├── memory.rs             # Memory operations (placeholder)
│   ├── io.rs                 # I/O operations (placeholder)
│   └── subroutines.rs        # Subroutine operations (placeholder)
├── parser/                   # Parser tests
│   ├── mod.rs
│   ├── grid_parser.rs        # Grid parsing tests (placeholder)
│   └── validator.rs          # Program validation tests (placeholder)
├── cli/                      # CLI tests
│   ├── mod.rs
│   └── commands.rs           # CLI command tests (placeholder)
├── property_tests.rs         # Property-based tests using proptest
├── benchmarks.rs             # Performance benchmarks
└── README.md                 # This file
```

## Running Tests

### Run all unit tests
```bash
cargo test --test unit
```

### Run specific module tests
```bash
cargo test --test unit types::coordinate
cargo test --test unit interpreter::droplet
cargo test --test unit property_tests
```

### Run benchmarks
```bash
cargo test --test unit benchmarks -- --nocapture
```

### Run property-based tests
```bash
cargo test --test unit property_tests
```

## Test Coverage

### Types Module (Complete)
- **Coordinate**: Position handling, arithmetic operations, manhattan distance, display formatting
- **Direction**: Movement vectors, turns, character parsing, display formatting
- **TubularBigInt**: Arbitrary precision arithmetic wrapper, conversions, operations
- **Error**: Error types, context handling, enhanced errors with suggestions

### Interpreter Module (Complete)
- **Droplet**: State management, movement, collision detection, value operations
- **Grid**: Program cell handling, bounding boxes, validation, iteration
- **Stack**: Data stack operations, depth tracking, bulk operations
- **Memory**: Sparse memory storage, coordinate conversion, adjacent cell access
- **Subroutines**: Call stack management, frame operations, return handling

### Property-Based Tests
- Mathematical properties (commutativity, associativity, distributivity)
- LIFO stack properties
- Symmetry properties (distance, collision detection)
- Consistency properties (memory storage and retrieval)

### Performance Benchmarks
- Critical path operations with performance assertions
- Large dataset handling (100K+ operations)
- Memory and time complexity verification

## Test Characteristics

### Comprehensive Coverage
- Public API testing for all modules
- Edge cases and error conditions
- Boundary values and overflow behavior
- Large number handling
- Memory safety verification

### Property-Based Testing
- Uses `proptest` for generating test cases
- Verifies mathematical properties
- Tests invariants across random inputs
- Helps catch edge cases

### Performance Testing
- Includes timing assertions
- Tests with large datasets
- Verifies algorithmic complexity
- Benchmarks critical operations

### Error Testing
- All error variants exercised
- Error context and suggestions
- Error chaining and display
- Recovery scenarios

## Best Practices Applied

1. **Isolation**: Each test is independent
2. **Descriptive Names**: Test names clearly indicate what's being tested
3. **Documentation**: Comments explain complex test scenarios
4. **Edge Cases**: Boundary conditions and error paths tested
5. **Property Testing**: Mathematical properties verified
6. **Performance**: Critical operations benchmarked
7. **Maintainability**: Well-structured and easy to extend

## Extending the Tests

To add new tests:

1. Add test functions to appropriate module files
2. Follow naming convention: `test_<functionality>_<scenario>`
3. Include property-based tests where applicable
4. Add performance benchmarks for critical new features
5. Update this README with new test descriptions

## Integration with Existing Tests

This unit test suite complements the existing integration tests in `tests/integration/`. While integration tests verify end-to-end functionality, unit tests focus on individual component correctness and edge cases.

## Test Configuration

Unit tests are configured in `Cargo.toml`:
```toml
[[test]]
name = "unit"
path = "tests/unit/mod.rs"
```

The test suite uses the following dev-dependencies:
- `proptest`: For property-based testing
- `criterion`: For performance benchmarking