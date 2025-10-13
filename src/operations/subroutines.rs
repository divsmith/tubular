use crate::interpreter::droplet::Droplet;
use crate::interpreter::stack::DataStack;
use crate::interpreter::subroutines::{CallStack, StackFrame};
use crate::interpreter::grid::ProgramGrid;
use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, SystemError};

/// Subroutine operations for call/return functionality
pub struct SubroutineOperations;

impl SubroutineOperations {
    /// Process subroutine Call operation (C)
    /// Pushes current position + direction to call stack, then jumps to new location
    ///
    /// Jump target calculation:
    /// - target_x = droplet.value (as isize)
    /// - target_y = stack.pop() (as isize)
    /// - new_direction = stack.pop() converted to Direction
    ///
    /// Behavior:
    /// - Pushes current position + direction to call stack
    /// - Stack loses two values (y-coordinate and direction)
    /// - Droplet jumps to target coordinate with new direction
    pub fn process_call_operation(
        droplet: &mut Droplet,
        stack: &mut DataStack,
        call_stack: &mut CallStack,
        grid: &ProgramGrid,
    ) -> Result<()> {
        // Save current position and direction for return
        let return_position = droplet.position;
        let return_direction = droplet.direction;

        // Calculate jump target: target_y, new_direction from stack
        let direction_value = stack.pop();
        let target_y_value = stack.pop();
        let target_x_value = droplet.value.clone();

        // Convert to target coordinates
        let target_x = target_x_value.to_i64().unwrap_or(0) as isize;
        let target_y = target_y_value.to_i64().unwrap_or(0) as isize;
        let target_position = Coordinate::new(target_x, target_y);

        // Convert direction value to Direction
        let new_direction = Self::value_to_direction(&direction_value);

        // Validate target position exists in grid
        if grid.get(target_position).is_none() {
            // If target doesn't exist, don't jump - just continue as regular move
            return Ok(());
        }

        // Push return frame to call stack
        let return_frame = StackFrame::new(return_position, return_direction);
        call_stack.push(return_frame);

        // Jump droplet to target position with new direction
        droplet.move_to(target_position);
        droplet.set_direction(new_direction);

        Ok(())
    }

    /// Process subroutine Return operation (R)
    /// Pops call stack and returns to saved position + direction
    ///
    /// Behavior:
    /// - Pops return frame from call stack
    /// - Droplet moves to saved position and direction
    /// - If call stack is empty, no operation (continue as regular move)
    pub fn process_return_operation(
        droplet: &mut Droplet,
        call_stack: &mut CallStack,
    ) -> Result<()> {
        // Pop return frame from call stack
        if let Some(return_frame) = call_stack.pop() {
            // Jump droplet to return position and direction
            droplet.move_to(return_frame.return_position);
            droplet.set_direction(return_frame.return_direction);
        }
        // If call stack is empty, continue as regular move (no-op)

        Ok(())
    }

    /// Convert a numeric value to Direction
    /// 0 = Up, 1 = Right, 2 = Down, 3 = Left (clockwise from up)
    /// Invalid values default to Down
    fn value_to_direction(value: &TubularBigInt) -> Direction {
        let val = value.to_i64().unwrap_or(2) as isize % 4; // Default to Down (2)
        match val {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::Down, // Shouldn't happen due to modulo
        }
    }

    /// Convert Direction to numeric value (inverse of value_to_direction)
    /// Up = 0, Right = 1, Down = 2, Left = 3
    pub fn direction_to_value(direction: Direction) -> TubularBigInt {
        match direction {
            Direction::Up => TubularBigInt::new(0),
            Direction::Right => TubularBigInt::new(1),
            Direction::Down => TubularBigInt::new(2),
            Direction::Left => TubularBigInt::new(3),
        }
    }

    /// Check if a symbol is a subroutine operation
    pub fn is_subroutine_operation(symbol: char) -> bool {
        matches!(symbol, 'C' | 'R')
    }

    /// Get the type of subroutine operation
    pub fn get_subroutine_operation_type(symbol: char) -> Option<SubroutineOperationType> {
        match symbol {
            'C' => Some(SubroutineOperationType::Call),
            'R' => Some(SubroutineOperationType::Return),
            _ => None,
        }
    }

    /// Validate that a call target coordinate is valid
    /// Checks if the coordinate exists in the program grid
    pub fn validate_call_target(target: Coordinate, grid: &ProgramGrid) -> bool {
        grid.get(target).is_some()
    }

    /// Get all valid call targets in the grid
    /// Returns coordinates that exist in the program (useful for debugging)
    pub fn get_valid_call_targets(grid: &ProgramGrid) -> Vec<Coordinate> {
        grid.cells.keys().copied().collect()
    }
}

/// Types of subroutine operations
#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineOperationType {
    /// Call operation - jump to subroutine
    Call,
    /// Return operation - return from subroutine
    Return,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::droplet::Droplet;
    use crate::interpreter::subroutines::{CallStack, StackFrame};
    use crate::interpreter::grid::{ProgramGrid, ProgramCell};

    fn create_test_droplet(value: i64, x: isize, y: isize, direction: Direction) -> Droplet {
        let mut droplet = Droplet::new(0, Coordinate::new(x, y), direction);
        droplet.set_value(TubularBigInt::new(value));
        droplet
    }

    fn create_test_grid() -> ProgramGrid {
        let mut grid = ProgramGrid::new();

        // Add some test cells
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(5, 10), 'A').unwrap();
        grid.add_cell(Coordinate::new(-3, 7), 'P').unwrap();

        grid
    }

    #[test]
    fn test_call_operation_pushes_return_frame() {
        let mut droplet = create_test_droplet(5, 0, 0, Direction::Down);
        let mut stack = DataStack::new();
        let mut call_stack = CallStack::new();
        let grid = create_test_grid();

        // Setup stack with direction and y-coordinate
        stack.push(TubularBigInt::new(1)); // Right direction
        stack.push(TubularBigInt::new(10)); // y-coordinate

        // Execute call operation
        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Verify return frame was pushed
        assert_eq!(call_stack.depth(), 1);
        let frame = call_stack.peek().unwrap();
        assert_eq!(frame.return_position, Coordinate::new(0, 0));
        assert_eq!(frame.return_direction, Direction::Down);

        // Verify droplet jumped to target position
        assert_eq!(droplet.position, Coordinate::new(5, 10));
        assert_eq!(droplet.direction, Direction::Right);

        // Verify stack lost two values
        assert_eq!(stack.depth(), 0);
    }

    #[test]
    fn test_call_operation_invalid_target_no_jump() {
        let mut droplet = create_test_droplet(99, 0, 0, Direction::Down); // Target (99, 10) doesn't exist
        let mut stack = DataStack::new();
        let mut call_stack = CallStack::new();
        let grid = create_test_grid();

        // Setup stack with direction and y-coordinate
        stack.push(TubularBigInt::new(1)); // Right direction
        stack.push(TubularBigInt::new(10)); // y-coordinate

        let original_position = droplet.position;
        let original_direction = droplet.direction;

        // Execute call operation
        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Verify no jump occurred (target invalid)
        assert_eq!(droplet.position, original_position);
        assert_eq!(droplet.direction, original_direction);

        // Verify no return frame was pushed
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_return_operation_pops_call_stack() {
        let mut droplet = create_test_droplet(0, 5, 5, Direction::Up);
        let mut call_stack = CallStack::new();

        // Setup call stack with return frame
        let return_frame = StackFrame::new(Coordinate::new(1, 1), Direction::Right);
        call_stack.push(return_frame);

        // Execute return operation
        SubroutineOperations::process_return_operation(&mut droplet, &mut call_stack).unwrap();

        // Verify droplet returned to saved position and direction
        assert_eq!(droplet.position, Coordinate::new(1, 1));
        assert_eq!(droplet.direction, Direction::Right);

        // Verify call stack is now empty
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_return_operation_empty_stack_no_op() {
        let mut droplet = create_test_droplet(0, 5, 5, Direction::Up);
        let mut call_stack = CallStack::new(); // Empty stack

        let original_position = droplet.position;
        let original_direction = droplet.direction;

        // Execute return operation
        SubroutineOperations::process_return_operation(&mut droplet, &mut call_stack).unwrap();

        // Verify no change (empty call stack)
        assert_eq!(droplet.position, original_position);
        assert_eq!(droplet.direction, original_direction);
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_nested_calls() {
        let mut droplet = create_test_droplet(5, 0, 0, Direction::Down);
        let mut stack = DataStack::new();
        let mut call_stack = CallStack::new();
        let grid = create_test_grid();

        // First call: (0,0,Down) -> (5,10,Right)
        stack.push(TubularBigInt::new(1)); // Right
        stack.push(TubularBigInt::new(10)); // y=10

        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Second call: (5,10,Right) -> (-3,7,Up)
        droplet.set_value(TubularBigInt::new(-3));
        stack.push(TubularBigInt::new(0)); // Up
        stack.push(TubularBigInt::new(7)); // y=7

        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Verify two frames on call stack
        assert_eq!(call_stack.depth(), 2);

        // First return: (-3,7,Up) -> (5,10,Right)
        SubroutineOperations::process_return_operation(&mut droplet, &mut call_stack).unwrap();
        assert_eq!(droplet.position, Coordinate::new(5, 10));
        assert_eq!(droplet.direction, Direction::Right);
        assert_eq!(call_stack.depth(), 1);

        // Second return: (5,10,Right) -> (0,0,Down)
        SubroutineOperations::process_return_operation(&mut droplet, &mut call_stack).unwrap();
        assert_eq!(droplet.position, Coordinate::new(0, 0));
        assert_eq!(droplet.direction, Direction::Down);
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_value_to_direction() {
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(0)), Direction::Up);
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(1)), Direction::Right);
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(2)), Direction::Down);
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(3)), Direction::Left);

        // Test wraparound
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(4)), Direction::Up);
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(-1)), Direction::Left);

        // Test invalid/zero value defaults
        assert_eq!(SubroutineOperations::value_to_direction(&TubularBigInt::new(999)), Direction::Right);
    }

    #[test]
    fn test_direction_to_value() {
        assert_eq!(SubroutineOperations::direction_to_value(Direction::Up), TubularBigInt::new(0));
        assert_eq!(SubroutineOperations::direction_to_value(Direction::Right), TubularBigInt::new(1));
        assert_eq!(SubroutineOperations::direction_to_value(Direction::Down), TubularBigInt::new(2));
        assert_eq!(SubroutineOperations::direction_to_value(Direction::Left), TubularBigInt::new(3));
    }

    #[test]
    fn test_direction_conversion_roundtrip() {
        for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let value = SubroutineOperations::direction_to_value(direction);
            let converted_back = SubroutineOperations::value_to_direction(&value);
            assert_eq!(direction, converted_back);
        }
    }

    #[test]
    fn test_subroutine_operation_detection() {
        assert!(SubroutineOperations::is_subroutine_operation('C'));
        assert!(SubroutineOperations::is_subroutine_operation('R'));
        assert!(!SubroutineOperations::is_subroutine_operation('A'));
        assert!(!SubroutineOperations::is_subroutine_operation(':'));
        assert!(!SubroutineOperations::is_subroutine_operation('!'));
    }

    #[test]
    fn test_subroutine_operation_type() {
        assert_eq!(
            SubroutineOperations::get_subroutine_operation_type('C'),
            Some(SubroutineOperationType::Call)
        );
        assert_eq!(
            SubroutineOperations::get_subroutine_operation_type('R'),
            Some(SubroutineOperationType::Return)
        );
        assert_eq!(SubroutineOperations::get_subroutine_operation_type('A'), None);
        assert_eq!(SubroutineOperations::get_subroutine_operation_type(':'), None);
    }

    #[test]
    fn test_call_target_validation() {
        let grid = create_test_grid();

        // Valid targets
        assert!(SubroutineOperations::validate_call_target(Coordinate::new(0, 0), &grid));
        assert!(SubroutineOperations::validate_call_target(Coordinate::new(5, 10), &grid));
        assert!(SubroutineOperations::validate_call_target(Coordinate::new(-3, 7), &grid));

        // Invalid targets
        assert!(!SubroutineOperations::validate_call_target(Coordinate::new(99, 99), &grid));
        assert!(!SubroutineOperations::validate_call_target(Coordinate::new(1, 1), &grid));
    }

    #[test]
    fn test_get_valid_call_targets() {
        let grid = create_test_grid();
        let targets = SubroutineOperations::get_valid_call_targets(&grid);

        assert_eq!(targets.len(), 3);
        assert!(targets.contains(&Coordinate::new(0, 0)));
        assert!(targets.contains(&Coordinate::new(5, 10)));
        assert!(targets.contains(&Coordinate::new(-3, 7)));
    }

    #[test]
    fn test_stack_underflow_handling() {
        let mut droplet = create_test_droplet(5, 0, 0, Direction::Down);
        let mut stack = DataStack::new(); // Empty stack
        let mut call_stack = CallStack::new();
        let grid = create_test_grid();

        // Call operation with empty stack should use defaults
        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Should still push return frame and attempt jump with defaults
        assert_eq!(call_stack.depth(), 1);
        // Default direction should be Down (2)
        // Default y-coordinate should be 0
        // Should attempt to jump to (5, 0) with Down direction
    }

    #[test]
    fn test_negative_coordinates_in_calls() {
        let mut droplet = create_test_droplet(-3, 0, 0, Direction::Down);
        let mut stack = DataStack::new();
        let mut call_stack = CallStack::new();
        let grid = create_test_grid();

        // Setup stack for negative coordinate jump
        stack.push(TubularBigInt::new(0)); // Up direction
        stack.push(TubularBigInt::new(7)); // y=7

        // Execute call to negative coordinates
        SubroutineOperations::process_call_operation(&mut droplet, &mut stack, &mut call_stack, &grid).unwrap();

        // Verify droplet jumped to negative coordinate
        assert_eq!(droplet.position, Coordinate::new(-3, 7));
        assert_eq!(droplet.direction, Direction::Up);
    }
}