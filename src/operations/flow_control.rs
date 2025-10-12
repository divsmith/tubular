use crate::types::direction::Direction;
use crate::interpreter::grid::ProgramCell;

/// Flow control operations for pipe symbols
pub struct FlowControlOperations;

impl FlowControlOperations {
    /// Process flow control symbol and return new direction
    pub fn process_pipe(symbol: char, current_direction: Direction) -> Direction {
        match symbol {
            '|' => Direction::Down,  // Vertical pipe - continue vertical
            '-' => current_direction, // Horizontal pipe - continue horizontal
            '/' => Self::process_forward_slash(current_direction),
            '\\' => Self::process_backslash(current_direction),
            '^' => Direction::Up,     // Force upward
            'v' => Direction::Down,   // Force downward
            '<' => Direction::Left,   // Force leftward
            '>' => Direction::Right,  // Force rightward
            _ => current_direction,   // No change for other symbols
        }
    }

    /// Process forward slash (/) - reflects 45 degrees
    fn process_forward_slash(current_direction: Direction) -> Direction {
        match current_direction {
            Direction::Right => Direction::Up,    // Coming from right, go up
            Direction::Down => Direction::Left,   // Coming from down, go left
            Direction::Left => Direction::Down,   // Coming from left, go down
            Direction::Up => Direction::Right,    // Coming from up, go right
        }
    }

    /// Process backslash (\) - reflects 45 degrees
    fn process_backslash(current_direction: Direction) -> Direction {
        match current_direction {
            Direction::Right => Direction::Down,  // Coming from right, go down
            Direction::Up => Direction::Left,     // Coming from up, go left
            Direction::Left => Direction::Up,     // Coming from left, go up
            Direction::Down => Direction::Right,  // Coming from down, go right
        }
    }

    /// Check if a symbol is a flow control pipe
    pub fn is_flow_control(symbol: char) -> bool {
        matches!(symbol, '|' | '-' | '/' | '\\' | '^' | 'v' | '<' | '>')
    }

    /// Get all possible exit directions for a flow control symbol
    pub fn get_exit_directions(symbol: char) -> Vec<Direction> {
        match symbol {
            '|' => vec![Direction::Up, Direction::Down],
            '-' => vec![Direction::Left, Direction::Right],
            '/' => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            '\\' => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            '^' => vec![Direction::Up],
            'v' => vec![Direction::Down],
            '<' => vec![Direction::Left],
            '>' => vec![Direction::Right],
            _ => vec![],
        }
    }

    /// Check if flow control allows entry from a specific direction
    pub fn can_enter_from(symbol: char, from_direction: Direction) -> bool {
        match symbol {
            '|' => matches!(from_direction, Direction::Up | Direction::Down),
            '-' => matches!(from_direction, Direction::Left | Direction::Right),
            '/' | '\\' => true, // Corner pipes accept entry from any direction
            '^' | 'v' | '<' | '>' => true, // Directional pipes accept from any direction
            _ => false,
        }
    }
}