use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, InterpreterError, ExecError};
use crate::interpreter::droplet::{Droplet, DropletId};
use crate::interpreter::grid::{ProgramGrid, ProgramCell};
use crate::interpreter::stack::DataStack;
use crate::interpreter::memory::Reservoir;
use crate::interpreter::subroutines::{CallStack, StackFrame};
use crate::operations::arithmetic::ArithmeticOperations;
use crate::operations::io::IoOperations;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

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
    Timeout(u64), // tick limit reached
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
}

/// Main interpreter that executes Tubular programs
pub struct TubularInterpreter {
    state: ExecutionState,
    grid: ProgramGrid,
    verbose: bool,
    trace: bool,
    max_ticks: Option<u64>,
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
            max_ticks: None,
        })
    }

    /// Set execution options
    pub fn with_options(mut self, verbose: bool, trace: bool, max_ticks: Option<u64>) -> Self {
        self.verbose = verbose;
        self.trace = trace;
        self.max_ticks = max_ticks;
        self
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

        // Check tick limit
        if let Some(max_ticks) = self.max_ticks {
            if self.state.tick >= max_ticks {
                self.state.status = ExecutionStatus::Timeout(max_ticks);
                return Ok(TickResult {
                    tick: self.state.tick,
                    droplets_active: 0,
                    collisions: 0,
                    output: None,
                });
            }
        }

        let mut next_positions: HashMap<Coordinate, Vec<DropletId>> = HashMap::new();
        let mut commands: Vec<DropletCommand> = Vec::new();
        let mut output_this_tick = String::new();

        // Phase 1: Calculate movements and generate commands
        for droplet in &self.state.droplets {
            if !droplet.active {
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
                    continue;
                }
            };

            let command = self.process_cell(droplet, current_cell, &mut output_this_tick)?;

            match command.action {
                Action::Move(direction) => {
                    let next_pos = droplet.position + direction;
                    next_positions.entry(next_pos).or_default().push(droplet.id);
                    commands.push(command);
                }
                Action::SetValue(_) => {
                    // SetValue operations don't move, just set the value
                    commands.push(command);
                }
                Action::SetValueAndMove(_, direction) => {
                    let next_pos = droplet.position + direction;
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
        for (position, droplet_ids) in &next_positions {
            if droplet_ids.len() > 1 {
                // Collision detected - destroy all droplets
                for id in droplet_ids {
                    destroyed_droplets.insert(*id);
                }
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

    /// Execute until completion or tick limit
    pub fn run(&mut self) -> Result<ExecutionResult> {
        let mut max_droplets = self.state.droplets.len();
        let mut total_ticks = 0;

        while self.state.status == ExecutionStatus::Running {
            max_droplets = max_droplets.max(self.state.droplets.len());

            let tick_result = self.execute_tick()?;
            total_ticks = tick_result.tick;

            // Output immediate results if available
            if let Some(output) = tick_result.output {
                print!("{}", output);
                io::stdout().flush()?;
            }

            if self.verbose {
                eprintln!("[TICK {:05}] Active droplets: {}, Collisions: {}",
                    tick_result.tick, tick_result.droplets_active, tick_result.collisions);
            }
        }

        Ok(ExecutionResult {
            total_ticks,
            final_output: self.state.output.clone(),
            status: self.state.status.clone(),
            max_droplets,
            max_stack_depth: self.state.stack.max_depth_reached(),
        })
    }

    /// Process a cell and determine the droplet's action
    fn process_cell(&self, droplet: &Droplet, cell: &ProgramCell, output: &mut String) -> Result<DropletCommand> {
        let symbol = cell.symbol;

        if self.trace {
            eprintln!("TRACE | TICK {:05} | Droplet {} | ({}) -> ({}) | value: {} -> {} | dir: {} -> {}",
                self.state.tick,
                droplet.id,
                droplet.position,
                droplet.next_position(),
                droplet.value,
                droplet.value, // Will be modified by operations
                droplet.direction,
                droplet.direction // Will be modified by flow control
            );
        }

        match symbol {
            // Flow control pipes
            '|' => Ok(DropletCommand::move_action(droplet.id, droplet.direction)),
            '-' => Ok(DropletCommand::move_action(droplet.id, droplet.direction)),
            '/' => {
                let new_dir = match droplet.direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                };
                Ok(DropletCommand::move_action(droplet.id, new_dir))
            }
            '\\' => {
                let new_dir = match droplet.direction {
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                };
                Ok(DropletCommand::move_action(droplet.id, new_dir))
            }
            '^' => Ok(DropletCommand::move_action(droplet.id, Direction::Up)),

            // Start symbol (just pass through)
            '@' => Ok(DropletCommand::move_action(droplet.id, droplet.direction)),

            // Sink symbol (destroy droplet)
            '!' => Ok(DropletCommand::destroy_action(droplet.id)),

            // Numeric literals (0-9) - set value and move
            '0'..='9' => {
                let value = symbol.to_digit(10).unwrap() as i64;
                Ok(DropletCommand::set_value_action(droplet.id, TubularBigInt::new(value), droplet.direction))
            }

            // Arithmetic and stack operations - these don't move, just modify state
            _ if ArithmeticOperations::is_arithmetic_operation(symbol) => {
                Ok(DropletCommand {
                    id: droplet.id,
                    action: Action::Stay,
                })
            }

            // Character output
            ',' => {
                let output_str = IoOperations::process_character_output(droplet)?;
                output.push_str(&output_str);
                Ok(DropletCommand::move_action(droplet.id, droplet.direction))
            }

            // Numeric output
            'n' => {
                let output_str = IoOperations::process_numeric_output(droplet)?;
                output.push_str(&output_str);
                Ok(DropletCommand::move_action(droplet.id, droplet.direction))
            }

            // For now, treat unknown symbols as empty space (destroy droplet)
            _ => Ok(DropletCommand::destroy_action(droplet.id)),
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