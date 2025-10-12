use std::cmp::{Eq, PartialEq};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }

    pub fn origin() -> Self {
        Coordinate::new(0, 0)
    }

    pub fn offset(&self, dx: isize, dy: isize) -> Coordinate {
        Coordinate::new(self.x + dx, self.y + dy)
    }

    pub fn manhattan_distance(&self, other: &Coordinate) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Coordinate {
        self.offset(direction.dx(), direction.dy())
    }
}

impl std::ops::Sub<Direction> for Coordinate {
    type Output = Coordinate;

    fn sub(self, direction: Direction) -> Coordinate {
        self.offset(-direction.dx(), -direction.dy())
    }
}

use crate::types::direction::Direction;