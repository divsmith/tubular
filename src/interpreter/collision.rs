use crate::interpreter::droplet::Droplet;
use crate::types::coordinate::Coordinate;
use std::collections::{HashMap, HashSet};

/// Collision detection and handling for droplets
pub struct CollisionDetector;

impl CollisionDetector {
    /// Create a new collision detector
    pub fn new() -> Self {
        CollisionDetector
    }

    /// Detect collisions between droplets moving to the same position
    pub fn detect_collisions(&self, droplets: &[Droplet]) -> HashSet<Coordinate> {
        let mut position_map: HashMap<Coordinate, Vec<usize>> = HashMap::new();
        let mut collision_positions = HashSet::new();

        // Map next positions to droplet indices
        for (index, droplet) in droplets.iter().enumerate() {
            if !droplet.is_active() {
                continue;
            }

            let next_pos = droplet.next_position();
            position_map.entry(next_pos).or_default().push(index);
        }

        // Find positions with multiple droplets (collisions)
        for (position, droplet_indices) in position_map {
            if droplet_indices.len() > 1 {
                collision_positions.insert(position);
            }
        }

        collision_positions
    }

    /// Get all droplets that would collide at the given positions
    pub fn get_colliding_droplets(&self, droplets: &[Droplet], collision_positions: &HashSet<Coordinate>) -> Vec<usize> {
        let mut colliding_droplets = Vec::new();

        for (index, droplet) in droplets.iter().enumerate() {
            if !droplet.is_active() {
                continue;
            }

            if collision_positions.contains(&droplet.next_position()) {
                colliding_droplets.push(index);
            }
        }

        colliding_droplets
    }

    /// Check if a specific droplet would collide with another
    pub fn will_collide(&self, droplet: &Droplet, other_droplets: &[Droplet]) -> bool {
        if !droplet.is_active() {
            return false;
        }

        let next_pos = droplet.next_position();

        for other in other_droplets {
            if !other.is_active() || other.id == droplet.id {
                continue;
            }

            if other.next_position() == next_pos {
                return true;
            }
        }

        false
    }
}

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new()
    }
}