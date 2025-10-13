//! Unit tests for the Droplet type

use tubular::interpreter::droplet::{Droplet, DropletId};
use tubular::types::Coordinate;
use tubular::types::direction::Direction;
use tubular::types::bigint::TubularBigInt;
use proptest::prelude::*;

#[cfg(test)]
mod droplet_tests {
    use super::*;

    #[test]
    fn test_droplet_new() {
        let id = 1;
        let position = Coordinate::new(5, 10);
        let direction = Direction::Right;

        let droplet = Droplet::new(id, position, direction);

        assert_eq!(droplet.id, id);
        assert_eq!(droplet.position, position);
        assert_eq!(droplet.direction, direction);
        assert_eq!(droplet.value, TubularBigInt::zero());
        assert!(droplet.is_active());
    }

    #[test]
    fn test_droplet_with_value() {
        let id = 2;
        let value = TubularBigInt::new(42);
        let position = Coordinate::new(0, 0);
        let direction = Direction::Down;

        let droplet = Droplet::with_value(id, value.clone(), position, direction);

        assert_eq!(droplet.id, id);
        assert_eq!(droplet.value, value);
        assert_eq!(droplet.position, position);
        assert_eq!(droplet.direction, direction);
        assert!(droplet.is_active());
    }

    #[test]
    fn test_droplet_is_active() {
        let droplet = Droplet::new(1, Coordinate::origin(), Direction::Up);
        assert!(droplet.is_active());
    }

    #[test]
    fn test_droplet_deactivate() {
        let mut droplet = Droplet::new(1, Coordinate::origin(), Direction::Up);
        assert!(droplet.is_active());

        droplet.deactivate();
        assert!(!droplet.is_active());
    }

    #[test]
    fn test_droplet_move_to() {
        let mut droplet = Droplet::new(1, Coordinate::new(0, 0), Direction::Right);
        let new_position = Coordinate::new(5, 10);

        droplet.move_to(new_position);
        assert_eq!(droplet.position, new_position);
    }

    #[test]
    fn test_droplet_set_direction() {
        let mut droplet = Droplet::new(1, Coordinate::origin(), Direction::Up);
        let new_direction = Direction::Left;

        droplet.set_direction(new_direction);
        assert_eq!(droplet.direction, new_direction);
    }

    #[test]
    fn test_droplet_set_value() {
        let mut droplet = Droplet::new(1, Coordinate::origin(), Direction::Up);
        let new_value = TubularBigInt::new(123);

        droplet.set_value(new_value.clone());
        assert_eq!(droplet.value, new_value);
    }

    #[test]
    fn test_droplet_next_position() {
        let position = Coordinate::new(5, 5);
        let droplet = Droplet::new(1, position, Direction::Right);

        let next = droplet.next_position();
        assert_eq!(next, Coordinate::new(6, 5));

        let droplet_up = Droplet::new(2, position, Direction::Up);
        let next_up = droplet_up.next_position();
        assert_eq!(next_up, Coordinate::new(5, 4));

        let droplet_down = Droplet::new(3, position, Direction::Down);
        let next_down = droplet_down.next_position();
        assert_eq!(next_down, Coordinate::new(5, 6));

        let droplet_left = Droplet::new(4, position, Direction::Left);
        let next_left = droplet_left.next_position();
        assert_eq!(next_left, Coordinate::new(4, 5));
    }

    #[test]
    fn test_droplet_will_collide_with() {
        let pos1 = Coordinate::new(0, 0);
        let pos2 = Coordinate::new(2, 0);
        let direction = Direction::Right;

        let mut droplet1 = Droplet::new(1, pos1, direction);
        let mut droplet2 = Droplet::new(2, pos2, direction);

        // Should collide at (1, 0) -> (2, 0) and (3, 0) -> (2, 0) respectively
        assert!(droplet1.will_collide_with(&droplet2));

        // Inactive droplets don't collide
        droplet2.deactivate();
        assert!(!droplet1.will_collide_with(&droplet2));

        droplet2 = Droplet::new(2, pos2, Direction::Left);
        assert!(!droplet1.will_collide_with(&droplet2)); // Moving away from each other

        // Different directions, same next position
        let droplet3 = Droplet::new(3, Coordinate::new(1, 1), Direction::Down);
        let droplet4 = Droplet::new(4, Coordinate::new(1, -1), Direction::Up);
        assert!(droplet3.will_collide_with(&droplet4));
    }

    #[test]
    fn test_droplet_is_at_same_position() {
        let position = Coordinate::new(5, 10);
        let droplet1 = Droplet::new(1, position, Direction::Up);
        let droplet2 = Droplet::new(2, position, Direction::Down);
        let droplet3 = Droplet::new(3, Coordinate::new(6, 10), Direction::Up);

        assert!(droplet1.is_at_same_position(&droplet2));
        assert!(!droplet1.is_at_same_position(&droplet3));
    }

    #[test]
    fn test_droplet_display() {
        let droplet = Droplet::new(1, Coordinate::new(5, 10), Direction::Right);
        let display_str = format!("{}", droplet);

        assert!(display_str.contains("Droplet"));
        assert!(display_str.contains("id=1"));
        assert!(display_str.contains("pos=(5, 10)"));
        assert!(display_str.contains("dir=>"));
        assert!(display_str.contains("value=0"));
    }

    #[test]
    fn test_droplet_equality() {
        let position = Coordinate::origin();
        let direction = Direction::Up;

        let droplet1 = Droplet::new(1, position, direction);
        let droplet2 = Droplet::new(1, position, direction);
        let droplet3 = Droplet::new(2, position, direction);

        assert_eq!(droplet1, droplet2);
        assert_ne!(droplet1, droplet3);

        // Different positions but same ID should be equal
        let droplet4 = Droplet::new(1, Coordinate::new(5, 10), direction);
        assert_eq!(droplet1, droplet4);
    }

    #[test]
    fn test_droplet_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let droplet1 = Droplet::new(1, Coordinate::origin(), Direction::Up);
        let droplet2 = Droplet::new(2, Coordinate::origin(), Direction::Up);
        let droplet3 = Droplet::new(1, Coordinate::new(5, 10), Direction::Down);

        set.insert(droplet1.clone());
        set.insert(droplet2);
        set.insert(droplet3); // Same ID as droplet1, should replace

        assert_eq!(set.len(), 2); // Only 2 unique IDs
        assert!(set.contains(&droplet1));
        assert!(set.contains(&droplet2));
    }

    #[test]
    fn test_droplet_debug() {
        let droplet = Droplet::new(1, Coordinate::new(1, 2), Direction::Right);
        let debug_str = format!("{:?}", droplet);

        assert!(debug_str.contains("Droplet"));
        assert!(debug_str.contains("id: 1"));
        assert!(debug_str.contains("value:"));
        assert!(debug_str.contains("position:"));
        assert!(debug_str.contains("direction:"));
        assert!(debug_str.contains("active:"));
    }

    #[test]
    fn test_droplet_copy_and_clone() {
        let droplet1 = Droplet::new(1, Coordinate::new(5, 10), Direction::Down);
        let droplet2 = droplet1.clone();

        assert_eq!(droplet1.id, droplet2.id);
        assert_eq!(droplet1.position, droplet2.position);
        assert_eq!(droplet1.direction, droplet2.direction);
        assert_eq!(droplet1.value, droplet2.value);
        assert_eq!(droplet1.is_active(), droplet2.is_active());
    }

    #[test]
    fn test_droplet_state_mutations() {
        let mut droplet = Droplet::new(1, Coordinate::origin(), Direction::Up);

        // Test multiple mutations
        droplet.set_value(TubularBigInt::new(100));
        droplet.set_direction(Direction::Right);
        droplet.move_to(Coordinate::new(10, 20));

        assert_eq!(droplet.value.to_i64(), Some(100));
        assert_eq!(droplet.direction, Direction::Right);
        assert_eq!(droplet.position, Coordinate::new(10, 20));
        assert!(droplet.is_active());

        // Deactivate
        droplet.deactivate();
        assert!(!droplet.is_active());
    }

    #[test]
    fn test_droplet_large_values() {
        let large_value = TubularBigInt::new(i64::MAX);
        let droplet = Droplet::with_value(
            1,
            large_value.clone(),
            Coordinate::new(isize::MAX, isize::MIN),
            Direction::Left
        );

        assert_eq!(droplet.value, large_value);
        assert_eq!(droplet.position, Coordinate::new(isize::MAX, isize::MIN));
        assert_eq!(droplet.direction, Direction::Left);
    }

    #[test]
    fn test_droplet_collision_scenarios() {
        // Head-on collision
        let d1 = Droplet::new(1, Coordinate::new(0, 0), Direction::Right);
        let d2 = Droplet::new(2, Coordinate::new(2, 0), Direction::Left);
        assert!(d1.will_collide_with(&d2));

        // Following collision
        let d3 = Droplet::new(3, Coordinate::new(0, 0), Direction::Right);
        let d4 = Droplet::new(4, Coordinate::new(1, 0), Direction::Right);
        assert!(d3.will_collide_with(&d4));

        // Intersection collision
        let d5 = Droplet::new(5, Coordinate::new(0, 1), Direction::Down);
        let d6 = Droplet::new(6, Coordinate::new(1, 0), Direction::Right);
        assert!(d5.will_collide_with(&d6));

        // No collision
        let d7 = Droplet::new(7, Coordinate::new(0, 0), Direction::Right);
        let d8 = Droplet::new(8, Coordinate::new(5, 5), Direction::Up);
        assert!(!d7.will_collide_with(&d8));
    }

    #[test]
    fn test_droplet_next_position_edge_cases() {
        // Test at coordinate boundaries
        let max_coord = Coordinate::new(isize::MAX, isize::MAX);
        let min_coord = Coordinate::new(isize::MIN, isize::MIN);

        let droplet_max = Droplet::new(1, max_coord, Direction::Right);
        let next_max = droplet_max.next_position();
        // Should overflow/wrap around (isize behavior)
        assert!(next_max.x < max_coord.x);

        let droplet_min = Droplet::new(2, min_coord, Direction::Up);
        let next_min = droplet_min.next_position();
        // Should overflow/wrap around (isize behavior)
        assert!(next_min.y > min_coord.y);
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_droplet_next_position_properties(
        x in any::<isize>(),
        y in any::<isize>(),
        direction in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])
    ) {
        let position = Coordinate::new(x, y);
        let droplet = Droplet::new(1, position, direction);
        let next = droplet.next_position();

        // Next position should be exactly one step in the direction
        let expected = position + direction;
        assert_eq!(next, expected);
    }

    #[test]
    fn test_droplet_collision_symmetry(
        x1 in any::<isize>(),
        y1 in any::<isize>(),
        x2 in any::<isize>(),
        y2 in any::<isize>(),
        dir1 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right]),
        dir2 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])
    ) {
        let pos1 = Coordinate::new(x1, y1);
        let pos2 = Coordinate::new(x2, y2);
        let droplet1 = Droplet::new(1, pos1, dir1);
        let droplet2 = Droplet::new(2, pos2, dir2);

        // Collision detection should be symmetric
        assert_eq!(droplet1.will_collide_with(&droplet2), droplet2.will_collide_with(&droplet1));
    }

    #[test]
    fn test_droplet_active_collision(
        x1 in any::<isize>(),
        y1 in any::<isize>(),
        x2 in any::<isize>(),
        y2 in any::<isize>(),
        dir1 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right]),
        dir2 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])
    ) {
        let pos1 = Coordinate::new(x1, y1);
        let pos2 = Coordinate::new(x2, y2);
        let mut droplet1 = Droplet::new(1, pos1, dir1);
        let mut droplet2 = Droplet::new(2, pos2, dir2);

        let would_collide_active = droplet1.will_collide_with(&droplet2);

        // If either droplet is inactive, no collision
        droplet1.deactivate();
        assert!(!droplet1.will_collide_with(&droplet2));

        droplet1 = Droplet::new(1, pos1, dir1); // Reset
        droplet2.deactivate();
        assert!(!droplet1.will_collide_with(&droplet2));

        // Both inactive
        droplet1.deactivate();
        assert!(!droplet1.will_collide_with(&droplet2));

        // Both active again
        droplet1 = Droplet::new(1, pos1, dir1);
        droplet2 = Droplet::new(2, pos2, dir2);
        assert_eq!(droplet1.will_collide_with(&droplet2), would_collide_active);
    }

    #[test]
    fn test_droplet_id_equality(
        id in any::<u64>(),
        x in any::<isize>(),
        y in any::<isize>(),
        dir in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])
    ) {
        let pos = Coordinate::new(x, y);
        let droplet1 = Droplet::new(id, pos, dir);
        let droplet2 = Droplet::new(id, pos, dir);

        assert_eq!(droplet1, droplet2);
        assert_eq!(droplet1.id, id);
        assert_eq!(droplet2.id, id);
    }

    #[test]
    fn test_droplet_position_same(
        x in any::<isize>(),
        y in any::<isize>(),
        dir1 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right]),
        dir2 in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])
    ) {
        let pos = Coordinate::new(x, y);
        let droplet1 = Droplet::new(1, pos, dir1);
        let droplet2 = Droplet::new(2, pos, dir2);

        assert!(droplet1.is_at_same_position(&droplet2));
        assert!(droplet2.is_at_same_position(&droplet1));
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_droplet_creation() {
        let start = Instant::now();

        for i in 0..1_000_000 {
            let _droplet = Droplet::new(
                i,
                Coordinate::new((i % 1000) as isize, (i % 1000) as isize),
                match i % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                }
            );
        }

        let duration = start.elapsed();
        println!("Droplet creation (1M): {:?}", duration);
        assert!(duration.as_millis() < 300);
    }

    #[test]
    fn benchmark_droplet_operations() {
        let mut droplets: Vec<_> = (0..10_000)
            .map(|i| Droplet::new(
                i,
                Coordinate::new(i as isize, i as isize),
                Direction::Right
            ))
            .collect();

        let start = Instant::now();

        for droplet in &mut droplets {
            droplet.set_value(TubularBigInt::new(42));
            droplet.move_to(droplet.next_position());
            droplet.set_direction(Direction::Up);
        }

        let duration = start.elapsed();
        println!("Droplet operations (30K): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_collision_detection() {
        let droplets: Vec<_> = (0..1_000)
            .map(|i| Droplet::new(
                i,
                Coordinate::new(i as isize, i as isize),
                Direction::Right
            ))
            .collect();

        let start = Instant::now();

        for i in 0..droplets.len() {
            for j in (i + 1)..droplets.len() {
                let _will_collide = droplets[i].will_collide_with(&droplets[j]);
            }
        }

        let duration = start.elapsed();
        println!("Collision detection (500K pairs): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn benchmark_next_position() {
        let droplets: Vec<_> = (0..100_000)
            .map(|i| Droplet::new(
                i,
                Coordinate::new(i as isize, i as isize),
                match i % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                }
            ))
            .collect();

        let start = Instant::now();

        for droplet in &droplets {
            let _next = droplet.next_position();
        }

        let duration = start.elapsed();
        println!("Next position calculation (100K): {:?}", duration);
        assert!(duration.as_millis() < 50);
    }
}