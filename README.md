# Tubular: A 2D Grid-Based Programming Language

[![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-green?style=for-the-badge)](Cargo.toml)

Tubular is a 2D grid-based programming language where programs are visual pipe systems. Droplets of data flow through ASCII grids, and their positions determine computation.

## ✨ Features

- **Visual Programming**: Write programs as 2D ASCII grids with pipe-based flow control
- **Droplet Execution**: Multiple droplets execute simultaneously with collision detection
- **Rich Operations**: Arithmetic, stack operations, memory access, and subroutines
- **Interactive I/O**: Character and numeric input/output support
- **High Performance**: Efficient interpreter supporting large programs (1000x1000 grids)
- **Comprehensive Tooling**: Validation, benchmarking, and debugging
- **Turing Complete**: Full computational power with arbitrary precision integers

## 🚀 Quick Start

### Installation

**From Source** (requires Rust 1.75+):
```bash
git clone https://github.com/divsmith/tubular.git
cd tubular
cargo install --path .
```

**Using Cargo**:
```bash
cargo install tubular
```

### Your First Program

Create `simple.tb`:

```
@       # Start point
|       # Vertical pipe
7       # Number 7
2       # Number 2
-
n       # Output as number
!       # Output sink
```

Run it:
```bash
tubular simple.tb
```

Output:
```
2
```

**How it works**: The droplet starts with value 0, moves down through the pipe `|`, encounters `7` and sets its value to 7, then encounters `2` and overwrites its value to 2, flows through `-` (horizontal pipe), and outputs the final value `2`.

#### Character Output Example

Create `hello.tb`:

```
@       # Start point
|       # Vertical pipe
7       # Number 7
-
,       # Character output (ASCII character 7: bell)
!
```

Run it:
```bash
tubular hello.tb
```

Output:
```
[bell sound]
```

The comma operator outputs the droplet's value as an ASCII character.

#### Arithmetic Example

For arithmetic operations, you need to use the stack. Here's how to compute 7 - 2:

```
@       # Start point
|       # Vertical pipe
7       # Number 7
:       # Push to stack
2       # Number 2
S       # Subtract (7 - 2)
n       # Output as number
!       # Output sink
```

Run it:
```bash
tubular subtraction.tb
```

Output:
```
-7
```

**How it works**: The droplet pushes 7 to the stack with `:`, then its value becomes 2, then the `S` operation pops both values (2 and 7) from the stack, subtracts them (2 - 7 = -7), and sets the droplet's value to the result.

#### Simple Increment

For simple arithmetic, you can use unary operators:

```
@       # Start point
|       # Vertical pipe
7       # Number 7
+       # Increment
+       # Increment
n       # Output as number
!       # Output sink
```

Output:
```
9
```

**How it works**: The droplet starts with value 7, encounters two `+` operators, and increments its value twice (7 → 8 → 9).

## 📖 Tutorial

### Basic Concepts

1. **Grid**: A 2D grid of ASCII characters forms your program
2. **Droplets**: Data units flow through pipes with values and directions
3. **Execution**: Starts at `@` with a droplet of value 0 moving downward
4. **Ticks**: Discrete time units where all droplets move simultaneously

### Essential Symbols

| Symbol | Name | Function |
|--------|------|----------|
| `@` | Start | Creates initial droplet (value 0, direction down) |
| `|` | Vertical Pipe | Guides droplets up/down |
| `-` | Horizontal Pipe | Guides droplets left/right |
| `/` `\` | Corner Pipes | Redirect flow with conditional branching |
| `0-9` | Numbers | Create droplets with numeric values |
| `!` | Output Sink | Consumes droplets and outputs their values |
| `,` `n` | Output | Print droplet value as character/number |
| `?` `??` | Input | Read character/number from user |
| `+` `~` | Unary | Increment/decrement droplet value |
| `:` `;` `d` | Stack | Push/pop/duplicate values on data stack |
| `A` `S` `M` `D` | Arithmetic | Add, subtract, multiply, divide |
| `G` `P` | Memory | Get/put values from reservoir |
| `C` `R` | Subroutines | Call/return from functions |

### Example Programs

#### 1. Countdown Loop
```
      @      # Start
      |      # Flow down
      5      # Set value to 5
      d      # Duplicate for output
      n,     # Output as number
      1-     # Decrement by 1
      d      # Duplicate for conditional
      0\     # Branch: continue if ≠ 0, exit if = 0
       /     # Continue loop
      /      # Loop back up
     /       #
    @        # Start point (visual reference)
```

**Output**: `5,4,3,2,1,`

#### 2. Interactive Calculator
```
@           # Start
|           # Flow down
??          # Read first number
:           # Push to stack
??          # Read second number
:           # Push to stack
S           # Subtract (first - second)
n,          # Output result as number
!           # End
```

**Run**: `tubular calculator.tb` (enter numbers when prompted)

#### 3. Memory Operations
```
@           # Start
|           # Flow down
7           # X coordinate
:           # Push to stack
15          # Y coordinate
:           # Push to stack
42          # Value to store
P           # Put at (7, 15)
|           # Flow down
7           # X coordinate
:           # Push to stack
15          # Y coordinate
:           # Push to stack
G           # Get from (7, 15)
n,          # Output: 42
!           # End
```

## 🛠️ CLI Reference

### Basic Execution
```bash
# Run a program
tubular program.tb

# Verbose execution
tubular --verbose program.tb

# Step-by-step tracing
tubular --trace program.tb

# Limit execution ticks
tubular --ticks 1000 program.tb
```

### Program Validation
```bash
# Validate syntax
tubular validate program.tb

# Strict validation
tubular validate --strict program.tb

# Validate from stdin
cat program.tb | tubular validate
```

### Interactive Programs
```bash
# Run with interactive input
tubular run --interactive program.tb

# Provide input as argument
tubular run --input "42" program.tb
```

### Performance Benchmarking
```bash
# Basic benchmark
tubular benchmark program.tb

# Detailed benchmark
tubular benchmark --iterations 100 --output json --save results.json program.tb

# Compare multiple programs
tubular benchmark program1.tb --compare program2.tb program3.tb
```

### Environment Variables
Configure default behavior:

```bash
export TUBULAR_TICK_LIMIT=5000      # Default tick limit
export TUBULAR_VERBOSE=true        # Enable verbose output
export TUBULAR_TRACE=false         # Disable tracing by default
export TUBULAR_BENCHMARK=true      # Enable benchmarking
export TUBULAR_STRICT=true         # Enable strict validation
```

## 📊 Performance

### Benchmarks
The interpreter delivers high performance:

- **Execution Speed**: ~50,000+ ticks per second
- **Memory Usage**: Efficient grid representation (~1MB per 1000x1000 grid)
- **Concurrency**: Supports 1000+ simultaneous droplets
- **Stack Depth**: 1000+ levels with arbitrary precision integers

### Example Benchmarks
```bash
$ tubular benchmark examples/countdown.tb --iterations 1000

Benchmark Results
=================
Program: examples/countdown.tb
Iterations: 1000
Average execution time: 0.245 ms
Average total ticks: 35
Average peak droplets: 1
Average memory usage: 0.012 MB
Instructions per second: 142,857
```

## 🧰 Development

### Project Structure
```
tubular/
├── src/                    # Core interpreter implementation
│   ├── interpreter/        # Execution engine
│   ├── operations/         # Language operations
│   ├── parser/            # Grid parsing and validation
│   ├── types/             # Core data types
│   └── cli/               # Command-line interface
├── examples/              # Example programs
├── tests/                 # Test suites
├── benches/               # Performance benchmarks
└── docs/                  # Additional documentation
```

### Building from Source
```bash
# Clone repository
git clone https://github.com/yourusername/tubular.git
cd tubular

# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Running Tests
```bash
# All tests
cargo test

# Specific test categories
cargo test unit
cargo test integration
cargo test property

# Tests with output
cargo test -- --nocapture
```

### Development Tools
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Watch for changes and re-run tests
cargo watch -x test

# Watch and run example
cargo watch -x "run -- examples/hello_world.tb"
```

## 🔧 Troubleshooting

### Common Issues

#### No Output
**Problem**: Program runs but produces no output
**Solution**: Ensure droplets reach an output sink (`!`, `,`, or `n`)
```bash
tubular --trace program.tb  # See execution flow
```

#### Infinite Loops
**Problem**: Program runs forever
**Solution**: Set tick limit or check conditional logic
```bash
tubular --ticks 100 program.tb  # Limit execution
```

#### Syntax Errors
**Problem**: Validation fails
**Solution**: Use verbose validation to see detailed errors
```bash
tubular validate --strict program.tb
```

#### Stack Underflow
**Problem**: Arithmetic operations fail
**Solution**: Ensure stack has enough values before operations
```bash
tubular --trace program.tb  # Monitor stack operations
```

### Debugging Tips

1. **Use Trace Mode**: See step-by-step execution
   ```bash
   tubular --trace program.tb
   ```

2. **Validate First**: Check syntax before running
   ```bash
   tubular validate program.tb && tubular program.tb
   ```

3. **Start Simple**: Test with minimal examples first
4. **Check Flow**: Ensure pipes connect properly
5. **Monitor Stack**: Use verbose mode to see stack changes

## 📚 Language Reference

### Complete Symbol Reference

#### Flow Control
- `@` - Start point (creates droplet with value 0, direction down)
- `|` - Vertical pipe (up/down flow)
- `-` - Horizontal pipe (left/right flow)
- `^` - Go up pipe (changes direction to up)
- `#` - Wall (stops and destroys droplets)

#### Corner Pipes (Conditional)
- `/` - Forward slash corner
  - From top: 0 → left, non-zero → right
  - From bottom: 0 → right, non-zero → left
  - From left → up, from right → down
- `\` - Back slash corner
  - From top: 0 → right, non-zero → left
  - From bottom: 0 → left, non-zero → right
  - From left → down, from right → up

#### Data Sources
- `0-9` - Number literals (create droplets with that value)
- `>` - Greater than flow control or operator
- `?` - Character input (read single character, returns ASCII code or -1 for EOF)
- `??` - Numeric input (read line, parse as integer, returns 0 on parse failure)

#### Data Sinks
- `!` - Output sink (outputs value, adds newline)
- `,` - Character output (outputs as ASCII character, no newline)
- `n` - Numeric output (outputs as number, no newline)

#### Unary Operators
- `+` - Increment (add 1 to droplet value)
- `~` - Decrement (subtract 1 from droplet value)

#### Stack Operations
- `:` - Push (push droplet value to stack)
- `;` - Pop (pop from stack, becomes droplet value)
- `d` - Duplicate (push copy of droplet value to stack)
- `A` - Add (pop two values, push sum)
- `S` - Subtract (pop two values, push difference)
- `M` - Multiply (pop two values, push product)
- `D` - Divide (pop two values, push integer division)
- `=` - Equal (pop two values, push 1 if equal, 0 if not)
- `<` - Less than (pop two values, push 1 if b < a, 0 if not)
- `>` - Greater than (pop two values, push 1 if b > a, 0 if not)
- `%` - Modulo (pop two values, push remainder)

#### Memory Operations (Reservoir)
- `G` - Get (pop y, x coordinates, push value from memory)
- `P` - Put (pop y, x, value, store value at coordinates)

#### Subroutine Operations
- `C` - Call (pop y, x coordinates, jump to subroutine)
- `R` - Return (return from subroutine to call location)

### Advanced Concepts

#### Droplet Collisions
When two droplets enter the same cell in the same tick, both are destroyed. This can be used for computation and synchronization.

#### Arbitrary Precision Integers
All numeric operations support arbitrary precision integers, enabling computation with very large numbers.

#### Reservoir Memory
The reservoir is an unbounded 2D memory grid with negative coordinate support. Uninitialized locations return 0.

#### Call Stack
Subroutines use a dedicated call stack for nested function calls with proper return handling.

## 🤝 Contributing

We welcome contributions!

### Getting Started

1. **Fork the Repository**
   ```bash
   git clone https://github.com/yourusername/tubular.git
   ```

2. **Create a Feature Branch**
   ```bash
   git checkout -b feature-name
   ```

3. **Make Your Changes**
   - Write clean, documented code
   - Add tests for new functionality
   - Ensure all existing tests pass

4. **Run Tests and Checks**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Submit a Pull Request**
   - Provide clear description of changes
   - Include tests and documentation
   - Link to any relevant issues

### Contribution Areas

- **Performance Optimization**: Improve interpreter speed and memory usage
- **Language Features**: Add new operations or capabilities
- **Tooling**: Enhance CLI tools and debugging capabilities
- **Examples**: Create interesting example programs
- **Documentation**: Improve guides and reference material
- **Testing**: Add comprehensive test coverage

### Development Standards

- Follow Rust best practices and idioms
- Maintain backward compatibility where possible
- Write comprehensive tests for new features
- Document public APIs and complex algorithms
- Use meaningful commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by esoteric programming languages and visual programming paradigms
- Built with Rust for performance and safety
- Community contributors and feedback
- Testers who helped identify bugs and improvements

## 🔗 Resources

- [Language Specification](docs/specification.md) - Complete formal language definition
- [Examples Gallery](examples/) - Collection of example programs
- [Performance Guide](docs/performance.md) - Optimization techniques and benchmarks
- [API Documentation](docs/api/) - Internal API reference
- [Community Forum](https://github.com/yourusername/tubular/discussions) - Discussion and support

---

**Tubular** - Where code flows like water 🌊

Made with ❤️ by the Tubular development team