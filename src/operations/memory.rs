use crate::interpreter::droplet::Droplet;
use crate::interpreter::stack::DataStack;
use crate::interpreter::memory::{Reservoir, ReservoirCoordinate};
use crate::types::coordinate::Coordinate;
use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, SystemError};

/// Memory operations for reservoir access
pub struct MemoryOperations;

impl MemoryOperations {
    /// Process reservoir Get operation (G)
    /// Reads value from coordinate (droplet.value, stack.pop()) and sets droplet value
    ///
    /// Coordinate calculation:
    /// - x = droplet.value (as isize)
    /// - y = stack.pop() (as isize)
    ///
    /// Behavior:
    /// - Reads from reservoir at calculated coordinate
    /// - Sets droplet value to retrieved value (or 0 if uninitialized)
    /// - Stack loses one value (the y-coordinate)
    pub fn process_get_operation(
        droplet: &mut Droplet,
        stack: &mut DataStack,
        reservoir: &Reservoir,
    ) -> Result<()> {
        // Calculate coordinate: (droplet.value, stack.pop())
        let y_value = stack.pop();
        let x_value = droplet.value.clone();

        // Convert to isize coordinates
        let x = x_value.to_i64().unwrap_or(0) as isize;
        let y = y_value.to_i64().unwrap_or(0) as isize;

        // Create reservoir coordinate
        let coord = ReservoirCoordinate::new(x, y);

        // Get value from reservoir (returns 0 if uninitialized)
        let value = reservoir.get(coord);

        // Set droplet value to retrieved value
        droplet.set_value(value);

        Ok(())
    }

    /// Process reservoir Put operation (P)
    /// Writes droplet value to coordinate (droplet.value, stack.pop())
    ///
    /// Coordinate calculation:
    /// - x = droplet.value (as isize)
    /// - y = stack.pop() (as isize)
    ///
    /// Behavior:
    /// - Writes droplet value to reservoir at calculated coordinate
    /// - Droplet value remains unchanged
    /// - Stack loses one value (the y-coordinate)
    /// - Overwrites any existing value at that coordinate
    pub fn process_put_operation(
        droplet: &Droplet,
        stack: &mut DataStack,
        reservoir: &mut Reservoir,
    ) -> Result<()> {
        // Calculate coordinate: (droplet.value, stack.pop())
        let y_value = stack.pop();
        let x_value = droplet.value.clone();

        // Convert to isize coordinates
        let x = x_value.to_i64().unwrap_or(0) as isize;
        let y = y_value.to_i64().unwrap_or(0) as isize;

        // Create reservoir coordinate
        let coord = ReservoirCoordinate::new(x, y);

        // Put droplet value into reservoir
        reservoir.put(coord, x_value);

        Ok(())
    }

    /// Calculate reservoir coordinate from droplet value and stack
    /// Helper method for coordinate calculation used by both Get and Put
    pub fn calculate_coordinate(
        droplet_value: &TubularBigInt,
        stack: &mut DataStack,
    ) -> Result<ReservoirCoordinate> {
        let y_value = stack.pop();

        let x = droplet_value.to_i64().unwrap_or(0) as isize;
        let y = y_value.to_i64().unwrap_or(0) as isize;

        Ok(ReservoirCoordinate::new(x, y))
    }

    /// Check if a symbol is a memory operation
    pub fn is_memory_operation(symbol: char) -> bool {
        matches!(symbol, 'G' | 'P')
    }

    /// Get the type of memory operation
    pub fn get_memory_operation_type(symbol: char) -> Option<MemoryOperationType> {
        match symbol {
            'G' => Some(MemoryOperationType::Get),
            'P' => Some(MemoryOperationType::Put),
            _ => None,
        }
    }
}

/// Types of memory operations
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryOperationType {
    /// Get operation - read from reservoir
    Get,
    /// Put operation - write to reservoir
    Put,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::droplet::Droplet;
    use crate::interpreter::memory::Reservoir;
    use crate::types::direction::Direction;

    fn create_test_droplet(value: i64, x: isize, y: isize) -> Droplet {
        let mut droplet = Droplet::new(0, Coordinate::new(x, y), Direction::Down);
        droplet.set_value(TubularBigInt::new(value));
        droplet
    }

    #[test]
    fn test_get_operation_reads_from_reservoir() {
        let mut droplet = create_test_droplet(5, 0, 0);
        let mut stack = DataStack::new();
        let mut reservoir = Reservoir::new();

        // Setup: put value 42 at coordinate (5, 10)
        reservoir.put(ReservoirCoordinate::new(5, 10), TubularBigInt::new(42));

        // Setup stack with y-coordinate
        stack.push(TubularBigInt::new(10));

        // Execute get operation
        MemoryOperations::process_get_operation(&mut droplet, &mut stack, &reservoir).unwrap();

        // Verify droplet now has the value from reservoir
        assert_eq!(droplet.value, TubularBigInt::new(42));

        // Verify stack lost the y-coordinate
        assert_eq!(stack.depth(), 0);
    }

    #[test]
    fn test_get_operation_uninitialized_returns_zero() {
        let mut droplet = create_test_droplet(5, 0, 0);
        let mut stack = DataStack::new();
        let reservoir = Reservoir::new(); // Empty reservoir

        // Setup stack with y-coordinate
        stack.push(TubularBigInt::new(10));

        // Execute get operation
        MemoryOperations::process_get_operation(&mut droplet, &mut stack, &reservoir).unwrap();

        // Verify droplet gets zero value
        assert_eq!(droplet.value, TubularBigInt::zero());

        // Verify stack lost the y-coordinate
        assert_eq!(stack.depth(), 0);
    }

    #[test]
    fn test_put_operation_writes_to_reservoir() {
        let droplet = create_test_droplet(5, 0, 0);
        let mut stack = DataStack::new();
        let mut reservoir = Reservoir::new();

        // Setup stack with y-coordinate
        stack.push(TubularBigInt::new(10));

        // Execute put operation
        MemoryOperations::process_put_operation(&droplet, &mut stack, &mut reservoir).unwrap();

        // Verify reservoir now contains the droplet's value
        let retrieved = reservoir.get(ReservoirCoordinate::new(5, 10));
        assert_eq!(retrieved, TubularBigInt::new(5));

        // Verify stack lost the y-coordinate
        assert_eq!(stack.depth(), 0);

        // Verify droplet value unchanged
        assert_eq!(droplet.value, TubularBigInt::new(5));
    }

    #[test]
    fn test_put_operation_overwrites_existing() {
        let droplet = create_test_droplet(5, 0, 0);
        let mut stack = DataStack::new();
        let mut reservoir = Reservoir::new();

        // Pre-populate reservoir with existing value
        reservoir.put(ReservoirCoordinate::new(5, 10), TubularBigInt::new(99));

        // Setup stack with y-coordinate
        stack.push(TubularBigInt::new(10));

        // Execute put operation
        MemoryOperations::process_put_operation(&droplet, &mut stack, &mut reservoir).unwrap();

        // Verify existing value was overwritten
        let retrieved = reservoir.get(ReservoirCoordinate::new(5, 10));
        assert_eq!(retrieved, TubularBigInt::new(5)); // New value, not 99
    }

    #[test]
    fn test_negative_coordinates() {
        let mut droplet = create_test_droplet(-5, 0, 0);
        let mut stack = DataStack::new();
        let mut reservoir = Reservoir::new();

        // Setup: put value 42 at negative coordinate (-5, -10)
        reservoir.put(ReservoirCoordinate::new(-5, -10), TubularBigInt::new(42));

        // Setup stack with negative y-coordinate
        stack.push(TubularBigInt::new(-10));

        // Execute get operation
        MemoryOperations::process_get_operation(&mut droplet, &mut stack, &reservoir).unwrap();

        // Verify negative coordinates work
        assert_eq!(droplet.value, TubularBigInt::new(42));
    }

    #[test]
    fn test_coordinate_calculation() {
        let droplet_value = TubularBigInt::new(5);
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(10));

        let coord = MemoryOperations::calculate_coordinate(&droplet_value, &mut stack).unwrap();

        assert_eq!(coord.x, 5);
        assert_eq!(coord.y, 10);
        assert_eq!(stack.depth(), 0); // Stack should lose the y-coordinate
    }

    #[test]
    fn test_memory_operation_detection() {
        assert!(MemoryOperations::is_memory_operation('G'));
        assert!(MemoryOperations::is_memory_operation('P'));
        assert!(!MemoryOperations::is_memory_operation('A'));
        assert!(!MemoryOperations::is_memory_operation(':'));
        assert!(!MemoryOperations::is_memory_operation('!'));
    }

    #[test]
    fn test_memory_operation_type() {
        assert_eq!(
            MemoryOperations::get_memory_operation_type('G'),
            Some(MemoryOperationType::Get)
        );
        assert_eq!(
            MemoryOperations::get_memory_operation_type('P'),
            Some(MemoryOperationType::Put)
        );
        assert_eq!(MemoryOperations::get_memory_operation_type('A'), None);
        assert_eq!(MemoryOperations::get_memory_operation_type(':'), None);
    }

    #[test]
    fn test_large_coordinates() {
        let mut droplet = create_test_droplet(1000000, 0, 0);
        let mut stack = DataStack::new();
        let mut reservoir = Reservoir::new();

        // Test very large coordinates
        reservoir.put(
            ReservoirCoordinate::new(1000000, 2000000),
            TubularBigInt::new(999)
        );

        stack.push(TubularBigInt::new(2000000));

        MemoryOperations::process_get_operation(&mut droplet, &mut stack, &reservoir).unwrap();

        assert_eq!(droplet.value, TubularBigInt::new(999));
    }

    #[test]
    fn test_stack_underflow_handling() {
        let mut droplet = create_test_droplet(5, 0, 0);
        let mut stack = DataStack::new(); // Empty stack
        let mut reservoir = Reservoir::new();

        // Put operation with empty stack should use y=0
        MemoryOperations::process_put_operation(&droplet, &mut stack, &mut reservoir).unwrap();

        // Verify the value was stored at (5, 0)
        let retrieved = reservoir.get(ReservoirCoordinate::new(5, 0));
        assert_eq!(retrieved, TubularBigInt::new(5));

        // Get operation with empty stack should use y=0
        MemoryOperations::process_get_operation(&mut droplet, &mut stack, &reservoir).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(5));
    }
}