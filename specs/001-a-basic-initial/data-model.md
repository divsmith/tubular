# Data Model: Tubular Language Interpreter

**Feature**: `001-a-basic-initial` | **Date**: 2025-10-11

## Core Entities

### Droplet
The fundamental execution unit representing data flow through the program.

```rust
pub struct Droplet {
    /// Unique identifier for tracking
    pub id: DropletId,
    /// Current integer value (arbitrary precision)
    pub value: BigInt,
    /// Current position in the grid
    pub position: Coordinate,
    /// Current direction of movement
    pub direction: Direction,
    /// Whether this droplet is active (will move next tick)
    pub active: bool,
}
```

**Key Attributes**:
- **Value**: Arbitrary precision integer supporting mathematical operations
- **Position**: 2D coordinate with potentially unbounded grid space
- **Direction**: One of four cardinal directions (Up, Down, Left, Right)
- **Lifecycle**: Created at start symbol (@), destroyed at sink (!) or collision

**Validation Rules**:
- Position must be within program grid bounds (or reservoir coordinates)
- Direction must be one of the four valid directions
- Value can be any integer (positive, negative, zero)

### Program Grid
Static 2D array representing the program structure with pipes and operators.

```rust
pub struct ProgramGrid {
    /// Sparse representation of program cells
    pub cells: HashMap<Coordinate, ProgramCell>,
    /// Bounding box of active program area
    pub bounds: BoundingBox,
    /// Start symbol location (must be exactly one)
    pub start: Coordinate,
}

pub struct ProgramCell {
    /// Character at this position
    pub symbol: char,
    /// Whether this cell affects droplet movement
    pub is_flow_control: bool,
    /// Whether this cell performs an operation
    pub is_operator: bool,
}
```

**Key Attributes**:
- **Sparse Storage**: Only store cells that contain program elements
- **Symbol Types**: Flow control (|,-,/,\\,^), operators (+,~,:;,dASMD etc.), data sources/sinks
- **Bounding Box**: Active program area for efficient iteration

**Validation Rules**:
- Must contain exactly one start symbol (@)
- All flow control paths must be valid (no orphaned pipes)
- Grid size minimum: 1000x1000 cells supported
- No invalid symbols allowed

### Data Stack
LIFO stack for temporary value storage during computation.

```rust
pub struct DataStack {
    /// Stack values (arbitrary precision integers)
    pub data: Vec<BigInt>,
    /// Maximum depth reached (for monitoring)
    pub max_depth: usize,
    /// Current stack pointer
    pub pointer: usize,
}
```

**Key Attributes**:
- **Capacity**: Minimum 1000 levels as per requirements
- **Operations**: Push (:), pop (;), duplicate (d), arithmetic operations
- **Underflow Protection**: Returns 0 when popping from empty stack

**Validation Rules**:
- Stack depth must not exceed minimum capacity
- Underflow operations return 0 (not error)
- All values are arbitrary precision integers

### Reservoir Memory
2D random-access memory grid with unbounded coordinates for persistent storage.

```rust
pub struct Reservoir {
    /// Sparse storage for memory cells
    pub data: HashMap<Coordinate, BigInt>,
    /// Cache for frequently accessed locations
    pub cache: LruCache<Coordinate, BigInt>,
}

pub struct ReservoirCoordinate {
    /// X coordinate (can be negative)
    pub x: isize,
    /// Y coordinate (can be negative)
    pub y: isize,
}
```

**Key Attributes**:
- **Unbounded Coordinates**: Supports negative coordinates as per spec
- **Sparse Storage**: Only store cells that have been written to
- **Operations**: Get (G) and Put (P) operations

**Validation Rules**:
- Coordinates can be any valid isize value
- Reading uninitialized memory returns 0
- No bounds checking required (unbounded memory)

### Call Stack
Stack for managing subroutine calls with return information.

```rust
pub struct CallStack {
    /// Stack of return positions
    pub frames: Vec<StackFrame>,
    /// Maximum depth reached
    pub max_depth: usize,
}

pub struct StackFrame {
    /// Position to return to
    pub return_position: Coordinate,
    /// Direction after return
    pub return_direction: Direction,
}
```

**Key Attributes**:
- **Nested Calls**: Supports arbitrary nesting depth
- **Return Information**: Stores both position and direction
- **Call Operations**: Call (C) and Return (R) operators

**Validation Rules**:
- Stack underflow on return behaves as no-op (continue execution)
- No explicit depth limit beyond memory constraints

## State Transitions

### Droplet Movement
```
Droplet at (x,y) with direction D
    ↓
Check ProgramGrid[(x,y)]
    ↓
If flow control: Update direction based on pipe
If operator: Perform operation, possibly modify droplet
    ↓
Calculate next position: (x',y') = (x,y) + direction_vector
    ↓
Check for collisions with other droplets at (x',y')
    ↓
If collision: Both droplets destroyed
Else: Move to (x',y')
```

### Stack Operations
```
Push (:) : Stack.push(droplet.value)
Pop (;)  : droplet.value = Stack.pop_or_zero()
Duplicate (d): Stack.push(Stack.peek())
Arithmetic: pop required operands, compute, push result
```

### Memory Operations
```
Put (P): Reservoir[coordinate] = droplet.value
Get (G): droplet.value = Reservoir[coordinate] (or 0 if unset)
```

## Relationships

### One-to-Many
- **ProgramGrid** contains many **ProgramCell** instances
- **DropletPool** manages many active **Droplet** instances

### Many-to-One
- Multiple **Droplet** instances can access the same **DataStack**
- Multiple **Droplet** instances can access the same **Reservoir**

### Composition
- **DataStack** is composed of **BigInt** values
- **CallStack** is composed of **StackFrame** instances
- **ProgramCell** references a **Coordinate** and **char** symbol

## Invariants

### Droplet Invariants
1. Active droplets always have a valid direction
2. Droplet values are always valid BigInt instances
3. Droplet positions are always valid coordinates within grid or reservoir

### Grid Invariants
1. Exactly one start symbol (@) exists
2. All flow control paths are syntactically valid
3. No invalid symbols are present in the grid

### Stack Invariants
1. Stack pointer never exceeds data length
2. Stack depth never exceeds minimum capacity requirement
3. All stack elements are BigInt values

### Memory Invariants
1. Reservoir coordinates are always valid isize values
2. Cache consistency maintained with main data store
3. Reading uninitialized memory always returns 0

## Error Conditions

### Fatal Errors (Stop Execution)
- Invalid program syntax (multiple start symbols, invalid characters)
- Grid size exceeds implementation limits
- Memory allocation failure

### Recoverable Conditions (Continue Execution)
- Stack underflow: return 0, continue execution
- Division by zero: return 0, continue execution
- Modulo by zero: return 0, continue execution
- Invalid memory access: return 0, continue execution
- Droplet collision: destroy both droplets, continue execution