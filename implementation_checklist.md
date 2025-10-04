# Tubular Language Implementation Checklist

This document outlines a minimal, incremental, and testable plan for implementing the Tubular programming language as per the v1.1 specification.

---

### [x] Step 1: Core Data Structures

**Goal:** Establish the fundamental data structures for the program grid and data droplets.

**Tasks:**
1.  Create a `Grid` class/struct that can hold a 2D array of characters. It should provide a method to get the character at a given `(x, y)` coordinate.
2.  Create a `Droplet` class/struct to store its `value` (integer), `position` (x, y), and `direction` (Up, Down, Left, Right).

**Verification:**
*   Write a unit test to create a `Grid` from a hardcoded 2D array.
*   Write a unit test to verify that `grid.get(x, y)` returns the correct character.
*   Write a unit test to create a `Droplet` and confirm its initial properties are set correctly.

---

### [x] Step 2: The Execution Engine

**Goal:** Create the main execution loop that processes ticks and moves droplets.

**Tasks:**
1.  Create an `Engine` or `VM` class that holds the `Grid` and a list of all active `Droplets`.
2.  Implement a `tick()` method. In this first version, the method should iterate through every active droplet and move it one cell in its current direction.
3.  For now, ignore character interactions. A droplet moving onto any character cell other than an empty space (` `) is simply destroyed.

**Verification:**
*   Create a test with a single droplet on a hardcoded grid. Run `tick()` and verify the droplet's position is updated correctly.
*   Create a test where a droplet is aimed at a non-empty cell. Verify the droplet is removed from the active list after the tick.

---

### [x] Step 3: Basic Flow Control Pipes

**Goal:** Implement the simplest pipes that redirect droplet flow without changing values.

**Tasks:**
1.  Update the `tick()` method to handle basic pipe characters.
2.  Implement the logic for `|` (Vertical Pipe) and `-` (Horizontal Pipe).
3.  Implement the logic for `^` (Go Up Pipe).
4.  Implement the logic for `#` (Wall), which destroys any droplet entering it.

**Verification:**
*   Test `|`: A droplet entering from the top should continue Down; from the bottom should continue Up.
*   Test `-`: A droplet entering from the left should continue Right; from the right should continue Left.
*   Test `^`: A droplet entering from any direction should have its direction changed to Up.
*   Test `#`: A droplet moving into a `#` cell should be destroyed.

---

### [x] Step 4: Program Start & Basic Output

**Goal:** Implement the program entry point and a way to see numeric output.

**Tasks:**
1.  Implement the `@` (Program Start) character. The engine's initialization should scan the grid for `@`, create the initial trigger droplet (value 0, direction Down), and place it in the active droplets list.
2.  Implement the `n` (Numeric Output) operator. When a droplet enters this cell, its value should be printed to the console, and the droplet should be destroyed.

**Verification:**
*   Create a program using a hardcoded grid:
    ```
    @
    |
    n
    ```
*   Run it. The program should output `0` and then terminate.

---

### [x] Step 5: File Loading and Parsing

**Goal:** Define a standard way to load Tubular programs from `.tub` files, replacing the need for hardcoded grids.

**Tasks:**
1.  Create a `Parser` or `Loader` component that takes a file path (e.g., `program.tub`) as input.
2.  The parser should read the file line by line to determine the grid's dimensions (max width and height).
3.  It should then populate a new `Grid` object with the characters from the file. Lines of varying lengths should be handled correctly by padding with spaces.
4.  Update the main application entry point to accept a filename, use the parser to create the `Grid`, and then pass that grid to the `Engine`.

**Verification:**
*   Create a sample `program.tub` file.
*   Write a test to load the file using the new parser.
*   Verify the resulting `Grid` object has the correct dimensions and character placements.
*   Run the program from Step 4 by loading it from a file. The output should still be `0`.

---

### [x] Step 6: Unary Operators

**Goal:** Implement the first value-modifying operators.

**Tasks:**
1.  Implement the `+` (Increment) operator. It should add 1 to the droplet's value.
2.  Implement the `~` (Decrement) operator. It should subtract 1 from the droplet's value.

**Verification:**
*   Create a `test.tub` file:
    ```
    @
    |
    +
    |
    +
    |
    ~
    |
    n
    ```
*   Run it using the new file-based loader. The program should output `1`.

---

### [x] Step 7: Conditional Branching

**Goal:** Implement the core logic pipes, `/` and `\`, to allow for conditional execution. This is a major milestone.

**Tasks:**
1.  Implement the `/` (Forward-Slash Corner) with its four behaviors based on entry direction and droplet value (zero vs. non-zero).
2.  Implement the `\` (Back-Slash Corner) with its four behaviors.

**Verification:**
*   Write specific tests for each of the 8 conditions (4 for `/`, 4 for `\`).
    *   Example test for `/`: Create a droplet with value `0` entering from the top. Verify its direction changes to Left.
    *   Example test for `\`: Create a droplet with value `42` entering from the top. Verify its direction changes to Left.

---

### [ ] Step 8: Data Sources

**Goal:** Implement the ability to generate new data within the program.

**Tasks:**
1.  Implement the `0`-`9` (Number Source) operators. When a droplet hits a number, the original droplet is destroyed, and a new one is created in its place with the corresponding integer value and direction `Down`.

**Verification:**
*   Create a program file:
    ```
    @
    |
    7
    |
    n
    ```
*   Run it. The program should output `7`.

---

### [ ] Step 9: The Data Stack (Part 1)

**Goal:** Implement the global LIFO data stack and its basic manipulation operators.

**Tasks:**
1.  Add a `Stack` data structure to your `Engine`.
2.  Implement `:` (Push): Pushes the droplet's value onto the stack.
3.  Implement `;` (Pop): Pops a value and replaces the droplet's value with it. Handle the edge case where popping an empty stack results in a value of `0`.
4.  Implement `d` (Duplicate): Pushes a copy of the droplet's value onto the stack.

**Verification:**
*   Test push/pop: `@ -> + -> : -> ~ -> ; -> n`. Output should be `1`.
*   Test duplicate: `@ -> + -> + -> d -> ; -> n`. Output should be `2`.
*   Test stack underflow: `@ -> ; -> n`. Output should be `0`.

---

### [ ] Step 10: The Data Stack (Part 2 - Operators)

**Goal:** Implement all stack-based arithmetic and comparison operators.

**Tasks:**
1.  Implement arithmetic operators: `A` (Add), `S` (Subtract), `M` (Multiply), `D` (Divide), `%` (Modulo).
    *   These operators pop two values, perform the calculation, push the result, and destroy the triggering droplet.
    *   Implement the edge case for `D` and `%` where division by zero results in `0`.
2.  Implement comparison operators: `=` (Equal), `<` (Less Than), `>` (Greater Than).
    *   These pop two values, compare them, and push `1` for true or `0` for false.

**Verification:**
*   For each operator, write a program that pushes two known values, triggers the operator, then pops the result into a droplet and prints it with `n`.
    *   Example for `A`: `5 -> : -> 3 -> : -> A -> ; -> n`. Output should be `8`.
    *   Example for `D` (by zero): `5 -> : -> 0 -> : -> D -> ; -> n`. Output should be `0`.

---

### [ ] Step 11: Advanced I/O

**Goal:** Implement character-based input and output.

**Tasks:**
1.  Implement `>` (Tape Reader). It reads characters to its right until a whitespace or pipe character, creating a new downward-moving droplet for each character's ASCII value.
2.  Implement `,` (Character Output). It consumes a droplet and prints its value as an ASCII character.
3.  Update `!` (Output Sink) to handle its dual behavior: print as a character if the droplet was created by a Tape Reader, otherwise print as an integer with a newline. (This may require adding a "source" flag to the `Droplet` structure).

**Verification:**
*   Implement the "Hello, World!" example from the specification in a `.tub` file.
    ```
      @
      |
     >Hello, World!
      |
      !
    ```
*   The output should be `Hello, World!`.

---

### [ ] Step 12: Interactive Input

**Goal:** Allow the program to accept input from the user at runtime.

**Tasks:**
1.  Implement `?` (Character Input). Halts execution, reads one character from stdin, and sets the droplet's value to its ASCII code. Sets value to -1 on EOF.
2.  Implement `??` (Numeric Input). Halts execution, reads a line from stdin, and sets the droplet's value to the parsed integer. Sets value to 0 on failure or EOF.

**Verification:**
*   Test `?`: `@ -> ? -> ,`. Run the program, type `A`, and verify `A` is printed to the console.
*   Test `??`: `@ -> ?? -> n`. Run the program, type `123`, and verify `123` is printed to the console.

---

### [ ] Step 13: The Reservoir (Random-Access Memory)

**Goal:** Implement the 2D random-access memory grid.

**Tasks:**
1.  Create a `Reservoir` class that can store integer values at `(x, y)` coordinates. It should use a dictionary or hash map for sparse storage and default to `0` for uninitialized locations.
2.  Implement `P` (Put): Pops `y`, `x`, and `value` from the data stack and writes to the Reservoir.
3.  Implement `G` (Get): Pops `y` and `x`, reads from the Reservoir, and pushes the value onto the data stack.

**Verification:**
*   Write a program to:
    1. Push a value, x, and y (`100`, `5`, `5`).
    2. Use `P` to store it.
    3. Push the same x and y (`5`, `5`).
    4. Use `G` to retrieve it.
    5. Pop the result and print it with `n`.
*   The output should be `100`.

---

### [ ] Step 14: Subroutines

**Goal:** Implement a call/return system for code reuse.

**Tasks:**
1.  Add a `CallStack` to your `Engine`.
2.  Implement `C` (Call): Pops `y` and `x`, pushes the droplet's current position and direction to the call stack, and moves the droplet to the new `(x, y)`.
3.  Implement `R` (Return): Pops a position and direction from the call stack, destroys the current droplet, and creates a new one at the return location.

**Verification:**
*   Create a program with a main flow and a separate subroutine area.
*   The main flow calls the subroutine using `C`.
*   The subroutine performs a simple task (e.g., increments a droplet's value).
*   The subroutine uses `R` to return.
*   Verify that execution resumes correctly in the main flow after the `C` instruction.

---

### [ ] Step 15: Final Review and Polish

**Goal:** Ensure the implementation is robust, correct, and adheres to all specification guidelines.

**Tasks:**
1.  Review the "Implementation Guidelines and Limitations" section of the spec.
    *   Confirm integer representation (or document limitations).
    *   Check stack depth and grid size limits.
2.  Implement the countdown example from the spec and verify its output.
3.  Review all edge cases (`D`/`%` by zero, stack underflow) and ensure they are handled correctly.
4.  Add droplet collision detection: in each tick, after calculating all next positions, check if any two droplets are moving to the same cell. If so, remove both from the active list.

**Verification:**
*   All previous tests should still pass.
*   The countdown example should run correctly and produce the specified output.
*   A test with two droplets aimed at the same cell should result in both being destroyed.
