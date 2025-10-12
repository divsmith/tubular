# Quickstart Guide: Tubular Language Implementation

**Feature**: `001-a-basic-initial` | **Date**: 2025-10-11

## Overview

This guide walks you through setting up the development environment and understanding the Tubular language interpreter implementation.

## Development Environment Setup

### Prerequisites
- Rust 1.75+ (for production implementation)
- Git
- A terminal/Command Prompt

### Repository Structure
```
tubular/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── interpreter/         # Core interpreter logic
│   ├── operations/          # Language operations
│   ├── parser/              # Grid parsing
│   ├── types/               # Core types
│   └── cli/                 # Command-line interface
├── tests/                   # Test suites
├── examples/                # Example programs
├── benches/                 # Performance benchmarks
└── docs/                    # Documentation
```

### Initial Setup Commands
```bash
# Clone the repository
git clone <repository-url>
cd tubular

# Install development dependencies
cargo install cargo-watch cargo-benchcmp

# Run initial build
cargo build

# Run tests to verify setup
cargo test

# Run example program
cargo run -- examples/hello_world.tb
```

## Understanding Tubular Programs

### Basic Program Structure
Tubular programs are ASCII grids where:
- `@` is the start point (droplet begins here)
- Pipes (`|`, `-`, `/`, `\`) control flow
- Numbers and operators perform computations
- `!` destroys droplets (program end)

### Hello World Example
```
examples/hello_world.tb:
    @
    |
    72,
    |
    101,
    |
    108,
    |
    108,
    |
    111,
    |
    44,
    |
    32,
    |
    87,
    |
    111,
    |
    114,
    |
    108,
    |
    100,
    |
    33,
    |
    !
```

**Execution Flow**:
1. Droplet starts at `@` with value 0, moving down
2. Hits `72` (ASCII for 'H'), outputs "H"
3. Continues down, outputting each character
4. Ends at `!` (destroyed)

### Countdown Example
```
examples/countdown.tb:
    @
    |
    5
    |
    d, n
    |
    1-
    |
    d
    |
    0\
      /
     /
    @
```

**Key Features Demonstrated**:
- Numbers (5) set droplet value
- `d` duplicates stack top for output
- `,` outputs as character, `n` as number
- `1-` decrements value by 1
- `\` conditional branch (continues if value ≠ 0)

## Common Development Tasks

### Running Programs
```bash
# Basic execution
cargo run -- <program.tb>

# Verbose output
cargo run -- --verbose <program.tb>

# Step-by-step tracing
cargo run -- --trace <program.tb>

# Performance benchmarking
cargo run -- --benchmark <program.tb>
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test category
cargo test unit
cargo test integration
cargo test property

# Run tests with output
cargo test -- --nocapture

# Run performance benchmarks
cargo bench
```

### Development Workflow
```bash
# Watch for changes and re-run tests
cargo watch -x test

# Watch for changes and run example
cargo watch -x "run -- examples/hello_world.tb"

# Format code
cargo fmt

# Run clippy for lints
cargo clippy -- -D warnings
```

## Key Implementation Concepts

### Droplet-Based Execution
- Each droplet represents an execution thread
- Droplets move through pipes one cell per tick
- Multiple droplets can exist simultaneously
- Collisions destroy both droplets

### Stack-Based Computation
- Data stack for temporary storage
- Stack operations: `:` (push), `;` (pop), `d` (duplicate)
- Arithmetic operations use stack operands
- Stack underflow returns 0 (no error)

### Memory Operations
- Reservoir provides 2D unbounded memory
- Coordinates can be negative
- Uninitialized memory returns 0
- Used for data persistence between executions

## Example Programs to Study

### 1. Simple Addition (`examples/addition.tb`)
```
   @
   |
   5:
   3:
   A,
   !
```
**Behavior**: Pushes 5, pushes 3, adds them, outputs result (8)

### 2. Loop Example (`examples/loop.tb`)
```
     @
     |
     5d
     |
     1-
     |
     0\
      /
     /
    @
```
**Behavior**: Counts down from 5, demonstrating conditional branching

### 3. Memory Test (`examples/memory.tb`)
```
   @
   |
   42
   |
   0,0P
   |
   0
   |
   0,0G,
   !
```
**Behavior**: Stores 42 at reservoir coordinate (0,0), then retrieves and outputs it

## Debugging Tips

### Common Issues
1. **No Output**: Check if droplet reaches sink (`!`)
2. **Wrong Output**: Verify number values and output operators
3. **Infinite Loops**: Add tick limit with `--ticks` option
4. **Collisions**: Use `--trace` to see droplet interactions

### Debugging Commands
```bash
# Enable verbose execution
cargo run -- --verbose program.tb

# Step-by-step execution
cargo run -- --trace program.tb

# Limit execution ticks
cargo run -- --ticks 100 program.tb

# Validate program syntax
cargo run -- validate program.tb
```

### Understanding Traces
Trace output shows:
```
TRACE | TICK 001 | Droplet 0 | (2,3) -> (2,4) | value: 5 -> 5 | dir: Down -> Down
TRACE | TICK 001 | Cell (+)  | Increment: 5 -> 6
TRACE | TICK 001 | Stack    | Push: 6 | Stack: [5, 6]
```

## Performance Considerations

### Optimization Targets
- Grid representation (sparse vs dense)
- Droplet collision detection
- Stack operations efficiency
- Memory usage patterns

### Benchmarking
```bash
# Run all benchmarks
cargo bench

# Compare benchmark results
cargo benchcmp old_bench.json new_bench.json

# Profile specific program
cargo run --release --benchmark examples/large_program.tb
```

### Performance Metrics
- **Ticks per second**: Execution speed
- **Memory usage**: Peak consumption
- **Droplet count**: Concurrent execution capability
- **Grid size**: Supported program complexity

## Next Steps

### Learning Resources
1. Read `docs/language_reference.md` for complete language spec
2. Study `examples/` directory for various programming patterns
3. Review `tests/` for expected behavior patterns
4. Examine `src/` implementation for technical details

### Contributing
1. Fork the repository
2. Create feature branch
3. Add tests for new functionality
4. Ensure all existing tests pass
5. Update documentation
6. Submit pull request

### Advanced Topics
- Property-based testing with `proptest`
- Performance optimization techniques
- Custom operator implementation
- Memory management strategies
- Concurrency and parallelization