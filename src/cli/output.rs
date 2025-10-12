use crate::interpreter::execution::{ExecutionResult, TickResult};
use crate::interpreter::droplet::Droplet;
use crate::interpreter::stack::DataStack;
use crate::types::coordinate::Coordinate;
use std::io::{self, Write};

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

    /// Format execution summary
    pub fn format_execution_summary(result: &ExecutionResult) -> String {
        let mut output = String::new();

        output.push_str("Execution Summary:\n");
        output.push_str(&format!("================\n"));
        output.push_str(&format!("Total Ticks: {}\n", result.total_ticks));
        output.push_str(&format!("Peak Droplets: {}\n", result.max_droplets));
        output.push_str(&format!("Peak Stack Depth: {}\n", result.max_stack_depth));
        output.push_str(&format!("Final Status: {:?}\n", result.status));

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
        use crate::types::error::InterpreterError;

        match error {
            InterpreterError::Initialization(_) => "E003",
            InterpreterError::Execution(exec_error) => match exec_error {
                crate::types::error::ExecError::StackUnderflow => "E004",
                crate::types::error::ExecError::DivisionByZero => "E005",
                crate::types::error::ExecError::ModuloByZero => "E006",
                crate::types::error::ExecError::InvalidMemoryAccess(_) => "E007",
                crate::types::error::ExecError::SubroutineUnderflow => "E008",
                crate::types::error::ExecError::DropletCollision(_) => "E009",
                crate::types::error::ExecError::ExecutionTimeout(_) => "E010",
                crate::types::error::ExecError::InternalError(_) => "E011",
                crate::types::error::ExecError::InvalidOperation(_) => "E015",
            },
            InterpreterError::System(sys_error) => match sys_error {
                crate::types::error::SystemError::OutOfMemory => "E012",
                crate::types::error::SystemError::IoError(_) => "E013",
                crate::types::error::SystemError::InternalError(_) => "E014",
            },
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