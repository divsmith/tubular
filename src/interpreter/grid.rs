use crate::types::coordinate::Coordinate;
use crate::types::error::{Result, InitError};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProgramCell {
    /// Character at this position
    pub symbol: char,
    /// Whether this cell affects droplet movement
    pub is_flow_control: bool,
    /// Whether this cell performs an operation
    pub is_operator: bool,
}

impl ProgramCell {
    pub fn new(symbol: char) -> Self {
        let is_flow_control = Self::is_flow_control_symbol(symbol);
        let is_operator = Self::is_operator_symbol(symbol);

        ProgramCell {
            symbol,
            is_flow_control,
            is_operator,
        }
    }

    pub fn is_flow_control_symbol(symbol: char) -> bool {
        matches!(symbol, '|' | '-' | '/' | '\\' | '^' | 'v' | '<' | '>')
    }

    pub fn is_operator_symbol(symbol: char) -> bool {
        matches!(symbol,
            '+' | '~' | ':' | ';' | 'd' | 'A' | 'S' | 'M' | 'D' | '=' | '<' | '>' | '%' |
            'G' | 'P' | 'C' | 'R' | '!' | ',' | 'n' | '?' | '0'..='9'
        )
    }

    pub fn is_start_symbol(symbol: char) -> bool {
        symbol == '@'
    }

    pub fn is_sink_symbol(symbol: char) -> bool {
        symbol == '!'
    }

    pub fn is_data_source(symbol: char) -> bool {
        matches!(symbol, '0'..='9' | '>' | '?')
    }

    pub fn is_data_sink(symbol: char) -> bool {
        matches!(symbol, '!' | ',' | 'n')
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min_x: isize,
    pub min_y: isize,
    pub max_x: isize,
    pub max_y: isize,
}

impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            min_x: isize::MAX,
            min_y: isize::MAX,
            max_x: isize::MIN,
            max_y: isize::MIN,
        }
    }

    pub fn include(&mut self, coord: Coordinate) {
        self.min_x = self.min_x.min(coord.x);
        self.min_y = self.min_y.min(coord.y);
        self.max_x = self.max_x.max(coord.x);
        self.max_y = self.max_y.max(coord.y);
    }

    pub fn width(&self) -> usize {
        if self.min_x > self.max_x {
            0
        } else {
            (self.max_x - self.min_x + 1) as usize
        }
    }

    pub fn height(&self) -> usize {
        if self.min_y > self.max_y {
            0
        } else {
            (self.max_y - self.min_y + 1) as usize
        }
    }

    pub fn contains(&self, coord: Coordinate) -> bool {
        coord.x >= self.min_x && coord.x <= self.max_x &&
        coord.y >= self.min_y && coord.y <= self.max_y
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ProgramGrid {
    /// Sparse representation of program cells
    pub cells: HashMap<Coordinate, ProgramCell>,
    /// Bounding box of active program area
    pub bounds: BoundingBox,
    /// Start symbol location (must be exactly one)
    pub start: Option<Coordinate>,
}

impl ProgramGrid {
    pub fn new() -> Self {
        ProgramGrid {
            cells: HashMap::new(),
            bounds: BoundingBox::new(),
            start: None,
        }
    }

    pub fn add_cell(&mut self, coord: Coordinate, symbol: char) -> Result<()> {
        if !symbol.is_ascii() {
            return Err(InitError::InvalidCharacter(symbol, coord).into());
        }

        let cell = ProgramCell::new(symbol);

        if ProgramCell::is_start_symbol(symbol) {
            if self.start.is_some() {
                return Err(InitError::MultipleStartSymbols.into());
            }
            self.start = Some(coord);
        }

        self.cells.insert(coord, cell);
        self.bounds.include(coord);

        Ok(())
    }

    pub fn get(&self, coord: Coordinate) -> Option<&ProgramCell> {
        self.cells.get(&coord)
    }

    pub fn get_symbol(&self, coord: Coordinate) -> Option<char> {
        self.cells.get(&coord).map(|cell| cell.symbol)
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.bounds.width(), self.bounds.height())
    }

    pub fn validate(&self) -> Result<()> {
        if self.start.is_none() {
            return Err(InitError::NoStartSymbol.into());
        }

        let (width, height) = self.dimensions();
        if width > 1000 || height > 1000 {
            return Err(InitError::GridSizeExceeded(width, height).into());
        }

        // Validate all symbols are valid
        for (coord, cell) in &self.cells {
            if !ProgramCell::is_flow_control_symbol(cell.symbol) &&
               !ProgramCell::is_operator_symbol(cell.symbol) &&
               !ProgramCell::is_start_symbol(cell.symbol) &&
               !cell.symbol.is_whitespace() {
                return Err(InitError::InvalidCharacter(cell.symbol, *coord).into());
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coordinate, &ProgramCell)> {
        self.cells.iter()
    }

    pub fn symbols_in_bounds(&self) -> Vec<String> {
        let width = self.bounds.width();
        let height = self.bounds.height();
        let mut lines = Vec::new();

        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                let coord = Coordinate::new(
                    self.bounds.min_x + x as isize,
                    self.bounds.min_y + y as isize
                );
                if let Some(cell) = self.get(coord) {
                    line.push(cell.symbol);
                } else {
                    line.push(' ');
                }
            }
            lines.push(line);
        }

        lines
    }
}

impl Default for ProgramGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProgramGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = self.symbols_in_bounds();
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}