use crate::interpreter::grid::{ProgramGrid, ProgramCell};
use crate::types::coordinate::Coordinate;
use crate::types::error::{Result, InitError, InterpreterError, ErrorType, ErrorSeverity, Position, ErrorContext};
use std::io::{self, Read};
use std::collections::HashMap;

/// Parsing context for tracking source information
#[derive(Debug, Clone)]
pub struct ParseContext {
    pub source_name: String,
    pub lines: Vec<String>,
    pub line_offsets: Vec<usize>,
}

impl ParseContext {
    pub fn new(source_name: String, content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
        let mut line_offsets = vec![0];
        let mut current_offset = 0;

        for line in &lines {
            current_offset += line.len() + 1; // +1 for newline
            line_offsets.push(current_offset);
        }

        Self {
            source_name,
            lines,
            line_offsets,
        }
    }

    pub fn get_line(&self, line_index: usize) -> Option<&str> {
        self.lines.get(line_index).map(|line| line.as_str())
    }

    pub fn get_surrounding_lines(&self, line_index: usize, context_size: usize) -> Vec<(usize, String)> {
        let start = line_index.saturating_sub(context_size);
        let end = std::cmp::min(line_index + context_size + 1, self.lines.len());

        self.lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| (start + i, line.clone()))
            .collect()
    }

    pub fn position_from_coordinate(&self, coord: Coordinate) -> Position {
        Position::new(
            coord.y as usize,
            coord.x as usize,
            coord
        )
    }

    pub fn create_error_context(&self, position: Position, span: Option<(usize, usize)>) -> ErrorContext {
        let source_line = self.get_line(position.line)
            .unwrap_or("")
            .to_string();

        let error_span = span.unwrap_or((position.column, position.column + 1));

        let context = ErrorContext::new(position.clone(), source_line)
            .with_span(error_span.0, error_span.1)
            .with_surrounding_lines(self.get_surrounding_lines(position.line, 2));

        context
    }
}

#[derive(Clone)]
pub struct GridParser {
    parse_context: Option<ParseContext>,
    collect_errors: bool,
    errors: Vec<InterpreterError>,
}

impl GridParser {
    pub fn new() -> Self {
        Self {
            parse_context: None,
            collect_errors: false,
            errors: Vec::new(),
        }
    }

    pub fn with_error_collection(mut self) -> Self {
        self.collect_errors = true;
        self
    }

    pub fn get_errors(&self) -> &[InterpreterError] {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    pub fn parse_file(&self, file_path: &str) -> Result<ProgramGrid> {
        let content = std::fs::read_to_string(file_path)?;
        let mut parser = self.clone();
        parser.parse_string_with_context(&content, file_path.to_string())
    }

    pub fn parse_string(&self, content: &str) -> Result<ProgramGrid> {
        let mut parser = self.clone();
        parser.parse_string_with_context(content, "<string>".to_string())
    }

    fn parse_string_with_context(&mut self, content: &str, source_name: String) -> Result<ProgramGrid> {
        self.parse_context = Some(ParseContext::new(source_name.clone(), content));
        self.errors.clear();

        let lines: Vec<&str> = content.lines().collect();
        self.parse_lines_with_context(&lines)
    }

    pub fn parse_lines(&self, lines: &[&str]) -> Result<ProgramGrid> {
        let mut parser = self.clone();
        parser.parse_lines_with_context(lines)
    }

    fn parse_lines_with_context(&mut self, lines: &[&str]) -> Result<ProgramGrid> {
        let mut grid = ProgramGrid::new();
        let mut invalid_chars = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }

                let coord = Coordinate::new(x as isize, y as isize);

                // Validate character before adding to grid
                match self.validate_character(ch, coord) {
                    Ok(()) => {
                        if let Err(e) = grid.add_cell(coord, ch) {
                            if self.collect_errors {
                                let context = self.create_error_context_for_coord(coord);
                                let enhanced_error = self.enhance_error_for_interpreter_error(e, context);
                                self.errors.push(enhanced_error);
                            } else {
                                return Err(e.into());
                            }
                        }
                    }
                    Err(e) => {
                        if self.collect_errors {
                            let context = self.create_error_context_for_coord(coord);
                            let enhanced_error = e.with_context(context);
                            self.errors.push(enhanced_error);
                            invalid_chars.push((coord, ch));
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }

        // Check for multiple start symbols
        self.validate_start_symbols(&grid)?;

        // If we're collecting errors, return the grid anyway with all errors collected
        if self.collect_errors && !self.errors.is_empty() {
            return Err(InterpreterError::enhanced(
                format!("Found {} parsing errors", self.errors.len()),
                ErrorType::Syntax
            ).with_severity(ErrorSeverity::Error));
        }

        Ok(grid)
    }

    fn validate_character(&self, ch: char, coord: Coordinate) -> Result<()> {
        if !ProgramCell::is_valid_symbol(ch) {
            let position = Position::new(coord.y as usize, coord.x as usize, coord);
            let mut error = InterpreterError::enhanced(
                format!("Invalid character '{}' found in program", ch),
                ErrorType::Syntax
            );

            if let Some(ref context) = self.parse_context {
                let error_context = context.create_error_context(position, None);
                error = error.with_context(error_context)
                    .with_suggestions(vec![
                        format!("Remove the '{}' character", ch),
                        "Check the Tubular language specification for valid symbols".to_string(),
                        "Common valid symbols include: @, digits 0-9, +, -, *, /, |, -, ^, v, <, >, etc.".to_string(),
                    ])
                    .with_help("Tubular programs can only contain valid symbols from the language specification. Invalid characters will cause parsing to fail.".to_string());
            }

            return Err(error);
        }

        Ok(())
    }

    fn validate_start_symbols(&mut self, grid: &ProgramGrid) -> Result<()> {
        let mut start_positions = Vec::new();

        for (coord, cell) in grid.iter() {
            if cell.symbol == '@' {
                start_positions.push(*coord);
            }
        }

        match start_positions.len() {
            0 => {
                let error = InterpreterError::enhanced(
                    "No start symbol (@) found in program".to_string(),
                    ErrorType::Initialization
                ).with_suggestions(vec![
                    "Add a start symbol '@' to your program".to_string(),
                    "The start symbol should be placed where you want execution to begin".to_string(),
                ]).with_help("Every Tubular program needs exactly one start symbol '@' to indicate where execution should begin.".to_string());

                if self.collect_errors {
                    self.errors.push(error);
                    return Ok(());
                } else {
                    return Err(error);
                }
            }
            1 => Ok(()),
            _ => {
                let error = InterpreterError::enhanced(
                    format!("Multiple start symbols (@) found: {} start symbols", start_positions.len()),
                    ErrorType::Initialization
                ).with_suggestions(vec![
                    "Remove all but one start symbol '@'".to_string(),
                    "Choose the location where you want execution to begin".to_string(),
                ]).with_help("A Tubular program can only have one start symbol '@'. Multiple start symbols create ambiguity about where execution should begin.".to_string());

                // Add context for each extra start symbol
                if let Some(ref context) = self.parse_context {
                    for (i, &coord) in start_positions.iter().enumerate() {
                        let position = context.position_from_coordinate(coord);
                        let error_context = context.create_error_context(position, None);

                        let extra_start_error = InterpreterError::enhanced(
                            format!("Additional start symbol #{}", i + 1),
                            ErrorType::Initialization
                        ).with_context(error_context);

                        if self.collect_errors {
                            self.errors.push(extra_start_error);
                        }
                    }
                }

                if self.collect_errors {
                    self.errors.push(error);
                    return Ok(());
                } else {
                    return Err(error);
                }
            }
        }
    }

    fn create_error_context_for_coord(&self, coord: Coordinate) -> ErrorContext {
        let position = Position::new(coord.y as usize, coord.x as usize, coord);
        self.parse_context
            .as_ref()
            .map(|ctx| ctx.create_error_context(position.clone(), None))
            .unwrap_or_else(|| ErrorContext::new(position, "".to_string()))
    }

    fn enhance_error(&self, error: InitError, context: ErrorContext) -> InterpreterError {
        let (message, suggestions, help) = match &error {
            InitError::InvalidCharacter(ch, _) => (
                format!("Invalid character '{}' in program", ch),
                vec![
                    format!("Remove the '{}' character", ch),
                    "Check the Tubular language specification for valid symbols".to_string(),
                ],
                Some("Only valid Tubular symbols are allowed in the program.".to_string()),
            ),
            InitError::GridSizeExceeded(width, height) => (
                format!("Program grid too large: {}x{} (max 1000x1000)", width, height),
                vec![
                    "Reduce the program size".to_string(),
                    "Split the program into smaller modules if possible".to_string(),
                ],
                Some("The Tubular interpreter has a maximum grid size limit of 1000x1000 characters.".to_string()),
            ),
            _ => (
                format!("Initialization error: {}", error),
                vec![],
                None,
            ),
        };

        InterpreterError::enhanced(message, ErrorType::Initialization)
            .with_context(context)
            .with_suggestions(suggestions)
            .with_help(help.unwrap_or_default())
    }

    fn enhance_error_for_interpreter_error(&self, error: InterpreterError, context: ErrorContext) -> InterpreterError {
        match error {
            InterpreterError::Initialization(init_err) => self.enhance_error(init_err, context),
            _ => error.with_context(context),
        }
    }

    pub fn parse_reader<R: Read>(&self, reader: &mut R) -> Result<ProgramGrid> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        self.parse_string(&content)
    }

    pub fn parse_from_stdin() -> Result<ProgramGrid> {
        let parser = GridParser::new();
        let mut stdin = io::stdin();
        parser.parse_reader(&mut stdin)
    }

    pub fn parse_with_origin(&self, content: &str, origin_x: isize, origin_y: isize) -> Result<ProgramGrid> {
        let lines: Vec<&str> = content.lines().collect();
        let mut grid = ProgramGrid::new();

        for (y_offset, line) in lines.iter().enumerate() {
            for (x_offset, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }

                let coord = Coordinate::new(
                    origin_x + x_offset as isize,
                    origin_y + y_offset as isize
                );
                grid.add_cell(coord, ch)?;
            }
        }

        Ok(grid)
    }

    pub fn validate_content(&self, content: &str) -> Result<()> {
        let grid = self.parse_string(content)?;
        grid.validate()
    }

    pub fn extract_symbols(&self, content: &str) -> Vec<(Coordinate, char)> {
        let mut symbols = Vec::new();

        for (y, line) in content.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }
                symbols.push((Coordinate::new(x as isize, y as isize), ch));
            }
        }

        symbols
    }

    pub fn count_symbols(&self, content: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for line in content.lines() {
            for ch in line.chars() {
                if ch.is_whitespace() {
                    continue;
                }
                *counts.entry(ch).or_insert(0) += 1;
            }
        }

        counts
    }

    pub fn find_symbol(&self, content: &str, symbol: char) -> Vec<Coordinate> {
        let mut positions = Vec::new();

        for (y, line) in content.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == symbol {
                    positions.push(Coordinate::new(x as isize, y as isize));
                }
            }
        }

        positions
    }

    pub fn get_dimensions(&self, content: &str) -> (usize, usize) {
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);
        (width, height)
    }

    pub fn normalize_coordinates(&self, content: &str) -> (ProgramGrid, Coordinate) {
        let lines: Vec<&str> = content.lines().collect();
        let mut grid = ProgramGrid::new();
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;

        // Find the minimum coordinates first
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }
                min_x = min_x.min(x as isize);
                min_y = min_y.min(y as isize);
            }
        }

        // Add cells with normalized coordinates
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }
                let coord = Coordinate::new(
                    x as isize - min_x,
                    y as isize - min_y
                );
                grid.add_cell(coord, ch).unwrap(); // Should never fail in this context
            }
        }

        (grid, Coordinate::new(min_x, min_y))
    }
}

impl Default for GridParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_grid() {
        let parser = GridParser::new();
        let content = "@-\n!";
        let grid = parser.parse_string(content).unwrap();

        assert!(grid.start.is_some());
        assert_eq!(grid.size(), 2);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let parser = GridParser::new();
        let content = "  @  \n  -  \n  !  ";
        let grid = parser.parse_string(content).unwrap();

        assert!(grid.start.is_some());
        assert_eq!(grid.size(), 3);
    }
}