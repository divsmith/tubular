use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;
use std::env;
use std::time::{Duration, Instant};
use std::path::Path;

// Import necessary modules
use crate::parser::grid_parser::GridParser;
use crate::parser::validator::ProgramValidator;
use crate::interpreter::execution::TubularInterpreter;
use crate::types::error::InterpreterError;
use crate::cli::output::{OutputFormatter, TraceConfig, TraceLevel, TraceFormat, TraceOperation};

/// Environment variable configuration
#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub tick_limit: Option<u64>,
    pub verbose: bool,
    pub trace: bool,
    pub benchmark: bool,
    pub strict: bool,
    pub trace_config: TraceConfig,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            tick_limit: Some(1000), // Default tick limit
            verbose: false,
            trace: false,
            benchmark: false,
            strict: false,
            trace_config: TraceConfig::default(),
        }
    }
}

impl EnvConfig {
    /// Load configuration from environment variables and .env file
    pub fn load() -> Result<Self> {
        // Try to load .env file if it exists, but don't fail if it doesn't
        let _ = dotenvy::dotenv();

        let mut config = Self::default();

        // Load TUBULAR_TICK_LIMIT
        if let Ok(tick_str) = env::var("TUBULAR_TICK_LIMIT") {
            match tick_str.parse::<u64>() {
                Ok(ticks) => {
                    if ticks == 0 {
                        eprintln!("Warning: TUBULAR_TICK_LIMIT cannot be 0, using default");
                        config.tick_limit = Some(1000);
                    } else {
                        config.tick_limit = Some(ticks);
                    }
                }
                Err(_) => {
                    eprintln!("Error: Invalid TUBULAR_TICK_LIMIT value '{}'. Must be a positive integer.", tick_str);
                    return Err(anyhow::anyhow!("Invalid TUBULAR_TICK_LIMIT value"));
                }
            }
        }

        // Load TUBULAR_VERBOSE
        config.verbose = parse_bool_env("TUBULAR_VERBOSE")?;

        // Load TUBULAR_TRACE
        config.trace = parse_bool_env("TUBULAR_TRACE")?;

        // Load TUBULAR_BENCHMARK
        config.benchmark = parse_bool_env("TUBULAR_BENCHMARK")?;

        // Load TUBULAR_STRICT
        config.strict = parse_bool_env("TUBULAR_STRICT")?;

        Ok(config)
    }

    /// Apply CLI overrides to environment configuration
    pub fn apply_cli_overrides(mut self, cli: &Cli) -> Self {
        // CLI flags override environment variables
        if cli.verbose {
            self.verbose = true;
        }
        if cli.trace {
            self.trace = true;
        }
        if cli.benchmark {
            self.benchmark = true;
        }

        // Apply trace configuration overrides
        self.trace_config = self.apply_trace_overrides(cli);

        // Note: tick_limit from CLI is handled separately since it's Option<u64>
        self
    }

    /// Apply trace-specific CLI overrides
    fn apply_trace_overrides(&self, cli: &Cli) -> TraceConfig {
        let mut trace_config = self.trace_config.clone();

        // Enable trace if any trace options are provided
        if cli.trace || cli.trace_level.is_some() || cli.trace_format.is_some() ||
           cli.trace_droplets.is_some() || cli.trace_operations.is_some() ||
           cli.trace_ticks.is_some() || cli.trace_max_events.is_some() ||
           cli.trace_performance || cli.trace_memory || cli.trace_stack ||
           cli.trace_subroutines || cli.trace_output.is_some() {
            trace_config.level = TraceLevel::Detailed; // Default when trace options are used
        }

        // Apply trace level
        if let Some(ref level_str) = cli.trace_level {
            trace_config.level = match level_str.as_str() {
                "basic" => TraceLevel::Basic,
                "detailed" => TraceLevel::Detailed,
                "verbose" => TraceLevel::Verbose,
                _ => TraceLevel::Detailed,
            };
        }

        // Apply trace format
        if let Some(ref format_str) = cli.trace_format {
            trace_config.format = match format_str.as_str() {
                "compact" => TraceFormat::Compact,
                "detailed" => TraceFormat::Detailed,
                "json" => TraceFormat::Json,
                _ => TraceFormat::Compact,
            };
        }

        // Apply droplet filter
        if let Some(ref droplets_str) = cli.trace_droplets {
            let droplet_ids: std::collections::HashSet<u64> = droplets_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if !droplet_ids.is_empty() {
                trace_config.droplet_filter = Some(droplet_ids);
            }
        }

        // Apply operation filter
        if let Some(ref operations_str) = cli.trace_operations {
            let operations: std::collections::HashSet<TraceOperation> = operations_str
                .split(',')
                .filter_map(|s| {
                    match s.trim().to_lowercase().as_str() {
                        "movement" => Some(TraceOperation::Movement),
                        "value_change" => Some(TraceOperation::ValueChange),
                        "stack_op" => Some(TraceOperation::StackOp),
                        "memory_op" => Some(TraceOperation::MemoryOp),
                        "arithmetic_op" => Some(TraceOperation::ArithmeticOp),
                        "io_op" => Some(TraceOperation::IoOp),
                        "subroutine_call" => Some(TraceOperation::SubroutineCall),
                        "subroutine_return" => Some(TraceOperation::SubroutineReturn),
                        "direction_change" => Some(TraceOperation::DirectionChange),
                        "collision" => Some(TraceOperation::Collision),
                        "droplet_lifecycle" => Some(TraceOperation::DropletLifecycle),
                        _ => None,
                    }
                })
                .collect();
            if !operations.is_empty() {
                trace_config.operation_filter = Some(operations);
            }
        }

        // Apply tick range filter
        if let Some(ref ticks_str) = cli.trace_ticks {
            if let Some((start_str, end_str)) = ticks_str.split_once('-') {
                if let (Ok(start), Ok(end)) = (start_str.trim().parse::<u64>(), end_str.trim().parse::<u64>()) {
                    trace_config.tick_range = Some((start, end));
                }
            }
        }

        // Apply max events limit
        if let Some(max_events) = cli.trace_max_events {
            trace_config.max_events = Some(max_events);
        }

        // Apply performance, memory, stack, and subroutine flags
        if cli.trace_performance {
            trace_config.include_performance = true;
        }
        if cli.trace_memory {
            trace_config.include_memory = true;
        }
        if cli.trace_stack {
            trace_config.include_stack = true;
        }
        if cli.trace_subroutines {
            trace_config.include_subroutines = true;
        }

        trace_config
    }

    /// Print configuration summary for verbose output
    pub fn print_config_summary(&self) {
        eprintln!("Configuration:");
        eprintln!("  Tick limit: {:?}", self.tick_limit);
        eprintln!("  Verbose: {}", self.verbose);
        eprintln!("  Trace: {}", self.trace);
        eprintln!("  Benchmark: {}", self.benchmark);
        eprintln!("  Strict: {}", self.strict);

        // Print trace configuration if trace is enabled
        if self.trace || self.trace_config.level != TraceLevel::Basic {
            eprintln!("  Trace Configuration:");
            eprintln!("    Level: {:?}", self.trace_config.level);
            eprintln!("    Format: {:?}", self.trace_config.format);
            if let Some(ref droplets) = self.trace_config.droplet_filter {
                eprintln!("    Droplet Filter: {:?}", droplets);
            }
            if let Some(ref operations) = self.trace_config.operation_filter {
                eprintln!("    Operation Filter: {:?}", operations);
            }
            if let Some((start, end)) = self.trace_config.tick_range {
                eprintln!("    Tick Range: {}-{}", start, end);
            }
            if let Some(max_events) = self.trace_config.max_events {
                eprintln!("    Max Events: {}", max_events);
            }
            eprintln!("    Include Performance: {}", self.trace_config.include_performance);
            eprintln!("    Include Memory: {}", self.trace_config.include_memory);
            eprintln!("    Include Stack: {}", self.trace_config.include_stack);
            eprintln!("    Include Subroutines: {}", self.trace_config.include_subroutines);
        }
    }
}

/// Parse a boolean environment variable with support for multiple formats
fn parse_bool_env(var_name: &str) -> Result<bool> {
    match env::var(var_name) {
        Ok(val) => {
            let normalized = val.trim().to_lowercase();
            match normalized.as_str() {
                "true" | "1" | "yes" | "on" | "y" => Ok(true),
                "false" | "0" | "no" | "off" | "n" => Ok(false),
                _ => {
                    eprintln!("Error: Invalid {} value '{}'. Accepted values: true/false, 1/0, yes/no, on/off, y/n",
                             var_name, val);
                    Err(anyhow::anyhow!("Invalid boolean environment variable value"))
                }
            }
        }
        Err(env::VarError::NotPresent) => Ok(false), // Default to false if not set
        Err(env::VarError::NotUnicode(_)) => {
            eprintln!("Error: Environment variable {} contains non-Unicode characters", var_name);
            Err(anyhow::anyhow!("Non-Unicode environment variable value"))
        }
    }
}

#[derive(Parser)]
#[command(name = "tubular")]
#[command(about = "A Tubular programming language interpreter")]
#[command(version = "0.1.0")]
#[command(after_help = "
ENVIRONMENT VARIABLES:
    TUBULAR_TICK_LIMIT <NUMBER>    Default tick limit for execution (default: 1000)
    TUBULAR_VERBOSE <BOOL>         Enable verbose output by default (default: false)
    TUBULAR_TRACE <BOOL>           Enable trace mode by default (default: false)
    TUBULAR_BENCHMARK <BOOL>       Enable benchmark mode by default (default: false)
    TUBULAR_STRICT <BOOL>          Enable strict validation by default (default: false)

TRACE OPTIONS:
    --trace-level <LEVEL>          Trace detail level: basic, detailed, verbose
    --trace-format <FORMAT>        Trace output format: compact, detailed, json
    --trace-droplets <IDS>         Filter by droplet IDs (comma-separated)
    --trace-operations <OPS>       Filter by operation types (comma-separated)
    --trace-ticks <RANGE>          Filter by tick range (e.g., 100-200)
    --trace-max-events <NUM>       Maximum number of trace events to capture
    --trace-performance            Include performance metrics in traces
    --trace-memory                 Include memory state changes in traces
    --trace-stack                  Include stack state changes in traces
    --trace-subroutines            Include subroutine call tracking in traces
    --trace-output <FILE>          Save trace output to specified file

Operation types for filtering: movement, value_change, stack_op, memory_op,
arithmetic_op, io_op, subroutine_call, subroutine_return, direction_change,
collision, droplet_lifecycle

Boolean values accept: true/false, 1/0, yes/no, on/off, y/n

All environment variables can be overridden by their corresponding command-line flags.
")]
pub struct Cli {
    /// Input file to execute
    #[arg(help = "Input file to execute. If not provided, runs in interactive mode.")]
    pub file: Option<String>,

    /// Enable verbose output
    #[arg(short, long, help = "Enable verbose output. Overrides TUBULAR_VERBOSE environment variable.")]
    pub verbose: bool,

    /// Maximum number of ticks to execute
    #[arg(short, long, help = "Maximum number of ticks to execute. Overrides TUBULAR_TICK_LIMIT environment variable.")]
    pub ticks: Option<u64>,

    /// Enable step-by-step execution tracing
    #[arg(long, help = "Enable step-by-step execution tracing. Overrides TUBULAR_TRACE environment variable.")]
    pub trace: bool,

    /// Run performance benchmarks
    #[arg(long, help = "Run performance benchmarks. Overrides TUBULAR_BENCHMARK environment variable.")]
    pub benchmark: bool,

    /// Trace level of detail (basic, detailed, verbose)
    #[arg(long = "trace-level", value_parser = ["basic", "detailed", "verbose"], help = "Trace level of detail: basic, detailed, or verbose")]
    pub trace_level: Option<String>,

    /// Trace output format (compact, detailed, json)
    #[arg(long = "trace-format", value_parser = ["compact", "detailed", "json"], help = "Trace output format: compact, detailed, or json")]
    pub trace_format: Option<String>,

    /// Filter traces by specific droplet IDs (comma-separated)
    #[arg(long = "trace-droplets", help = "Filter traces by specific droplet IDs (comma-separated)")]
    pub trace_droplets: Option<String>,

    /// Filter traces by operation types (comma-separated)
    #[arg(long = "trace-operations", help = "Filter traces by operation types (comma-separated): movement, value_change, stack_op, memory_op, arithmetic_op, io_op, subroutine_call, subroutine_return, direction_change, collision, droplet_lifecycle")]
    pub trace_operations: Option<String>,

    /// Filter traces by tick range (start-end)
    #[arg(long = "trace-ticks", help = "Filter traces by tick range (e.g., 100-200)")]
    pub trace_ticks: Option<String>,

    /// Maximum number of trace events to capture
    #[arg(long = "trace-max-events", help = "Maximum number of trace events to capture")]
    pub trace_max_events: Option<usize>,

    /// Include performance metrics in traces
    #[arg(long = "trace-performance", help = "Include performance metrics in traces")]
    pub trace_performance: bool,

    /// Include memory state changes in traces
    #[arg(long = "trace-memory", help = "Include memory state changes in traces")]
    pub trace_memory: bool,

    /// Include stack state changes in traces
    #[arg(long = "trace-stack", help = "Include stack state changes in traces")]
    pub trace_stack: bool,

    /// Include subroutine call tracking in traces
    #[arg(long = "trace-subroutines", help = "Include subroutine call tracking in traces")]
    pub trace_subroutines: bool,

    /// Save trace output to file
    #[arg(long = "trace-output", help = "Save trace output to specified file")]
    pub trace_output: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate program syntax without execution
    Validate {
        /// Input file to validate (optional, reads from stdin if not provided)
        #[arg(help = "Input file to validate. If not provided, reads from stdin.")]
        file: Option<String>,
        /// Enable strict validation mode
        #[arg(long, help = "Enable strict validation mode. Overrides TUBULAR_STRICT environment variable.")]
        strict: bool,
    },
    /// Execute program with interactive input support
    Run {
        /// Input file to execute
        #[arg(help = "Input file to execute.")]
        file: String,
        /// Enable interactive input mode
        #[arg(short, long, help = "Enable interactive input mode for programs that read from stdin.")]
        interactive: bool,
        /// Provide input as command line argument
        #[arg(long, help = "Provide input as command line argument instead of stdin.")]
        input: Option<String>,
    },
    /// Run comprehensive performance benchmarks
    Benchmark {
        /// Input file to benchmark
        #[arg(help = "Input file to benchmark.")]
        file: String,
        /// Number of benchmark iterations
        #[arg(long, default_value = "10", help = "Number of benchmark iterations.")]
        iterations: usize,
        /// Output format (json, csv, table)
        #[arg(long, default_value = "table", help = "Benchmark output format: json, csv, or table.")]
        output: String,
        /// Number of warmup iterations
        #[arg(long, default_value = "3", help = "Number of warmup iterations before benchmarking.")]
        warmup: usize,
        /// Time limit for each benchmark iteration (seconds)
        #[arg(long, default_value = "60", help = "Time limit for each benchmark iteration in seconds.")]
        time_limit: u64,
        /// Output file for benchmark results
        #[arg(long, help = "Save benchmark results to specified file.")]
        save: Option<String>,
        /// Enable detailed per-iteration metrics
        #[arg(long, help = "Show detailed metrics for each iteration.")]
        verbose: bool,
        /// Compare multiple programs
        #[arg(long, help = "Compare with additional program files.")]
        compare: Vec<String>,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        // Load environment configuration
        let env_config = match EnvConfig::load() {
            Ok(config) => {
                if self.verbose || config.verbose {
                    eprintln!("Environment configuration loaded successfully");
                }
                config
            }
            Err(e) => {
                eprintln!("Warning: Failed to load environment configuration: {}", e);
                eprintln!("Using default configuration values");
                EnvConfig::default()
            }
        };

        // Apply CLI overrides to environment config
        let config = env_config.apply_cli_overrides(&self);

        // Print configuration summary if verbose
        if config.verbose {
            config.print_config_summary();
        }

        match self.command {
            Some(Commands::Validate { ref file, strict }) => {
                let final_strict = strict || config.strict;
                self.validate_program(file.as_deref(), final_strict, &config)
            }
            Some(Commands::Run { ref file, interactive, ref input }) => {
                self.execute_program_interactive(file, interactive, input.clone(), &config)
            }
            Some(Commands::Benchmark { ref file, iterations, ref output, warmup, time_limit, ref save, verbose, ref compare }) => {
                self.run_benchmark(file, iterations, &output, warmup, time_limit, save.as_deref(), verbose, &compare, &config)
            }
            None => {
                if let Some(ref file) = self.file {
                    self.execute_program(file, &config)
                } else {
                    println!("No file specified. Use --help for usage information.");
                    Ok(())
                }
            }
        }
    }

    /// Validate a program from file or stdin
    fn validate_program(&self, file_path: Option<&str>, strict: bool, config: &EnvConfig) -> Result<()> {
        // Read input content
        let (content, source_name) = match file_path {
            Some(path) => {
                let content = fs::read_to_string(path)
                    .map_err(|e| InterpreterError::System(
                        crate::types::error::SystemError::IoError(e.to_string())
                    ))?;
                (content, path.to_string())
            }
            None => {
                // Read from stdin
                use std::io::Read;
                let mut content = String::new();
                let mut stdin = std::io::stdin();
                stdin.read_to_string(&mut content)
                    .map_err(|e| InterpreterError::System(
                        crate::types::error::SystemError::IoError(e.to_string())
                    ))?;
                (content, "<stdin>".to_string())
            }
        };

        // If content is empty and reading from stdin, it's likely a usage error
        if content.trim().is_empty() {
            eprintln!("Error: No input provided");
            eprintln!("Usage: cargo run -- validate <file>");
            eprintln!("   or: cat <file> | cargo run -- validate");
            std::process::exit(1);
        }

        // Parse the program
        let parser = GridParser::new();
        let grid = match parser.parse_string(&content) {
            Ok(grid) => grid,
            Err(e) => {
                self.print_validation_error(&e, &content, &source_name);
                std::process::exit(1);
            }
        };

        // Validate the program
        let validator = if strict {
            ProgramValidator::strict()
        } else {
            ProgramValidator::new()
        };

        match validator.validate(&grid) {
            Ok(()) => {
                self.print_validation_success(&grid, &source_name);
                std::process::exit(0);
            }
            Err(e) => {
                self.print_validation_error(&e, &content, &source_name);
                std::process::exit(1);
            }
        }
    }

    /// Print successful validation result
    fn print_validation_success(&self, grid: &crate::interpreter::grid::ProgramGrid, source_name: &str) {
        println!("[OK] Program validation passed: {}", source_name);
        println!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
        println!("  Program cells: {}", grid.size());

        if let Some(start) = grid.start {
            println!("  Start position: ({}, {})", start.x, start.y);
        }
    }

    /// Print validation error with context
    fn print_validation_error(&self, error: &InterpreterError, content: &str, source_name: &str) {
        eprintln!("[ERROR] Validation failed: {}", source_name);

        match error {
            InterpreterError::Initialization(init_err) => {
                match init_err {
                    crate::types::error::InitError::NoStartSymbol => {
                        eprintln!("  No start symbol (@) found in program");
                    }
                    crate::types::error::InitError::MultipleStartSymbols => {
                        eprintln!("  Multiple start symbols (@) found in program");
                    }
                    crate::types::error::InitError::InvalidCharacter(ch, coord) => {
                        self.print_character_error(*ch, *coord, content, source_name);
                    }
                    crate::types::error::InitError::GridSizeExceeded(width, height) => {
                        eprintln!("  Grid size {}x{} exceeds maximum supported size of 1000x1000", width, height);
                    }
                }
            }
            InterpreterError::System(sys_err) => {
                eprintln!("  System error: {}", sys_err);
            }
            InterpreterError::Execution(exec_err) => {
                eprintln!("  Execution error: {}", exec_err);
            }
            InterpreterError::Enhanced { info, source } => {
                eprintln!("  {}: {}", info.error_type, info.message);

                // Print context if available
                if let Some(context) = &info.context {
                    let line_num = context.position.line + 1;
                    let col_num = context.position.column + 1;
                    eprintln!("  Location: line {}, column {}", line_num, col_num);

                    if !context.source_line.is_empty() {
                        eprintln!("  Line {}: {}", line_num, context.source_line);

                        // Show pointer to the error position
                        let start = context.error_span.0;
                        let end = context.error_span.1;
                        let pointer = " ".repeat(start) + &"^".repeat(end.saturating_sub(start));
                        eprintln!("         {}", pointer);
                    }

                    // Show surrounding lines
                    for (line_num, line) in &context.surrounding_lines {
                        eprintln!("  Line {}: {}", line_num + 1, line);
                    }
                }

                // Print suggestions
                if !info.suggestions.is_empty() {
                    eprintln!("  Suggestions:");
                    for suggestion in &info.suggestions {
                        eprintln!("    • {}", suggestion);
                    }
                }

                // Print help text
                if let Some(help) = &info.help_text {
                    eprintln!("  Help: {}", help);
                }

                // Print source error if available
                if let Some(source_error) = source {
                    eprintln!("  Caused by: {}", source_error);
                }
            }
        }
    }

    /// Print character error with line/column context
    fn print_character_error(&self, ch: char, coord: crate::types::coordinate::Coordinate, content: &str, source_name: &str) {
        let line_num = coord.y + 1;
        let col_num = coord.x + 1;

        eprintln!("  Invalid character '{}' at line {}, column {}", ch, line_num, col_num);

        // Show the line with context
        let lines: Vec<&str> = content.lines().collect();
        if let Some(line) = lines.get(coord.y as usize) {
            eprintln!("  Line {}: {}", line_num, line);

            // Show pointer to the exact position
            let pointer = " ".repeat(coord.x as usize) + "^";
            eprintln!("         {}", pointer);
        }

        eprintln!("  Source: {}", source_name);
    }

    /// Execute a program file
    fn execute_program(&self, file_path: &str, config: &EnvConfig) -> Result<()> {
        eprintln!("DEBUG: Starting execute_program for {}", file_path);

        // Read and parse the program
        eprintln!("DEBUG: Reading file...");
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::System(
                crate::types::error::SystemError::IoError(e.to_string())
            ))?;
        eprintln!("DEBUG: File read successfully, {} bytes", content.len());

        if config.verbose {
            eprintln!("Parsing program: {}", file_path);
        }

        let parser = GridParser::new();
        let grid = parser.parse_string(&content)?;

        if config.verbose {
            eprintln!("Program parsed successfully:");
            eprintln!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
            eprintln!("  Program cells: {}", grid.size());
            eprintln!("  Start position: {:?}", grid.start);
        }

        // Create and run interpreter
        let mut interpreter = TubularInterpreter::new(grid)?;

        // Determine final tick limit: CLI overrides environment
        let final_ticks = self.ticks.or(config.tick_limit);

        interpreter = interpreter.with_options(config.verbose, config.trace, final_ticks);

        if config.verbose {
            eprintln!("Starting execution...");
        }

        let result = interpreter.run()?;

        // Handle trace output if trace is enabled
        if config.trace || config.trace_config.level != TraceLevel::Basic {
            self.handle_trace_output(&config.trace_config, &self.trace_output)?;
        }

        // Print execution results
        match result.status {
            crate::interpreter::execution::ExecutionStatus::Completed => {
                if config.verbose {
                    eprintln!("[OK] Program completed successfully");
                    eprintln!("  Total ticks: {}", result.total_ticks);
                    eprintln!("  Max droplets: {}", result.max_droplets);
                    eprintln!("  Max stack depth: {}", result.max_stack_depth);
                }
            }
            crate::interpreter::execution::ExecutionStatus::TickTimeout(ticks) => {
                eprintln!("[TIMEOUT] Program execution timed out after {} ticks", ticks);
            }
            crate::interpreter::execution::ExecutionStatus::WallClockTimeout(time_ms) => {
                eprintln!("[TIMEOUT] Program execution timed out after {}ms", time_ms);
            }
            crate::interpreter::execution::ExecutionStatus::Error(err) => {
                eprintln!("[ERROR] Program execution failed: {}", err);
                return Err(err.into());
            }
            _ => {}
        }

        Ok(())
    }

    /// Execute a program file with interactive input support
    fn execute_program_interactive(&self, file_path: &str, interactive: bool, input: Option<String>, config: &EnvConfig) -> Result<()> {
        // Read and parse the program
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::System(
                crate::types::error::SystemError::IoError(e.to_string())
            ))?;

        if config.verbose {
            eprintln!("Parsing program: {}", file_path);
        }

        let parser = GridParser::new();
        let grid = parser.parse_string(&content)?;

        if config.verbose {
            eprintln!("Program parsed successfully:");
            eprintln!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
            eprintln!("  Program cells: {}", grid.size());
            eprintln!("  Start position: {:?}", grid.start);
            if interactive {
                eprintln!("  Interactive mode: enabled");
            }
            if let Some(ref input_str) = input {
                eprintln!("  Input provided: {}", input_str);
            }
        }

        // Handle interactive input setup
        if interactive {
            eprintln!("[INFO] Interactive mode enabled - program can read from stdin");
            if input.is_some() {
                eprintln!("[INFO] Using provided command line input instead of stdin");
            }
        }

        // Create and run interpreter
        // Determine final tick limit: CLI overrides environment
        let final_ticks = self.ticks.or(config.tick_limit);

        let mut interpreter = TubularInterpreter::new(grid)?
            .with_options(config.verbose, config.trace, final_ticks);

        if config.verbose {
            eprintln!("Starting execution...");
        }

        let result = interpreter.run()?;

        // Handle trace output if trace is enabled
        if config.trace || config.trace_config.level != TraceLevel::Basic {
            self.handle_trace_output(&config.trace_config, &self.trace_output)?;
        }

        // Print execution results
        match result.status {
            crate::interpreter::execution::ExecutionStatus::Completed => {
                if config.verbose {
                    eprintln!("[OK] Program completed successfully");
                    eprintln!("  Total ticks: {}", result.total_ticks);
                    eprintln!("  Max droplets: {}", result.max_droplets);
                    eprintln!("  Max stack depth: {}", result.max_stack_depth);
                }

                if interactive {
                    eprintln!("[INFO] Interactive execution completed");
                }
            }
            crate::interpreter::execution::ExecutionStatus::TickTimeout(ticks) => {
                eprintln!("[TIMEOUT] Program execution timed out after {} ticks", ticks);
            }
            crate::interpreter::execution::ExecutionStatus::WallClockTimeout(time_ms) => {
                eprintln!("[TIMEOUT] Program execution timed out after {}ms", time_ms);
            }
            crate::interpreter::execution::ExecutionStatus::Error(err) => {
                eprintln!("[ERROR] Program execution failed: {}", err);
                return Err(err.into());
            }
            _ => {}
        }

        Ok(())
    }

    /// Run comprehensive benchmark for a Tubular program
    fn run_benchmark(
        &self,
        file_path: &str,
        iterations: usize,
        output_format: &str,
        warmup_iterations: usize,
        time_limit: u64,
        save_file: Option<&str>,
        verbose_benchmark: bool,
        compare_files: &[String],
        config: &EnvConfig,
    ) -> Result<()> {
        if config.verbose {
            eprintln!("Starting benchmark for: {}", file_path);
            eprintln!("Iterations: {}, Warmup: {}, Time limit: {}s", iterations, warmup_iterations, time_limit);
        }

        // Collect all files to benchmark
        let mut files_to_benchmark = vec![file_path.to_string()];
        files_to_benchmark.extend_from_slice(compare_files);

        let mut all_results = Vec::new();

        for file in &files_to_benchmark {
            if config.verbose {
                eprintln!("\nBenchmarking: {}", file);
            }

            let result = self.benchmark_single_file(
                file,
                iterations,
                warmup_iterations,
                time_limit,
                verbose_benchmark,
                config,
            )?;

            all_results.push((file.clone(), result));
        }

        // Format and output results
        let output = if files_to_benchmark.len() == 1 {
            // Single program benchmark
            let (file, result) = &all_results[0];
            self.format_benchmark_results(file, result, output_format, verbose_benchmark)?
        } else {
            // Multiple program comparison
            self.format_comparison_results(&all_results, output_format, verbose_benchmark)?
        };

        println!("{}", output);

        // Save results to file if requested
        if let Some(save_path) = save_file {
            fs::write(save_path, output)?;
            eprintln!("Results saved to: {}", save_path);
        }

        Ok(())
    }

    /// Benchmark a single program file
    fn benchmark_single_file(
        &self,
        file_path: &str,
        iterations: usize,
        warmup_iterations: usize,
        time_limit_seconds: u64,
        verbose_benchmark: bool,
        config: &EnvConfig,
    ) -> Result<BenchmarkResult> {
        // Read and parse the program once
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::System(
                crate::types::error::SystemError::IoError(e.to_string())
            ))?;

        let parser = GridParser::new();
        let grid = parser.parse_string(&content)?;

        if config.verbose {
            eprintln!("Program parsed successfully:");
            eprintln!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
            eprintln!("  Program cells: {}", grid.size());
        }

        // Warmup iterations
        if warmup_iterations > 0 {
            if config.verbose {
                eprintln!("Running {} warmup iterations...", warmup_iterations);
            }
            for i in 0..warmup_iterations {
                if verbose_benchmark {
                    eprint!("Warmup {}/{}\r", i + 1, warmup_iterations);
                }
                let mut interpreter = TubularInterpreter::new(grid.clone())?
                    .with_options(false, false, Some(time_limit_seconds));
                let _ = interpreter.run();
            }
            if verbose_benchmark {
                eprintln!("\nWarmup completed.");
            }
        }

        // Benchmark iterations
        let mut execution_times = Vec::new();
        let mut tick_counts = Vec::new();
        let mut peak_droplet_counts = Vec::new();
        let mut memory_usage = Vec::new();

        if config.verbose {
            eprintln!("Running {} benchmark iterations...", iterations);
        }

        for i in 0..iterations {
            if verbose_benchmark {
                eprint!("Iteration {}/{}\r", i + 1, iterations);
            }

            let start_time = Instant::now();

            // Create fresh interpreter for each iteration
            let mut interpreter = TubularInterpreter::new(grid.clone())?
                .with_options(false, false, Some(time_limit_seconds));

            let result = interpreter.run()?;

            let elapsed = start_time.elapsed();

            execution_times.push(elapsed);
            tick_counts.push(result.total_ticks);
            peak_droplet_counts.push(result.max_droplets);

            // Estimate memory usage (rough approximation)
            let memory_mb = self.estimate_memory_usage(&result, &grid);
            memory_usage.push(memory_mb);

            if verbose_benchmark && i == iterations - 1 {
                eprintln!("\n");
            }
        }

        // Calculate statistics
        let avg_time = execution_times.iter().sum::<Duration>() / iterations as u32;
        let min_time = execution_times.iter().min().unwrap();
        let max_time = execution_times.iter().max().unwrap();

        let avg_ticks = tick_counts.iter().sum::<u64>() / iterations as u64;
        let avg_droplets = peak_droplet_counts.iter().sum::<usize>() / iterations;
        let avg_memory = memory_usage.iter().sum::<f64>() / iterations as f64;

        let total_instructions = tick_counts.iter().sum::<u64>();
        let total_time_ms = avg_time.as_millis() as u64;
        let instructions_per_sec = if total_time_ms > 0 {
            (total_instructions as f64 * 1000.0) / total_time_ms as f64
        } else {
            0.0
        };

        Ok(BenchmarkResult {
            program_file: file_path.to_string(),
            iterations,
            avg_execution_time: avg_time,
            min_execution_time: *min_time,
            max_execution_time: *max_time,
            avg_total_ticks: avg_ticks,
            avg_peak_droplets: avg_droplets,
            avg_memory_usage_mb: avg_memory,
            instructions_per_second: instructions_per_sec,
            execution_times,
            tick_counts,
            peak_droplet_counts,
            memory_usage,
        })
    }

    /// Estimate memory usage for a program execution
    fn estimate_memory_usage(&self, result: &crate::interpreter::execution::ExecutionResult, grid: &crate::interpreter::grid::ProgramGrid) -> f64 {
        // Rough estimation of memory usage in MB
        let grid_memory = grid.size() * std::mem::size_of::<crate::interpreter::grid::ProgramCell>();
        let droplet_memory = result.max_droplets * std::mem::size_of::<crate::interpreter::droplet::Droplet>();
        let stack_memory = result.max_stack_depth * std::mem::size_of::<crate::types::bigint::TubularBigInt>();

        (grid_memory + droplet_memory + stack_memory) as f64 / (1024.0 * 1024.0)
    }

    /// Format benchmark results according to specified format
    fn format_benchmark_results(
        &self,
        file_path: &str,
        result: &BenchmarkResult,
        output_format: &str,
        verbose: bool,
    ) -> Result<String> {
        match output_format.to_lowercase().as_str() {
            "json" => Ok(self.format_benchmark_json(file_path, result)),
            "csv" => Ok(self.format_benchmark_csv(file_path, result)),
            "table" | _ => Ok(self.format_benchmark_table(file_path, result, verbose)),
        }
    }

    /// Format benchmark results as table
    fn format_benchmark_table(&self, file_path: &str, result: &BenchmarkResult, verbose: bool) -> String {
        let mut output = String::new();

        // Use existing OutputFormatter for basic table
        output.push_str(&OutputFormatter::format_benchmark_table(
            file_path,
            result.avg_execution_time.as_millis() as u64,
            result.avg_total_ticks,
            result.avg_peak_droplets,
            result.avg_memory_usage_mb,
            result.instructions_per_second,
        ));

        // Add additional statistical information
        output.push_str("\nStatistical Details:\n");
        output.push_str("===================\n");
        output.push_str(&format!("Iterations: {}\n", result.iterations));
        output.push_str(&format!("Min Execution Time: {:.3} ms\n", result.min_execution_time.as_millis()));
        output.push_str(&format!("Max Execution Time: {:.3} ms\n", result.max_execution_time.as_millis()));
        output.push_str(&format!("Time Std Dev: {:.3} ms\n", self.calculate_std_dev(&result.execution_times)));

        if verbose {
            output.push_str("\nPer-Iteration Details:\n");
            output.push_str("-----------------------\n");
            for (i, time) in result.execution_times.iter().enumerate() {
                output.push_str(&format!(
                    "Iter {}: {:.3} ms, {} ticks, {} droplets\n",
                    i + 1,
                    time.as_millis(),
                    result.tick_counts[i],
                    result.peak_droplet_counts[i]
                ));
            }
        }

        output
    }

    /// Format benchmark results as JSON
    fn format_benchmark_json(&self, file_path: &str, result: &BenchmarkResult) -> String {
        format!(
            r#"{{
  "program": "{}",
  "timestamp": "{}",
  "iterations": {},
  "results": {{
    "execution_time": {{
      "average_ms": {},
      "min_ms": {},
      "max_ms": {},
      "std_dev_ms": {}
    }},
    "total_ticks": {{
      "average": {},
      "values": {:?}
    }},
    "peak_droplets": {{
      "average": {},
      "values": {:?}
    }},
    "memory_usage_mb": {{
      "average": {:.3},
      "values": {:?}
    }},
    "instructions_per_second": {:.2}
  }}
}}"#,
            file_path,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            result.iterations,
            result.avg_execution_time.as_millis(),
            result.min_execution_time.as_millis(),
            result.max_execution_time.as_millis(),
            self.calculate_std_dev(&result.execution_times),
            result.avg_total_ticks,
            result.tick_counts,
            result.avg_peak_droplets,
            result.peak_droplet_counts,
            result.avg_memory_usage_mb,
            result.memory_usage,
            result.instructions_per_second
        )
    }

    /// Format benchmark results as CSV
    fn format_benchmark_csv(&self, file_path: &str, result: &BenchmarkResult) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("program,iteration,execution_time_ms,ticks,peak_droplets,memory_usage_mb\n");

        // CSV data
        for i in 0..result.iterations {
            output.push_str(&format!(
                "{},{},{:.3},{},{},{:.3}\n",
                file_path,
                i + 1,
                result.execution_times[i].as_millis(),
                result.tick_counts[i],
                result.peak_droplet_counts[i],
                result.memory_usage[i]
            ));
        }

        output
    }

    /// Format comparison results for multiple programs
    fn format_comparison_results(
        &self,
        all_results: &[(String, BenchmarkResult)],
        output_format: &str,
        _verbose: bool,
    ) -> Result<String> {
        match output_format.to_lowercase().as_str() {
            "json" => self.format_comparison_json(all_results),
            "csv" => self.format_comparison_csv(all_results),
            "table" | _ => self.format_comparison_table(all_results),
        }
    }

    /// Format comparison results as table
    fn format_comparison_table(&self, all_results: &[(String, BenchmarkResult)]) -> Result<String> {
        let mut output = String::new();

        output.push_str("Benchmark Comparison Results\n");
        output.push_str("===========================\n\n");

        // Header
        output.push_str(&format!(
            "{:<25} | {:>12} | {:>12} | {:>12} | {:>12} | {:>15}\n",
            "Program", "Avg Time (ms)", "Avg Ticks", "Peak Droplets", "Memory (MB)", "Instructions/sec"
        ));
        output.push_str(&"-".repeat(85));
        output.push_str("\n");

        // Results
        for (file, result) in all_results {
            output.push_str(&format!(
                "{:<25} | {:>12.3} | {:>12} | {:>12} | {:>12.3} | {:>15.0}\n",
                Path::new(file).file_name().unwrap_or_default().to_string_lossy(),
                result.avg_execution_time.as_millis(),
                result.avg_total_ticks,
                result.avg_peak_droplets,
                result.avg_memory_usage_mb,
                result.instructions_per_second
            ));
        }

        Ok(output)
    }

    /// Format comparison results as JSON
    fn format_comparison_json(&self, all_results: &[(String, BenchmarkResult)]) -> Result<String> {
        let mut output = String::new();

        output.push_str("{\n");
        output.push_str(&format!("  \"timestamp\": \"{}\",\n",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()));
        output.push_str("  \"programs\": [\n");

        for (i, (file, result)) in all_results.iter().enumerate() {
            output.push_str(&format!(
                r#"    {{
      "program": "{}",
      "results": {{
        "execution_time_ms": {:.3},
        "total_ticks": {},
        "peak_droplets": {},
        "memory_usage_mb": {:.3},
        "instructions_per_second": {:.2}
      }}
    }}"#,
                file,
                result.avg_execution_time.as_millis(),
                result.avg_total_ticks,
                result.avg_peak_droplets,
                result.avg_memory_usage_mb,
                result.instructions_per_second
            ));

            if i < all_results.len() - 1 {
                output.push_str(",");
            }
            output.push_str("\n");
        }

        output.push_str("  ]\n");
        output.push_str("}\n");

        Ok(output)
    }

    /// Format comparison results as CSV
    fn format_comparison_csv(&self, all_results: &[(String, BenchmarkResult)]) -> Result<String> {
        let mut output = String::new();

        // CSV header
        output.push_str("program,avg_execution_time_ms,avg_ticks,avg_peak_droplets,avg_memory_mb,instructions_per_second\n");

        // CSV data
        for (file, result) in all_results {
            output.push_str(&format!(
                "{},{:.3},{},{},{:.3},{:.2}\n",
                file,
                result.avg_execution_time.as_millis(),
                result.avg_total_ticks,
                result.avg_peak_droplets,
                result.avg_memory_usage_mb,
                result.instructions_per_second
            ));
        }

        Ok(output)
    }

    /// Calculate standard deviation for a set of durations
    fn calculate_std_dev(&self, durations: &[Duration]) -> f64 {
        if durations.len() <= 1 {
            return 0.0;
        }

        let mean = durations.iter().sum::<Duration>() / durations.len() as u32;
        let mean_ms = mean.as_millis() as f64;

        let variance: f64 = durations
            .iter()
            .map(|d| {
                let diff = d.as_millis() as f64 - mean_ms;
                diff * diff
            })
            .sum::<f64>() / (durations.len() - 1) as f64;

        variance.sqrt()
    }

    /// Handle trace output after execution
    fn handle_trace_output(&self, trace_config: &TraceConfig, trace_output_file: &Option<String>) -> Result<()> {
        // For now, this is a placeholder - the actual trace events will be generated
        // by the execution engine in a future implementation
        let formatter = OutputFormatter;

        // Create a placeholder trace event to show the feature works
        let placeholder_events = vec![
            OutputFormatter::create_movement_trace_event(
                0,
                0,
                crate::types::coordinate::Coordinate::new(0, 0),
                crate::types::coordinate::Coordinate::new(0, 1),
                crate::types::direction::Direction::Down,
                &crate::types::bigint::TubularBigInt::zero(),
                Some('@'),
            )
        ];

        let trace_output = formatter.format_trace_events(&placeholder_events, trace_config);

        // Output trace results
        if let Some(file_path) = trace_output_file {
            // Save to file
            fs::write(file_path, trace_output)
                .map_err(|e| anyhow::anyhow!("Failed to write trace output to '{}': {}", file_path, e))?;
            eprintln!("Trace output saved to: {}", file_path);
        } else {
            // Print to stdout
            println!("{}", trace_output);
        }

        Ok(())
    }
}

/// Benchmark result data structure
#[derive(Debug, Clone)]
struct BenchmarkResult {
    program_file: String,
    iterations: usize,
    avg_execution_time: Duration,
    min_execution_time: Duration,
    max_execution_time: Duration,
    avg_total_ticks: u64,
    avg_peak_droplets: usize,
    avg_memory_usage_mb: f64,
    instructions_per_second: f64,
    execution_times: Vec<Duration>,
    tick_counts: Vec<u64>,
    peak_droplet_counts: Vec<usize>,
    memory_usage: Vec<f64>,
}