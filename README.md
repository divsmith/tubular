# Tubular Language Implementation

A Python implementation of the Tubular programming language - a two-dimensional, visual, dataflow programming language inspired by pipes and fluid dynamics.

## Overview

Tubular is a unique programming language where programs are represented as 2D grids of ASCII characters forming "pipe systems" through which "droplets" of data flow. The language is designed to be Turing-complete while encouraging a visual style of programming that differs significantly from conventional text-based languages.

## Core Concepts

### The Grid
Tubular programs are 2D grids of ASCII characters where the location of each character is significant. Empty cells (spaces) are considered empty pipe sections.

### Droplets
The fundamental unit of data in Tubular is a **droplet** with two properties:
- **Value**: An integer
- **Direction**: The direction it travels (Up, Down, Left, Right)

### Execution Model
- Execution begins at the `@` character, creating an initial trigger droplet with value `0` moving Down
- Programs execute in discrete **ticks**, where every active droplet moves one cell per tick
- When a droplet enters a cell, the character determines the action performed
- **Droplet Collision**: If two droplets attempt to enter the same cell simultaneously, both are destroyed

## Project Structure

```
tubular/
├── src/           # Source code directory
├── tests/         # Test files directory
├── docs/          # Documentation directory
│   └── api/       # API documentation
├── examples/      # Example programs directory
├── Makefile       # Build system
├── README.md      # This file
└── tubular_spec.md # Language specification
```

## Build System

This project includes a Makefile with three main commands:

### `make compile`
Builds the project executable from the source code in `src/`.

### `make test`
Runs the test suite using Python's unittest framework.

### `make clean`
Removes all build artifacts, temporary files, and Python bytecode.

## Implementation Plan

This implementation follows a structured approach as outlined in `implementation_checklist.md`:

1. **Step 0: Project Setup and Infrastructure** ✓
2. **Step 1: Core Data Structures** - Grid and Droplet classes
3. **Step 2: The Execution Engine** - Main execution loop
4. **Step 3: Basic Flow Control Pipes** - Pipe characters and collision detection
5. **Step 4: Program Start & Basic Output** - Entry point and numeric output
6. **Step 5: Unary Operators** - Increment and decrement operations
7. **Step 6: Conditional Branching** - Corner pipes for flow control
8. **Step 7: Data Sources** - Number sources and tape reader
9. **Step 8: The Data Stack (Part 1)** - Basic stack operations
10. **Step 9: The Data Stack (Part 2)** - Arithmetic and comparison operators
11. **Step 10: Advanced I/O** - Character input/output and tape reader
12. **Step 11: Interactive Input** - User input operations
13. **Step 12: The Reservoir** - Random-access memory
14. **Step 13: Subroutines** - Call/return mechanism
15. **Step 14: Final Review and Polish** - Testing and optimization

## Key Features

- **Visual Programming**: 2D grid-based program representation
- **Dataflow Execution**: Droplets carrying values through pipe networks
- **Stack-Based Operations**: Global LIFO data stack for complex computations
- **Random-Access Memory**: The Reservoir for variable storage
- **Subroutine Support**: Call/return mechanism for code reuse
- **Interactive I/O**: Character and numeric input/output capabilities

## Examples

### Hello, World!
```
  @
  |
 >Hello, World!
  |
  !
```

### Simple Counter
```
  @
  |
  5
  |
  n
```

## Development

To work with this project:

1. **Setup**: Ensure all directories are created and dependencies are installed
2. **Build**: Run `make compile` to build the executable
3. **Test**: Run `make test` to execute the test suite
4. **Clean**: Run `make clean` to remove build artifacts

## Specification

The complete language specification is available in `tubular_spec.md`, including detailed descriptions of all pipe characters, operators, and behavior guidelines.

## License

This project is part of an educational implementation of the Tubular programming language specification.

## Contributing

This implementation follows the official Tubular v1.1 specification. Contributions should maintain compatibility with the specification and include appropriate tests.