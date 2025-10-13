use crate::interpreter::execution::{ExecutionResult, TickResult};
use crate::interpreter::droplet::Droplet;
use crate::interpreter::stack::DataStack;
use crate::interpreter::memory::Reservoir;
use crate::interpreter::subroutines::CallStack;
use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::bigint::TubularBigInt;
use std::io::{self, Write};
use std::collections::HashSet;
use std::time::{Duration, Instant};

/// Trace configuration for execution debugging
#[derive(Debug, Clone)]
pub struct TraceConfig {
    /// Trace level of detail
    pub level: TraceLevel,
    /// Output format for traces
    pub format: TraceFormat,
    /// Filter traces by droplet IDs (None = all droplets)
    pub droplet_filter: Option<HashSet<u64>>,
    /// Filter traces by operation types (None = all operations)
    pub operation_filter: Option<HashSet<TraceOperation>>,
    /// Filter traces by tick range (None = all ticks)
    pub tick_range: Option<(u64, u64)>,
    /// Maximum number of trace events to capture (None = unlimited)
    pub max_events: Option<usize>,
    /// Whether to include performance metrics
    pub include_performance: bool,
    /// Whether to include memory state changes
    pub include_memory: bool,
    /// Whether to include stack state changes
    pub include_stack: bool,
    /// Whether to include subroutine call tracking
    pub include_subroutines: bool,
}

/// Level of detail for execution tracing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceLevel {
    /// Basic tick-level information only
    Basic,
    /// Detailed operation-level information
    Detailed,
    /// Verbose execution with all state changes
    Verbose,
}

/// Output format for trace information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceFormat {
    /// Compact single-line format
    Compact,
    /// Detailed multi-line format
    Detailed,
    /// JSON structured format
    Json,
}

/// Types of operations that can be traced
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraceOperation {
    /// Droplet movement
    Movement,
    /// Value changes
    ValueChange,
    /// Stack operations (push, pop, etc.)
    StackOp,
    /// Memory operations (get, put)
    MemoryOp,
    /// Arithmetic operations
    ArithmeticOp,
    /// Input/output operations
    IoOp,
    /// Subroutine calls
    SubroutineCall,
    /// Subroutine returns
    SubroutineReturn,
    /// Direction changes
    DirectionChange,
    /// Collision detection
    Collision,
    /// Droplet creation/destruction
    DropletLifecycle,
}

/// Detailed trace event for comprehensive execution tracking
#[derive(Debug, Clone)]
pub struct TraceEvent {
    /// Tick number when this event occurred
    pub tick: u64,
    /// Timestamp for performance tracking
    pub timestamp: Duration,
    /// ID of the droplet involved (if applicable)
    pub droplet_id: Option<u64>,
    /// Type of operation
    pub operation: TraceOperation,
    /// Position in grid (if applicable)
    pub position: Option<Coordinate>,
    /// Cell symbol being processed (if applicable)
    pub cell_symbol: Option<char>,
    /// Detailed description of the event
    pub description: String,
    /// Previous state (for state changes)
    pub before_state: Option<TraceState>,
    /// New state (for state changes)
    pub after_state: Option<TraceState>,
    /// Additional metadata
    pub metadata: TraceMetadata,
}

/// State information for tracing
#[derive(Debug, Clone)]
pub struct TraceState {
    /// Droplet value
    pub droplet_value: Option<TubularBigInt>,
    /// Droplet direction
    pub droplet_direction: Option<Direction>,
    /// Stack contents (if enabled)
    pub stack_contents: Option<Vec<TubularBigInt>>,
    /// Call stack depth
    pub call_stack_depth: Option<usize>,
    /// Memory coordinates accessed (if applicable)
    pub memory_coord: Option<Coordinate>,
}

/// Additional metadata for trace events
#[derive(Debug, Clone)]
pub struct TraceMetadata {
    /// Execution time for this operation
    pub execution_time_us: Option<u64>,
    /// Number of active droplets at this point
    pub active_droplets: usize,
    /// Total memory usage estimate
    pub memory_usage_bytes: usize,
    /// Collision count for this tick
    pub collision_count: usize,
    /// Additional key-value data
    pub extra: std::collections::HashMap<String, String>,
}

/// Performance metrics collected during execution
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Time taken for this tick
    pub tick_time_us: u64,
    /// Number of droplet operations
    pub droplet_operations: usize,
    /// Stack operations count
    pub stack_operations: usize,
    /// Memory operations count
    pub memory_operations: usize,
    /// Collision detection time
    pub collision_time_us: u64,
    /// Grid access count
    pub grid_accesses: usize,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            level: TraceLevel::Basic,
            format: TraceFormat::Compact,
            droplet_filter: None,
            operation_filter: None,
            tick_range: None,
            max_events: None,
            include_performance: false,
            include_memory: false,
            include_stack: false,
            include_subroutines: false,
        }
    }
}

impl TraceConfig {
    /// Create a new trace configuration with basic settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set trace level
    pub fn with_level(mut self, level: TraceLevel) -> Self {
        self.level = level;
        self
    }

    /// Set trace format
    pub fn with_format(mut self, format: TraceFormat) -> Self {
        self.format = format;
        self
    }

    /// Filter by specific droplet IDs
    pub fn with_droplet_filter(mut self, droplet_ids: HashSet<u64>) -> Self {
        self.droplet_filter = Some(droplet_ids);
        self
    }

    /// Filter by specific operation types
    pub fn with_operation_filter(mut self, operations: HashSet<TraceOperation>) -> Self {
        self.operation_filter = Some(operations);
        self
    }

    /// Filter by tick range
    pub fn with_tick_range(mut self, start: u64, end: u64) -> Self {
        self.tick_range = Some((start, end));
        self
    }

    /// Set maximum number of events to capture
    pub fn with_max_events(mut self, max_events: usize) -> Self {
        self.max_events = Some(max_events);
        self
    }

    /// Enable performance metrics collection
    pub fn with_performance(mut self, include: bool) -> Self {
        self.include_performance = include;
        self
    }

    /// Enable memory state tracking
    pub fn with_memory(mut self, include: bool) -> Self {
        self.include_memory = include;
        self
    }

    /// Enable stack state tracking
    pub fn with_stack(mut self, include: bool) -> Self {
        self.include_stack = include;
        self
    }

    /// Enable subroutine tracking
    pub fn with_subroutines(mut self, include: bool) -> Self {
        self.include_subroutines = include;
        self
    }

    /// Create a comprehensive trace configuration for debugging
    pub fn comprehensive() -> Self {
        Self {
            level: TraceLevel::Verbose,
            format: TraceFormat::Detailed,
            droplet_filter: None,
            operation_filter: None,
            tick_range: None,
            max_events: None,
            include_performance: true,
            include_memory: true,
            include_stack: true,
            include_subroutines: true,
        }
    }

    /// Check if an event should be included based on filters
    pub fn should_include_event(&self, event: &TraceEvent) -> bool {
        // Check droplet filter
        if let Some(ref droplet_filter) = self.droplet_filter {
            if let Some(droplet_id) = event.droplet_id {
                if !droplet_filter.contains(&droplet_id) {
                    return false;
                }
            } else {
                // Event has no droplet ID, skip if droplet filter is active
                return false;
            }
        }

        // Check operation filter
        if let Some(ref operation_filter) = self.operation_filter {
            if !operation_filter.contains(&event.operation) {
                return false;
            }
        }

        // Check tick range filter
        if let Some((start, end)) = self.tick_range {
            if event.tick < start || event.tick > end {
                return false;
            }
        }

        true
    }
}

/// CLI output formatting for Tubular interpreter
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format verbose output for a tick
    pub fn format_verbose_tick(tick_result: &TickResult, droplets: &[Droplet], stack: &DataStack) -> String {
        let mut output = String::new();

        // Main tick information
        output.push_str(&format!(
            "[TICK {:05}] Active droplets: {}, Collisions: {}\n",
            tick_result.tick, tick_result.droplets_active, tick_result.collisions
        ));

        // Droplet details
        for droplet in droplets {
            if droplet.is_active() {
                output.push_str(&format!(
                    "  Droplet({}) at ({}) value={} direction={:?}\n",
                    droplet.id,
                    droplet.position,
                    droplet.value,
                    droplet.direction
                ));
            }
        }

        // Stack information
        if !stack.is_empty() {
            output.push_str(&format!("  Stack: {:?}\n", stack.as_slice()));
        }

        // Output from this tick
        if let Some(tick_output) = &tick_result.output {
            output.push_str(&format!("  Output: {}\n", tick_output));
        }

        output
    }

    /// Format trace output for step-by-step execution
    pub fn format_trace_tick(
        tick: u64,
        droplet: &Droplet,
        from_coord: Coordinate,
        to_coord: Coordinate,
        value_from: &str,
        value_to: &str,
        direction_from: &str,
        direction_to: &str,
        cell_symbol: char,
        operation: Option<&str>,
    ) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "TRACE | TICK {:05} | Droplet {} | ({}) -> ({}) | value: {} -> {} | dir: {} -> {}",
            tick,
            droplet.id,
            from_coord,
            to_coord,
            value_from,
            value_to,
            direction_from,
            direction_to
        ));

        // Add operation details if available
        if let Some(op_desc) = operation {
            output.push_str(&format!(" | Cell ({})  | {}", cell_symbol, op_desc));
        }

        output.push('\n');
        output
    }

    // === COMPREHENSIVE TRACING METHODS ===

    /// Create a basic trace event for droplet movement
    pub fn create_movement_trace_event(
        tick: u64,
        droplet_id: u64,
        from_pos: Coordinate,
        to_pos: Coordinate,
        direction: Direction,
        value: &TubularBigInt,
        cell_symbol: Option<char>,
    ) -> TraceEvent {
        let timestamp = Duration::from_millis(tick); // Simplified timestamp

        TraceEvent {
            tick,
            timestamp,
            droplet_id: Some(droplet_id),
            operation: TraceOperation::Movement,
            position: Some(to_pos),
            cell_symbol,
            description: format!(
                "Droplet {} moves from {} to {} heading {:?} with value {}",
                droplet_id, from_pos, to_pos, direction, value
            ),
            before_state: Some(TraceState {
                droplet_value: Some(value.clone()),
                droplet_direction: Some(direction),
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(from_pos),
            }),
            after_state: Some(TraceState {
                droplet_value: Some(value.clone()),
                droplet_direction: Some(direction),
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(to_pos),
            }),
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: 1,
                memory_usage_bytes: 0,
                collision_count: 0,
                extra: std::collections::HashMap::new(),
            },
        }
    }

    /// Create a trace event for stack operations
    pub fn create_stack_trace_event(
        tick: u64,
        droplet_id: u64,
        operation: char,
        position: Coordinate,
        stack_before: &[TubularBigInt],
        stack_after: &[TubularBigInt],
        droplet_value: &TubularBigInt,
    ) -> TraceEvent {
        let operation_type = match operation {
            ':' => TraceOperation::StackOp,
            ';' => TraceOperation::StackOp,
            'd' => TraceOperation::StackOp,
            'A' | 'S' | 'M' | 'D' | '=' | '<' | '>' | '%' => TraceOperation::ArithmeticOp,
            '+' | '~' => TraceOperation::ValueChange,
            _ => TraceOperation::StackOp,
        };

        let operation_desc = match operation {
            ':' => "push",
            ';' => "pop",
            'd' => "duplicate",
            'A' => "add",
            'S' => "subtract",
            'M' => "multiply",
            'D' => "divide",
            '=' => "equals",
            '<' => "less_than",
            '>' => "greater_than",
            '%' => "modulo",
            '+' => "increment",
            '~' => "decrement",
            _ => "unknown_stack_op",
        };

        TraceEvent {
            tick,
            timestamp: Duration::from_millis(tick),
            droplet_id: Some(droplet_id),
            operation: operation_type,
            position: Some(position),
            cell_symbol: Some(operation),
            description: format!(
                "Droplet {} performs {} operation at {} with value {}",
                droplet_id, operation_desc, position, droplet_value
            ),
            before_state: Some(TraceState {
                droplet_value: Some(droplet_value.clone()),
                droplet_direction: None,
                stack_contents: Some(stack_before.to_vec()),
                call_stack_depth: None,
                memory_coord: Some(position),
            }),
            after_state: Some(TraceState {
                droplet_value: Some(droplet_value.clone()),
                droplet_direction: None,
                stack_contents: Some(stack_after.to_vec()),
                call_stack_depth: None,
                memory_coord: Some(position),
            }),
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: 1,
                memory_usage_bytes: stack_before.len() * std::mem::size_of::<TubularBigInt>(),
                collision_count: 0,
                extra: {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("stack_depth_before".to_string(), stack_before.len().to_string());
                    extra.insert("stack_depth_after".to_string(), stack_after.len().to_string());
                    extra
                },
            },
        }
    }

    /// Create a trace event for memory operations
    pub fn create_memory_trace_event(
        tick: u64,
        droplet_id: u64,
        operation: char, // 'G' for get, 'P' for put
        position: Coordinate,
        memory_coord: Coordinate,
        memory_value: &TubularBigInt,
        droplet_value: &TubularBigInt,
    ) -> TraceEvent {
        let operation_type = match operation {
            'G' => TraceOperation::MemoryOp,
            'P' => TraceOperation::MemoryOp,
            _ => TraceOperation::MemoryOp,
        };

        let operation_desc = match operation {
            'G' => "get",
            'P' => "put",
            _ => "unknown_memory_op",
        };

        TraceEvent {
            tick,
            timestamp: Duration::from_millis(tick),
            droplet_id: Some(droplet_id),
            operation: operation_type,
            position: Some(position),
            cell_symbol: Some(operation),
            description: format!(
                "Droplet {} performs memory {} at coordinate {} with value {}",
                droplet_id, operation_desc, memory_coord, memory_value
            ),
            before_state: Some(TraceState {
                droplet_value: Some(droplet_value.clone()),
                droplet_direction: None,
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(memory_coord),
            }),
            after_state: Some(TraceState {
                droplet_value: Some(match operation {
                    'G' => memory_value.clone(), // After get, droplet has the memory value
                    'P' => droplet_value.clone(), // After put, droplet value unchanged
                    _ => droplet_value.clone(),
                }),
                droplet_direction: None,
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(memory_coord),
            }),
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: 1,
                memory_usage_bytes: std::mem::size_of::<TubularBigInt>(),
                collision_count: 0,
                extra: {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("memory_operation".to_string(), operation.to_string());
                    extra.insert("memory_coord_x".to_string(), memory_coord.x.to_string());
                    extra.insert("memory_coord_y".to_string(), memory_coord.y.to_string());
                    extra
                },
            },
        }
    }

    /// Create a trace event for I/O operations
    pub fn create_io_trace_event(
        tick: u64,
        droplet_id: u64,
        operation: char, // ',' for char output, 'n' for numeric output, '?' for input
        position: Coordinate,
        io_value: &str,
        droplet_value: &TubularBigInt,
    ) -> TraceEvent {
        let operation_type = match operation {
            ',' | 'n' => TraceOperation::IoOp,
            '?' => TraceOperation::IoOp,
            _ => TraceOperation::IoOp,
        };

        let operation_desc = match operation {
            ',' => "char_output",
            'n' => "numeric_output",
            '?' => "input",
            _ => "unknown_io_op",
        };

        TraceEvent {
            tick,
            timestamp: Duration::from_millis(tick),
            droplet_id: Some(droplet_id),
            operation: operation_type,
            position: Some(position),
            cell_symbol: Some(operation),
            description: format!(
                "Droplet {} performs {} at {} with value '{}'",
                droplet_id, operation_desc, position, io_value
            ),
            before_state: Some(TraceState {
                droplet_value: Some(droplet_value.clone()),
                droplet_direction: None,
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(position),
            }),
            after_state: Some(TraceState {
                droplet_value: Some(droplet_value.clone()),
                droplet_direction: None,
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(position),
            }),
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: 1,
                memory_usage_bytes: 0,
                collision_count: 0,
                extra: {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("io_value".to_string(), io_value.to_string());
                    extra.insert("io_operation".to_string(), operation.to_string());
                    extra
                },
            },
        }
    }

    /// Create a trace event for collision detection
    pub fn create_collision_trace_event(
        tick: u64,
        collision_position: Coordinate,
        colliding_droplet_ids: &[u64],
        active_droplet_count: usize,
    ) -> TraceEvent {
        TraceEvent {
            tick,
            timestamp: Duration::from_millis(tick),
            droplet_id: None, // Multiple droplets involved
            operation: TraceOperation::Collision,
            position: Some(collision_position),
            cell_symbol: None,
            description: format!(
                "Collision at {} involving {} droplets: {:?}",
                collision_position, colliding_droplet_ids.len(), colliding_droplet_ids
            ),
            before_state: None,
            after_state: None,
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: active_droplet_count,
                memory_usage_bytes: 0,
                collision_count: colliding_droplet_ids.len(),
                extra: {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("colliding_droplets".to_string(),
                        format!("[{}]", colliding_droplet_ids.iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")));
                    extra
                },
            },
        }
    }

    /// Create a trace event for droplet lifecycle (creation/destruction)
    pub fn create_lifecycle_trace_event(
        tick: u64,
        droplet_id: u64,
        event_type: &str, // "created" or "destroyed"
        position: Coordinate,
        value: &TubularBigInt,
        direction: Direction,
    ) -> TraceEvent {
        TraceEvent {
            tick,
            timestamp: Duration::from_millis(tick),
            droplet_id: Some(droplet_id),
            operation: TraceOperation::DropletLifecycle,
            position: Some(position),
            cell_symbol: None,
            description: format!(
                "Droplet {} {} at {} with value {} heading {:?}",
                droplet_id, event_type, position, value, direction
            ),
            before_state: None,
            after_state: Some(TraceState {
                droplet_value: Some(value.clone()),
                droplet_direction: Some(direction),
                stack_contents: None,
                call_stack_depth: None,
                memory_coord: Some(position),
            }),
            metadata: TraceMetadata {
                execution_time_us: None,
                active_droplets: 1,
                memory_usage_bytes: std::mem::size_of::<TubularBigInt>(),
                collision_count: 0,
                extra: {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("lifecycle_event".to_string(), event_type.to_string());
                    extra
                },
            },
        }
    }

    /// Format a single trace event according to the specified format
    pub fn format_trace_event(&self, event: &TraceEvent, config: &TraceConfig) -> String {
        match config.format {
            TraceFormat::Compact => self.format_trace_event_compact(event, config),
            TraceFormat::Detailed => self.format_trace_event_detailed(event, config),
            TraceFormat::Json => self.format_trace_event_json(event, config),
        }
    }

    /// Format trace event in compact format
    fn format_trace_event_compact(&self, event: &TraceEvent, config: &TraceConfig) -> String {
        let mut output = String::new();

        // Basic tick and operation info
        output.push_str(&format!(
            "[{:05}] {:?}",
            event.tick, event.operation
        ));

        // Add droplet ID if available
        if let Some(droplet_id) = event.droplet_id {
            output.push_str(&format!(" D{}", droplet_id));
        }

        // Add position if available
        if let Some(pos) = event.position {
            output.push_str(&format!(" @{}", pos));
        }

        // Add cell symbol if available
        if let Some(symbol) = event.cell_symbol {
            output.push_str(&format!(" '{}'", symbol));
        }

        // Add brief description based on level
        match config.level {
            TraceLevel::Basic => {
                // Just the operation type
            }
            TraceLevel::Detailed => {
                output.push_str(&format!(": {}", event.description));
            }
            TraceLevel::Verbose => {
                output.push_str(&format!(": {}", event.description));

                // Add value change if available
                if let (Some(before), Some(after)) = (&event.before_state, &event.after_state) {
                    if let (Some(before_val), Some(after_val)) = (&before.droplet_value, &after.droplet_value) {
                        if before_val != after_val {
                            output.push_str(&format!(" ({} -> {})", before_val, after_val));
                        }
                    }
                }
            }
        }

        output.push('\n');
        output
    }

    /// Format trace event in detailed format
    fn format_trace_event_detailed(&self, event: &TraceEvent, config: &TraceConfig) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "╔══ Tick {:05} - {:?}\n", event.tick, event.operation
        ));

        // Droplet information
        if let Some(droplet_id) = event.droplet_id {
            output.push_str(&format!("║ Droplet: {}\n", droplet_id));
        }

        // Position and cell information
        if let Some(pos) = event.position {
            output.push_str(&format!("║ Position: {}\n", pos));
        }
        if let Some(symbol) = event.cell_symbol {
            output.push_str(&format!("║ Cell Symbol: '{}'\n", symbol));
        }

        // Description
        output.push_str(&format!("║ Description: {}\n", event.description));

        // State changes (if enabled)
        if config.level == TraceLevel::Verbose {
            if let (Some(before), Some(after)) = (&event.before_state, &event.after_state) {
                output.push_str("║ State Changes:\n");

                if let (Some(before_val), Some(after_val)) = (&before.droplet_value, &after.droplet_value) {
                    if before_val != after_val {
                        output.push_str(&format!("║   Value: {} -> {}\n", before_val, after_val));
                    }
                }

                if let (Some(before_dir), Some(after_dir)) = (&before.droplet_direction, &after.droplet_direction) {
                    if before_dir != after_dir {
                        output.push_str(&format!("║   Direction: {:?} -> {:?}\n", before_dir, after_dir));
                    }
                }

                if config.include_stack {
                    if let Some(stack_before) = &before.stack_contents {
                        output.push_str(&format!("║   Stack Before: [{}]\n",
                            stack_before.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")));
                    }
                    if let Some(stack_after) = &after.stack_contents {
                        output.push_str(&format!("║   Stack After: [{}]\n",
                            stack_after.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")));
                    }
                }
            }
        }

        // Performance metrics (if enabled)
        if config.include_performance {
            output.push_str("║ Performance:\n");
            if let Some(exec_time) = event.metadata.execution_time_us {
                output.push_str(&format!("║   Execution Time: {}μs\n", exec_time));
            }
            output.push_str(&format!("║   Active Droplets: {}\n", event.metadata.active_droplets));
            output.push_str(&format!("║   Memory Usage: {} bytes\n", event.metadata.memory_usage_bytes));
        }

        // Additional metadata
        if !event.metadata.extra.is_empty() {
            output.push_str("║ Metadata:\n");
            for (key, value) in &event.metadata.extra {
                output.push_str(&format!("║   {}: {}\n", key, value));
            }
        }

        output.push_str("╚══\n");
        output
    }

    /// Format trace event in JSON format
    fn format_trace_event_json(&self, event: &TraceEvent, _config: &TraceConfig) -> String {
        let mut json_parts = Vec::new();

        // Basic fields
        json_parts.push(format!("\"tick\": {}", event.tick));
        json_parts.push(format!("\"operation\": \"{:?}\"", event.operation));
        json_parts.push(format!("\"timestamp_us\": {}", event.timestamp.as_micros()));

        if let Some(droplet_id) = event.droplet_id {
            json_parts.push(format!("\"droplet_id\": {}", droplet_id));
        }

        if let Some(pos) = event.position {
            json_parts.push(format!("\"position\": {{\"x\": {}, \"y\": {}}}", pos.x, pos.y));
        }

        if let Some(symbol) = event.cell_symbol {
            json_parts.push(format!("\"cell_symbol\": \"{}\"", symbol));
        }

        json_parts.push(format!("\"description\": \"{}\"",
            event.description.replace('"', "\\\"")));

        // State information
        if let (Some(before), Some(after)) = (&event.before_state, &event.after_state) {
            let before_json = self.format_trace_state_json(before);
            let after_json = self.format_trace_state_json(after);
            json_parts.push(format!("\"before_state\": {}", before_json));
            json_parts.push(format!("\"after_state\": {}", after_json));
        }

        // Metadata
        json_parts.push(format!("\"active_droplets\": {}", event.metadata.active_droplets));
        json_parts.push(format!("\"memory_usage_bytes\": {}", event.metadata.memory_usage_bytes));
        json_parts.push(format!("\"collision_count\": {}", event.metadata.collision_count));

        if let Some(exec_time) = event.metadata.execution_time_us {
            json_parts.push(format!("\"execution_time_us\": {}", exec_time));
        }

        if !event.metadata.extra.is_empty() {
            let extra_json = event.metadata.extra.iter()
                .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
                .collect::<Vec<_>>()
                .join(", ");
            json_parts.push(format!("\"extra\": {{{}}}", extra_json));
        }

        format!("{{\n  {}\n}}", json_parts.join(",\n  "))
    }

    /// Helper to format TraceState as JSON
    fn format_trace_state_json(&self, state: &TraceState) -> String {
        let mut parts = Vec::new();

        if let Some(ref value) = state.droplet_value {
            parts.push(format!("\"droplet_value\": \"{}\"", value));
        }

        if let Some(ref direction) = state.droplet_direction {
            parts.push(format!("\"droplet_direction\": \"{:?}\"", direction));
        }

        if let Some(ref stack) = state.stack_contents {
            let stack_json = stack.iter()
                .map(|v| format!("\"{}\"", v))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("\"stack_contents\": [{}]", stack_json));
        }

        if let Some(depth) = state.call_stack_depth {
            parts.push(format!("\"call_stack_depth\": {}", depth));
        }

        if let Some(ref coord) = state.memory_coord {
            parts.push(format!("\"memory_coord\": {{\"x\": {}, \"y\": {}}}", coord.x, coord.y));
        }

        if parts.is_empty() {
            "{{}}".to_string()
        } else {
            format!("{{\n    {} }}", parts.join(",\n    "))
        }
    }

    /// Format multiple trace events with filtering
    pub fn format_trace_events(&self, events: &[TraceEvent], config: &TraceConfig) -> String {
        let mut output = String::new();

        // Apply filters
        let filtered_events: Vec<&TraceEvent> = events.iter()
            .filter(|event| config.should_include_event(event))
            .take(config.max_events.unwrap_or(events.len()))
            .collect();

        // Add header
        match config.format {
            TraceFormat::Compact => {
                output.push_str(&format!(
                    "Trace Output - {} events (level: {:?})\n",
                    filtered_events.len(), config.level
                ));
            }
            TraceFormat::Detailed => {
                output.push_str(&format!(
                    "╔════════════════════════════════════════════════════════════╗\n"
                ));
                output.push_str(&format!(
                    "║                    EXECUTION TRACE                         ║\n"
                ));
                output.push_str(&format!(
                    "║  Events: {} | Level: {:?} | Format: {:?}               ║\n",
                    filtered_events.len(), config.level, config.format
                ));
                output.push_str(&format!(
                    "╚════════════════════════════════════════════════════════════╝\n"
                ));
            }
            TraceFormat::Json => {
                output.push_str("{\n");
                output.push_str(&format!("  \"trace_info\": {{\n"));
                output.push_str(&format!("    \"total_events\": {},\n", filtered_events.len()));
                output.push_str(&format!("    \"level\": \"{:?}\",\n", config.level));
                output.push_str(&format!("    \"format\": \"{:?}\"\n", config.format));
                output.push_str(&format!("  }},\n"));
                output.push_str(&format!("  \"events\": [\n"));
            }
        }

        // Format each event
        for (i, event) in filtered_events.iter().enumerate() {
            let formatted = self.format_trace_event(event, config);

            match config.format {
                TraceFormat::Json => {
                    output.push_str("    ");
                    // Add comma between events except last one
                    if i < filtered_events.len() - 1 {
                        output.push_str(&formatted.replace('\n', "\n    "));
                        output.push_str(",");
                    } else {
                        output.push_str(&formatted.replace('\n', "\n    "));
                    }
                    output.push('\n');
                }
                _ => {
                    output.push_str(&formatted);
                }
            }
        }

        // Add footer
        match config.format {
            TraceFormat::Json => {
                output.push_str("  ]\n");
                output.push_str("}\n");
            }
            TraceFormat::Detailed => {
                output.push_str("╔════════════════════════════════════════════════════════════╗\n");
                output.push_str("║                     END OF TRACE                          ║\n");
                output.push_str("╚════════════════════════════════════════════════════════════╝\n");
            }
            _ => {
                output.push_str(&format!("End of trace - {} events displayed\n", filtered_events.len()));
            }
        }

        output
    }

    /// Format performance metrics summary
    pub fn format_performance_metrics(&self, metrics: &PerformanceMetrics) -> String {
        format!(
            "Performance Metrics:\n\
             Tick Time: {}μs\n\
             Droplet Operations: {}\n\
             Stack Operations: {}\n\
             Memory Operations: {}\n\
             Collision Detection: {}μs\n\
             Grid Accesses: {}\n\
             Operations per Second: {:.2}\n",
            metrics.tick_time_us,
            metrics.droplet_operations,
            metrics.stack_operations,
            metrics.memory_operations,
            metrics.collision_time_us,
            metrics.grid_accesses,
            if metrics.tick_time_us > 0 {
                (metrics.droplet_operations + metrics.stack_operations + metrics.memory_operations) as f64 * 1_000_000.0 / metrics.tick_time_us as f64
            } else {
                0.0
            }
        )
    }

    /// Format execution summary
    pub fn format_execution_summary(result: &ExecutionResult) -> String {
        let mut output = String::new();

        output.push_str("Execution Summary:\n");
        output.push_str("==================\n");
        output.push_str(&format!("Total Ticks: {}\n", result.total_ticks));
        output.push_str(&format!("Execution Time: {}ms\n", result.execution_time_ms));
        output.push_str(&format!("Peak Droplets: {}\n", result.max_droplets));
        output.push_str(&format!("Peak Stack Depth: {}\n", result.max_stack_depth));
        output.push_str(&format!("Final Status: {:?}\n", result.status));

        // Display timeout information
        match &result.status {
            crate::interpreter::execution::ExecutionStatus::TickTimeout(limit) => {
                output.push_str(&format!("⏹️  Stopped: Tick limit of {} reached\n", limit));
            }
            crate::interpreter::execution::ExecutionStatus::WallClockTimeout(limit) => {
                output.push_str(&format!("⏹️  Stopped: Time limit of {}ms reached\n", limit));
            }
            _ => {}
        }

        // Display warnings if any
        if !result.warnings_issued.is_empty() {
            output.push_str("Warnings issued:\n");
            for warning in &result.warnings_issued {
                match warning {
                    crate::interpreter::execution::ExecutionWarning::SoftTickLimit(limit) => {
                        output.push_str(&format!("  ⚠️  Approaching tick limit of {}\n", limit));
                    }
                    crate::interpreter::execution::ExecutionWarning::SoftTimeLimit(limit) => {
                        output.push_str(&format!("  ⚠️  Approaching time limit of {}ms\n", limit));
                    }
                }
            }
        }

        // Display progress reports if available
        if !result.progress_reports.is_empty() {
            output.push_str("Progress Reports:\n");
            let last_report = result.progress_reports.last().unwrap();
            output.push_str(&format!("  Final: {} ticks, {}ms elapsed, {} droplets active\n",
                last_report.tick, last_report.elapsed_time_ms, last_report.active_droplets));
        }

        if !result.final_output.is_empty() {
            output.push_str(&format!("Program Output: {}\n", result.final_output));
        }

        output
    }

    /// Format benchmark results (table format)
    pub fn format_benchmark_table(
        program_file: &str,
        execution_time_ms: u64,
        total_ticks: u64,
        peak_droplets: usize,
        peak_memory_mb: f64,
        instructions_per_sec: f64,
    ) -> String {
        let mut output = String::new();

        output.push_str(&format!("Benchmark Results for {}\n", program_file));
        output.push_str("========================================\n");
        output.push_str(&format!(
            "{:<25} | {:>12} | {:<6}\n",
            "Metric",
            "Value",
            "Unit"
        ));
        output.push_str("---------------------------|--------------|------\n");
        output.push_str(&format!("{:<25} | {:>12} | {:<6}\n", "Execution Time", execution_time_ms, "ms"));
        output.push_str(&format!("{:<25} | {:>12} | {:<6}\n", "Total Ticks", total_ticks, "ticks"));
        output.push_str(&format!("{:<25} | {:>12} | {:<6}\n", "Peak Droplet Count", peak_droplets, "droplets"));
        output.push_str(&format!("{:<25} | {:>12.1} | {:<6}\n", "Peak Memory Usage", peak_memory_mb, "MB"));
        output.push_str(&format!("{:<25} | {:>12} | {:<6}\n", "Instructions/sec", instructions_per_sec, "ops/sec"));

        output
    }

    /// Format benchmark results (JSON format)
    pub fn format_benchmark_json(
        program_file: &str,
        execution_time_ms: u64,
        total_ticks: u64,
        peak_droplets: usize,
        peak_memory_mb: f64,
        instructions_per_sec: f64,
    ) -> String {
        // Simple JSON implementation
        format!(
            r#"{{
  "program": "{}",
  "timestamp": "{}",
  "results": {{
    "execution_time_ms": {},
    "total_ticks": {},
    "peak_droplet_count": {},
    "peak_memory_usage_mb": {},
    "instructions_per_second": {}
  }}
}}"#,
            program_file,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            execution_time_ms,
            total_ticks,
            peak_droplets,
            peak_memory_mb,
            instructions_per_sec
        )
    }

    /// Format program validation results
    pub fn format_validation_result(
        file_path: &str,
        is_valid: bool,
        grid_size: (usize, usize),
        cell_count: usize,
        errors: &[String],
    ) -> String {
        let mut output = String::new();

        if is_valid {
            output.push_str(&format!(" Validation passed: {}\n", file_path));
            output.push_str(&format!("  Grid size: {}x{}\n", grid_size.0, grid_size.1));
            output.push_str(&format!("  Program cells: {}\n", cell_count));
        } else {
            output.push_str(&format!(" Validation failed: {}\n", file_path));
            output.push_str(&format!("  Grid size: {}x{}\n", grid_size.0, grid_size.1));
            output.push_str(&format!("  Program cells: {}\n", cell_count));

            if !errors.is_empty() {
                output.push_str("  Errors:\n");
                for error in errors {
                    output.push_str(&format!("    {}\n", error));
                }
            }
        }

        output
    }

    /// Format execution error with context
    pub fn format_execution_error(
        error: &crate::types::error::InterpreterError,
        program_file: &str,
        line: Option<usize>,
        column: Option<usize>,
        context: Option<&str>,
    ) -> String {
        let mut output = String::new();

        let error_code = Self::extract_error_code(error);
        output.push_str(&format!("Error: [{}] - {}\n", error_code, error));
        output.push_str(&format!("File: {}", program_file));

        if let Some(l) = line {
            output.push_str(&format!(", Line: {}", l));
            if let Some(c) = column {
                output.push_str(&format!(", Column: {}", c));
            }
            output.push('\n');

            if let Some(ctx) = context {
                output.push_str(&format!("Context: {}\n", ctx));
                if let Some(col) = column {
                    output.push_str(&format!("{}^\n",
                        " ".repeat(col.saturating_sub(1))
                    ));
                }
            }
        } else {
            output.push('\n');
        }

        output
    }

    /// Extract error code from InterpreterError
    fn extract_error_code(error: &crate::types::error::InterpreterError) -> &'static str {
        use crate::types::error::{InterpreterError, ExecError, SystemError};

        match error {
            InterpreterError::Initialization(_) => "E003",
            InterpreterError::Execution(exec_error) => match exec_error {
                ExecError::StackUnderflow => "E004",
                ExecError::DivisionByZero => "E005",
                ExecError::ModuloByZero => "E006",
                ExecError::InvalidMemoryAccess(_) => "E007",
                ExecError::SubroutineUnderflow => "E008",
                ExecError::DropletCollision(_) => "E009",
                ExecError::ExecutionTimeout(_) => "E010",
                ExecError::WallClockTimeout(_) => "E016",
                ExecError::SoftTickLimitWarning(_) => "E017",
                ExecError::SoftTimeLimitWarning(_) => "E018",
                ExecError::InternalError(_) => "E011",
                ExecError::InvalidOperation(_) => "E015",
            },
            InterpreterError::System(sys_error) => match sys_error {
                SystemError::OutOfMemory => "E012",
                SystemError::IoError(_) => "E013",
                SystemError::InternalError(_) => "E014",
            },
            InterpreterError::Enhanced { info, .. } => {
                use crate::types::error::ErrorType;
                match info.error_type {
                    ErrorType::Syntax => "E001",
                    ErrorType::Validation => "E002",
                    ErrorType::Initialization => "E003",
                    ErrorType::Execution => "E004",
                    ErrorType::Runtime => "E005",
                    ErrorType::System => "E006",
                    ErrorType::Semantic => "E007",
                }
            }
        }
    }

    /// Print colored output if supported
    pub fn print_colored(output: &str, color: Color) -> Result<(), io::Error> {
        use std::env;

        if env::var("NO_COLOR").is_ok() {
            // Color output disabled
            print!("{}", output);
        } else {
            // Use ANSI color codes
            let color_code = match color {
                Color::Green => "\x1b[32m",
                Color::Red => "\x1b[31m",
                Color::Yellow => "\x1b[33m",
                Color::Blue => "\x1b[34m",
                Color::Reset => "\x1b[0m",
            };

            print!("{}{}{}", color_code, output, Color::Reset.color_code());
        }

        io::stdout().flush()
    }
}

/// Color options for output
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    Reset,
}

impl Color {
    fn color_code(&self) -> &'static str {
        match self {
            Color::Green => "\x1b[32m",
            Color::Red => "\x1b[31m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Reset => "\x1b[0m",
        }
    }
}

/// Simple JSON serialization for benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkJson {
    pub program: String,
    pub timestamp: String,
    pub results: BenchmarkResults,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub execution_time_ms: u64,
    pub total_ticks: u64,
    pub peak_droplet_count: usize,
    pub peak_memory_usage_mb: f64,
    pub instructions_per_second: f64,
}

// Include serde JSON support for benchmarking
#[cfg(feature = "json")]
mod json {
    use super::*;
    use serde_json;

    impl BenchmarkJson {
        pub fn to_json(&self) -> String {
            serde_json::to_string_pretty(self).unwrap_or_default()
        }
    }
}