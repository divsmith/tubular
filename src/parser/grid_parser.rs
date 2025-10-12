use crate::interpreter::grid::{ProgramGrid, ProgramCell};
use crate::types::coordinate::Coordinate;
use crate::types::error::{Result, InitError};
use std::io::{self, Read};

pub struct GridParser;

impl GridParser {
    pub fn new() -> Self {
        GridParser
    }

    pub fn parse_file(&self, file_path: &str) -> Result<ProgramGrid> {
        let content = std::fs::read_to_string(file_path)?;
        self.parse_string(&content)
    }

    pub fn parse_string(&self, content: &str) -> Result<ProgramGrid> {
        let lines: Vec<&str> = content.lines().collect();
        self.parse_lines(&lines)
    }

    pub fn parse_lines(&self, lines: &[&str]) -> Result<ProgramGrid> {
        let mut grid = ProgramGrid::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    continue;
                }

                let coord = Coordinate::new(x as isize, y as isize);
                grid.add_cell(coord, ch)?;
            }
        }

        Ok(grid)
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

use std::collections::HashMap;

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