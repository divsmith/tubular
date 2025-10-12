# CLI API Contract: Tubular Interpreter

**Feature**: `001-a-basic-initial` | **Date**: 2025-10-11

## Command Line Interface

### Main Command
```bash
tubular [OPTIONS] <INPUT_FILE>
```

#### Arguments
- **INPUT_FILE** (required): Path to the Tubular program file (.tb extension)

#### Options
- **`-v, --verbose`**: Enable verbose output with execution details
- **`-t, --ticks <COUNT>`**: Maximum number of ticks to execute (default: unlimited)
- **`--trace`**: Enable step-by-step execution tracing
- **`--benchmark`**: Run performance benchmarks
- **`-h, --help`**: Print help information
- **`--version`**: Print version information

#### Examples
```bash
# Basic execution
tubular hello_world.tb

# Verbose execution with tick limit
tubular --verbose --ticks 1000 complex_program.tb

# Step-by-step debugging
tubular --trace countdown.tb

# Performance benchmarking
tubular --benchmark --ticks 10000 performance_test.tb
```

### Subcommands

#### validate
```bash
tubular validate [OPTIONS] <INPUT_FILE>
```
Validate program syntax without execution.

**Options**:
- **`--strict`**: Enable strict validation mode

**Exit Codes**:
- 0: Valid program
- 1: Syntax errors found
- 2: File not found or unreadable

#### run
```bash
tubular run [OPTIONS] <INPUT_FILE>
```
Execute program with interactive input support.

**Options**:
- **`-i, --interactive`**: Enable interactive input mode
- **`--input <STRING>`**: Provide input as command line argument

#### benchmark
```bash
tubular benchmark [OPTIONS] <INPUT_FILE>
```
Run comprehensive performance benchmarks.

**Options**:
- **`--iterations <COUNT>`**: Number of benchmark iterations (default: 10)
- **`--output <FORMAT>`**: Output format (json, csv, table) (default: table)

## Input Format Specification

### File Format
- **Encoding**: UTF-8 ASCII
- **Line Endings**: LF or CRLF
- **File Extension**: `.tb` (recommended)

### Grid Structure
- Characters represent program elements
- Whitespace preserved as empty cells
- Lines can be of varying lengths
- Minimum grid size: 1x1
- Maximum supported grid size: 1000x1000

### Valid Characters
```
Flow Control:
| ^ - / \  # Pipes and directional flow

Start/Sink:
@           # Start symbol (exactly one required)
!           # Sink symbol (destroys droplets)

Data Sources:
0-9         # Numeric literals (0-9)
>           # Tape reader
?           # Character input
??          # Numeric input

Data Sinks:
,           # Character output
n           # Numeric output

Unary Operators:
+           # Increment
~           # Decrement

Stack Operations:
:           # Push
;           # Pop
d           # Duplicate
A S M D = < > %  # Arithmetic operations

Memory Operations:
G           # Get from reservoir
P           # Put to reservoir

Subroutine Operations:
C           # Call subroutine
R           # Return from subroutine
```

## Output Format Specification

### Standard Output
Program output follows these rules:
- Character output (`,`) writes single character without newline
- Numeric output (`n`) writes decimal representation without newline
- Multiple output operations concatenate in execution order

### Verbose Output
When `--verbose` is enabled:
```
[TICK 001] Droplet(0) at (2,3) value=5 direction=Down
[TICK 001] Droplet(1) at (4,1) value=0 direction=Right
[TICK 001] Stack: [5, 0]
[TICK 001] Output: Hello
[TICK 002] Droplet(0) at (2,4) value=6 direction=Down (incremented)
[TICK 002] Collision at (3,5): Droplet(1) and Droplet(2) destroyed
```

### Trace Output
When `--trace` is enabled:
```
TRACE | TICK 001 | Droplet 0 | (2,3) -> (2,4) | value: 5 -> 5 | dir: Down -> Down
TRACE | TICK 001 | Cell (+)  | Increment: 5 -> 6
TRACE | TICK 001 | Stack    | Push: 6 | Stack: [5, 6]
TRACE | TICK 001 | Output   | Character: 'A' (65)
```

### Benchmark Output
Default (table format):
```
Benchmark Results for complex_program.tb
========================================
Metric                    | Value        | Unit
---------------------------|--------------|------
Execution Time            | 1,234        | ms
Total Ticks               | 5,432        | ticks
Peak Droplet Count        | 127          | droplets
Peak Memory Usage         | 15.7         | MB
Instructions per Second   | 4,402        | ops/sec
```

JSON format (`--output json`):
```json
{
  "program": "complex_program.tb",
  "timestamp": "2025-10-11T14:30:00Z",
  "results": {
    "execution_time_ms": 1234,
    "total_ticks": 5432,
    "peak_droplet_count": 127,
    "peak_memory_usage_mb": 15.7,
    "instructions_per_second": 4402
  }
}
```

## Error Handling

### Error Messages Format
```
Error: [ERROR_CODE] - Description
File: example.tb, Line: 5, Column: 3
Context: |@--|
          | ^ Invalid start symbol placement
```

### Error Codes
- **E001**: File not found or unreadable
- **E002**: Invalid file encoding (non-ASCII)
- **E003**: No start symbol found
- **E004**: Multiple start symbols found
- **E005**: Invalid character in program
- **E006**: Grid size exceeds limits
- **E007**: Execution timeout (tick limit exceeded)
- **E008**: Memory allocation failure
- **E009**: Internal interpreter error

### Exit Codes
- **0**: Success
- **1**: Program execution error
- **2**: File/input error
- **3**: Validation error
- **4**: Internal error
- **130**: Interrupted (Ctrl+C)

## Environment Variables

### Configuration
- **`TUBULAR_DEBUG`**: Enable debug logging (same as --verbose)
- **`TUBULAR_TICK_LIMIT`**: Default tick limit (overrides built-in default)
- **`TUBULAR_MEMORY_LIMIT`**: Memory limit in MB (default: 1000)
- **`NO_COLOR`**: Disable colored output

### Examples
```bash
# Set default tick limit
export TUBULAR_TICK_LIMIT=10000
tubular large_program.tb

# Enable debug mode
TUBULAR_DEBUG=1 tubular program.tb

# Increase memory limit
TUBULAR_MEMORY_LIMIT=2000 tubular memory_intensive.tb
```