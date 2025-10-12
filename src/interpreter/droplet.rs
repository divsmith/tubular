use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use crate::types::bigint::TubularBigInt;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Droplet {
    /// Unique identifier for tracking
    pub id: DropletId,
    /// Current integer value (arbitrary precision)
    pub value: TubularBigInt,
    /// Current position in the grid
    pub position: Coordinate,
    /// Current direction of movement
    pub direction: Direction,
    /// Whether this droplet is active (will move next tick)
    pub active: bool,
}

pub type DropletId = u64;

impl Droplet {
    pub fn new(id: DropletId, position: Coordinate, direction: Direction) -> Self {
        Droplet {
            id,
            value: TubularBigInt::zero(),
            position,
            direction,
            active: true,
        }
    }

    pub fn with_value(id: DropletId, value: TubularBigInt, position: Coordinate, direction: Direction) -> Self {
        Droplet {
            id,
            value,
            position,
            direction,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn move_to(&mut self, new_position: Coordinate) {
        self.position = new_position;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_value(&mut self, value: TubularBigInt) {
        self.value = value;
    }

    pub fn next_position(&self) -> Coordinate {
        self.position + self.direction
    }

    pub fn will_collide_with(&self, other: &Droplet) -> bool {
        if !self.active || !other.active {
            return false;
        }
        self.next_position() == other.next_position()
    }

    pub fn is_at_same_position(&self, other: &Droplet) -> bool {
        self.position == other.position
    }
}

impl fmt::Display for Droplet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Droplet(id={}, value={}, pos={}, dir={})",
               self.id, self.value, self.position, self.direction)
    }
}

impl PartialEq for Droplet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Droplet {}

impl std::hash::Hash for Droplet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}