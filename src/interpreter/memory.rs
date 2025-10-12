use crate::types::coordinate::Coordinate;
use crate::types::bigint::TubularBigInt;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReservoirCoordinate {
    /// X coordinate (can be negative)
    pub x: isize,
    /// Y coordinate (can be negative)
    pub y: isize,
}

impl ReservoirCoordinate {
    pub fn new(x: isize, y: isize) -> Self {
        ReservoirCoordinate { x, y }
    }

    pub fn from_program_coordinate(coord: Coordinate) -> Self {
        ReservoirCoordinate::new(coord.x, coord.y)
    }

    pub fn to_program_coordinate(&self) -> Coordinate {
        Coordinate::new(self.x, self.y)
    }
}

impl From<Coordinate> for ReservoirCoordinate {
    fn from(coord: Coordinate) -> Self {
        Self::from_program_coordinate(coord)
    }
}

impl From<ReservoirCoordinate> for Coordinate {
    fn from(coord: ReservoirCoordinate) -> Self {
        coord.to_program_coordinate()
    }
}

#[derive(Debug, Clone)]
pub struct Reservoir {
    /// Sparse storage for memory cells
    pub data: HashMap<ReservoirCoordinate, TubularBigInt>,
}

impl Reservoir {
    pub fn new() -> Self {
        Reservoir {
            data: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Reservoir {
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, coord: ReservoirCoordinate) -> TubularBigInt {
        self.data.get(&coord).cloned().unwrap_or_else(|| TubularBigInt::zero())
    }

    pub fn put(&mut self, coord: ReservoirCoordinate, value: TubularBigInt) -> TubularBigInt {
        self.data.insert(coord, value.clone());
        value
    }

    pub fn get_or_zero(&self, coord: ReservoirCoordinate) -> TubularBigInt {
        self.get(coord)
    }

    pub fn contains(&self, coord: &ReservoirCoordinate) -> bool {
        self.data.contains_key(coord)
    }

    pub fn remove(&mut self, coord: &ReservoirCoordinate) -> Option<TubularBigInt> {
        self.data.remove(coord)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ReservoirCoordinate, &TubularBigInt)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ReservoirCoordinate, &mut TubularBigInt)> {
        self.data.iter_mut()
    }

    pub fn keys(&self) -> impl Iterator<Item = &ReservoirCoordinate> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &TubularBigInt> {
        self.data.values()
    }

    pub fn into_values(self) -> impl Iterator<Item = TubularBigInt> {
        self.data.into_values()
    }

    pub fn get_adjacent(&self, coord: ReservoirCoordinate) -> [(TubularBigInt, ReservoirCoordinate); 8] {
        [
            (self.get(ReservoirCoordinate::new(coord.x - 1, coord.y - 1)), ReservoirCoordinate::new(coord.x - 1, coord.y - 1)),
            (self.get(ReservoirCoordinate::new(coord.x, coord.y - 1)), ReservoirCoordinate::new(coord.x, coord.y - 1)),
            (self.get(ReservoirCoordinate::new(coord.x + 1, coord.y - 1)), ReservoirCoordinate::new(coord.x + 1, coord.y - 1)),
            (self.get(ReservoirCoordinate::new(coord.x - 1, coord.y)), ReservoirCoordinate::new(coord.x - 1, coord.y)),
            (self.get(ReservoirCoordinate::new(coord.x + 1, coord.y)), ReservoirCoordinate::new(coord.x + 1, coord.y)),
            (self.get(ReservoirCoordinate::new(coord.x - 1, coord.y + 1)), ReservoirCoordinate::new(coord.x - 1, coord.y + 1)),
            (self.get(ReservoirCoordinate::new(coord.x, coord.y + 1)), ReservoirCoordinate::new(coord.x, coord.y + 1)),
            (self.get(ReservoirCoordinate::new(coord.x + 1, coord.y + 1)), ReservoirCoordinate::new(coord.x + 1, coord.y + 1)),
        ]
    }

    pub fn bounding_box(&self) -> Option<(ReservoirCoordinate, ReservoirCoordinate)> {
        if self.data.is_empty() {
            return None;
        }

        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        for coord in self.data.keys() {
            min_x = min_x.min(coord.x);
            min_y = min_y.min(coord.y);
            max_x = max_x.max(coord.x);
            max_y = max_y.max(coord.y);
        }

        Some((
            ReservoirCoordinate::new(min_x, min_y),
            ReservoirCoordinate::new(max_x, max_y)
        ))
    }

    pub fn count_non_zero(&self) -> usize {
        self.data.values()
            .filter(|value| !value.is_zero())
            .count()
    }

    pub fn filter_zero_values(&mut self) {
        self.data.retain(|_, value| !value.is_zero());
    }
}

impl Default for Reservoir {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Reservoir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Reservoir ({} cells):", self.len())?;
        if let Some((min, max)) = self.bounding_box() {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let coord = ReservoirCoordinate::new(x, y);
                    if let Some(value) = self.data.get(&coord) {
                        write!(f, "{} ", value)?;
                    } else {
                        write!(f, "0 ")?;
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl From<HashMap<ReservoirCoordinate, TubularBigInt>> for Reservoir {
    fn from(data: HashMap<ReservoirCoordinate, TubularBigInt>) -> Self {
        Reservoir { data }
    }
}