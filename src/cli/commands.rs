use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;

// Import necessary modules
use crate::parser::grid_parser::GridParser;
use crate::parser::validator::ProgramValidator;
use crate::interpreter::execution::TubularInterpreter;
use crate::types::error::InterpreterError;

#[derive(Parser)]
#[command(name = "tubular")]
#[command(about = "A Tubular programming language interpreter")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Input file to execute
    pub file: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Maximum number of ticks to execute
    #[arg(short, long)]
    pub ticks: Option<u64>,

    /// Enable step-by-step execution tracing
    #[arg(long)]
    pub trace: bool,

    /// Run performance benchmarks
    #[arg(long)]
    pub benchmark: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate program syntax without execution
    Validate {
        /// Input file to validate
        file: String,
        /// Enable strict validation mode
        #[arg(long)]
        strict: bool,
    },
    /// Execute program with interactive input support
    Run {
        /// Input file to execute
        file: String,
        /// Enable interactive input mode
        #[arg(short, long)]
        interactive: bool,
        /// Provide input as command line argument
        #[arg(long)]
        input: Option<String>,
    },
    /// Run comprehensive performance benchmarks
    Benchmark {
        /// Input file to benchmark
        file: String,
        /// Number of benchmark iterations
        #[arg(long, default_value = "10")]
        iterations: usize,
        /// Output format (json, csv, table)
        #[arg(long, default_value = "table")]
        output: String,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Some(Commands::Validate { ref file, strict }) => {
                self.validate_program(file, strict)
            }
            Some(Commands::Run { ref file, interactive: _, input: _ }) => {
                self.execute_program(file)
            }
            Some(Commands::Benchmark { ref file, iterations: _, output: _ }) => {
                self.execute_program(file) // For now, just run the program
            }
            None => {
                if let Some(ref file) = self.file {
                    self.execute_program(file)
                } else {
                    println!("No file specified. Use --help for usage information.");
                    Ok(())
                }
            }
        }
    }

    /// Validate a program file
    fn validate_program(&self, file_path: &str, strict: bool) -> Result<()> {
        // Read and parse the program
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::System(
                crate::types::error::SystemError::IoError(e.to_string())
            ))?;

        let parser = GridParser::new();
        let grid = parser.parse_string(&content)?;

        // Validate the program
        let validator = if strict {
            ProgramValidator::strict()
        } else {
            ProgramValidator::new()
        };

        validator.validate(&grid)?;

        println!("✓ Program validation passed: {}", file_path);
        println!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
        println!("  Program cells: {}", grid.size());

        Ok(())
    }

    /// Execute a program file
    fn execute_program(&self, file_path: &str) -> Result<()> {
        // Read and parse the program
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::System(
                crate::types::error::SystemError::IoError(e.to_string())
            ))?;

        if self.verbose {
            eprintln!("Parsing program: {}", file_path);
        }

        let parser = GridParser::new();
        let grid = parser.parse_string(&content)?;

        if self.verbose {
            eprintln!("Program parsed successfully:");
            eprintln!("  Grid size: {}x{}", grid.dimensions().0, grid.dimensions().1);
            eprintln!("  Program cells: {}", grid.size());
            eprintln!("  Start position: {:?}", grid.start);
        }

        // Create and run interpreter
        let mut interpreter = TubularInterpreter::new(grid)?
            .with_options(self.verbose, self.trace, self.ticks);

        if self.verbose {
            eprintln!("Starting execution...");
        }

        let result = interpreter.run()?;

        // Print execution results
        match result.status {
            crate::interpreter::execution::ExecutionStatus::Completed => {
                if self.verbose {
                    eprintln!("✓ Program completed successfully");
                    eprintln!("  Total ticks: {}", result.total_ticks);
                    eprintln!("  Max droplets: {}", result.max_droplets);
                    eprintln!("  Max stack depth: {}", result.max_stack_depth);
                }
            }
            crate::interpreter::execution::ExecutionStatus::Timeout(ticks) => {
                eprintln!("⚠ Program execution timed out after {} ticks", ticks);
            }
            crate::interpreter::execution::ExecutionStatus::Error(err) => {
                eprintln!("✗ Program execution failed: {}", err);
                return Err(err.into());
            }
            _ => {}
        }

        Ok(())
    }
}