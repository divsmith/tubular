# Tubular Language Reference

**Version**: 0.1.0
**Date**: 2025-10-12
**Feature**: `001-a-basic-initial`

## Table of Contents

1. [Language Overview and Philosophy](#language-overview-and-philosophy)
2. [Program Structure and Syntax](#program-structure-and-syntax)
3. [Complete Symbol Reference](#complete-symbol-reference)
4. [Flow Control Operations](#flow-control-operations)
5. [Arithmetic and Stack Operations](#arithmetic-and-stack-operations)
6. [Input/Output Operations](#inputoutput-operations)
7. [Memory and Subroutine Operations](#memory-and-subroutine-operations)
8. [Advanced Programming Patterns](#advanced-programming-patterns)
9. [Performance Optimization](#performance-optimization)
10. [Troubleshooting and Debugging](#troubleshooting-and-debugging)
11. [Language Implementation Details](#language-implementation-details)

## Language Overview and Philosophy

Tubular is a 2D grid-based esoteric programming language where computation occurs through the flow of "droplets" through a system of pipes and operators. The language draws inspiration from fluid dynamics and visual programming paradigms.

### Core Philosophy

- **Visual Programming**: Programs are ASCII grids where the spatial arrangement of symbols determines execution flow
- **Data Flow**: Computation is modeled as droplets carrying integer values flowing through pipes
- **Deterministic**: Given the same input and program, execution always produces the same result
- **Minimalist**: Uses single-character symbols for all operations, creating concise visual programs
- **Turing Complete**: Despite the simple appearance, supports arbitrary computation with loops, conditionals, and memory

### Key Concepts

- **Droplets**: Active execution units containing an integer value and travel direction
- **Grid**: 2D ASCII canvas where programs are laid out visually
- **Ticks**: Discrete time units where all droplets move simultaneously
- **Collisions**: When droplets meet, both are destroyed (can be used for computation)
- **Stack**: LIFO data structure for temporary value storage
- **Reservoir**: 2D unbounded memory for persistent data storage

## Program Structure and Syntax

### Basic Program Anatomy

A Tubular program consists of ASCII characters arranged in a 2D grid with the following essential components:

```
@       # Start point (exactly one required)
|       # Vertical pipe for flow control
5       # Data source (sets droplet value)
+       # Operation (increments droplet value)
!       # Sink (destroys droplet, ends execution)
```

### Grid Coordinates

- Grid positions are referenced as (x, y) where:
  - x: column number (0-based, left to right)
  - y: row number (0-based, top to bottom)
- Negative coordinates are supported in the reservoir (memory) only

### Program Execution Flow

1. **Initialization**: Droplet created at `@` symbol with value 0, direction Down
2. **Movement**: Each tick, droplets move one cell in their current direction
3. **Processing**: When entering a cell, droplets interact with the symbol
4. **Termination**: Program ends when all droplets are destroyed

### Valid Characters

| Category | Symbols | Description |
|----------|---------|-------------|
| Flow Control | `|` `-` `/` `\` `^` | Pipes and directional flow |
| Start/End | `@` `!` | Start point and sink |
| Data Sources | `0-9` `>` `?` `??` | Numbers and input |
| Data Sinks | `,` `n` | Output operations |
| Unary Ops | `+` `~` | Increment/decrement |
| Stack Ops | `:` `;` `d` `A` `S` `M` `D` `=` `<` `>` `%` | Stack manipulation |
| Memory | `G` `P` | Reservoir operations |
| Subroutines | `C` `R` | Function calls |

## Complete Symbol Reference

### Flow Control Symbols

#### `@` - Start Point
- **Function**: Creates initial droplet
- **Value**: 0
- **Direction**: Down
- **Usage**: Exactly one required per program
- **Example**:
  ```
  @
  ```

#### `|` - Vertical Pipe
- **Function**: Guides droplets up/down
- **Direction Change**: None
- **Bidirectional**: Supports both up and down flow
- **Example**:
  ```
  @
  |
  5
  |
  !
  ```

#### `-` - Horizontal Pipe
- **Function**: Guides droplets left/right
- **Direction Change**: None
- **Bidirectional**: Supports both left and right flow
- **Example**:
  ```
  @--5--!
  ```

#### `/` - Forward Slash Corner (Conditional)
- **Function**: Redirects flow with conditional branching
- **Conditional Behavior**: Based on droplet value (zero vs non-zero)

| From Direction | Zero Value | Non-zero Value |
|----------------|------------|----------------|
| Top (Down) | Left | Right |
| Bottom (Up) | Right | Left |
| Left (Right) | Up | Up |
| Right (Left) | Down | Down |

- **Example**:
  ```
  @
  |
  5
  0\     # Continues left if value ≠ 0
    /    # Turns right if value = 0
  ```

#### `\` - Back Slash Corner (Conditional)
- **Function**: Redirects flow with conditional branching
- **Conditional Behavior**: Based on droplet value (zero vs non-zero)

| From Direction | Zero Value | Non-zero Value |
|----------------|------------|----------------|
| Top (Down) | Right | Left |
| Bottom (Up) | Left | Right |
| Left (Right) | Down | Down |
| Right (Left) | Up | Up |

- **Example**:
  ```
  @
  |
  0      # Value 0
  \
   \     # Goes right (zero value)
  ```

#### `^` - Go Up Pipe
- **Function**: Forces droplet direction to Up
- **Direction Change**: Always Up
- **Usage**: Override current direction
- **Example**:
  ```
  @--^
  ```

### Data Source Symbols

#### `0`-`9` - Number Literals
- **Function**: Sets droplet value to the digit
- **Value**: Corresponding digit (0-9)
- **Direction**: Unchanged
- **Example**:
  ```
  @
  |
  5       # Droplet value becomes 5
  ```

#### `>` - Tape Reader
- **Function**: Emits ASCII values for adjacent characters
- **Value**: ASCII code of character to the right
- **Direction**: Unchanged
- **Usage**: For embedding literal strings in programs
- **Example**:
  ```
  @
  |
  >Hello   # Emits ASCII codes for H, e, l, l, o
  ```

#### `?` - Character Input
- **Function**: Reads single character from stdin
- **Value**: ASCII code of input character, -1 for EOF
- **Direction**: Unchanged
- **Interactive**: Requires user input
- **Example**:
  ```
  @
  |
  ?       # Waits for user to type a character
  n,      # Outputs the character
  !
  ```

#### `??` - Numeric Input
- **Function**: Reads integer from stdin
- **Value**: Parsed integer value, 0 on parse failure
- **Direction**: Unchanged
- **Interactive**: Requires user input
- **Example**:
  ```
  @
  |
  ??      # Waits for user to type a number
  n,      # Echoes the number
  !
  ```

### Data Sink Symbols

#### `!` - Output Sink
- **Function**: Destroys droplet (ends execution)
- **Output**: None
- **Usage**: Program termination point
- **Example**:
  ```
  @--!
  ```

#### `,` - Character Output
- **Function**: Outputs droplet value as ASCII character
- **Output**: Single character (no newline)
- **Droplet**: Continues execution
- **Value Range**: 0-127 for standard ASCII
- **Example**:
  ```
  @
  |
  65,     # Outputs 'A'
  !
  ```

#### `n` - Numeric Output
- **Function**: Outputs droplet value as decimal number
- **Output**: Decimal string (no newline)
- **Droplet**: Continues execution
- **Example**:
  ```
  @
  |
  42
  n,      # Outputs "42"
  !
  ```

### Unary Operator Symbols

#### `+` - Increment
- **Function**: Adds 1 to droplet value
- **Value**: value + 1
- **Direction**: Unchanged
- **Precision**: Arbitrary precision integers
- **Example**:
  ```
  @
  |
  5
  +       # Value becomes 6
  n,
  !
  ```

#### `~` - Decrement
- **Function**: Subtracts 1 from droplet value
- **Value**: value - 1
- **Direction**: Unchanged
- **Precision**: Arbitrary precision integers
- **Example**:
  ```
  @
  |
  5
  ~       # Value becomes 4
  n,
  !
  ```

### Stack Operation Symbols

#### `:` - Push
- **Function**: Pushes droplet value to data stack
- **Stack**: push(droplet.value)
- **Droplet Value**: Unchanged
- **Example**:
  ```
  @
  |
  5:
  3:
  A       # Adds 5 and 3 from stack
  n,
  !
  ```

#### `;` - Pop
- **Function**: Pops value from data stack to droplet
- **Droplet Value**: stack.pop() (or 0 if empty)
- **Stack**: Removes top value
- **Underflow**: Returns 0, no error
- **Example**:
  ```
  @
  |
  5:
  ;
  n,      # Outputs 5
  !
  ```

#### `d` - Duplicate
- **Function**: Pushes copy of top stack value
- **Stack**: push(stack.peek())
- **Droplet Value**: Unchanged
- **Empty Stack**: Pushes 0
- **Example**:
  ```
  @
  |
  5:
  d       # Stack: [5, 5]
  A       # Adds them: 5 + 5 = 10
  n,
  !
  ```

#### `A` - Add
- **Function**: Adds two stack values
- **Stack**: pop() + pop()
- **Droplet Value**: Unchanged
- **Stack Result**: push(sum)
- **Underflow**: Missing values treated as 0
- **Example**:
  ```
  @
  |
  5:
  3:
  A       # Pushes 8 to stack
  ;
  n,      # Outputs 8
  !
  ```

#### `S` - Subtract
- **Function**: Subtracts two stack values
- **Stack**: second - first (pop order matters)
- **Stack Result**: push(difference)
- **Formula**: result = second_pop - first_pop
- **Example**:
  ```
  @
  |
  3:
  5:
  S       # 5 - 3 = 2
  ;
  n,      # Outputs 2
  !
  ```

#### `M` - Multiply
- **Function**: Multiplies two stack values
- **Stack**: pop() × pop()
- **Stack Result**: push(product)
- **Precision**: Arbitrary precision
- **Example**:
  ```
  @
  |
  6:
  7:
  M       # 6 × 7 = 42
  ;
  n,      # Outputs 42
  !
  ```

#### `D` - Divide
- **Function**: Integer division of two stack values
- **Stack**: second ÷ first
- **Stack Result**: push(quotient)
- **Division by Zero**: Returns 0
- **Integer Division**: Truncates toward zero
- **Example**:
  ```
  @
  |
  3:
  7:
  D       # 7 ÷ 3 = 2
  ;
  n,      # Outputs 2
  !
  ```

#### `=` - Equal
- **Function**: Compares two stack values for equality
- **Stack**: second == first
- **Stack Result**: push(1 if equal, 0 if not)
- **Example**:
  ```
  @
  |
  5:
  5:
  =       # 5 == 5, pushes 1
  ;
  n,      # Outputs 1
  !
  ```

#### `<` - Less Than
- **Function**: Compares if second < first
- **Stack**: second < first
- **Stack Result**: push(1 if true, 0 if false)
- **Example**:
  ```
  @
  |
  7:
  5:
  <       # 5 < 7, pushes 1
  ;
  n,      # Outputs 1
  !
  ```

#### `>` - Greater Than
- **Function**: Compares if second > first
- **Stack**: second > first
- **Stack Result**: push(1 if true, 0 if false)
- **Example**:
  ```
  @
  |
  5:
  7:
  >       # 7 > 5, pushes 1
  ;
  n,      # Outputs 1
  !
  ```

#### `%` - Modulo
- **Function**: Remainder of integer division
- **Stack**: second % first
- **Stack Result**: push(remainder)
- **Modulo by Zero**: Returns 0
- **Sign**: Result has same sign as second operand
- **Example**:
  ```
  @
  |
  3:
  7:
  %       # 7 % 3 = 1
  ;
  n,      # Outputs 1
  !
  ```

### Memory Operation Symbols

#### `G` - Get from Reservoir
- **Function**: Retrieves value from 2D memory
- **Coordinate**: (x=droplet.value, y=stack.pop())
- **Droplet Value**: memory[x,y] (or 0 if unset)
- **Memory**: Unbounded 2D grid with negative coordinate support
- **Uninitialized**: Returns 0
- **Example**:
  ```
  @
  |
  7       # X coordinate
  :
  15      # Y coordinate
  G       # Gets value from (7, 15)
  n,      # Output retrieved value
  !
  ```

#### `P` - Put to Reservoir
- **Function**: Stores value in 2D memory
- **Coordinate**: (x=droplet.value, y=stack.pop())
- **Value Stored**: Current droplet value
- **Memory**: Overwrites existing value
- **Persistence**: Values persist between droplet accesses
- **Example**:
  ```
  @
  |
  7       # X coordinate
  :
  15      # Y coordinate
  :
  42      # Value to store
  P       # Stores 42 at (7, 15)
  !
  ```

### Subroutine Operation Symbols

#### `C` - Call Subroutine
- **Function**: Jumps to subroutine location
- **Target**: (x=droplet.value, y=stack.pop())
- **Direction**: stack.pop() (0=Up, 1=Right, 2=Down, 3=Left)
- **Call Stack**: Pushes current position and direction
- **Example**:
  ```
  @
  |
  1       # X coordinate of subroutine
  :
  5       # Y coordinate of subroutine
  :
  2       # Direction after jump (Down)
  C       # Call subroutine at (1, 5)
  !
  ```

#### `R` - Return from Subroutine
- **Function**: Returns to caller location
- **Call Stack**: Pops position and direction
- **Empty Stack**: No operation (continues execution)
- **Example**:
  ```
  # Subroutine at (1, 5)
  1,5
  |
  72,     # Output 'H'
  R       # Return to caller
  ```

## Flow Control Operations

### Conditional Branching

Conditional branching is achieved through the corner pipes `/` and `\`, which direct droplets based on whether their value is zero or non-zero.

#### Zero/Non-zero Patterns

**Countdown Loop**:
```
     @
     |
     5      # Start with value 5
     d,     # Output and duplicate
     n,
     1-     # Decrement
     0\     # Continue if ≠ 0
      /
     /
    @      # Visual reference
```

**Conditional Execution**:
```
@       # Start
|
??      # Read number
d       # Duplicate
0\      # If zero, go left
 /       # If non-zero, go right
 |       # Right path: output "Non-zero"
 n,
!
```

### Loop Patterns

#### Fixed Count Loop
```
     @
     |
     5      # Loop counter
     d      # Duplicate counter
     n,     # Output current count
     1-     # Decrement
     d      # Duplicate for check
     0\     # Continue if not zero
      /
     /
    @
```

#### Input-Driven Loop
```
@       # Start
|
??      # Read input value
d       # Duplicate
0\      # Continue if not zero
 /       # Exit if zero
 |       # Loop body
 n,      # Process value
 +       # Modify value
 \       # Loop back
```

### Advanced Flow Control

#### Multi-way Branching
```
@       # Start
|
??      # Read number
d       # Duplicate
1=      # Compare with 1
0\      # If not 1, continue
 /       # If 1, go to case 1
 |       # Case 1 handler
 ...
```

#### Maze Navigation
```
@-----\
       |
       >-----!
```

## Arithmetic and Stack Operations

### Stack Manipulation Patterns

#### Stack-based Calculator
```
@       # Start
|
??      # Read first number
:       # Push to stack
??      # Read second number
:       # Push to stack
A       # Add them
;
n,      # Output result
!
```

#### Stack Operations Chain
```
@       # Start
|
5       # First number
:       # Push
3       # Second number
:       # Push
d       # Duplicate top (3)
M       # Multiply: 3 × 3 = 9
A       # Add: 5 + 9 = 14
;
n,      # Output 14
!
```

### Arithmetic Expressions

#### Complex Expression: (5 + 3) × 2 - 1
```
@       # Start
|
5       # First operand
:       # Push
3       # Second operand
:       # Push
A       # Add: 5 + 3 = 8
:       # Push result
2       # Multiplier
:       # Push
M       # Multiply: 8 × 2 = 16
:       # Push result
1       # Subtrahend
:       # Push
S       # Subtract: 16 - 1 = 15
;
n,      # Output 15
!
```

#### Factorial Calculation (Stack-based)
```
@       # Start
|
5       # Number to factorial
:       # Push original n
1       # Start with 1
:       # Push accumulator
d       # Duplicate current number
1=      # Compare with 1
0\      # Continue if not 1
 /       # Exit if 1
 |       # Loop body
 M       # Multiply accumulator by current number
 ~       # Decrement current number
 d       # Duplicate for next iteration
 \       # Loop back
```

### Comparison Operations

#### Number Comparison
```
@       # Start
|
??      # Read first number
:       # Push
??      # Read second number
:       # Push
>       # Compare: second > first
;
d       # Duplicate result
n,      # Output comparison result (0 or 1)
0\      # Branch based on result
 /       # Second > First
 |       # Output "Greater"
 n,
!
```

#### Range Checking
```
@       # Start
|
??      # Read number to check
:       # Push
10      # Upper bound
:       # Push
<       # Compare: number < 10
;
0\      # If not less than 10
 /       # Number >= 10
 |       # Output "Out of range"
 n,
!
\       # Number < 10
 |       # Continue with lower bound check
...
```

## Input/Output Operations

### Character Output Patterns

#### Hello World
```
@       # Start
|
72,     # 'H'
|
101,    # 'e'
|
108,    # 'l'
|
108,    # 'l'
|
111,    # 'o'
|
44,     # ','
|
32,     # ' '
|
87,     # 'W'
|
111,    # 'o'
|
114,    # 'r'
|
108,    # 'l'
|
100,    # 'd'
|
33,     # '!'
|
!       # End
```

#### Character by Character Output
```
@       # Start
|
>       # Tape reader
Hello   # Characters to output
|
!       # End
```

### Numeric Output Patterns

#### Number Formatting
```
@       # Start
|
42      # Number to output
:       # Push to stack
d       # Duplicate
n,      # Output as number
,       # Output as newline (ASCII 10)
!
```

#### Multiple Number Output
```
@       # Start
|
5       # First number
n,
44,     # Comma separator
3       # Second number
n,
44,     # Comma separator
7       # Third number
n,
!
```

### Input Processing Patterns

#### Echo Program
```
@       # Start
|
?       # Read character
,       # Output character
!       # End
```

#### Number Reader and Processor
```
@       # Start
|
??      # Read number
:       # Push to stack
d       # Duplicate
n,      # Echo number
2       # Multiplier
:       # Push
M       # Multiply: number × 2
;
n,      # Output doubled number
!
```

#### Interactive Calculator
```
@       # Start
|
??      # Read first number
:       # Push
??      # Read second number
:       # Push
A       # Add
;
n,      # Output result
!
```

### Output Formatting

#### Table Format
```
@       # Start
|
1       # Row 1
n,
9,      # Tab character
2       # Value 1
n,
10,     # Newline
|
3       # Row 2
n,
9,      # Tab character
4       # Value 2
n,
10,     # Newline
!
```

## Memory and Subroutine Operations

### Reservoir Memory Patterns

#### Memory Storage and Retrieval
```
@       # Start
|
7       # X coordinate
:       # Push
15      # Y coordinate
:       # Push
42      # Value to store
P       # Put at (7, 15)
|
7       # X coordinate for retrieval
:       # Push
15      # Y coordinate for retrieval
G       # Get from (7, 15)
n,      # Output: 42
!
```

#### Memory Array Operations
```
# Store array elements
@       # Start
|
0       # Index 0
:       # Push
10      # Value 10
P       # Store at (0, 0)
|
1       # Index 1
:       # Push
20      # Value 20
P       # Store at (1, 0)
|
2       # Index 2
:       # Push
30      # Value 30
P       # Store at (2, 0)
|
# Retrieve and sum array elements
0       # Start with index 0
:       # Push
0       # Y coordinate
G       # Get array[0]
:       # Push result
1       # Index 1
:       # Push
0       # Y coordinate
G       # Get array[1]
A       # Add to running total
:       # Push result
2       # Index 2
:       # Push
0       # Y coordinate
G       # Get array[2]
A       # Add to running total
;
n,      # Output sum: 60
!
```

#### Memory as 2D Grid
```
# Initialize 2x2 grid
@
|
0       # X = 0
:
0       # Y = 0
:
1       # Value = 1
P       # grid[0][0] = 1
|
1       # X = 1
:
0       # Y = 0
:
2       # Value = 2
P       # grid[1][0] = 2
|
0       # X = 0
:
1       # Y = 1
:
3       # Value = 3
P       # grid[0][1] = 3
|
1       # X = 1
:
1       # Y = 1
:
4       # Value = 4
P       # grid[1][1] = 4
|
# Diagonal sum: grid[0][0] + grid[1][1]
0       # X = 0
:
0       # Y = 0
G       # Get grid[0][0] = 1
:
1       # X = 1
:
1       # Y = 1
G       # Get grid[1][1] = 4
A       # Sum: 1 + 4 = 5
;
n,      # Output: 5
!
```

### Subroutine Patterns

#### Simple Subroutine Call
```
# Main program
@       # Start
|
1       # X coordinate of subroutine
:       # Push
5       # Y coordinate of subroutine
:       # Push
2       # Direction after jump (Down)
C       # Call subroutine
!
# Subroutine at (1, 5)
1,5
|
72,     # Output 'H'
101,    # Output 'e'
108,    # Output 'l'
108,    # Output 'l'
111,    # Output 'o'
R       # Return to main
```

#### Subroutine with Parameters
```
# Main program
@       # Start
|
5       # Parameter to pass
:       # Push
1       # X coordinate of subroutine
:       # Push
5       # Y coordinate of subroutine
:       # Push
2       # Direction after jump
C       # Call subroutine with parameter
!
# Subroutine at (1, 5) - prints parameter twice
1,5
|
d       # Duplicate parameter
n,      # Output first copy
44,     # Comma
;       # Pop second copy
n,      # Output second copy
R       # Return
```

#### Nested Subroutine Calls
```
# Main program
@       # Start
|
1       # X coordinate of subroutine A
:       # Push
5       # Y coordinate of subroutine A
:       # Push
2       # Direction after jump
C       # Call subroutine A
!
# Subroutine A at (1, 5)
1,5
|
87,     # Output 'W'
111,    # Output 'o'
114,    # Output 'r'
108,    # Output 'l'
100,    # Output 'd'
|
3       # X coordinate of subroutine B
:       # Push
10      # Y coordinate of subroutine B
:       # Push
2       # Direction after jump
C       # Call nested subroutine B
|
R       # Return to main
# Subroutine B at (3, 10)
3,10
|
33,     # Output '!'
R       # Return to subroutine A
```

#### Recursive Subroutine (Limited by Call Stack)
```
# Main program
@       # Start
|
5       # Number for factorial
:       # Push
1       # X coordinate of factorial subroutine
:       # Push
5       # Y coordinate of factorial subroutine
:       # Push
2       # Direction after jump
C       # Call factorial subroutine
!
# Factorial subroutine at (1, 5)
1,5
|
d       # Duplicate current number
1=      # Compare with 1
0\      # If not 1, continue recursion
 /       # If 1, return 1
 |       # Return 1 for base case
 1
 R
 \       # Recursive case
 |       # Continue with factorial calculation
 ~       # Decrement number
 d       # Duplicate for recursive call
 1
 :
 5
 :
 2
 C       # Recursive call
 :
 M       # Multiply result by current number
 R       # Return
```

### Memory and Subroutine Combination

#### Memoization Pattern
```
# Memoized factorial using memory
@       # Start
|
5       # Number for factorial
:       # Push
1       # Check if result already in memory
:       # Push
20      # Y coordinate for memo storage
:       # Push
G       # Check memory[5][20]
d       # Duplicate result
0\      # If not zero, use cached result
 /       # If zero, compute factorial
 |       # Return cached result
 ;
 n,
 !
\       # Compute factorial and store
 |       # ... factorial computation
 d       # Duplicate result
 5       # X coordinate
 :
 20      # Y coordinate
 :
 P       # Store in memory
 ;
 n,
 !
```

## Advanced Programming Patterns

### Parallel Processing

#### Multiple Droplets
```
@       # First droplet
|
5       # Value 5
--\     # Split into two paths
   @    # Second droplet start
   |
   3    # Value 3
   |
   +    # Both paths merge here
   |
   n,   # Output sum: 8
   !
```

#### Collision-based Computation
```
@       # First droplet
|
5       # Value 5
--\
   @    # Second droplet
   |
   3    # Value 3
   |
   --/  # Paths cross, collision destroys both
   !    # No output (both destroyed)
```

### Data Structure Patterns

#### Stack Implementation
```
# Stack operations using memory as backing store
@       # Start
|
??      # Read operation (1=push, 2=pop, 3=peek)
:       # Push operation code
1=      # Compare with push
0\      # If not push
 /       # Push operation
 |       # Handle push
 ??      # Read value to push
 :       # Push value
 0       # Stack pointer X
 :
 0       # Stack pointer Y
 :
 G       # Get current stack pointer
 d       # Duplicate pointer
 1       # Increment
 P       # Store new pointer
 :       # Push original pointer
 G       # Get stack position
 ??      # Read value
 P       # Store value at stack position
 \
```

#### Queue Implementation
```
# Queue using circular buffer in memory
@       # Start
|
??      # Read operation (1=enqueue, 2=dequeue)
:       # Push operation
1=      # Compare with enqueue
0\      # If not enqueue
 /       # Enqueue operation
 |       # Handle enqueue
 ??      # Read value to enqueue
 :       # Push value
 0       # Head pointer X
 :
 0       # Head pointer Y
 :
 G       # Get head pointer
 10      # Queue size
 %       # Calculate position
 :       # Push position
 ??      # Read value
 P       # Store value at position
 \
```

### Algorithm Patterns

#### Bubble Sort (Conceptual)
```
# Sort two numbers
@       # Start
|
??      # Read first number
:       # Push
??      # Read second number
:       # Push
>       # Compare second > first
;
0\      # If not greater
 /       # First number is larger or equal
 |       # Output first then second
 ;
 n,
 44,    # Comma
 ;
 n,
 !
\       # Second number is larger
 |       # Output second then first
 ;
 n,
 44,    # Comma
 ;
 n,
 !
```

#### Binary Search Pattern
```
# Search in sorted array using memory
@       # Start
|
??      # Read target value
:       # Push
0       # Left bound
:       # Push
10      # Right bound
:       # Push
# Binary search loop would continue here
```

### String Processing

#### String Length Calculator
```
@       # Start
|
>       # Tape reader
Hello   # String to measure
|
0       # Initialize counter
:       # Push counter
# Loop through characters would continue here
```

#### Character Frequency Counter
```
@       # Start
|
>       # Tape reader
hello   # String to analyze
|
0       # Initialize frequency array
# Would use memory to store character counts
```

### Mathematical Computations

#### Fibonacci Sequence
```
# Generate Fibonacci numbers
@       # Start
|
0       # First Fibonacci number
:       # Push
1       # Second Fibonacci number
:       # Push
n,      # Output first number
44,     # Comma
;
n,      # Output second number
44,     # Comma
# Loop to generate next numbers
```

#### Prime Number Checker
```
@       # Start
|
??      # Read number to check
:       # Push
2       # Start checking from 2
:       # Push
# Division test loop would continue here
```

### State Machine Patterns

#### Simple State Machine
```
# State machine with 3 states
@       # Start (state 0)
|
0       # Current state
:       # Push
1=      # Check if state 1
0\      # If not state 1
 /       # State 1 handler
 |       # Handle state 1
 2       # Transition to state 2
 :
 P       # Store new state
 \
 |       # Check if state 2
 2=
 0\
 /       # State 2 handler
 |       # Handle state 2
 0       # Transition to state 0
 :
 P       # Store new state
 \
```

#### Parser Pattern
```
# Simple expression parser
@       # Start
|
?       # Read character
:       # Push
48=     # Compare with '0'
0\      # If not '0'
 /       # Handle digit
 |       # Parse number
 \
 |       # Check for operators
 43=     # '+'
 0\
 /       # Handle addition
 \
 |       # Other characters
 \
```

## Performance Optimization

### Efficiency Guidelines

#### Grid Layout Optimization
- **Minimize Path Length**: Place frequently used operations close together
- **Avoid Backtracking**: Design linear flows when possible
- **Localize Related Code**: Group related functionality spatially
- **Use Direct Paths**: Minimize unnecessary pipe segments

#### Stack Optimization
- **Balance Push/Pop**: Maintain reasonable stack depth
- **Minimize Duplication**: Use `d` judiciously to avoid stack bloat
- **Clear Stack**: Clean up unused values to prevent memory growth
- **Plan Stack Usage**: Design algorithms with stack depth in mind

#### Memory Usage Patterns
- **Sparse Memory Access**: Use memory only when necessary
- **Coordinate Optimization**: Choose efficient coordinate schemes
- **Cache Frequently Used Values**: Keep hot data in stack when possible
- **Memory Cleanup**: Reset memory locations when no longer needed

### Performance Considerations

#### Droplet Management
- **Limit Concurrent Droplets**: Each droplet adds overhead
- **Collision Avoidance**: Design flows to minimize unintended collisions
- **Early Termination**: Destroy droplets when their work is complete
- **Path Planning**: Avoid circular paths that create infinite loops

#### Execution Speed
- **Operator Efficiency**: Some operations are more expensive than others
- **Tick Minimization**: Reduce total number of ticks required
- **Parallel Execution**: Use multiple droplets for independent tasks
- **Conditional Optimization**: Use efficient branching patterns

#### Memory Efficiency
```
# Efficient memory usage pattern
@       # Start
|
5       # Use stack for temporary values
:       # Push
3       # Instead of storing everything in memory
:       # Use stack for computation
A       # Compute result
;
n,      # Output
!       # End
# Memory usage: minimal
```

#### Inefficient memory usage pattern
```
# Inefficient memory usage
@       # Start
|
5       # Store in memory instead of stack
0       # X coordinate
:
0       # Y coordinate
:
P       # Put in memory
|
3       # Second value
0       # X coordinate
:
1       # Y coordinate
:
P       # Put in memory
|
0       # Retrieve first value
0       # X coordinate
:
0       # Y coordinate
:
G       # Get from memory
:
0       # Retrieve second value
0       # X coordinate
:
1       # Y coordinate
:
G       # Get from memory
:
A       # Add
;
n,      # Output
!
# Memory usage: higher due to unnecessary storage
```

### Benchmarking and Profiling

#### Performance Measurement
```bash
# Run with benchmarking
tubular --benchmark program.tb

# Run with verbose output
tubular --verbose program.tb

# Run with trace for detailed analysis
tubular --trace program.tb

# Limit ticks for performance testing
tubular --ticks 1000 program.tb
```

#### Common Performance Bottlenecks
1. **Excessive Droplet Creation**: Each droplet adds overhead
2. **Large Grid Scans**: Interpreter scans entire active grid area
3. **Deep Stack Operations**: Stack operations scale with depth
4. **Memory Access Patterns**: Random memory access is slower
5. **Collision Detection**: Checking all droplet pairs each tick

#### Optimization Techniques
1. **Droplet Pooling**: Reuse droplet objects when possible
2. **Grid Clipping**: Limit active area to occupied cells
3. **Stack Preallocation**: Pre-allocate stack space
4. **Memory Caching**: Cache frequently accessed memory locations
5. **Collision Spatial Indexing**: Use spatial data structures for collision detection

## Troubleshooting and Debugging

### Common Issues

#### No Output
**Problem**: Program runs but produces no output
**Causes**:
- Droplet doesn't reach output sink
- All droplets destroyed before output
- Infinite loop prevents reaching output

**Solutions**:
```bash
# Use trace mode to see execution flow
tubular --trace program.tb

# Check for flow issues
tubular --verbose program.tb

# Limit execution to prevent infinite loops
tubular --ticks 100 program.tb
```

**Example Fix**:
```
# Broken - no path to output
@
|
5
+       # Droplet stops here

# Fixed - add path to output
@
|
5
+
|
n,      # Output
!
```

#### Infinite Loops
**Problem**: Program runs forever
**Causes**:
- Circular paths without exit conditions
- Missing conditional branching
- Loop condition never becomes false

**Solutions**:
```bash
# Set tick limit to identify infinite loops
tubular --ticks 1000 program.tb

# Use trace to see loop patterns
tubular --trace program.tb
```

**Example Fix**:
```
# Broken - infinite loop
@
|
5
1-     # Decrement but no exit condition
|
@      # Loop back

# Fixed - add conditional exit
@
|
5
d      # Duplicate for check
1-
d      # Duplicate for conditional
0\     # Exit if zero
  /
 /
@      # Loop back only if not zero
```

#### Stack Underflow
**Problem**: Arithmetic operations fail
**Causes**:
- Not enough values on stack
- Incorrect order of operations
- Missing push operations

**Solutions**:
```bash
# Monitor stack with verbose mode
tubular --verbose program.tb

# Use trace to see stack changes
tubular --trace program.tb
```

**Example Fix**:
```
# Broken - stack underflow
@
|
5       # Only one value on stack
A       # Needs two values for addition

# Fixed - ensure enough values
@
|
5       # First value
:       # Push
3       # Second value
:       # Push
A       # Now has two values
```

#### Unexpected Output
**Problem**: Output doesn't match expectations
**Causes**:
- Wrong output operator (character vs numeric)
- ASCII vs decimal confusion
- Stack order issues

**Solutions**:
- Verify output operators (`,` vs `n`)
- Check ASCII codes for character output
- Use trace to see values before output

**Example Fix**:
```
# Broken - outputs character instead of number
@
|
65,     # Outputs 'A' instead of 65

# Fixed - use numeric output
@
|
65
n,      # Outputs "65"
```

### Debugging Techniques

#### Step-by-Step Execution
```bash
# Run with detailed trace
tubular --trace program.tb

# Trace output format:
TRACE | TICK 001 | Droplet 0 | (2,3) -> (2,4) | value: 5 -> 5 | dir: Down -> Down
TRACE | TICK 001 | Cell (+)  | Increment: 5 -> 6
TRACE | TICK 001 | Stack    | Push: 6 | Stack: [5, 6]
```

#### Program Validation
```bash
# Validate syntax before execution
tubular validate program.tb

# Strict validation for better error reporting
tubular validate --strict program.tb
```

#### Performance Profiling
```bash
# Benchmark program performance
tubular benchmark program.tb

# Multiple iterations for accurate measurements
tubular benchmark --iterations 100 program.tb
```

### Error Reference

#### E001: File Not Found
**Solution**: Check file path and ensure file exists

#### E002: Invalid Encoding
**Solution**: Ensure file is ASCII/UTF-8 encoded

#### E003: No Start Symbol
**Solution**: Add exactly one `@` symbol to program

#### E004: Multiple Start Symbols
**Solution**: Remove extra `@` symbols, keep only one

#### E005: Invalid Character
**Solution**: Remove or replace invalid characters

#### E006: Grid Size Exceeded
**Solution**: Reduce program size or use more efficient layout

#### E007: Execution Timeout
**Solution**: Fix infinite loops or increase tick limit

#### E008: Memory Allocation Failure
**Solution**: Reduce memory usage or increase system memory

#### E009: Internal Error
**Solution**: Report bug with program and trace output

### Debugging Checklist

1. **Validation**: Run `tubular validate program.tb` first
2. **Syntax Check**: Ensure all symbols are valid
3. **Start Symbol**: Verify exactly one `@` exists
4. **Flow Check**: Ensure pipes connect properly
5. **Output Path**: Verify droplets can reach output
6. **Stack Planning**: Check stack operations order
7. **Memory Coordinates**: Verify memory access coordinates
8. **Subroutine Returns**: Ensure subroutines have `R` operators
9. **Conditionals**: Check conditional branching logic
10. **Loop Termination**: Verify loops have exit conditions

## Language Implementation Details

### Internal Data Structures

#### Droplet Structure
```rust
pub struct Droplet {
    pub id: DropletId,           // Unique identifier
    pub value: BigInt,          // Arbitrary precision integer
    pub position: Coordinate,   // Grid position (x, y)
    pub direction: Direction,   // Movement direction
    pub active: bool,           // Whether droplet moves next tick
}
```

#### Grid Representation
- **Sparse Storage**: Only occupied cells stored in memory
- **Bounding Box**: Active area calculated for efficiency
- **Coordinate System**: (0,0) at top-left, positive right/down
- **Unbounded Size**: Limited only by system memory

#### Stack Implementation
- **LIFO Structure**: Last-in-first-out semantics
- **Arbitrary Precision**: Unlimited integer size
- **Underflow Protection**: Returns 0 for empty stack
- **Memory Efficient**: Grows as needed

### Execution Algorithm

#### Tick Processing
1. **Movement Calculation**: Calculate next positions for all droplets
2. **Collision Detection**: Identify droplets entering same cell
3. **Operation Processing**: Process cell interactions
4. **State Updates**: Update droplet positions and values
5. **Cleanup**: Remove destroyed droplets

#### Collision Handling
- **Mutual Destruction**: Both droplets destroyed on collision
- **Simultaneous Processing**: All collisions processed in same tick
- **No Survivors**: Collision always results in destruction

#### Memory Model
- **Reservoir**: Unbounded 2D memory grid
- **Sparse Storage**: Only written cells consume memory
- **Negative Coordinates**: Supported for reservoir memory
- **Persistence**: Values persist between accesses

### Performance Characteristics

#### Time Complexity
- **Grid Access**: O(1) for sparse representation
- **Collision Detection**: O(n²) for n droplets
- **Stack Operations**: O(1) for all operations
- **Memory Access**: O(1) with caching

#### Space Complexity
- **Grid Storage**: O(m) where m = occupied cells
- **Droplet Storage**: O(n) where n = active droplets
- **Stack Storage**: O(k) where k = stack depth
- **Memory Storage**: O(p) where p = written memory cells

#### Scaling Limits
- **Maximum Grid Size**: 1000×1000 cells (minimum requirement)
- **Maximum Droplets**: 1000+ concurrent droplets
- **Stack Depth**: 1000+ levels
- **Memory Size**: Limited by system memory

### Error Handling Strategy

#### Recoverable Errors
- **Stack Underflow**: Returns 0, continues execution
- **Division by Zero**: Returns 0, continues execution
- **Invalid Memory Access**: Returns 0, continues execution
- **Subroutine Underflow**: No operation, continues execution

#### Fatal Errors
- **Invalid Program Syntax**: Stops execution
- **Memory Allocation Failure**: Stops execution
- **Internal Errors**: Stops execution

#### Error Recovery
- **Graceful Degradation**: Continue execution when possible
- **Error Logging**: Log errors for debugging
- **Safe Defaults**: Provide sensible fallback values

### Implementation Extensions

#### Potential Optimizations
1. **Spatial Indexing**: Use quadtrees for collision detection
2. **Parallel Execution**: Process droplets in parallel
3. **JIT Compilation**: Compile frequently executed paths
4. **Memory Pooling**: Reuse droplet objects
5. **Grid Compression**: Compress sparse grid representations

#### Language Extensions
1. **New Operators**: Additional arithmetic or logical operations
2. **Enhanced I/O**: File I/O, network operations
3. **Debugging Support**: Breakpoints, watchpoints
4. **Macros**: Programmatic code generation
5. **Libraries**: Standard library of common operations

This language reference serves as the definitive guide for Tubular programming. For additional examples and tutorials, see the `examples/` directory and the quickstart guide at `/Users/parker/code/tubular/specs/001-a-basic-initial/quickstart.md`.

---

## Quick Reference Summary

### Essential Symbols
- **Start**: `@` - Creates droplet with value 0, direction Down
- **Flow**: `|` `-` `/` `\` `^` - Pipes and conditional corners
- **Numbers**: `0-9` - Set droplet value
- **I/O**: `?` `??` `,` `n` - Input/output operations
- **Stack**: `:` `;` `d` `A` `S` `M` `D` `=` `<` `>` `%` - Stack operations
- **Memory**: `G` `P` - Reservoir memory
- **Subroutines**: `C` `R` - Function calls
- **End**: `!` - Destroy droplet (program termination)

### Basic Program Template
```
@       # Start
|       # Flow
5       # Set value
n,      # Output
!       # End
```

### CLI Commands
```bash
tubular program.tb                    # Basic execution
tubular --verbose program.tb          # Verbose output
tubular --trace program.tb            # Step-by-step trace
tubular --ticks 1000 program.tb       # Limit execution
tubular validate program.tb           # Check syntax
tubular benchmark program.tb          # Performance test
```

### Common Patterns
- **Loop**: Use conditional corners `/` `\` with zero/non-zero checks
- **Stack Calculator**: Push values with `:`, operate with `A` `S` `M` `D`, pop with `;`
- **Memory**: Use `P` to store at coordinates, `G` to retrieve
- **Subroutines**: Use `C` to jump to coordinates, `R` to return

### Debugging
- Use `--trace` to see step-by-step execution
- Use `--verbose` for execution details
- Use `validate` to check syntax
- Set tick limits to prevent infinite loops

### Performance Tips
- Minimize droplet count
- Keep stack depth reasonable
- Use memory sparingly
- Design linear flows when possible
- Place related code close together

---

**Document Location**: `/Users/parker/code/tubular/docs/language_reference.md`
**Last Updated**: 2025-10-12
**Version**: 0.1.0
**Feature Branch**: `001-a-basic-initial`