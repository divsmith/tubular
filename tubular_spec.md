# Tubular Language Specification v1.1

## 1. Philosophy and Core Concepts

Tubular is a two-dimensional, visual, dataflow programming language. The theme of the language is pipes and fluid dynamics. The source code of a Tubular program is a 2D grid of characters that form a "pipe system" through which "droplets" of data flow.

Execution is the process of a droplet moving through the system. The path a droplet takes, and the pipes it flows through, determine the computation. The language is designed to be Turing-complete and to encourage a style of programming that does not resemble conventional text-based languages.

### 1.1. The Grid
A Tubular program is a grid of ASCII characters. The location of each character is significant. Whitespace (` `) is considered an empty cell.

### 1.2. Droplets
The fundamental unit of data is a **droplet**. Each droplet has two properties:
*   **Value:** An integer.
*   **Direction:** The direction it is currently traveling (Up, Down, Left, or Right).

### 1.3. Execution Model
*   Execution begins at the `@` character. An initial "trigger" droplet is created with value `0` and direction `Down`.
*   The program proceeds in discrete **ticks**. In each tick, every active droplet moves one cell in its current direction.
*   When a droplet enters a cell, the character in that cell determines the action to be performed, which may change the droplet's value, direction, or create/destroy other droplets.
*   **Droplet Collision:** If two droplets attempt to enter the same cell in the same tick, they are both annihilated (destroyed).

## 2. Pipe Reference

### 2.1. Flow Control Pipes
These pipes guide the path of a droplet without changing its value.

*   `|`: **Vertical Pipe.** A droplet entering from the top continues Down. A droplet entering from the bottom continues Up.
*   `-`: **Horizontal Pipe.** A droplet entering from the left continues Right. A droplet entering from the right continues Left.
*   `^`: **Go Up Pipe.** Any droplet entering this pipe has its direction changed to Up.
*   `#`: **Wall.** A wall stops a droplet's movement, effectively destroying it.

### 2.2. Corner Pipes (Branching and Looping)
Corner pipes are the core of flow redirection, looping, and conditional logic. Their behavior depends on the droplet's entry direction.

*   `/`: **Forward-Slash Corner**
    *   **Enters from Top:** Conditional Branch. If droplet value is `0`, direction becomes Left. If non-zero, direction becomes Right.
    *   **Enters from Bottom:** Conditional Branch. If droplet value is `0`, direction becomes Right. If non-zero, direction becomes Left.
    *   **Enters from Left:** Redirects Up.
    *   **Enters from Right:** Redirects Down.

*   `\`: **Back-Slash Corner**
    *   **Enters from Top:** Conditional Branch. If droplet value is `0`, direction becomes Right. If non-zero, direction becomes Left.
    *   **Enters from Bottom:** Conditional Branch. If droplet value is `0`, direction becomes Left. If non-zero, direction becomes Right.
    *   **Enters from Left:** Redirects Down.
    *   **Enters from Right:** Redirects Up.

### 2.3. Data Sources
*   `@`: **Program Start.** The entry point of the program. Creates a single droplet with value `0` and direction `Down`.
*   `0`-`9`: **Number Source.** When a droplet hits a number character, that character emits a new droplet downwards with the corresponding integer value. The original droplet is destroyed.
*   `>`: **Tape Reader.** When a droplet hits this character, it begins reading all adjacent characters to its right until it hits a whitespace or a pipe character. For each character read, it emits a new droplet downwards containing that character's ASCII value. The original droplet is destroyed.
*   `?`: **Character Input.** Halts execution and waits for user input from the standard input stream. It reads a single character and replaces the droplet's value with the character's ASCII code. On End-of-File (EOF), it sets the droplet's value to -1.
*   `??`: **Numeric Input.** Halts execution and waits for user input from the standard input stream. It reads a line of input and attempts to parse it as an integer. If successful, it replaces the droplet's value with the parsed integer. If parsing fails or EOF is reached, it sets the droplet's value to 0.

### 2.4. Data Sinks
*   `!`: **Output Sink.** Consumes any droplet that enters it.
    *   If the droplet was created by a Tape Reader (`>`), its value is interpreted as an ASCII code and the corresponding character is printed to the console.
    *   Otherwise, the droplet's integer value is printed, followed by a newline.
*   `,`: **Character Output.** Consumes any droplet that enters it and prints the droplet's value as an ASCII character to the console (no newline).
*   `n`: **Numeric Output.** Consumes any droplet that enters it and prints the droplet's integer value to the console (no newline).

### 2.5. Unary Operators
These operators modify the value of a droplet that passes through them.

*   `+`: **Increment.** Adds 1 to the droplet's value.
*   `~`: **Decrement.** Subtracts 1 from the droplet's value.

### 2.6. Data Stack Operators
Tubular includes a global LIFO data stack for temporary value storage.

*   `:`: **Push.** The droplet's current value is pushed onto the data stack. The droplet passes through unchanged.
*   `;`: **Pop.** A value is popped from the data stack. The droplet's value is replaced with the popped value. If the stack is empty, the droplet's value becomes `0`.
*   `d`: **Duplicate.** Pushes a copy of the droplet's current value onto the data stack. The droplet passes through unchanged.
*   `A`: **Add.** Pops two values (`a` and `b`) from the data stack, calculates `b + a`, and pushes the result back onto the stack. The droplet that triggered this is destroyed.
*   `S`: **Subtract.** Pops two values (`a` and `b`) from the data stack, calculates `b - a`, and pushes the result back onto the stack. The droplet is destroyed.
*   `M`: **Multiply.** Pops two values (`a` and `b`) from the data stack, calculates `b * a`, and pushes the result back onto the stack. The droplet is destroyed.
*   `D`: **Divide.** Pops two values (`a` and `b`) from the data stack, calculates integer division `b / a`, and pushes the result back onto the stack. The droplet is destroyed.
*   `=`: **Equal.** Pops two values (`a` and `b`) from the data stack, pushes `1` if `b == a`, otherwise pushes `0`. The droplet is destroyed.
*   `<`: **Less Than.** Pops two values (`a` and `b`) from the data stack, pushes `1` if `b < a`, otherwise pushes `0`. The droplet is destroyed.
*   `>`: **Greater Than.** Pops two values (`a` and `b`) from the data stack, pushes `1` if `b > a`, otherwise pushes `0`. The droplet is destroyed.
*   `%`: **Modulo.** Pops two values (`a` and `b`) from the data stack, calculates `b % a` (remainder of `b` divided by `a`), and pushes the result back onto the stack. The droplet is destroyed.

### 2.7. Reservoir (Random-Access Memory) Operators
For more complex data structures, Tubular provides a 2D random-access memory grid called "The Reservoir".

The Reservoir supports negative coordinates and is unbounded, expanding dynamically as needed. Uninitialized Reservoir locations contain the value `0`.

*   `G`: **Get.** Pops a `y` then an `x` coordinate from the data stack. Reads the value from The Reservoir at `(x, y)` and pushes it onto the data stack.
*   `P`: **Put.** Pops a `y`, an `x`, and a `value` from the data stack. Writes the `value` to The Reservoir at `(x, y)`.

### 2.8. Subroutine (Function) Pipes
To facilitate code re-use and organization, Tubular supports subroutines via a dedicated call stack.

*   `C`: **Call.** Pops a `y` then an `x` coordinate from the data stack. Pushes the current droplet's position and direction onto the **call stack**. The droplet is then transported to the new `(x, y)` coordinates to begin execution.
*   `R`: **Return.** Pops a position and direction from the call stack. The current droplet is destroyed, and a new one is created at the return location, moving in the stored direction.

## 3. Examples

### 3.1. Hello, World!
```
  @
  |
 >Hello, World!
  |
  !
```
**Explanation:** The `@` trigger hits the `>` tape reader, which emits the ASCII values for "Hello, World!" as a sequence of droplets. The `!` sink prints each one as a character.

### 3.2. Countdown from 5
```
      @
      |
      5
      |
  /---d---\
  |   |   |
  !   ~   ^
  |       |
  \-------/
```
**Explanation:**
1.  A droplet hits `5`, creating a `5` droplet.
2.  The `d` (duplicate) pushes `5` to the stack. The droplet continues to the `\` corner.
3.  Since its value is non-zero, it's sent left into the loop.
4.  `!` prints the current value.
5.  `~` decrements the value.
6.  `^` sends it back up.
7.  The `/` corner sends it right, back to the `d` (duplicate).
8.  This repeats until the value is `0`. When the `0` droplet reaches the `\` corner, it is sent right, exiting the loop.
**Output:**
```
5
4
3
2
1
```

## 4. Implementation Guidelines and Limitations

### 4.1. Integer Representation
Implementations should support arbitrary precision integers to ensure Turing completeness. While smaller implementations may use 32-bit or 64-bit signed integers for performance, this may limit the language's computational power for certain algorithms.

### 4.2. Stack Depth Limits
The data stack should support a minimum depth of 1000 levels to enable complex algorithms. Implementations may support deeper stacks but should not impose artificial limitations below this threshold.

### 4.3. Grid Size Limitations
The program grid should support a minimum size of 1000x1000 cells to accommodate complex programs. Implementations may support larger grids but should not impose artificial limitations below this threshold.

### 4.4. Edge Case Behavior
*   **Division by Zero:** The `D` (Divide) operator should handle division by zero by pushing `0` as the result.
*   **Stack Underflow:** When popping from an empty stack, operations should return `0` as the popped value rather than causing an error.
*   **Modulo by Zero:** The `%` (Modulo) operator should handle modulo by zero by returning `0` as the result.