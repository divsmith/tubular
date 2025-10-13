//! Unit tests for the Direction type

use tubular::types::direction::Direction;

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn test_direction_dx() {
        assert_eq!(Direction::Up.dx(), 0);
        assert_eq!(Direction::Down.dx(), 0);
        assert_eq!(Direction::Left.dx(), -1);
        assert_eq!(Direction::Right.dx(), 1);
    }

    #[test]
    fn test_direction_dy() {
        assert_eq!(Direction::Up.dy(), -1);
        assert_eq!(Direction::Down.dy(), 1);
        assert_eq!(Direction::Left.dy(), 0);
        assert_eq!(Direction::Right.dy(), 0);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn test_opposite_roundtrip() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for direction in &directions {
            assert_eq!(direction.opposite().opposite(), *direction);
        }
    }

    #[test]
    fn test_turn_left() {
        assert_eq!(Direction::Up.turn_left(), Direction::Left);
        assert_eq!(Direction::Down.turn_left(), Direction::Right);
        assert_eq!(Direction::Left.turn_left(), Direction::Down);
        assert_eq!(Direction::Right.turn_left(), Direction::Up);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(Direction::Up.turn_right(), Direction::Right);
        assert_eq!(Direction::Down.turn_right(), Direction::Left);
        assert_eq!(Direction::Left.turn_right(), Direction::Up);
        assert_eq!(Direction::Right.turn_right(), Direction::Down);
    }

    #[test]
    fn test_turn_consistency() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for direction in &directions {
            // Left turn followed by right turn should return to original
            assert_eq!(direction.turn_left().turn_right(), *direction);
            assert_eq!(direction.turn_right().turn_left(), *direction);

            // Two left turns should equal two right turns (opposite direction)
            assert_eq!(direction.turn_left().turn_left(), direction.turn_right().turn_right());
            assert_eq!(direction.turn_left().turn_left(), direction.opposite());
        }
    }

    #[test]
    fn test_full_rotation() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for direction in &directions {
            // Four left turns should return to original
            assert_eq!(
                direction.turn_left().turn_left().turn_left().turn_left(),
                *direction
            );

            // Four right turns should return to original
            assert_eq!(
                direction.turn_right().turn_right().turn_right().turn_right(),
                *direction
            );
        }
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Direction::from_char('^'), Some(Direction::Up));
        assert_eq!(Direction::from_char('v'), Some(Direction::Down));
        assert_eq!(Direction::from_char('<'), Some(Direction::Left));
        assert_eq!(Direction::from_char('>'), Some(Direction::Right));

        // Test invalid characters
        assert_eq!(Direction::from_char('a'), None);
        assert_eq!(Direction::from_char('1'), None);
        assert_eq!(Direction::from_char(' '), None);
        assert_eq!(Direction::from_char('\n'), None);
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::Up), "^");
        assert_eq!(format!("{}", Direction::Down), "v");
        assert_eq!(format!("{}", Direction::Left), "<");
        assert_eq!(format!("{}", Direction::Right), ">");
    }

    #[test]
    fn test_direction_equality() {
        assert_eq!(Direction::Up, Direction::Up);
        assert_eq!(Direction::Down, Direction::Down);
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);

        assert_ne!(Direction::Up, Direction::Down);
        assert_ne!(Direction::Left, Direction::Right);
        assert_ne!(Direction::Up, Direction::Left);
        assert_ne!(Direction::Down, Direction::Right);
    }

    #[test]
    fn test_direction_copy_and_clone() {
        let dir1 = Direction::Up;
        let dir2 = dir1;
        let dir3 = dir1.clone();

        assert_eq!(dir1, dir2);
        assert_eq!(dir1, dir3);
    }

    #[test]
    fn test_direction_debug() {
        let dir = Direction::Up;
        let debug_str = format!("{:?}", dir);
        assert!(debug_str.contains("Up"));
    }

    #[test]
    fn test_direction_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Direction::Up);
        set.insert(Direction::Down);
        set.insert(Direction::Left);
        set.insert(Direction::Right);

        // All directions should be unique in the set
        assert_eq!(set.len(), 4);
        assert!(set.contains(&Direction::Up));
        assert!(set.contains(&Direction::Down));
        assert!(set.contains(&Direction::Left));
        assert!(set.contains(&Direction::Right));
    }

    #[test]
    fn test_all_directions_distinct() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for (i, dir1) in directions.iter().enumerate() {
            for (j, dir2) in directions.iter().enumerate() {
                if i != j {
                    assert_ne!(dir1, dir2, "Directions at indices {} and {} should be different", i, j);
                }
            }
        }
    }

    #[test]
    fn test_direction_iter_properties() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        // Test that we can iterate over all directions and count unique dx/dy combinations
        let mut dx_dy_set = std::collections::HashSet::new();
        for direction in &directions {
            dx_dy_set.insert((direction.dx(), direction.dy()));
        }
        assert_eq!(dx_dy_set.len(), 4);
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_opposite_consistency(direction in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])) {
        // Opposite of opposite should be original
        assert_eq!(direction.opposite().opposite(), direction);

        // Original should not equal opposite (except in pathological cases)
        assert_ne!(direction, direction.opposite());
    }

    #[test]
    fn test_turn_properties(direction in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])) {
        // Left turn followed by right turn should return to original
        assert_eq!(direction.turn_left().turn_right(), direction);
        assert_eq!(direction.turn_right().turn_left(), direction);

        // Two left turns should equal two right turns
        assert_eq!(direction.turn_left().turn_left(), direction.turn_right().turn_right());
    }

    #[test]
    fn test_dx_dy_properties(direction in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])) {
        let dx = direction.dx();
        let dy = direction.dy();

        // At least one of dx or dy should be zero
        assert!(dx == 0 || dy == 0);

        // The non-zero component should be ±1
        if dx != 0 {
            assert_eq!(dx.abs(), 1);
        }
        if dy != 0 {
            assert_eq!(dy.abs(), 1);
        }

        // Both components shouldn't be zero
        assert!(dx != 0 || dy != 0);
    }

    #[test]
    fn test_opposite_dx_dy(direction in prop::sample::select(&[Direction::Up, Direction::Down, Direction::Left, Direction::Right])) {
        let opposite = direction.opposite();

        // Opposite direction should have negated dx and dy
        assert_eq!(direction.dx(), -opposite.dx());
        assert_eq!(direction.dy(), -opposite.dy());
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_direction_operations() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let start = Instant::now();

        for _ in 0..10_000_000 {
            for direction in &directions {
                let _dx = direction.dx();
                let _dy = direction.dy();
                let _opp = direction.opposite();
                let _left = direction.turn_left();
                let _right = direction.turn_right();
            }
        }

        let duration = start.elapsed();
        println!("Direction operations (40M): {:?}", duration);
        assert!(duration.as_millis() < 500); // Should be very fast
    }

    #[test]
    fn benchmark_from_char() {
        let chars = ['^', 'v', '<', '>', 'a', 'b', 'c'];
        let start = Instant::now();

        for _ in 0..1_000_000 {
            for &ch in &chars {
                let _direction = Direction::from_char(ch);
            }
        }

        let duration = start.elapsed();
        println!("Direction::from_char (7M): {:?}", duration);
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn benchmark_display() {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let start = Instant::now();

        for _ in 0..1_000_000 {
            for direction in &directions {
                let _string = format!("{}", direction);
            }
        }

        let duration = start.elapsed();
        println!("Direction display (4M): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }
}