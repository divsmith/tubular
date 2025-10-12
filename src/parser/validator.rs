use crate::interpreter::grid::{ProgramGrid, ProgramCell};
use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::error::{Result, InitError, ExecError};
use std::collections::{HashMap, HashSet};

pub struct ProgramValidator {
    strict_mode: bool,
}

impl ProgramValidator {
    pub fn new() -> Self {
        ProgramValidator {
            strict_mode: false,
        }
    }

    pub fn strict() -> Self {
        ProgramValidator {
            strict_mode: true,
        }
    }

    pub fn validate(&self, grid: &ProgramGrid) -> Result<()> {
        // Basic validation
        grid.validate()?;

        // Additional semantic validation
        self.validate_start_symbol(grid)?;
        self.validate_flow_control(grid)?;
        self.validate_symbols(grid)?;
        self.validate_reachable_code(grid)?;

        if self.strict_mode {
            self.validate_strict_rules(grid)?;
        }

        Ok(())
    }

    fn validate_start_symbol(&self, grid: &ProgramGrid) -> Result<()> {
        if grid.start.is_none() {
            return Err(InitError::NoStartSymbol.into());
        }

        // Ensure start symbol is not at an edge if strict mode
        if self.strict_mode {
            if let Some(start_pos) = grid.start {
                if start_pos.x == 0 || start_pos.y == 0 {
                    return Err(InitError::InvalidCharacter('@', start_pos).into());
                }
            }
        }

        Ok(())
    }

    fn validate_flow_control(&self, grid: &ProgramGrid) -> Result<()> {
        let mut invalid_pipes = Vec::new();

        for (coord, cell) in grid.iter() {
            if !ProgramCell::is_flow_control_symbol(cell.symbol) {
                continue;
            }

            if self.is_invalid_pipe_placement(grid, *coord, cell.symbol) {
                invalid_pipes.push((*coord, cell.symbol));
            }
        }

        if !invalid_pipes.is_empty() {
            let (coord, symbol) = invalid_pipes[0];
            return Err(InitError::InvalidCharacter(symbol, coord).into());
        }

        Ok(())
    }

    fn is_invalid_pipe_placement(&self, grid: &ProgramGrid, coord: Coordinate, symbol: char) -> bool {
        match symbol {
            '|' | '-' => false, // Straight pipes are always valid
            '^' => self.has_invalid_vertical_connection(grid, coord),
            'v' => self.has_invalid_vertical_connection(grid, coord),
            '<' => self.has_invalid_horizontal_connection(grid, coord),
            '>' => self.has_invalid_horizontal_connection(grid, coord),
            '/' | '\\' => self.has_invalid_corner_placement(grid, coord, symbol),
            _ => false,
        }
    }

    fn has_invalid_vertical_connection(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        let above = Coordinate::new(coord.x, coord.y - 1);
        let below = Coordinate::new(coord.x, coord.y + 1);

        // In strict mode, directional pipes need connections
        if self.strict_mode {
            let has_above = grid.get(above).is_some();
            let has_below = grid.get(below).is_some();
            !has_above && !has_below
        } else {
            false
        }
    }

    fn has_invalid_horizontal_connection(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        let left = Coordinate::new(coord.x - 1, coord.y);
        let right = Coordinate::new(coord.x + 1, coord.y);

        if self.strict_mode {
            let has_left = grid.get(left).is_some();
            let has_right = grid.get(right).is_some();
            !has_left && !has_right
        } else {
            false
        }
    }

    fn has_invalid_corner_placement(&self, grid: &ProgramGrid, coord: Coordinate, symbol: char) -> bool {
        // Corner pipes need at least one valid connection in strict mode
        if !self.strict_mode {
            return false;
        }

        let valid_pairs = match symbol {
            '/' => vec![
                (Direction::Right, Direction::Up),
                (Direction::Down, Direction::Left),
            ],
            '\\' => vec![
                (Direction::Right, Direction::Down),
                (Direction::Up, Direction::Left),
            ],
            _ => return false,
        };

        let mut has_valid_connection = false;

        for (from_dir, to_dir) in valid_pairs {
            let from_coord = coord - from_dir;
            let to_coord = coord + to_dir;

            if grid.get(from_coord).is_some() || grid.get(to_coord).is_some() {
                has_valid_connection = true;
                break;
            }
        }

        !has_valid_connection
    }

    fn validate_symbols(&self, grid: &ProgramGrid) -> Result<()> {
        let mut symbol_counts = HashMap::new();

        for (_, cell) in grid.iter() {
            *symbol_counts.entry(cell.symbol).or_insert(0) += 1;
        }

        // Validate start symbol count
        let start_count = symbol_counts.get(&'@').unwrap_or(&0);
        if *start_count != 1 {
            return Err(InitError::NoStartSymbol.into());
        }

        // Check for potentially problematic symbol combinations
        if self.strict_mode {
            self.validate_symbol_combinations(&symbol_counts, grid)?;
        }

        Ok(())
    }

    fn validate_symbol_combinations(&self, symbol_counts: &HashMap<char, usize>, grid: &ProgramGrid) -> Result<()> {
        // In strict mode, ensure certain operators have appropriate context
        for (coord, cell) in grid.iter() {
            match cell.symbol {
                ':' | ';' | 'd' => {
                    // Stack operations should have access to data
                    if !self.has_data_access(grid, *coord) {
                        return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                    }
                }
                'A' | 'S' | 'M' | 'D' => {
                    // Arithmetic operations need at least one operand
                    if !self.has_data_source_nearby(grid, *coord) {
                        return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn has_data_access(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        // Check if there are data sources or stack operations nearby
        let adjacent_coords = [
            Coordinate::new(coord.x - 1, coord.y),
            Coordinate::new(coord.x + 1, coord.y),
            Coordinate::new(coord.x, coord.y - 1),
            Coordinate::new(coord.x, coord.y + 1),
        ];

        for adjacent_coord in adjacent_coords {
            if let Some(cell) = grid.get(adjacent_coord) {
                if ProgramCell::is_data_source(cell.symbol) ||
                   cell.symbol == ':' ||
                   cell.symbol == '0' {
                    return true;
                }
            }
        }

        false
    }

    fn has_data_source_nearby(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        // Check if there are numbers, input sources, or stack operations nearby
        let adjacent_coords = [
            Coordinate::new(coord.x - 1, coord.y),
            Coordinate::new(coord.x + 1, coord.y),
            Coordinate::new(coord.x, coord.y - 1),
            Coordinate::new(coord.x, coord.y + 1),
        ];

        for adjacent_coord in adjacent_coords {
            if let Some(cell) = grid.get(adjacent_coord) {
                if cell.symbol.is_ascii_digit() ||
                   ProgramCell::is_data_source(cell.symbol) ||
                   cell.symbol == ':' ||
                   cell.symbol == ';' {
                    return true;
                }
            }
        }

        false
    }

    fn validate_reachable_code(&self, grid: &ProgramGrid) -> Result<()> {
        if grid.start.is_none() {
            return Ok(()); // Already caught by basic validation
        }

        let start_pos = grid.start.unwrap();
        let mut visited = HashSet::new();
        let mut to_visit = vec![start_pos];

        while let Some(current_pos) = to_visit.pop() {
            if visited.contains(&current_pos) {
                continue;
            }
            visited.insert(current_pos);

            if let Some(cell) = grid.get(current_pos) {
                let next_positions = self.get_next_positions(grid, current_pos, cell);
                for next_pos in next_positions {
                    if !visited.contains(&next_pos) {
                        to_visit.push(next_pos);
                    }
                }
            }
        }

        // In strict mode, warn about unreachable code
        if self.strict_mode {
            let unreachable_count = grid.size() - visited.len();
            if unreachable_count > 0 {
                // For now, we'll just allow unreachable code
                // In a stricter implementation, we might return an error
            }
        }

        Ok(())
    }

    fn get_next_positions(&self, grid: &ProgramGrid, pos: Coordinate, cell: &ProgramCell) -> Vec<Coordinate> {
        let mut positions = Vec::new();

        if ProgramCell::is_flow_control_symbol(cell.symbol) {
            // Follow flow control rules
            let directions = self.get_flow_directions(cell.symbol);
            for direction in directions {
                positions.push(pos + direction);
            }
        } else if ProgramCell::is_operator_symbol(cell.symbol) {
            // Operators allow flow through (default direction)
            positions.push(Coordinate::new(pos.x, pos.y + 1)); // Default down
        }

        positions
    }

    fn get_flow_directions(&self, symbol: char) -> Vec<Direction> {
        match symbol {
            '|' => vec![Direction::Up, Direction::Down],
            '-' => vec![Direction::Left, Direction::Right],
            '^' => vec![Direction::Up],
            'v' => vec![Direction::Down],
            '<' => vec![Direction::Left],
            '>' => vec![Direction::Right],
            '/' => vec![Direction::Up, Direction::Left], // Simplified
            '\\' => vec![Direction::Down, Direction::Left], // Simplified
            _ => vec![],
        }
    }

    fn validate_strict_rules(&self, grid: &ProgramGrid) -> Result<()> {
        // Additional strict validation rules

        // 1. Ensure no orphaned pipes
        self.validate_no_orphaned_pipes(grid)?;

        // 2. Ensure proper I/O placement
        self.validate_io_placement(grid)?;

        // 3. Ensure memory operations have valid coordinates
        self.validate_memory_operations(grid)?;

        Ok(())
    }

    fn validate_no_orphaned_pipes(&self, grid: &ProgramGrid) -> Result<()> {
        for (coord, cell) in grid.iter() {
            if ProgramCell::is_flow_control_symbol(cell.symbol) {
                if self.is_orphaned_pipe(grid, *coord, cell.symbol) {
                    return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                }
            }
        }
        Ok(())
    }

    fn is_orphaned_pipe(&self, grid: &ProgramGrid, coord: Coordinate, symbol: char) -> bool {
        let connections = self.count_connections(grid, coord, symbol);
        connections == 0
    }

    fn count_connections(&self, grid: &ProgramGrid, coord: Coordinate, symbol: char) -> usize {
        let mut connections = 0;
        let adjacent_coords = [
            (Coordinate::new(coord.x, coord.y - 1), Direction::Down),
            (Coordinate::new(coord.x, coord.y + 1), Direction::Up),
            (Coordinate::new(coord.x - 1, coord.y), Direction::Right),
            (Coordinate::new(coord.x + 1, coord.y), Direction::Left),
        ];

        for (adj_coord, expected_dir) in adjacent_coords {
            if let Some(adj_cell) = grid.get(adj_coord) {
                if self.can_connect(symbol, adj_cell.symbol, expected_dir) {
                    connections += 1;
                }
            }
        }

        connections
    }

    fn can_connect(&self, from_symbol: char, to_symbol: char, expected_dir: Direction) -> bool {
        // Simplified connection logic
        ProgramCell::is_flow_control_symbol(to_symbol) ||
        ProgramCell::is_operator_symbol(to_symbol) ||
        ProgramCell::is_start_symbol(to_symbol)
    }

    fn validate_io_placement(&self, grid: &ProgramGrid) -> Result<()> {
        for (coord, cell) in grid.iter() {
            match cell.symbol {
                ',' | 'n' => {
                    // Output operations should have flow control leading to them
                    if !self.has_upstream_connection(grid, *coord) {
                        return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                    }
                }
                '?' => {
                    // Input operations should not be at dead ends
                    if self.is_dead_end(grid, *coord) {
                        return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn has_upstream_connection(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        let upstream = Coordinate::new(coord.x, coord.y - 1);
        grid.get(upstream).is_some()
    }

    fn is_dead_end(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        let exits = self.count_exits(grid, coord);
        exits == 0
    }

    fn count_exits(&self, grid: &ProgramGrid, coord: Coordinate) -> usize {
        let mut exits = 0;
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for direction in directions {
            let next_coord = coord + direction;
            if grid.get(next_coord).is_some() {
                exits += 1;
            }
        }

        exits
    }

    fn validate_memory_operations(&self, grid: &ProgramGrid) -> Result<()> {
        for (coord, cell) in grid.iter() {
            if cell.symbol == 'G' || cell.symbol == 'P' {
                // Memory operations should have access to stack for coordinates
                if !self.can_access_stack_coordinates(grid, *coord) {
                    return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
                }
            }
        }
        Ok(())
    }

    fn can_access_stack_coordinates(&self, grid: &ProgramGrid, coord: Coordinate) -> bool {
        // Check if there are stack operations or data sources nearby
        let nearby_coords = [
            Coordinate::new(coord.x - 1, coord.y),
            Coordinate::new(coord.x + 1, coord.y),
            Coordinate::new(coord.x, coord.y - 1),
            Coordinate::new(coord.x, coord.y + 1),
        ];

        for nearby_coord in nearby_coords {
            if let Some(cell) = grid.get(nearby_coord) {
                if matches!(cell.symbol, ':' | ';' | 'd' | '0'..='9') {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for ProgramValidator {
    fn default() -> Self {
        Self::new()
    }
}