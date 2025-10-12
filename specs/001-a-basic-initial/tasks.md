# Tasks: 001-a-basic-initial

**Input**: Design documents from `/specs/001-a-basic-initial/`
**Prerequisites**: plan.md (completed), spec.md (completed for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for this feature - comprehensive testing is required by constitution principle IV (Test-Driven Language Design). Test tasks are included for all core functionality.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- **Paths below follow plan.md structure** for CLI interpreter project

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Rust project structure according to plan.md
- [x] T002 Initialize Cargo project with dependencies: clap, ndarray, num-bigint, anyhow, proptest, criterion
- [x] T003 [P] Configure development tools: cargo-watch, cargo-benchcmp
- [x] T004 [P] Setup basic project directories: src/, tests/, benches/, examples/, docs/
- [x] T005 [P] Create initial module structure: main.rs, lib.rs, and subdirectories

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 [P] Implement core types in src/types/mod.rs (Coordinate, Direction, Error types)
- [x] T007 [P] Create Droplet entity in src/interpreter/droplet.rs with value, position, direction, active state
- [x] T008 [P] Implement sparse ProgramGrid in src/interpreter/grid.rs with HashMap storage and bounds
- [x] T009 [P] Create DataStack in src/interpreter/stack.rs with LIFO operations and underflow protection
- [x] T010 [P] Implement Reservoir memory in src/interpreter/memory.rs with unbounded coordinates
- [x] T011 [P] Create CallStack in src/interpreter/subroutines.rs with return frame management
- [x] T012 [P] Implement ASCII grid parser in src/parser/grid_parser.rs for .tb files
- [x] T013 [P] Create program validator in src/parser/validator.rs for start symbols and syntax
- [x] T014 [P] Implement basic error types and handling in src/types/error.rs
- [x] T015 [P] Create CLI argument structure in src/cli/commands.rs with clap derive macros
- [x] T016 [P] Implement arbitrary precision integer type in src/types/bigint.rs using num-bigint crate
- [x] T017 [P] Create arithmetic operations for arbitrary precision in src/operations/bigint_arith.rs
- [x] T018 Setup test utilities and fixtures in tests/common/mod.rs
- [x] T019 Create example program files in examples/ directory for testing

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Basic Program Execution (Priority: P1) 🎯 MVP

**Goal**: Enable developers to write and run simple Tubular programs with basic flow control and output capabilities

**Independent Test**: Run the Hello World example program and verify it outputs "Hello, World!" to the console

### Implementation for User Story 1

- [x] T020 [US1] Implement flow control operations in src/operations/flow_control.rs (|, -, ^, /, \ pipes)
- [x] T021 [US1] Implement data source operations in src/operations/arithmetic.rs (0-9 literals, > tape reader) - COMPLETE
- [x] T022 [US1] Implement data sink operations in src/operations/io.rs (! output, , character output, n numeric output) - COMPLETE
- [x] T023 [US1] Create core execution engine in src/interpreter/execution.rs with tick-based simulation
- [x] T024 [US1] Implement droplet movement and collision detection in src/interpreter/collision.rs
- [x] T025 [US1] Create main execution loop in src/interpreter/mod.rs
- [x] T026 [US1] Implement basic CLI run command in src/cli/commands.rs
- [ ] T027 [US1] Add verbose output formatting in src/cli/output.rs - FILE EMPTY
- [x] T028 [US1] Create Hello World example in examples/hello_world.tb
- [ ] T029 [US1] Add basic integration tests for Hello World execution in tests/integration/test_hello_world.rs - NO TESTS

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Data Input/Output Operations (Priority: P2)

**Goal**: Enable interactive programs that accept user input and display results using all I/O operators

**Independent Test**: Create a program that reads a number from input, performs a calculation, and outputs the result

### Implementation for User Story 2

- [ ] T030 [P] [US2] Implement character input (?) in src/operations/io.rs with stdin reading
- [ ] T031 [P] [US2] Implement numeric input (??) in src/operations/io.rs with parsing
- [ ] T032 [US2] Enhance CLI with interactive mode support in src/cli/commands.rs
- [ ] T033 [US2] Add input buffering and validation in src/operations/io.rs
- [ ] T034 [US2] Create interactive calculator example in examples/calculator.tb
- [ ] T035 [US2] Add integration tests for I/O operations in tests/integration/test_io.rs
- [ ] T036 [US2] Implement CLI subcommand 'run' with interactive flag in src/cli/commands.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Complex Computational Logic (Priority: P3)

**Goal**: Enable algorithms requiring loops, conditionals, arithmetic, and stack operations

**Independent Test**: Run the countdown example program which uses loops, decrements, output, and conditional logic

### Implementation for User Story 3

- [ ] T037 [P] [US3] Implement unary operators (+, ~) in src/operations/arithmetic.rs
- [ ] T038 [P] [US3] Implement stack operations (:, ;, d) in src/operations/arithmetic.rs
- [ ] T039 [P] [US3] Implement arithmetic operators (A, S, M, D) in src/operations/arithmetic.rs
- [ ] T040 [P] [US3] Implement comparison operators (=, <, >, %) in src/operations/arithmetic.rs
- [ ] T041 [US3] Enhance flow control with conditional branching in src/operations/flow_control.rs
- [ ] T042 [US3] Implement division by zero and modulo by zero handling (return 0)
- [ ] T043 [US3] Create countdown example in examples/countdown.tb
- [ ] T044 [US3] Add integration tests for arithmetic operations in tests/integration/test_arithmetic.rs
- [ ] T045 [US3] Add property-based tests for stack operations in tests/property/test_stack.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Memory and Subroutine Operations (Priority: P4)

**Goal**: Enable complex data structures and reusable code using reservoir memory and subroutine call system

**Independent Test**: Create a program that stores a value in the reservoir, retrieves it, performs an operation, and uses a subroutine call

### Implementation for User Story 4

- [ ] T046 [P] [US4] Implement reservoir Get operation in src/operations/memory.rs with coordinate calculation
- [ ] T047 [P] [US4] Implement reservoir Put operation in src/operations/memory.rs with coordinate calculation
- [ ] T048 [P] [US4] Implement subroutine Call operation in src/operations/subroutines.rs with stack management
- [ ] T049 [P] [US4] Implement subroutine Return operation in src/operations/subroutines.rs with underflow handling
- [ ] T050 [US4] Enhance coordinate system to support negative coordinates in src/types/coordinate.rs
- [ ] T051 [US4] Create memory test example in examples/memory_test.tb
- [ ] T052 [US4] Create subroutine example in examples/subroutine_test.tb
- [ ] T053 [US4] Add integration tests for memory operations in tests/integration/test_memory.rs
- [ ] T054 [US4] Add integration tests for subroutines in tests/integration/test_subroutines.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T055 [P] Implement execution tracing in src/cli/output.rs with detailed step-by-step output
- [ ] T056 [P] Add performance benchmarking in benches/ directory using criterion
- [ ] T057 [P] Implement tick limit and timeout handling in src/interpreter/execution.rs
- [ ] T058 [P] Add comprehensive error messages with line/column context in src/parser/validator.rs
- [ ] T059 [P] Create CLI validate subcommand in src/cli/commands.rs for syntax checking
- [ ] T060 [P] Implement CLI benchmark subcommand in src/cli/commands.rs
- [ ] T061 [P] Add environment variable support in src/cli/commands.rs
- [ ] T062 [P] Create comprehensive unit test suite in tests/unit/
- [ ] T063 [P] Add property-based tests for core invariants in tests/property/
- [ ] T064 [P] Add grid size validation in src/parser/validator.rs for 1000x1000 minimum support
- [ ] T065 [P] Implement stack depth monitoring in src/interpreter/stack.rs with 1000-level validation
- [ ] T066 [P] Add integration tests for maximum grid and stack limits in tests/integration/test_limits.rs
- [ ] T067 Update README.md with usage examples and getting started guide
- [ ] T068 Create language reference documentation in docs/language_reference.md
- [ ] T069 Add performance optimization for large grids (spatial partitioning)
- [ ] T070 Implement memory usage optimization for large droplet counts

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 core components
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Uses arithmetic operations from US1/US2
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Builds on all previous stories

### Within Each User Story

- Models/Types before Services/Operations
- Core operations before Integration/CLI
- Core implementation before testing
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Operations within stories marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members
- All Polish tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all operations for User Story 1 together:
Task: "Implement flow control operations in src/operations/flow_control.rs"
Task: "Implement data source operations in src/operations/arithmetic.rs"
Task: "Implement data sink operations in src/operations/io.rs"

# Launch all core components together:
Task: "Create core execution engine in src/interpreter/execution.rs"
Task: "Implement droplet movement and collision detection in src/interpreter/collision.rs"
Task: "Create main execution loop in src/interpreter/mod.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test Hello World program independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test Hello World independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test I/O operations independently → Deploy/Demo
4. Add User Story 3 → Test arithmetic/logic independently → Deploy/Demo
5. Add User Story 4 → Test memory/subroutines independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (core execution)
   - Developer B: User Story 2 (I/O operations)
   - Developer C: User Story 3 (arithmetic/logic)
   - Developer D: User Story 4 (memory/subroutines)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Focus on property-based testing for complex state transitions
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Performance targets: <10ms per tick, 1000+ concurrent droplets, <1000MB memory
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence