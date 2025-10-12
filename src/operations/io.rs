use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, ExecError, InitError};
use crate::interpreter::droplet::Droplet;

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

    /// Check if a character is an I/O operation
    pub fn is_io_operation(symbol: char) -> bool {
        matches!(symbol, ',' | 'n' | '!')
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
}