# Tubular Self-Hosting Compiler Bootstrap Checklist

This document outlines the necessary phases and steps to develop the Tubular programming language from its specification into a self-hosting compiler.

---

## Phase 1: The Interpreter (The "Seed")

**Goal:** Create a working Tubular interpreter in an existing host language (e.g., Python, Rust, TypeScript). This allows for testing the language design and running the first simple programs.

- [x] **Choose a Host Language:** Python
- [x] **Implement Core Simulation:**
    - [x] Create the 2D `Grid` data structure.
    - [x] Implement the `Droplet` data structure (value, direction).
    - [x] Build the main execution loop that processes droplets each "tick".
- [x] **Implement Language Features:**
    - [x] Implement all pipe operators as defined in `tubular_spec.md v1.1`.
    - [x] Implement the global Data Stack.
    - [x] Implement the global Call Stack.
    - [x] Implement "The Reservoir" as a 2D random-access memory grid.
- [x] **Build the CLI:**
    - [x] Create a command-line interface that can load and run a `.tub` file.
- [x] **Create a Test Suite:**
    - [x] Write a comprehensive set of small `.tub` programs that test every single operator and feature to verify correctness.

---

## Phase 2: The First Compiler (in the Host Language)

**Goal:** Write a compiler in the host language that translates Tubular source code into a lower-level language like C or WebAssembly (WASM).

- [ ] **Choose a Compilation Target:** Decide on the output language (WASM is a good, portable choice).
- [ ] **Design Compiler Architecture:**
    - [ ] **Lexer:** A simple component that reads the `.tub` grid and validates characters.
    - [ ] **Code Generator:** The main component that analyzes the pipe layout and emits equivalent target code (e.g., WASM text format). A direct-to-target approach is likely simpler than a multi-stage IR approach for the first compiler.
- [ ] **Implement the Compiler:**
    - [ ] Write the compiler (`tubularc-v1`) in your chosen host language.
- [ ] **Verify Compiler Correctness:**
    - [ ] Compile the test suite from Phase 1 using `tubularc-v1`.
    - [ ] Run the compiled executables and ensure their output is identical to the interpreter's output from Phase 1.

---

## Phase 3: The Self-Hosting Compiler (in Tubular)

**Goal:** Rewrite the compiler from Phase 2, but this time in the Tubular language itself. This is the most significant and challenging step.

- [ ] **Design Data Structures in Tubular:**
    - [ ] Plan how to represent strings, arrays, and lookup tables using The Reservoir. This is a critical design task.
- [ ] **Write the Compiler in Tubular:**
    - [ ] Create `compiler.tub`, the source code for the new compiler.
    - [ ] **Lexer/Parser:** Write Tubular code that reads a `.tub` file (via the `?` operator) from standard input and builds a representation of it in The Reservoir.
    - [ ] **Code Generator:** Write Tubular code that traverses your in-memory representation and prints the target language (WASM/C) to standard output (via the `!` operator).
- [ ] **Compile the Self-Hosting Compiler:**
    - [ ] Use the compiler from Phase 2 (`tubularc-v1`) to compile `compiler.tub`.
    - [ ] The result is your first self-hosted compiler executable, `tubularc-v2`.

---

## Phase 4: The Bootstrap

**Goal:** Use the self-hosting compiler to compile itself, proving the language is truly self-sufficient.

- [ ] **Execute the Self-Compilation:**
    - [ ] Run `tubularc-v2` (the compiler written in Tubular) and give it its own source code, `compiler.tub`, as input.
- [ ] **Verify the Result:**
    - [ ] The output of this process should be a new executable, `tubularc-v3`.
    - [ ] Compare `tubularc-v3` and `tubularc-v2`. If they are bit-for-bit identical, the bootstrap is successful.
- [ ] **Celebrate:** The language is now officially self-hosting. `tubularc-v1` (written in the host language) is no longer needed for compiler development.

---

## Phase 5: Ecosystem and Future Development

**Goal:** Grow the language beyond the compiler.

- [ ] **Develop a Standard Library:** Create `.tub` files with reusable functions for common tasks (math, string manipulation, data structures).
- [ ] **Design a Module System:** Define how Tubular programs can import and use code from other files.
- [ ] **Write Comprehensive Documentation:** Create user-facing guides and tutorials.
- [ ] **Build Developer Tools:**
    - [ ] A package manager.
    - [ ] A code formatter.
    - [ ] A visual debugger to watch droplets flow through the grid.
