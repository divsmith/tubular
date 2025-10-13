use crate::interpreter::grid::{ProgramGrid, ProgramCell};
use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::error::{Result, InitError, InterpreterError, ErrorType, ErrorSeverity, Position, ErrorContext};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct ProgramValidator {
    strict_mode: bool,
    collect_errors: bool,
    errors: Vec<InterpreterError>,
    source_content: Option<String>,
}

impl ProgramValidator {
    pub fn new() -> Self {
        ProgramValidator {
            strict_mode: false,
            collect_errors: false,
            errors: Vec::new(),
            source_content: None,
        }
    }

    pub fn strict() -> Self {
        ProgramValidator {
            strict_mode: true,
            collect_errors: false,
            errors: Vec::new(),
            source_content: None,
        }
    }

    pub fn with_error_collection(mut self) -> Self {
        self.collect_errors = true;
        self
    }

    pub fn with_source_content(mut self, content: String) -> Self {
        self.source_content = Some(content);
        self
    }

    pub fn get_errors(&self) -> &[InterpreterError] {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    pub fn validate(&self, grid: &ProgramGrid) -> Result<()> {
        let mut validator = self.clone();
        validator.validate_with_collection(grid)
    }

    fn validate_with_collection(&mut self, grid: &ProgramGrid) -> Result<()> {
        self.errors.clear();

        // Basic validation
        if let Err(e) = grid.validate() {
            if self.collect_errors {
                self.add_enhanced_error(e, None);
            } else {
                return Err(e);
            }
        }

        // Additional semantic validation
        if let Err(e) = self.validate_start_symbol_with_context(grid) {
            if self.collect_errors {
                self.errors.push(e);
            } else {
                return Err(e);
            }
        }

        if let Err(e) = self.validate_flow_control_with_context(grid) {
            if self.collect_errors {
                self.errors.push(e);
            } else {
                return Err(e);
            }
        }

        if let Err(e) = self.validate_symbols_with_context(grid) {
            if self.collect_errors {
                self.errors.push(e);
            } else {
                return Err(e);
            }
        }

        if let Err(e) = self.validate_reachable_code_with_context(grid) {
            if self.collect_errors {
                self.errors.push(e);
            } else {
                return Err(e);
            }
        }

        if self.strict_mode {
            if let Err(e) = self.validate_strict_rules_with_context(grid) {
                if self.collect_errors {
                    self.errors.push(e);
                } else {
                    return Err(e);
                }
            }
        }

        // If we're collecting errors, return an error if any were collected
        if self.collect_errors && !self.errors.is_empty() {
            return Err(InterpreterError::enhanced(
                format!("Found {} validation errors", self.errors.len()),
                ErrorType::Validation
            ).with_severity(ErrorSeverity::Error));
        }

        Ok(())
    }

    fn add_enhanced_error(&mut self, error: InterpreterError, coord: Option<Coordinate>) {
        let enhanced_error = if let Some(coord) = coord {
            let context = self.create_error_context_for_coord(coord);
            error.with_context(context)
        } else {
            error
        };
        self.errors.push(enhanced_error);
    }

    fn create_error_context_for_coord(&self, coord: Coordinate) -> ErrorContext {
        let position = Position::new(coord.y as usize, coord.x as usize, coord);

        if let Some(ref content) = self.source_content {
            let lines: Vec<&str> = content.lines().collect();

            let source_line = lines.get(position.line)
                .map(|line| line.to_string())
                .unwrap_or_default();

            let surrounding_lines = {
                let start = position.line.saturating_sub(2);
                let end = std::cmp::min(position.line + 3, lines.len());

                lines[start..end]
                    .iter()
                    .enumerate()
                    .map(|(i, line)| (start + i, line.to_string()))
                    .collect()
            };

            ErrorContext::new(position.clone(), source_line)
                .with_surrounding_lines(surrounding_lines)
        } else {
            ErrorContext::new(position, "".to_string())
        }
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

    fn validate_start_symbol_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        if grid.start.is_none() {
            let error = InterpreterError::enhanced(
                "No start symbol (@) found in program".to_string(),
                ErrorType::Initialization
            ).with_suggestions(vec![
                "Add a start symbol '@' to your program".to_string(),
                "The start symbol should be placed where you want execution to begin".to_string(),
            ]).with_help("Every Tubular program needs exactly one start symbol '@' to indicate where execution should begin.".to_string());

            return Err(error);
        }

        // Ensure start symbol is not at an edge if strict mode
        if self.strict_mode {
            if let Some(start_pos) = grid.start {
                if start_pos.x == 0 || start_pos.y == 0 {
                    let context = self.create_error_context_for_coord(start_pos);
                    let error = InterpreterError::enhanced(
                        "Start symbol '@' cannot be placed at the edge in strict mode".to_string(),
                        ErrorType::Validation
                    ).with_context(context)
                    .with_suggestions(vec![
                        "Move the start symbol away from the edge".to_string(),
                        "Add at least one space margin around the start symbol".to_string(),
                    ]).with_help("In strict mode, the start symbol should have at least one cell of space around it to ensure proper flow.".to_string());

                    return Err(error);
                }
            }
        }

        Ok(InterpreterError::enhanced("Validation passed".to_string(), ErrorType::Validation))
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

    fn validate_flow_control_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
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
            let errors: Vec<InterpreterError> = invalid_pipes.into_iter().map(|(coord, symbol)| {
                let context = self.create_error_context_for_coord(coord);
                let suggestions = self.get_pipe_placement_suggestions(symbol, coord, grid);

                InterpreterError::enhanced(
                    format!("Invalid flow control pipe '{}' at this position", symbol),
                    ErrorType::Validation
                ).with_context(context)
                .with_suggestions(suggestions)
                .with_help(self.get_pipe_help_text(symbol))
            }).collect();

            // Return the first error, or create a combined error if multiple
            if let Some(first_error) = errors.into_iter().next() {
                return Err(first_error);
            }
        }

        Ok(InterpreterError::enhanced("Flow control validation passed".to_string(), ErrorType::Validation))
    }

    fn get_pipe_placement_suggestions(&self, symbol: char, coord: Coordinate, grid: &ProgramGrid) -> Vec<String> {
        let mut suggestions = Vec::new();

        match symbol {
            '^' | 'v' => {
                let has_above = grid.get(Coordinate::new(coord.x, coord.y - 1)).is_some();
                let has_below = grid.get(Coordinate::new(coord.x, coord.y + 1)).is_some();

                if !has_above && !has_below {
                    suggestions.push("Add a cell above or below this vertical pipe".to_string());
                    suggestions.push("Use '|' for bidirectional vertical flow".to_string());
                } else if !has_above {
                    suggestions.push("Add a cell above to complete the vertical connection".to_string());
                } else if !has_below {
                    suggestions.push("Add a cell below to complete the vertical connection".to_string());
                }
            }
            '<' | '>' => {
                let has_left = grid.get(Coordinate::new(coord.x - 1, coord.y)).is_some();
                let has_right = grid.get(Coordinate::new(coord.x + 1, coord.y)).is_some();

                if !has_left && !has_right {
                    suggestions.push("Add a cell to the left or right of this horizontal pipe".to_string());
                    suggestions.push("Use '-' for bidirectional horizontal flow".to_string());
                } else if !has_left {
                    suggestions.push("Add a cell to the left to complete the horizontal connection".to_string());
                } else if !has_right {
                    suggestions.push("Add a cell to the right to complete the horizontal connection".to_string());
                }
            }
            '/' | '\\' => {
                suggestions.push("Ensure corner pipes have proper connections on their valid sides".to_string());
                suggestions.push("Check that adjacent cells align with the corner's direction".to_string());
            }
            _ => {
                suggestions.push("Ensure the pipe symbol has appropriate connections".to_string());
            }
        }

        suggestions
    }

    fn get_pipe_help_text(&self, symbol: char) -> String {
        match symbol {
            '^' => "This pipe directs flow upward. In strict mode, it should have at least one connection (above or below).".to_string(),
            'v' => "This pipe directs flow downward. In strict mode, it should have at least one connection (above or below).".to_string(),
            '<' => "This pipe directs flow leftward. In strict mode, it should have at least one connection (left or right).".to_string(),
            '>' => "This pipe directs flow rightward. In strict mode, it should have at least one connection (left or right).".to_string(),
            '|' => "This pipe allows bidirectional vertical flow and connects cells above and below.".to_string(),
            '-' => "This pipe allows bidirectional horizontal flow and connects cells left and right.".to_string(),
            '/' => "This corner pipe redirects flow from right to up or from down to left.".to_string(),
            '\\' => "This corner pipe redirects flow from left to down or from up to right.".to_string(),
            _ => "This is a flow control symbol that directs droplet movement.".to_string(),
        }
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

    fn validate_symbols_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        let mut symbol_counts = HashMap::new();

        for (_, cell) in grid.iter() {
            *symbol_counts.entry(cell.symbol).or_insert(0) += 1;
        }

        // Validate start symbol count
        let start_count = symbol_counts.get(&'@').unwrap_or(&0);
        if *start_count != 1 {
            let error = if *start_count == 0 {
                InterpreterError::enhanced(
                    "No start symbol (@) found in program".to_string(),
                    ErrorType::Initialization
                ).with_suggestions(vec![
                    "Add a start symbol '@' to your program".to_string(),
                    "The start symbol should be placed where you want execution to begin".to_string(),
                ]).with_help("Every Tubular program needs exactly one start symbol '@' to indicate where execution should begin.".to_string())
            } else {
                InterpreterError::enhanced(
                    format!("Multiple start symbols (@) found: {} start symbols", start_count),
                    ErrorType::Initialization
                ).with_suggestions(vec![
                    "Remove all but one start symbol '@'".to_string(),
                    "Choose the location where you want execution to begin".to_string(),
                ]).with_help("A Tubular program can only have one start symbol '@'. Multiple start symbols create ambiguity about where execution should begin.".to_string())
            };

            return Err(error);
        }

        // Check for potentially problematic symbol combinations
        if self.strict_mode {
            if let Err(e) = self.validate_symbol_combinations_with_context(grid) {
                return Err(e);
            }
        }

        Ok(InterpreterError::enhanced("Symbol validation passed".to_string(), ErrorType::Validation))
    }

    fn validate_symbol_combinations(&self, _symbol_counts: &HashMap<char, usize>, grid: &ProgramGrid) -> Result<()> {
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

    fn validate_symbol_combinations_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        // In strict mode, ensure certain operators have appropriate context
        for (coord, cell) in grid.iter() {
            match cell.symbol {
                ':' | ';' | 'd' => {
                    // Stack operations should have access to data
                    if !self.has_data_access(grid, *coord) {
                        let context = self.create_error_context_for_coord(*coord);
                        let error = InterpreterError::enhanced(
                            format!("Stack operation '{}' has no access to data", cell.symbol),
                            ErrorType::Semantic
                        ).with_context(context)
                        .with_suggestions(vec![
                            "Add a data source nearby (numbers, input symbol '?', or other stack operations)".to_string(),
                            "Move the operation closer to data flow".to_string(),
                        ]).with_help("Stack operations need access to data to function properly. Place them near data sources or in the flow of operations.".to_string());
                        return Err(error);
                    }
                }
                'A' | 'S' | 'M' | 'D' => {
                    // Arithmetic operations need at least one operand
                    if !self.has_data_source_nearby(grid, *coord) {
                        let context = self.create_error_context_for_coord(*coord);
                        let operation_name = match cell.symbol {
                            'A' => "Addition",
                            'S' => "Subtraction",
                            'M' => "Multiplication",
                            'D' => "Division",
                            _ => "Arithmetic",
                        };
                        let error = InterpreterError::enhanced(
                            format!("{} operation '{}' has no operands available", operation_name, cell.symbol),
                            ErrorType::Semantic
                        ).with_context(context)
                        .with_suggestions(vec![
                            "Add numeric literals (0-9) nearby".to_string(),
                            "Ensure data flow reaches this operation".to_string(),
                            "Add input operations or other data sources upstream".to_string(),
                        ]).with_help("Arithmetic operations need at least one operand to work with. Make sure there's data flowing into this operation.".to_string());
                        return Err(error);
                    }
                }
                _ => {}
            }
        }

        Ok(InterpreterError::enhanced("Symbol combination validation passed".to_string(), ErrorType::Validation))
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

    // Enhanced validation methods with context
    fn validate_reachable_code_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        if grid.start.is_none() {
            return Ok(InterpreterError::enhanced("Reachable code validation skipped - no start symbol".to_string(), ErrorType::Validation));
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
                let error = InterpreterError::enhanced(
                    format!("Found {} unreachable code cells", unreachable_count),
                    ErrorType::Validation
                ).with_suggestions(vec![
                    "Remove unused code cells".to_string(),
                    "Connect unreachable code to the main flow".to_string(),
                    "Add flow control to reach the isolated cells".to_string(),
                ]).with_help("Unreachable code cannot be executed and may indicate a bug in your program logic or missing flow connections.".to_string());
                return Err(error);
            }
        }

        Ok(InterpreterError::enhanced("Reachable code validation passed".to_string(), ErrorType::Validation))
    }

    fn validate_strict_rules_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        // 1. Ensure no orphaned pipes
        if let Err(e) = self.validate_no_orphaned_pipes_with_context(grid) {
            return Err(e);
        }

        // 2. Ensure proper I/O placement
        if let Err(e) = self.validate_io_placement_with_context(grid) {
            return Err(e);
        }

        // 3. Ensure memory operations have valid coordinates
        if let Err(e) = self.validate_memory_operations_with_context(grid) {
            return Err(e);
        }

        Ok(InterpreterError::enhanced("Strict validation passed".to_string(), ErrorType::Validation))
    }

    fn validate_no_orphaned_pipes_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        for (coord, cell) in grid.iter() {
            if ProgramCell::is_flow_control_symbol(cell.symbol) {
                if self.is_orphaned_pipe(grid, *coord, cell.symbol) {
                    let context = self.create_error_context_for_coord(*coord);
                    let error = InterpreterError::enhanced(
                        format!("Orphaned flow control pipe '{}' has no connections", cell.symbol),
                        ErrorType::Validation
                    ).with_context(context)
                    .with_suggestions(vec![
                        "Add adjacent cells to connect this pipe".to_string(),
                        "Remove the orphaned pipe if not needed".to_string(),
                        "Check for gaps in your flow control network".to_string(),
                    ]).with_help("Flow control pipes need to be connected to other cells to be useful. An orphaned pipe cannot guide droplets anywhere.".to_string());
                    return Err(error);
                }
            }
        }
        Ok(InterpreterError::enhanced("Orphaned pipe validation passed".to_string(), ErrorType::Validation))
    }

    fn validate_io_placement_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        for (coord, cell) in grid.iter() {
            match cell.symbol {
                ',' | 'n' => {
                    // Output operations should have flow control leading to them
                    if !self.has_upstream_connection(grid, *coord) {
                        let context = self.create_error_context_for_coord(*coord);
                        let error = InterpreterError::enhanced(
                            format!("Output operation '{}' has no upstream connection", cell.symbol),
                            ErrorType::Validation
                        ).with_context(context)
                        .with_suggestions(vec![
                            "Add flow control leading to this output operation".to_string(),
                            "Place the output operation in the main flow path".to_string(),
                            "Connect this operation to upstream cells".to_string(),
                        ]).with_help("Output operations need to be reachable by droplets to function. Ensure there's a path for droplets to reach this cell.".to_string());
                        return Err(error);
                    }
                }
                '?' => {
                    // Input operations should not be at dead ends
                    if self.is_dead_end(grid, *coord) {
                        let context = self.create_error_context_for_coord(*coord);
                        let error = InterpreterError::enhanced(
                            "Input operation '?' is at a dead end".to_string(),
                            ErrorType::Validation
                        ).with_context(context)
                        .with_suggestions(vec![
                            "Add flow control after the input operation".to_string(),
                            "Connect the input operation to downstream cells".to_string(),
                            "Ensure input can flow to the rest of the program".to_string(),
                        ]).with_help("Input operations should provide data to the rest of the program. If it's at a dead end, the input data won't be used.".to_string());
                        return Err(error);
                    }
                }
                _ => {}
            }
        }
        Ok(InterpreterError::enhanced("I/O placement validation passed".to_string(), ErrorType::Validation))
    }

    fn validate_memory_operations_with_context(&self, grid: &ProgramGrid) -> Result<InterpreterError> {
        for (coord, cell) in grid.iter() {
            if cell.symbol == 'G' || cell.symbol == 'P' {
                // Memory operations should have access to stack for coordinates
                if !self.can_access_stack_coordinates(grid, *coord) {
                    let context = self.create_error_context_for_coord(*coord);
                    let operation_name = match cell.symbol {
                        'G' => "Get (read)",
                        'P' => "Put (write)",
                        _ => "Memory",
                    };
                    let error = InterpreterError::enhanced(
                        format!("{} memory operation '{}' cannot access stack coordinates", operation_name, cell.symbol),
                        ErrorType::Validation
                    ).with_context(context)
                    .with_suggestions(vec![
                        "Add stack operations nearby to provide coordinates".to_string(),
                        "Place numeric literals for direct addressing".to_string(),
                        "Move the operation closer to coordinate sources".to_string(),
                    ]).with_help("Memory operations need coordinates from the stack or direct addressing. Ensure there's a way to provide coordinates to this operation.".to_string());
                    return Err(error);
                }
            }
        }
        Ok(InterpreterError::enhanced("Memory operations validation passed".to_string(), ErrorType::Validation))
    }
}

impl Default for ProgramValidator {
    fn default() -> Self {
        Self::new()
    }
}