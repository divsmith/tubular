use crate::interpreter::droplet::Droplet;
use crate::types::error::{Result, SystemError};
use crate::types::bigint::TubularBigInt;
use std::io::{self, BufRead, Read};
use std::sync::{Arc, Mutex};

/// Thread-safe input buffer for managing program input
#[derive(Debug, Clone)]
pub struct InputBuffer {
    buffer: Arc<Mutex<Vec<String>>>,
    current_line: Arc<Mutex<Option<String>>>,
    position: Arc<Mutex<usize>>,
}

impl InputBuffer {
    /// Create a new input buffer
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            current_line: Arc::new(Mutex::new(None)),
            position: Arc::new(Mutex::new(0)),
        }
    }

    /// Create a new input buffer with predefined input
    pub fn with_input(input: String) -> Self {
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        Self {
            buffer: Arc::new(Mutex::new(lines)),
            current_line: Arc::new(Mutex::new(None)),
            position: Arc::new(Mutex::new(0)),
        }
    }

    /// Read a single character from input
    pub fn read_char(&self) -> Result<char> {
        let mut current_line = self.current_line.lock().unwrap();

        // If we don't have a current line, get one from the buffer
        if current_line.is_none() {
            let mut buffer = self.buffer.lock().unwrap();
            let mut position = self.position.lock().unwrap();

            if *position < buffer.len() {
                *current_line = Some(buffer[*position].clone());
                *position += 1;
            } else {
                // No more buffered input, read from stdin
                drop(current_line);
                drop(buffer);
                drop(position);
                return self.read_char_from_stdin();
            }
        }

        // Extract character from current line
        if let Some(line) = current_line.as_ref() {
            let mut position = self.position.lock().unwrap();

            if *position < line.len() {
                let ch = line.chars().nth(*position).unwrap();
                *position += 1;

                // If we've consumed the entire line, clear it
                if *position >= line.len() {
                    *current_line = None;
                    *position = 0; // Reset position for next line
                }

                return Ok(ch);
            } else {
                // End of line, move to next line
                *current_line = None;
                *position = 0;
                return Ok('\n');
            }
        }

        // Fallback: try stdin
        drop(current_line);
        self.read_char_from_stdin()
    }

    /// Read a line of text from input
    pub fn read_line(&self) -> Result<String> {
        let mut buffer = self.buffer.lock().unwrap();
        let mut position = self.position.lock().unwrap();

        if *position < buffer.len() {
            let line = buffer[*position].clone();
            *position += 1;
            Ok(line)
        } else {
            // No more buffered input, read from stdin
            drop(buffer);
            drop(position);
            self.read_line_from_stdin()
        }
    }

    /// Read a single character from stdin
    fn read_char_from_stdin(&self) -> Result<char> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(ch) = input.chars().next() {
                    Ok(ch)
                } else {
                    Ok('\n') // End of input
                }
            }
            Err(e) => Err(SystemError::IoError(format!("Failed to read character from stdin: {}", e)).into()),
        }
    }

    /// Read a line from stdin
    fn read_line_from_stdin(&self) -> Result<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(e) => Err(SystemError::IoError(format!("Failed to read line from stdin: {}", e)).into()),
        }
    }

    /// Check if input validation should be strict
    pub fn validation_mode(&self) -> ValidationMode {
        ValidationMode::Lenient
    }
}

/// Input validation modes
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationMode {
    /// Lenient: invalid input becomes 0 or default value
    Lenient,
    /// Strict: invalid input causes an error
    Strict,
    /// Permissive: accepts any input and tries to parse intelligently
    Permissive,
}

/// I/O operations for Tubular programs
pub struct IoOperations;

impl IoOperations {
    /// Process character output (,) - output droplet value as character
    pub fn process_character_output(droplet: &Droplet) -> Result<String> {
        if let Some(ch) = droplet.value.to_char() {
            Ok(ch.to_string())
        } else {
            // If value cannot be converted to character, output nothing
            Ok(String::new())
        }
    }

    /// Process numeric output (n) - output droplet value as decimal
    pub fn process_numeric_output(droplet: &Droplet) -> Result<String> {
        Ok(droplet.value.to_string())
    }

    /// Process sink output (!) - destroy droplet, no output
    pub fn process_sink_output() -> Result<String> {
        Ok(String::new())
    }

    /// Process character input (?) - read single character from stdin with buffering
    pub fn process_character_input() -> Result<String> {
        let buffer = InputBuffer::new();
        Self::process_character_input_with_buffer(&buffer)
    }

    /// Process character input with a specific buffer
    pub fn process_character_input_with_buffer(buffer: &InputBuffer) -> Result<String> {
        match buffer.read_char() {
            Ok(ch) => Ok(ch.to_string()),
            Err(e) => Err(SystemError::IoError(format!("Failed to read character input: {}", e)).into()),
        }
    }

    /// Process numeric input (??) - read number from stdin with enhanced validation
    pub fn process_numeric_input() -> Result<String> {
        let buffer = InputBuffer::new();
        Self::process_numeric_input_with_buffer(&buffer, ValidationMode::Lenient)
    }

    /// Process numeric input with a specific buffer and validation mode
    pub fn process_numeric_input_with_buffer(buffer: &InputBuffer, mode: ValidationMode) -> Result<String> {
        match buffer.read_line() {
            Ok(input_str) => {
                Self::validate_and_parse_numeric(&input_str, mode)
            }
            Err(e) => Err(SystemError::IoError(format!("Failed to read numeric input: {}", e)).into()),
        }
    }

    /// Validate and parse numeric input based on validation mode
    fn validate_and_parse_numeric(input: &str, mode: ValidationMode) -> Result<String> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Ok("0".to_string());
        }

        match mode {
            ValidationMode::Lenient => {
                // Lenient: try to parse, fall back to 0 on error
                if Self::is_valid_integer(trimmed) {
                    Ok(trimmed.to_string())
                } else {
                    // Extract any valid numbers from the input
                    if let Some(number) = Self::extract_number(trimmed) {
                        Ok(number.to_string())
                    } else {
                        Ok("0".to_string())
                    }
                }
            }
            ValidationMode::Strict => {
                // Strict: return error on invalid input
                if Self::is_valid_integer(trimmed) {
                    Ok(trimmed.to_string())
                } else {
                    Err(SystemError::IoError(format!("Invalid numeric input: '{}'", trimmed)).into())
                }
            }
            ValidationMode::Permissive => {
                // Permissive: intelligent parsing
                if let Some(number) = Self::parse_intelligently(trimmed) {
                    Ok(number.to_string())
                } else {
                    Ok("0".to_string())
                }
            }
        }
    }

    /// Check if a string represents a valid integer
    fn is_valid_integer(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        // Allow optional leading minus sign
        let chars = s.chars().collect::<Vec<_>>();
        let start_idx = if chars[0] == '-' && chars.len() > 1 { 1 } else { 0 };

        // All remaining characters must be digits
        chars[start_idx..].iter().all(|c| c.is_ascii_digit())
    }

    /// Extract the first valid number from a string
    fn extract_number(s: &str) -> Option<i64> {
        let mut current_number = String::new();
        let mut found_number = false;
        let mut has_sign = false;

        for ch in s.chars() {
            if ch == '-' && current_number.is_empty() && !has_sign {
                current_number.push(ch);
                has_sign = true;
            } else if ch.is_ascii_digit() {
                current_number.push(ch);
                found_number = true;
            } else if found_number {
                // We've found a number and now hit a non-digit
                break;
            } else {
                // Reset if we haven't found a valid number yet
                current_number.clear();
                has_sign = false;
            }
        }

        if found_number {
            current_number.parse::<i64>().ok()
        } else {
            None
        }
    }

    /// Parse input intelligently (handles various formats)
    fn parse_intelligently(s: &str) -> Option<i64> {
        // Try direct parsing first
        if let Ok(n) = s.parse::<i64>() {
            return Some(n);
        }

        // Try extracting number
        if let Some(n) = Self::extract_number(s) {
            return Some(n);
        }

        // Handle common words/representations
        match s.to_lowercase().as_str() {
            "zero" | "null" | "nil" => Some(0),
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            "ten" => Some(10),
            _ => None,
        }
    }

    /// Check if a character is an I/O operation
    pub fn is_io_operation(symbol: char) -> bool {
        matches!(symbol, ',' | 'n' | '!' | '?')
    }

    /// Check if a character is a data source operation (input)
    pub fn is_data_source(symbol: char) -> bool {
        matches!(symbol, '?')
    }

    /// Check if a character is a data sink operation
    pub fn is_data_sink(symbol: char) -> bool {
        matches!(symbol, ',' | 'n' | '!')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::coordinate::Coordinate;
    use crate::types::direction::Direction;

    fn create_test_droplet(id: u64, value: i64) -> Droplet {
        let mut droplet = Droplet::new(id, Coordinate::new(0, 0), Direction::Down);
        droplet.set_value(TubularBigInt::new(value));
        droplet
    }

    #[test]
    fn test_character_output() {
        let droplet = create_test_droplet(0, 65); // ASCII 'A'
        let output = IoOperations::process_character_output(&droplet).unwrap();
        assert_eq!(output, "A");

        let droplet = create_test_droplet(0, 72); // ASCII 'H'
        let output = IoOperations::process_character_output(&droplet).unwrap();
        assert_eq!(output, "H");

        let droplet = create_test_droplet(0, 33); // ASCII '!'
        let output = IoOperations::process_character_output(&droplet).unwrap();
        assert_eq!(output, "!");

        // Test invalid ASCII (should output nothing)
        let droplet = create_test_droplet(0, 128); // Outside ASCII range
        let output = IoOperations::process_character_output(&droplet).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_numeric_output() {
        let droplet = create_test_droplet(0, 42);
        let output = IoOperations::process_numeric_output(&droplet).unwrap();
        assert_eq!(output, "42");

        let droplet = create_test_droplet(0, -123);
        let output = IoOperations::process_numeric_output(&droplet).unwrap();
        assert_eq!(output, "-123");

        let droplet = create_test_droplet(0, 0);
        let output = IoOperations::process_numeric_output(&droplet).unwrap();
        assert_eq!(output, "0");
    }

    #[test]
    fn test_sink_output() {
        let output = IoOperations::process_sink_output().unwrap();
        assert_eq!(output, ""); // Sink produces no output
    }

    #[test]
    fn test_io_operation_detection() {
        assert!(IoOperations::is_io_operation(','));
        assert!(IoOperations::is_io_operation('n'));
        assert!(IoOperations::is_io_operation('!'));
        assert!(!IoOperations::is_io_operation('5'));
        assert!(!IoOperations::is_io_operation('A'));
        assert!(!IoOperations::is_io_operation('|'));
    }

    #[test]
    fn test_data_sink_detection() {
        assert!(IoOperations::is_data_sink(','));
        assert!(IoOperations::is_data_sink('n'));
        assert!(IoOperations::is_data_sink('!'));
        assert!(!IoOperations::is_data_sink('5'));
        assert!(!IoOperations::is_data_sink('A'));
        assert!(!IoOperations::is_data_sink('|'));
    }

    #[test]
    fn test_data_source_detection() {
        assert!(IoOperations::is_data_source('?'));
        assert!(!IoOperations::is_data_source(','));
        assert!(!IoOperations::is_data_source('5'));
        assert!(!IoOperations::is_data_source('|'));
    }

    #[test]
    fn test_character_input_operations() {
        // These tests would require mocking stdin, so we just test the detection
        assert!(IoOperations::is_io_operation('?'));
    }

    #[test]
    fn test_extended_io_operation_detection() {
        // Test that all I/O operations are detected
        assert!(IoOperations::is_io_operation(','));  // character output
        assert!(IoOperations::is_io_operation('n'));  // numeric output
        assert!(IoOperations::is_io_operation('!'));  // sink
        assert!(IoOperations::is_io_operation('?'));  // character input

        // Test that non-I/O operations are not detected
        assert!(!IoOperations::is_io_operation('5'));
        assert!(!IoOperations::is_io_operation('A'));
        assert!(!IoOperations::is_io_operation('|'));
        assert!(!IoOperations::is_io_operation(':'));
        assert!(!IoOperations::is_io_operation('+'));
    }

    #[test]
    fn test_input_buffer_creation() {
        let buffer = InputBuffer::new();
        assert_eq!(buffer.validation_mode(), ValidationMode::Lenient);

        let buffer_with_input = InputBuffer::with_input("hello\nworld".to_string());
        assert_eq!(buffer_with_input.validation_mode(), ValidationMode::Lenient);
    }

    #[test]
    fn test_validation_modes() {
        assert_eq!(ValidationMode::Lenient, ValidationMode::Lenient);
        assert_ne!(ValidationMode::Strict, ValidationMode::Lenient);
        assert_ne!(ValidationMode::Permissive, ValidationMode::Strict);
    }

    #[test]
    fn test_numeric_validation() {
        // Test valid integers
        assert!(IoOperations::is_valid_integer("123"));
        assert!(IoOperations::is_valid_integer("-456"));
        assert!(IoOperations::is_valid_integer("0"));
        assert!(!IoOperations::is_valid_integer(""));  // Empty string
        assert!(!IoOperations::is_valid_integer("-"));  // Just minus
        assert!(!IoOperations::is_valid_integer("12a3"));  // Contains letter
        assert!(!IoOperations::is_valid_integer("12.3"));  // Contains decimal
        assert!(!IoOperations::is_valid_integer("abc"));  // Only letters
    }

    #[test]
    fn test_number_extraction() {
        assert_eq!(IoOperations::extract_number("123"), Some(123));
        assert_eq!(IoOperations::extract_number("-456"), Some(-456));
        assert_eq!(IoOperations::extract_number("abc123def"), Some(123));
        assert_eq!(IoOperations::extract_number("abc-123def"), Some(-123));
        assert_eq!(IoOperations::extract_number("abc"), None);
        assert_eq!(IoOperations::extract_number(""), None);
        assert_eq!(IoOperations::extract_number("-"), None);
        assert_eq!(IoOperations::extract_number("123abc456"), Some(123)); // First number
    }

    #[test]
    fn test_intelligent_parsing() {
        // Direct numbers
        assert_eq!(IoOperations::parse_intelligently("123"), Some(123));
        assert_eq!(IoOperations::parse_intelligently("-456"), Some(-456));

        // Number extraction
        assert_eq!(IoOperations::parse_intelligently("abc123def"), Some(123));

        // Word parsing
        assert_eq!(IoOperations::parse_intelligently("zero"), Some(0));
        assert_eq!(IoOperations::parse_intelligently("one"), Some(1));
        assert_eq!(IoOperations::parse_intelligently("five"), Some(5));
        assert_eq!(IoOperations::parse_intelligently("ten"), Some(10));
        assert_eq!(IoOperations::parse_intelligently("ZERO"), Some(0)); // Case insensitive

        // Invalid input
        assert_eq!(IoOperations::parse_intelligently("abc"), None);
        assert_eq!(IoOperations::parse_intelligently(""), None);
    }

    #[test]
    fn test_numeric_validation_modes() {
        // Test lenient mode
        let result = IoOperations::validate_and_parse_numeric("123", ValidationMode::Lenient).unwrap();
        assert_eq!(result, "123");

        let result = IoOperations::validate_and_parse_numeric("abc", ValidationMode::Lenient).unwrap();
        assert_eq!(result, "0"); // Falls back to 0

        let result = IoOperations::validate_and_parse_numeric("", ValidationMode::Lenient).unwrap();
        assert_eq!(result, "0"); // Empty input becomes 0

        // Test strict mode
        let result = IoOperations::validate_and_parse_numeric("123", ValidationMode::Strict).unwrap();
        assert_eq!(result, "123");

        let result = IoOperations::validate_and_parse_numeric("abc", ValidationMode::Strict);
        assert!(result.is_err()); // Should error on invalid input

        // Test permissive mode
        let result = IoOperations::validate_and_parse_numeric("five", ValidationMode::Permissive).unwrap();
        assert_eq!(result, "5"); // Parses word to number

        let result = IoOperations::validate_and_parse_numeric("abc123def", ValidationMode::Permissive).unwrap();
        assert_eq!(result, "123"); // Extracts number

        let result = IoOperations::validate_and_parse_numeric("xyz", ValidationMode::Permissive).unwrap();
        assert_eq!(result, "0"); // Falls back to 0
    }
}