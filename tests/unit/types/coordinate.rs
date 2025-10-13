//! Unit tests for the Coordinate type

use tubular::types::Coordinate;
use tubular::types::direction::Direction;
use proptest::prelude::*;

#[cfg(test)]
mod coordinate_tests {
    use super::*;

    #[test]
    fn test_coordinate_new() {
        let coord = Coordinate::new(5, -3);
        assert_eq!(coord.x, 5);
        assert_eq!(coord.y, -3);
    }

    #[test]
    fn test_coordinate_origin() {
        let origin = Coordinate::origin();
        assert_eq!(origin.x, 0);
        assert_eq!(origin.y, 0);
        assert_eq!(origin, Coordinate::new(0, 0));
    }

    #[test]
    fn test_coordinate_offset() {
        let coord = Coordinate::new(2, 3);
        let offset = coord.offset(1, -2);
        assert_eq!(offset.x, 3);
        assert_eq!(offset.y, 1);

        // Original coordinate should be unchanged
        assert_eq!(coord.x, 2);
        assert_eq!(coord.y, 3);
    }

    #[test]
    fn test_coordinate_offset_zero() {
        let coord = Coordinate::new(-5, 10);
        let offset = coord.offset(0, 0);
        assert_eq!(offset, coord);
    }

    #[test]
    fn test_coordinate_offset_negative() {
        let coord = Coordinate::new(1, 1);
        let offset = coord.offset(-2, -3);
        assert_eq!(offset.x, -1);
        assert_eq!(offset.y, -2);
    }

    #[test]
    fn test_manhattan_distance() {
        let coord1 = Coordinate::new(0, 0);
        let coord2 = Coordinate::new(3, 4);
        assert_eq!(coord1.manhattan_distance(&coord2), 7);

        let coord3 = Coordinate::new(-2, -3);
        assert_eq!(coord1.manhattan_distance(&coord3), 5);

        let coord4 = Coordinate::new(5, -2);
        assert_eq!(coord3.manhattan_distance(&coord4), 12);
    }

    #[test]
    fn test_manhattan_distance_same_coordinate() {
        let coord = Coordinate::new(10, -5);
        assert_eq!(coord.manhattan_distance(&coord), 0);
    }

    #[test]
    fn test_coordinate_display() {
        let coord = Coordinate::new(5, -3);
        assert_eq!(format!("{}", coord), "(5, -3)");

        let origin = Coordinate::origin();
        assert_eq!(format!("{}", origin), "(0, 0)");

        let negative = Coordinate::new(-10, 20);
        assert_eq!(format!("{}", negative), "(-10, 20)");
    }

    #[test]
    fn test_coordinate_add_direction() {
        let coord = Coordinate::new(2, 2);

        // Test adding each direction
        let up = coord + Direction::Up;
        assert_eq!(up, Coordinate::new(2, 1));

        let down = coord + Direction::Down;
        assert_eq!(down, Coordinate::new(2, 3));

        let left = coord + Direction::Left;
        assert_eq!(left, Coordinate::new(1, 2));

        let right = coord + Direction::Right;
        assert_eq!(right, Coordinate::new(3, 2));
    }

    #[test]
    fn test_coordinate_sub_direction() {
        let coord = Coordinate::new(2, 2);

        // Test subtracting each direction
        let up = coord - Direction::Up;
        assert_eq!(up, Coordinate::new(2, 3));

        let down = coord - Direction::Down;
        assert_eq!(down, Coordinate::new(2, 1));

        let left = coord - Direction::Left;
        assert_eq!(left, Coordinate::new(3, 2));

        let right = coord - Direction::Right;
        assert_eq!(right, Coordinate::new(1, 2));
    }

    #[test]
    fn test_coordinate_chained_operations() {
        let coord = Coordinate::origin()
            .offset(5, 3)
            .offset(-2, 1);
        assert_eq!(coord, Coordinate::new(3, 4));
    }

    #[test]
    fn test_coordinate_large_values() {
        let coord = Coordinate::new(isize::MAX, isize::MIN);
        assert_eq!(coord.x, isize::MAX);
        assert_eq!(coord.y, isize::MIN);

        // Test manhattan distance with large values
        let origin = Coordinate::origin();
        let distance = origin.manhattan_distance(&coord);
        assert_eq!(distance, (isize::MAX as usize) + (isize::MIN as usize).abs());
    }

    #[test]
    fn test_coordinate_equality() {
        let coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(1, 2);
        let coord3 = Coordinate::new(2, 1);

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }

    #[test]
    fn test_coordinate_copy_and_clone() {
        let coord1 = Coordinate::new(5, 10);
        let coord2 = coord1;
        let coord3 = coord1.clone();

        assert_eq!(coord1, coord2);
        assert_eq!(coord1, coord3);
    }

    #[test]
    fn test_coordinate_debug() {
        let coord = Coordinate::new(1, 2);
        let debug_str = format!("{:?}", coord);
        assert!(debug_str.contains("Coordinate"));
        assert!(debug_str.contains("x: 1"));
        assert!(debug_str.contains("y: 2"));
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_manhattan_distance_symmetry(x1 in any::<isize>(), y1 in any::<isize>(), x2 in any::<isize>(), y2 in any::<isize>()) {
        let coord1 = Coordinate::new(x1, y1);
        let coord2 = Coordinate::new(x2, y2);

        // Manhattan distance should be symmetric
        assert_eq!(coord1.manhattan_distance(&coord2), coord2.manhattan_distance(&coord1));
    }

    #[test]
    fn test_manhattan_distance_triangle_inequality(x1 in any::<isize>(), y1 in any::<isize>(), x2 in any::<isize>(), y2 in any::<isize>(), x3 in any::<isize>(), y3 in any::<isize>()) {
        let coord1 = Coordinate::new(x1, y1);
        let coord2 = Coordinate::new(x2, y2);
        let coord3 = Coordinate::new(x3, y3);

        let d12 = coord1.manhattan_distance(&coord2);
        let d23 = coord2.manhattan_distance(&coord3);
        let d13 = coord1.manhattan_distance(&coord3);

        // Triangle inequality: d(coord1, coord3) <= d(coord1, coord2) + d(coord2, coord3)
        assert!(d13 <= d12 + d23);
    }

    #[test]
    fn test_offset_roundtrip(x in any::<isize>(), y in any::<isize>(), dx in any::<isize>(), dy in any::<isize>()) {
        let coord = Coordinate::new(x, y);
        let offset = coord.offset(dx, dy);
        let back = offset.offset(-dx, -dy);

        assert_eq!(coord, back);
    }

    #[test]
    fn test_coordinate_add_sub_roundtrip(x in any::<isize>(), y in any::<isize>()) {
        let coord = Coordinate::new(x, y);

        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let added = coord + direction;
            let back = added - direction;
            assert_eq!(coord, back);
        }
    }

    #[test]
    fn test_manhattan_distance_zero_for_same_coordinate(x in any::<isize>(), y in any::<isize>()) {
        let coord = Coordinate::new(x, y);
        assert_eq!(coord.manhattan_distance(&coord), 0);
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_coordinate_creation() {
        let start = Instant::now();
        for i in 0..1_000_000 {
            let _coord = Coordinate::new(i, i * 2);
        }
        let duration = start.elapsed();
        println!("Coordinate creation (1M): {:?}", duration);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn benchmark_manhattan_distance() {
        let coords: Vec<_> = (0..10_000)
            .map(|i| Coordinate::new(i, i * 2))
            .collect();

        let start = Instant::now();
        for i in 0..coords.len() - 1 {
            let _dist = coords[i].manhattan_distance(&coords[i + 1]);
        }
        let duration = start.elapsed();
        println!("Manhattan distance (10K): {:?}", duration);
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn benchmark_offset_operations() {
        let coord = Coordinate::origin();
        let start = Instant::now();
        for i in 0..1_000_000 {
            let _offset = coord.offset(i % 100 - 50, i % 100 - 50);
        }
        let duration = start.elapsed();
        println!("Offset operations (1M): {:?}", duration);
        assert!(duration.as_millis() < 100);
    }
}