use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, InterpreterError, ExecError};
use crate::interpreter::droplet::{Droplet, DropletId};
use crate::interpreter::grid::ProgramGrid;
use crate::interpreter::stack::DataStack;
use crate::interpreter::memory::Reservoir;
use crate::interpreter::subroutines::CallStack;
use crate::operations::arithmetic::ArithmeticOperations;
use crate::operations::io::IoOperations;
use crate::operations::flow_control::FlowControlOperations;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::time::Instant;

/// Configuration for execution limits and timeouts
#[derive(Debug, Clone)]
pub struct ExecutionLimits {
    /// Maximum number of ticks before hard timeout (None = no limit)
    pub max_ticks: Option<u64>,
    /// Maximum wall-clock time before hard timeout in ms (None = no limit)
    pub max_time_ms: Option<u64>,
    /// Soft tick limit for warnings (None = no warnings)
    pub soft_tick_limit: Option<u64>,
    /// Soft time limit for warnings in ms (None = no warnings)
    pub soft_time_limit_ms: Option<u64>,
    /// Progress reporting interval in ticks (None = no progress reports)
    pub progress_interval: Option<u64>,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self {
            max_ticks: Some(1000),
            max_time_ms: Some(5000), // 5 seconds default
            soft_tick_limit: Some(800), // Warn at 80% of hard limit
            soft_time_limit_ms: Some(4000), // Warn at 80% of hard limit
            progress_interval: Some(100), // Report every 100 ticks
        }
    }
}

impl ExecutionLimits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_ticks(mut self, max_ticks: Option<u64>) -> Self {
        self.max_ticks = max_ticks;
        self
    }

    pub fn with_max_time_ms(mut self, max_time_ms: Option<u64>) -> Self {
        self.max_time_ms = max_time_ms;
        self
    }

    pub fn with_soft_tick_limit(mut self, soft_limit: Option<u64>) -> Self {
        self.soft_tick_limit = soft_limit;
        self
    }

    pub fn with_soft_time_limit_ms(mut self, soft_limit: Option<u64>) -> Self {
        self.soft_time_limit_ms = soft_limit;
        self
    }

    pub fn with_progress_interval(mut self, interval: Option<u64>) -> Self {
        self.progress_interval = interval;
        self
    }

    pub fn unlimited() -> Self {
        Self {
            max_ticks: None,
            max_time_ms: None,
            soft_tick_limit: None,
            soft_time_limit_ms: None,
            progress_interval: None,
        }
    }
}

#[derive(Debug, Clone)]
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
    /// Collected output
    pub output: String,
    /// Next droplet ID
    pub next_droplet_id: DropletId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Running,
    Completed,
    Error(InterpreterError),
    TickTimeout(u64), // tick limit reached
    WallClockTimeout(u64), // wall-clock time limit reached in ms
    Warning(ExecutionWarning), // soft limit warning
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionWarning {
    SoftTickLimit(u64),
    SoftTimeLimit(u64),
}

#[derive(Debug, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub droplets_active: usize,
    pub collisions: usize,
    pub output: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub total_ticks: u64,
    pub final_output: String,
    pub status: ExecutionStatus,
    pub max_droplets: usize,
    pub max_stack_depth: usize,
    pub execution_time_ms: u64,
    pub warnings_issued: Vec<ExecutionWarning>,
    pub progress_reports: Vec<ProgressReport>,
}

#[derive(Debug, Clone)]
pub struct ProgressReport {
    pub tick: u64,
    pub elapsed_time_ms: u64,
    pub active_droplets: usize,
    pub total_collisions: usize,
    pub stack_depth: usize,
}

/// Main interpreter that executes Tubular programs
pub struct TubularInterpreter {
    state: ExecutionState,
    grid: ProgramGrid,
    verbose: bool,
    trace: bool,
    limits: ExecutionLimits,
    start_time: Option<Instant>,
    warnings_issued: Vec<ExecutionWarning>,
    progress_reports: Vec<ProgressReport>,
    total_collisions: usize,
}

impl TubularInterpreter {
    /// Create a new interpreter with the given program grid
    pub fn new(grid: ProgramGrid) -> Result<Self> {
        // Validate the grid
        grid.validate()?;

        // Find start position
        let start_pos = grid.start.ok_or(InterpreterError::Initialization(
            crate::types::error::InitError::NoStartSymbol
        ))?;

        // Create initial droplet
        let initial_droplet = Droplet::new(0, start_pos, Direction::Down);

        let state = ExecutionState {
            tick: 0,
            droplets: vec![initial_droplet],
            stack: DataStack::new(),
            reservoir: Reservoir::new(),
            call_stack: CallStack::new(),
            status: ExecutionStatus::Running,
            output: String::new(),
            next_droplet_id: 1,
        };

        Ok(TubularInterpreter {
            state,
            grid,
            verbose: false,
            trace: false,
            limits: ExecutionLimits::default(),
            start_time: None,
            warnings_issued: Vec::new(),
            progress_reports: Vec::new(),
            total_collisions: 0,
        })
    }

    /// Set execution options (maintains backward compatibility)
    pub fn with_options(mut self, verbose: bool, trace: bool, max_ticks: Option<u64>) -> Self {
        self.verbose = verbose;
        self.trace = trace;
        if let Some(max_ticks) = max_ticks {
            self.limits.max_ticks = Some(max_ticks);
        }
        self
    }

    /// Set execution limits with full control
    pub fn with_limits(mut self, limits: ExecutionLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Get current execution limits
    pub fn limits(&self) -> &ExecutionLimits {
        &self.limits
    }

    /// Get elapsed execution time in milliseconds
    pub fn elapsed_time_ms(&self) -> Option<u64> {
        self.start_time.map(|start| start.elapsed().as_millis() as u64)
    }

    /// Get current execution state
    pub fn state(&self) -> &ExecutionState {
        &self.state
    }

    /// Execute a single tick
    pub fn execute_tick(&mut self) -> Result<TickResult> {
        if self.state.status != ExecutionStatus::Running {
            return Ok(TickResult {
                tick: self.state.tick,
                droplets_active: 0,
                collisions: 0,
                output: None,
            });
        }

        // Initialize start time if this is the first tick
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        let elapsed_ms = self.elapsed_time_ms().unwrap_or(0);

        // Check hard limits first
        if let Some(max_ticks) = self.limits.max_ticks {
            if self.state.tick >= max_ticks {
                self.state.status = ExecutionStatus::TickTimeout(max_ticks);
                self.cleanup();
                return Ok(TickResult {
                    tick: self.state.tick,
                    droplets_active: 0,
                    collisions: 0,
                    output: None,
                });
            }
        }

        if let Some(max_time_ms) = self.limits.max_time_ms {
            if elapsed_ms >= max_time_ms {
                self.state.status = ExecutionStatus::WallClockTimeout(max_time_ms);
                self.cleanup();
                return Ok(TickResult {
                    tick: self.state.tick,
                    droplets_active: 0,
                    collisions: 0,
                    output: None,
                });
            }
        }

        // Check soft limits and issue warnings (but don't stop execution)
        if let Some(soft_tick_limit) = self.limits.soft_tick_limit {
            if self.state.tick >= soft_tick_limit && !self.warnings_issued.iter().any(|w| matches!(w, ExecutionWarning::SoftTickLimit(_))) {
                let warning = ExecutionWarning::SoftTickLimit(soft_tick_limit);
                self.warnings_issued.push(warning.clone());

                if self.verbose {
                    eprintln!("⚠️  Warning: Approaching tick limit ({} ticks)", soft_tick_limit);
                }
            }
        }

        if let Some(soft_time_limit_ms) = self.limits.soft_time_limit_ms {
            if elapsed_ms >= soft_time_limit_ms && !self.warnings_issued.iter().any(|w| matches!(w, ExecutionWarning::SoftTimeLimit(_))) {
                let warning = ExecutionWarning::SoftTimeLimit(soft_time_limit_ms);
                self.warnings_issued.push(warning.clone());

                if self.verbose {
                    eprintln!("⚠️  Warning: Approaching time limit ({}ms)", soft_time_limit_ms);
                }
            }
        }

        let mut next_positions: HashMap<Coordinate, Vec<DropletId>> = HashMap::new();
        let mut commands: Vec<DropletCommand> = Vec::new();
        let mut output_this_tick = String::new();

        // Phase 1: Calculate movements and generate commands
        let mut i = 0;
        while i < self.state.droplets.len() {
            let droplet_id = self.state.droplets[i].id;
            let droplet = &mut self.state.droplets[i];

            if !droplet.active {
                i += 1;
                continue;
            }

            let current_cell = match self.grid.get(droplet.position) {
                Some(cell) => cell,
                None => {
                    // Droplet moved out of bounds - destroy it
                    commands.push(DropletCommand {
                        id: droplet.id,
                        action: Action::Destroy,
                    });
                    i += 1;
                    continue;
                }
            };

            // Process the cell and handle input operations inline to avoid borrow conflicts
            let command = match current_cell.symbol {
                '?' => {
                    // Input operations - need to handle inline
                    let next_pos = droplet.position + droplet.direction;
                    if let Some(next_cell) = self.grid.get(next_pos) {
                        if next_cell.symbol == '?' {
                            // This is ?? (numeric input)
                            let input_str = IoOperations::process_numeric_input()?;
                            if let Ok(value) = input_str.parse::<i64>() {
                                droplet.set_value(TubularBigInt::new(value));
                            } else {
                                droplet.set_value(TubularBigInt::zero());
                            }
                            DropletCommand::move_action(droplet_id, droplet.direction)
                        } else {
                            // Single ? (character input)
                            let input_str = IoOperations::process_character_input()?;
                            if input_str.len() >= 1 {
                                let char_value = input_str.chars().next().unwrap_or('\0') as u8;
                                droplet.set_value(TubularBigInt::new(char_value as i64));
                            }
                            DropletCommand::move_action(droplet_id, droplet.direction)
                        }
                    } else {
                        // Single ? at boundary (character input)
                        let input_str = IoOperations::process_character_input()?;
                        if input_str.len() >= 1 {
                            let char_value = input_str.chars().next().unwrap_or('\0') as u8;
                            droplet.set_value(TubularBigInt::new(char_value as i64));
                        }
                        DropletCommand::move_action(droplet_id, droplet.direction)
                    }
                }
                _ => {
                    // Process all other symbols using a simplified inline version
                    match current_cell.symbol {
                        // Flow control pipes
                        '|' | '-' => DropletCommand::move_action(droplet_id, droplet.direction),
                        '/' => {
                            let new_dir = match droplet.direction {
                                Direction::Right => Direction::Up,
                                Direction::Down => Direction::Left,
                                Direction::Left => Direction::Down,
                                Direction::Up => Direction::Right,
                            };
                            DropletCommand::move_action(droplet_id, new_dir)
                        }
                        '\\' => {
                            // Handle conditional branching for backslash
                            let new_dir = FlowControlOperations::process_conditional_branch(droplet, droplet.direction);
                            DropletCommand::move_action(droplet_id, new_dir)
                        }
                        '^' => DropletCommand::move_action(droplet_id, Direction::Up),
                        '@' => DropletCommand::move_action(droplet_id, droplet.direction),
                        '!' => DropletCommand::destroy_action(droplet_id),
                        '0'..='9' => {
                            let value = current_cell.symbol.to_digit(10).unwrap() as i64;
                            DropletCommand::set_value_action(droplet_id, TubularBigInt::new(value), droplet.direction)
                        }
                        _ if ArithmeticOperations::is_arithmetic_operation(current_cell.symbol) => {
                            DropletCommand { id: droplet_id, action: Action::Stay }
                        }
                        ',' => {
                            // Character output
                            let output_str = IoOperations::process_character_output(droplet)?;
                            output_this_tick.push_str(&output_str);
                            DropletCommand::move_action(droplet_id, droplet.direction)
                        }
                        'n' => {
                            // Numeric output
                            let output_str = IoOperations::process_numeric_output(droplet)?;
                            output_this_tick.push_str(&output_str);
                            DropletCommand::move_action(droplet_id, droplet.direction)
                        }
                        _ => DropletCommand::destroy_action(droplet_id),
                    }
                }
            };

            match command.action {
                Action::Move(direction) => {
                    let next_pos = droplet.position + direction;
                    next_positions.entry(next_pos).or_default().push(droplet_id);
                    commands.push(command);
                }
                Action::SetValue(_) => {
                    // SetValue operations don't move, just set the value
                    commands.push(command);
                }
                Action::SetValueAndMove(_, direction) => {
                    let next_pos = droplet.position + direction;
                    next_positions.entry(next_pos).or_default().push(droplet_id);
                    commands.push(command);
                }
                Action::Destroy => {
                    commands.push(DropletCommand {
                        id: droplet_id,
                        action: Action::Destroy,
                    });
                }
                Action::Stay => {
                    commands.push(command);
                }
            }
            i += 1;
        }

        // Phase 2: Detect collisions
        let mut destroyed_droplets: HashSet<DropletId> = HashSet::new();
        for (position, droplet_ids) in &next_positions {
            if droplet_ids.len() > 1 {
                // Collision detected - destroy all droplets
                for id in droplet_ids {
                    destroyed_droplets.insert(*id);
                }
                self.total_collisions += droplet_ids.len();
                if self.verbose {
                    eprintln!("[TICK {:05}] Collision at {} - {} droplets destroyed",
                        self.state.tick, position, droplet_ids.len());
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

        // Phase 5: Check if execution is complete
        if self.state.droplets.is_empty() {
            self.state.status = ExecutionStatus::Completed;
        }

        // Progress reporting
        if let Some(progress_interval) = self.limits.progress_interval {
            if self.state.tick % progress_interval == 0 {
                let progress_report = ProgressReport {
                    tick: self.state.tick,
                    elapsed_time_ms: elapsed_ms,
                    active_droplets: self.state.droplets.len(),
                    total_collisions: self.total_collisions,
                    stack_depth: self.state.stack.depth(),
                };
                self.progress_reports.push(progress_report.clone());

                if self.verbose {
                    eprintln!("[PROGRESS] Tick: {}, Time: {}ms, Droplets: {}, Collisions: {}, Stack: {}",
                        progress_report.tick, progress_report.elapsed_time_ms, progress_report.active_droplets,
                        progress_report.total_collisions, progress_report.stack_depth);
                }
            }
        }

        // Debug: Print droplet positions
        if self.trace {
            for droplet in &self.state.droplets {
                eprintln!("DEBUG: Droplet {} at {} direction: {}, active: {}",
                    droplet.id, droplet.position, droplet.direction, droplet.active);
            }
        }

        // Add output from this tick
        if !output_this_tick.is_empty() {
            self.state.output.push_str(&output_this_tick);
        }

        let result = TickResult {
            tick: self.state.tick,
            droplets_active: self.state.droplets.len(),
            collisions: destroyed_droplets.len(),
            output: if output_this_tick.is_empty() { None } else { Some(output_this_tick) },
        };

        self.state.tick += 1;
        Ok(result)
    }

    /// Execute until completion or timeout
    pub fn run(&mut self) -> Result<ExecutionResult> {
        // Initialize start time
        self.start_time = Some(Instant::now());

        let mut max_droplets = self.state.droplets.len();
        let mut total_ticks = 0;

        if self.verbose {
            eprintln!("Starting execution with limits: {:?}", self.limits);
        }

        while self.state.status == ExecutionStatus::Running {
            max_droplets = max_droplets.max(self.state.droplets.len());

            let tick_result = self.execute_tick()?;
            total_ticks = tick_result.tick;

            // Output immediate results if available
            if let Some(output) = tick_result.output {
                print!("{}", output);
                io::stdout().flush()?;
            }

            // Verbose logging
            if self.verbose {
                eprintln!("[TICK {:05}] Active droplets: {}, Collisions: {}",
                    tick_result.tick, tick_result.droplets_active, tick_result.collisions);
            }
        }

        // Handle timeout states with graceful shutdown
        let execution_time_ms = self.elapsed_time_ms().unwrap_or(0);

        // Report execution result
        if self.verbose {
            match &self.state.status {
                ExecutionStatus::TickTimeout(limit) => {
                    eprintln!("⏹️  Execution stopped: Tick limit of {} reached", limit);
                }
                ExecutionStatus::WallClockTimeout(limit) => {
                    eprintln!("⏹️  Execution stopped: Time limit of {}ms reached", limit);
                }
                ExecutionStatus::Completed => {
                    eprintln!("✅ Execution completed successfully");
                }
                ExecutionStatus::Error(error) => {
                    eprintln!("❌ Execution failed: {}", error);
                }
                _ => {}
            }
        }

        // Final progress report if we have any
        if let Some(_last_progress) = self.progress_reports.last() {
            if self.verbose {
                eprintln!("Final stats: {} ticks, {}ms, {} max droplets, {} total collisions",
                    total_ticks, execution_time_ms, max_droplets, self.total_collisions);
            }
        }

        Ok(ExecutionResult {
            total_ticks,
            final_output: self.state.output.clone(),
            status: self.state.status.clone(),
            max_droplets,
            max_stack_depth: self.state.stack.max_depth_reached(),
            execution_time_ms,
            warnings_issued: self.warnings_issued.clone(),
            progress_reports: self.progress_reports.clone(),
        })
    }

    /// Perform graceful cleanup when execution is terminated
    fn cleanup(&mut self) {
        if self.verbose {
            eprintln!("Performing graceful cleanup...");
        }

        // Clear all active droplets
        self.state.droplets.clear();

        // Clear any temporary resources
        self.state.call_stack.clear();

        // Mark as completed to prevent further execution
        if matches!(self.state.status, ExecutionStatus::TickTimeout(_) | ExecutionStatus::WallClockTimeout(_)) {
            // Keep the timeout status for reporting
        } else {
            self.state.status = ExecutionStatus::Completed;
        }

        if self.verbose {
            eprintln!("Cleanup completed");
        }
    }

    
    /// Execute a droplet command
    fn execute_command(&mut self, command: DropletCommand) -> Result<()> {
        let droplet = self.state.droplets.iter_mut()
            .find(|d| d.id == command.id)
            .ok_or_else(|| InterpreterError::Execution(ExecError::InternalError(
                format!("Droplet {} not found", command.id)
            )))?;

        match command.action {
            Action::Move(direction) => {
                droplet.set_direction(direction);
                droplet.move_to(droplet.next_position());
            }
            Action::SetValue(value) => {
                droplet.set_value(value);
            }
            Action::SetValueAndMove(value, direction) => {
                droplet.set_value(value);
                droplet.set_direction(direction);
                droplet.move_to(droplet.next_position());
            }
            Action::Destroy => {
                droplet.deactivate();
            }
            Action::Stay => {
                // Process operations when droplet stays in place
                let current_cell = self.grid.get(droplet.position)
                    .ok_or_else(|| InterpreterError::Execution(ExecError::InternalError(
                        format!("No cell found at position {}", droplet.position)
                    )))?;

                // Handle stack and arithmetic operations
                if ArithmeticOperations::is_arithmetic_operation(current_cell.symbol) {
                    ArithmeticOperations::process_stack_operation(
                        current_cell.symbol,
                        droplet,
                        &mut self.state.stack,
                    )?;

                    // After processing the operation, move the droplet forward
                    droplet.move_to(droplet.next_position());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DropletCommand {
    id: DropletId,
    action: Action,
}

impl DropletCommand {
    fn move_action(id: DropletId, direction: Direction) -> Self {
        DropletCommand {
            id,
            action: Action::Move(direction),
        }
    }

    fn set_value_action(id: DropletId, value: TubularBigInt, direction: Direction) -> Self {
        DropletCommand {
            id,
            action: Action::SetValueAndMove(value, direction),
        }
    }

    fn destroy_action(id: DropletId) -> Self {
        DropletCommand {
            id,
            action: Action::Destroy,
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    Move(Direction),
    SetValue(TubularBigInt),
    SetValueAndMove(TubularBigInt, Direction),
    Destroy,
    Stay,
}