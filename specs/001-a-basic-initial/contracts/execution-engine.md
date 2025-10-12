# Execution Engine Contract: Tubular Interpreter

**Feature**: `001-a-basic-initial` | **Date**: 2025-10-11

## Core Execution Interface

### Interpreter API
```rust
pub trait TubularInterpreter {
    /// Initialize interpreter with program grid
    fn initialize(grid: ProgramGrid) -> Result<Self, InterpreterError>;

    /// Execute single tick
    fn execute_tick(&mut self) -> TickResult;

    /// Execute until completion or tick limit
    fn run(&mut self, max_ticks: Option<u64>) -> ExecutionResult;

    /// Get current execution state
    fn state(&self) -> &ExecutionState;

    /// Reset interpreter to initial state
    fn reset(&mut self) -> Result<(), InterpreterError>;
}
```

### Execution State
```rust
pub struct ExecutionState {
    /// Current tick number
    pub tick: u64,
    /// Active droplets
    pub droplets: Vec<Droplet>,
    /// Data stack
    pub stack: DataStack,
    /// Reservoir memory
    pub reservoir: Reservoir,
    /// Call stack
    pub call_stack: CallStack,
    /// Execution status
    pub status: ExecutionStatus,
}

pub enum ExecutionStatus {
    Running,
    Completed,
    Error(InterpreterError),
    Timeout(u64), // tick limit reached
}
```

## Operation Contracts

### Flow Control Operations
```rust
pub trait FlowControl {
    /// Process droplet movement through flow control cell
    fn process_flow_control(
        &self,
        droplet: &Droplet,
        cell: &ProgramCell,
    ) -> FlowControlResult;
}

pub struct FlowControlResult {
    /// New direction after pipe processing
    pub direction: Direction,
    /// Whether droplet should continue moving
    pub should_continue: bool,
    /// Any side effects to apply
    pub effects: Vec<DropletEffect>,
}
```

**Pipe Behavior Matrix**:
| Input Direction | Pipe | Output Direction | Notes |
|-----------------|------|------------------|-------|
| Any | `|` | Same (vertical) | Vertical pipe |
| Any | `-` | Same (horizontal) | Horizontal pipe |
| Any | `^` | Up | Force upward |
| Right | `/` | Up | Corner up-left |
| Down | `/` | Left | Corner up-left |
| Right | `\` | Down | Corner down-left |
| Up | `\` | Left | Corner down-left |
| Left | `/` | Down | Corner down-right |
| Up | `/` | Right | Corner down-right |
| Left | `\` | Up | Corner up-right |
| Down | `\` | Right | Corner up-right |

### Data Source Operations
```rust
pub trait DataSource {
    /// Generate value for droplet from data source
    fn generate_value(
        &mut self,
        source: &DataSourceType,
        droplet: &Droplet,
    ) -> Result<BigInt, SourceError>;
}

pub enum DataSourceType {
    Literal(i64),           // 0-9
    TapeReader,            // >
    CharacterInput,        // ?
    NumericInput,          // ??
}

pub enum SourceError {
    InputUnavailable,
    InvalidInput,
    IOError(std::io::Error),
}
```

**Data Source Behavior**:
- **Literal (0-9)**: Returns corresponding integer value
- **Tape Reader (>)**: Reads next character from input tape, returns ASCII code
- **Character Input (?)**: Reads single character from stdin, returns ASCII code
- **Numeric Input (??)**: Reads integer from stdin, returns parsed value

### Data Sink Operations
```rust
pub trait DataSink {
    /// Consume droplet value at sink
    fn consume_value(
        &mut self,
        sink: &DataSinkType,
        value: &BigInt,
    ) -> Result<SinkResult, SinkError>;
}

pub enum DataSinkType {
    Output,               // !
    CharacterOutput,      // ,
    NumericOutput,        // n
}

pub struct SinkResult {
    /// Whether droplet is destroyed
    pub destroys_droplet: bool,
    /// Any output produced
    pub output: Option<String>,
}
```

**Data Sink Behavior**:
- **Output (!)**: Destroys droplet, no output
- **Character Output (,)**: Outputs character represented by value, droplet continues
- **Numeric Output (n)**: Outputs decimal representation, droplet continues

### Stack Operations
```rust
pub trait StackOperations {
    /// Execute stack operation on droplet
    fn execute_stack_operation(
        &mut self,
        operation: StackOperation,
        droplet: &mut Droplet,
    ) -> Result<(), StackError>;
}

pub enum StackOperation {
    Push,                 // :
    Pop,                  // ;
    Duplicate,            // d
    Add,                  // A
    Subtract,             // S
    Multiply,             // M
    Divide,               // D
    Equal,                // =
    LessThan,             // <
    GreaterThan,          // >
    Modulo,               // %
}
```

**Stack Operation Semantics**:
- **Push (:)**: Push droplet value to stack, droplet value unchanged
- **Pop (;)**: Pop value from stack to droplet, stack underflow returns 0
- **Duplicate (d)**: Push copy of top stack value
- **Arithmetic**: Pop required operands, compute result, push result back
- **Division**: Division by zero returns 0
- **Modulo**: Modulo by zero returns 0

### Memory Operations
```rust
pub trait MemoryOperations {
    /// Execute reservoir memory operation
    fn execute_memory_operation(
        &mut self,
        operation: MemoryOperation,
        droplet: &mut Droplet,
        coordinate: Coordinate,
    ) -> Result<(), MemoryError>;
}

pub enum MemoryOperation {
    Get,                  // G
    Put,                  // P
}
```

**Memory Operation Semantics**:
- **Get (G)**: Read value from reservoir coordinate to droplet
  - Coordinate = (droplet.value, stack.pop())
  - Uninitialized memory returns 0
- **Put (P)**: Write droplet value to reservoir coordinate
  - Coordinate = (droplet.value, stack.pop())
  - Overwrites existing value

### Subroutine Operations
```rust
pub trait SubroutineOperations {
    /// Execute subroutine operation
    fn execute_subroutine_operation(
        &mut self,
        operation: SubroutineOperation,
        droplet: &mut Droplet,
    ) -> Result<(), SubroutineError>;
}

pub enum SubroutineOperation {
    Call,                 // C
    Return,               // R
}
```

**Subroutine Operation Semantics**:
- **Call (C)**: Push current position + direction to call stack, jump to new location
  - Target coordinate = (droplet.value, stack.pop())
  - Direction after jump = stack.pop() (converted to direction)
- **Return (R)**: Pop call stack, return to saved position + direction
  - Empty call stack = no operation

## Tick Execution Algorithm

```rust
pub fn execute_tick(&mut self) -> TickResult {
    let mut next_positions: HashMap<Coordinate, Vec<DropletId>> = HashMap::new();
    let mut commands: Vec<DropletCommand> = Vec::new();

    // Phase 1: Calculate movements and generate commands
    for droplet in &self.state.droplets {
        if !droplet.active { continue; }

        let current_cell = self.grid.get(droplet.position);
        let command = self.process_cell(droplet, current_cell)?;

        match command.action {
            Action::Move(direction) => {
                let next_pos = droplet.position + direction_vector(direction);
                next_positions.entry(next_pos).or_default().push(droplet.id);
                commands.push(command);
            }
            Action::Destroy => {
                commands.push(DropletCommand {
                    id: droplet.id,
                    action: Action::Destroy,
                });
            }
            Action::Stay => {
                commands.push(command);
            }
        }
    }

    // Phase 2: Detect collisions
    let mut destroyed_droplets: HashSet<DropletId> = HashSet::new();
    for (position, droplet_ids) in next_positions {
        if droplet_ids.len() > 1 {
            // Collision detected - destroy all droplets
            for id in droplet_ids {
                destroyed_droplets.insert(id);
            }
        }
    }

    // Phase 3: Execute commands (except destroyed droplets)
    for command in commands {
        if destroyed_droplets.contains(&command.id) {
            continue;
        }
        self.execute_command(command)?;
    }

    // Phase 4: Remove destroyed droplets
    self.state.droplets.retain(|d| !destroyed_droplets.contains(&d.id));

    TickResult {
        tick: self.state.tick,
        droplets_active: self.state.droplets.len(),
        collisions: destroyed_droplets.len(),
    }
}
```

## Performance Contracts

### Timing Requirements
- **Maximum Tick Time**: 10ms for typical programs (1000 droplets)
- **Memory Allocation**: Minimal per-tick allocations
- **Collision Detection**: O(n) where n = active droplets
- **Grid Access**: O(1) for cell lookup

### Memory Requirements
- **Droplet Storage**: ~64 bytes per droplet
- **Stack Storage**: ~16 bytes per stack element
- **Grid Storage**: Proportional to active cells, not total grid size
- **Reservoir Storage**: Proportional to written cells only

### Scalability Guarantees
- Support for 1000+ concurrent droplets
- Grid sizes up to 1000x1000 without performance degradation
- Stack depth up to 1000 levels
- Unbounded reservoir memory (limited by system memory)

## Error Handling Contracts

### Error Categories
```rust
pub enum InterpreterError {
    /// Errors during initialization
    InitializationError(InitError),

    /// Errors during execution
    ExecutionError(ExecError),

    /// System-level errors
    SystemError(SystemError),
}

pub enum InitError {
    NoStartSymbol,
    MultipleStartSymbols,
    InvalidCharacter(char, Coordinate),
    GridSizeExceeded,
}

pub enum ExecError {
    StackUnderflow,
    DivisionByZero,
    InvalidMemoryAccess,
    SubroutineUnderflow,
}

pub enum SystemError {
    OutOfMemory,
    IoError(std::io::Error),
    InternalError(String),
}
```

### Error Recovery
- **Non-fatal errors**: Log and continue execution
- **Fatal errors**: Stop execution and return error code
- **Recoverable conditions**: Handle gracefully per language spec

### Debugging Support
```rust
pub trait DebugSupport {
    /// Enable execution tracing
    fn enable_tracing(&mut self) -> Result<(), DebugError>;

    /// Get execution trace
    fn get_trace(&self) -> &[TraceEvent];

    /// Set breakpoints
    fn set_breakpoint(&mut self, coordinate: Coordinate) -> Result<(), DebugError>;

    /// Get current call stack
    fn get_call_stack(&self) -> &[StackFrame];
}
```