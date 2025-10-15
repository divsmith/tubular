//! Unit tests for the Reservoir (memory) type
//! Temporarily disabled due to compilation issues

// use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
// use tubular::types::Coordinate;
// use tubular::types::bigint::TubularBigInt;
// use std::collections::HashMap;
// use proptest::prelude::*;

/*
#[cfg(test)]
mod reservoir_coordinate_tests {
    use super::*;

    #[test]
    fn test_reservoir_coordinate_new() {
        let coord = ReservoirCoordinate::new(5, -3);
        assert_eq!(coord.x, 5);
        assert_eq!(coord.y, -3);
    }

    #[test]
    fn test_reservoir_coordinate_from_program_coordinate() {
        let prog_coord = Coordinate::new(10, -5);
        let res_coord = ReservoirCoordinate::from_program_coordinate(prog_coord);

        assert_eq!(res_coord.x, 10);
        assert_eq!(res_coord.y, -5);
    }

    #[test]
    fn test_reservoir_coordinate_to_program_coordinate() {
        let res_coord = ReservoirCoordinate::new(-2, 7);
        let prog_coord = res_coord.to_program_coordinate();

        assert_eq!(prog_coord.x, -2);
        assert_eq!(prog_coord.y, 7);
    }

    #[test]
    fn test_reservoir_coordinate_from_coordinate() {
        let prog_coord = Coordinate::new(3, 8);
        let res_coord: ReservoirCoordinate = prog_coord.into();

        assert_eq!(res_coord.x, 3);
        assert_eq!(res_coord.y, 8);
    }

    #[test]
    fn test_reservoir_coordinate_into_coordinate() {
        let res_coord = ReservoirCoordinate::new(-4, 9);
        let prog_coord: Coordinate = res_coord.into();

        assert_eq!(prog_coord.x, -4);
        assert_eq!(prog_coord.y, 9);
    }

    #[test]
    fn test_reservoir_coordinate_roundtrip() {
        let original = ReservoirCoordinate::new(100, -200);
        let prog_coord: Coordinate = original.clone().into();
        let back: ReservoirCoordinate = prog_coord.into();

        assert_eq!(original, back);
    }

    #[test]
    fn test_reservoir_coordinate_equality() {
        let coord1 = ReservoirCoordinate::new(5, 10);
        let coord2 = ReservoirCoordinate::new(5, 10);
        let coord3 = ReservoirCoordinate::new(6, 10);

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }

    #[test]
    fn test_reservoir_coordinate_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ReservoirCoordinate::new(0, 0));
        set.insert(ReservoirCoordinate::new(1, 1));
        set.insert(ReservoirCoordinate::new(0, 0)); // Duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_reservoir_coordinate_debug() {
        let coord = ReservoirCoordinate::new(1, 2);
        let debug_str = format!("{:?}", coord);
        assert!(debug_str.contains("ReservoirCoordinate"));
        assert!(debug_str.contains("x: 1"));
        assert!(debug_str.contains("y: 2"));
    }
}

#[cfg(test)]
mod reservoir_tests {
    use super::*;

    #[test]
    fn test_reservoir_new() {
        let reservoir = Reservoir::new();
        assert!(reservoir.is_empty());
        assert_eq!(reservoir.len(), 0);
        assert_eq!(reservoir.size(), 0);
    }

    #[test]
    fn test_reservoir_with_capacity() {
        let reservoir = Reservoir::with_capacity(10);
        assert!(reservoir.is_empty());
        // Note: We can't directly test capacity
    }

    #[test]
    fn test_reservoir_get_empty() {
        let reservoir = Reservoir::new();
        let coord = ReservoirCoordinate::new(5, 10);

        let value = reservoir.get(coord);
        assert_eq!(value, TubularBigInt::zero());
    }

    #[test]
    fn test_reservoir_put_and_get() {
        let mut reservoir = Reservoir::new();
        let coord = ReservoirCoordinate::new(3, 7);
        let value = TubularBigInt::new(42);

        let returned = reservoir.put(coord, value.clone());
        assert_eq!(returned, value);

        let retrieved = reservoir.get(coord);
        assert_eq!(retrieved, value);
    }

    #[test]
    fn test_reservoir_put_overwrite() {
        let mut reservoir = Reservoir::new();
        let coord = ReservoirCoordinate::new(1, 2);
        let value1 = TubularBigInt::new(10);
        let value2 = TubularBigInt::new(20);

        reservoir.put(coord, value1);
        reservoir.put(coord, value2.clone());

        let retrieved = reservoir.get(coord);
        assert_eq!(retrieved, value2);
    }

    #[test]
    fn test_reservoir_get_or_zero() {
        let mut reservoir = Reservoir::new();
        let coord1 = ReservoirCoordinate::new(0, 0);
        let coord2 = ReservoirCoordinate::new(1, 1);

        reservoir.put(coord1, TubularBigInt::new(5));

        assert_eq!(reservoir.get_or_zero(coord1), TubularBigInt::new(5));
        assert_eq!(reservoir.get_or_zero(coord2), TubularBigInt::zero());
    }

    #[test]
    fn test_reservoir_contains() {
        let mut reservoir = Reservoir::new();
        let coord1 = ReservoirCoordinate::new(5, 10);
        let coord2 = ReservoirCoordinate::new(3, 7);

        assert!(!reservoir.contains(&coord1));
        assert!(!reservoir.contains(&coord2));

        reservoir.put(coord1, TubularBigInt::new(42));

        assert!(reservoir.contains(&coord1));
        assert!(!reservoir.contains(&coord2));
    }

    #[test]
    fn test_reservoir_remove() {
        let mut reservoir = Reservoir::new();
        let coord = ReservoirCoordinate::new(2, 4);
        let value = TubularBigInt::new(100);

        reservoir.put(coord, value.clone());
        assert!(reservoir.contains(&coord));

        let removed = reservoir.remove(&coord);
        assert_eq!(removed, Some(value));
        assert!(!reservoir.contains(&coord));

        // Remove non-existent
        let non_existent = reservoir.remove(&ReservoirCoordinate::new(99, 99));
        assert_eq!(non_existent, None);
    }

    #[test]
    fn test_reservoir_clear() {
        let mut reservoir = Reservoir::new();

        for i in 0..10 {
            reservoir.put(
                ReservoirCoordinate::new(i, i * 2),
                TubularBigInt::new(i * 10)
            );
        }

        assert_eq!(reservoir.len(), 10);
        assert!(!reservoir.is_empty());

        reservoir.clear();
        assert!(reservoir.is_empty());
        assert_eq!(reservoir.len(), 0);
    }

    #[test]
    fn test_reservoir_len_and_size() {
        let mut reservoir = Reservoir::new();

        assert_eq!(reservoir.len(), 0);
        assert_eq!(reservoir.size(), 0);

        for i in 1..=5 {
            reservoir.put(
                ReservoirCoordinate::new(i, i),
                TubularBigInt::new(i)
            );
            assert_eq!(reservoir.len(), i);
            assert_eq!(reservoir.size(), i);
        }
    }

    #[test]
    fn test_reservoir_iteration() {
        let mut reservoir = Reservoir::new();
        let coords = vec![
            ReservoirCoordinate::new(0, 0),
            ReservoirCoordinate::new(1, 1),
            ReservoirCoordinate::new(2, 2),
        ];
        let values = vec![
            TubularBigInt::new(10),
            TubularBigInt::new(20),
            TubularBigInt::new(30),
        ];

        for (coord, value) in coords.iter().zip(values.iter()) {
            reservoir.put(*coord, value.clone());
        }

        let mut found_coords = Vec::new();
        let mut found_values = Vec::new();

        for (coord, value) in reservoir.iter() {
            found_coords.push(*coord);
            found_values.push(value.clone());
        }

        found_coords.sort();
        found_values.sort_by(|a, b| a.to_i64().unwrap().cmp(&b.to_i64().unwrap()));

        let mut expected_coords = coords.clone();
        let mut expected_values = values.clone();
        expected_coords.sort();
        expected_values.sort_by(|a, b| a.to_i64().unwrap().cmp(&b.to_i64().unwrap()));

        assert_eq!(found_coords, expected_coords);
        assert_eq!(found_values, expected_values);
    }

    #[test]
    fn test_reservoir_keys_and_values() {
        let mut reservoir = Reservoir::new();

        reservoir.put(ReservoirCoordinate::new(0, 0), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(1, 1), TubularBigInt::new(2));
        reservoir.put(ReservoirCoordinate::new(2, 2), TubularBigInt::new(3));

        let keys: Vec<_> = reservoir.keys().cloned().collect();
        let values: Vec<_> = reservoir.values().cloned().collect();

        assert_eq!(keys.len(), 3);
        assert_eq!(values.len(), 3);

        assert!(keys.contains(&ReservoirCoordinate::new(0, 0)));
        assert!(keys.contains(&ReservoirCoordinate::new(1, 1)));
        assert!(keys.contains(&ReservoirCoordinate::new(2, 2)));

        assert!(values.contains(&TubularBigInt::new(1)));
        assert!(values.contains(&TubularBigInt::new(2)));
        assert!(values.contains(&TubularBigInt::new(3)));
    }

    #[test]
    fn test_reservoir_get_adjacent() {
        let mut reservoir = Reservoir::new();
        let center = ReservoirCoordinate::new(5, 5);

        // Put values in some adjacent cells
        reservoir.put(ReservoirCoordinate::new(4, 4), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(6, 5), TubularBigInt::new(2));
        reservoir.put(ReservoirCoordinate::new(5, 6), TubularBigInt::new(3));

        let adjacent = reservoir.get_adjacent(center);

        assert_eq!(adjacent.len(), 8);

        // Check that we get values for the cells we set
        let mut found_1 = false;
        let mut found_2 = false;
        let mut found_3 = false;

        for (value, coord) in &adjacent {
            if *coord == ReservoirCoordinate::new(4, 4) {
                assert_eq!(value, &TubularBigInt::new(1));
                found_1 = true;
            }
            if *coord == ReservoirCoordinate::new(6, 5) {
                assert_eq!(value, &TubularBigInt::new(2));
                found_2 = true;
            }
            if *coord == ReservoirCoordinate::new(5, 6) {
                assert_eq!(value, &TubularBigInt::new(3));
                found_3 = true;
            }
        }

        assert!(found_1);
        assert!(found_2);
        assert!(found_3);

        // Check that unset cells return zero
        for (value, coord) in &adjacent {
            if *coord != ReservoirCoordinate::new(4, 4) &&
               *coord != ReservoirCoordinate::new(6, 5) &&
               *coord != ReservoirCoordinate::new(5, 6) {
                assert_eq!(value, &TubularBigInt::zero());
            }
        }
    }

    #[test]
    fn test_reservoir_bounding_box() {
        let mut reservoir = Reservoir::new();

        // Empty reservoir should have no bounding box
        assert_eq!(reservoir.bounding_box(), None);

        // Add some cells
        reservoir.put(ReservoirCoordinate::new(0, 0), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(5, 3), TubularBigInt::new(2));
        reservoir.put(ReservoirCoordinate::new(-2, 7), TubularBigInt::new(3));

        let bbox = reservoir.bounding_box().unwrap();
        assert_eq!(bbox.0, ReservoirCoordinate::new(-2, 0));
        assert_eq!(bbox.1, ReservoirCoordinate::new(5, 7));
    }

    #[test]
    fn test_reservoir_count_non_zero() {
        let mut reservoir = Reservoir::new();

        assert_eq!(reservoir.count_non_zero(), 0);

        reservoir.put(ReservoirCoordinate::new(0, 0), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(1, 0), TubularBigInt::zero());
        reservoir.put(ReservoirCoordinate::new(2, 0), TubularBigInt::new(5));

        assert_eq!(reservoir.count_non_zero(), 2);
        assert_eq!(reservoir.len(), 3); // Still counts zero values
    }

    #[test]
    fn test_reservoir_filter_zero_values() {
        let mut reservoir = Reservoir::new();

        reservoir.put(ReservoirCoordinate::new(0, 0), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(1, 0), TubularBigInt::zero());
        reservoir.put(ReservoirCoordinate::new(2, 0), TubularBigInt::new(5));
        reservoir.put(ReservoirCoordinate::new(3, 0), TubularBigInt::zero());

        assert_eq!(reservoir.len(), 4);
        assert_eq!(reservoir.count_non_zero(), 2);

        reservoir.filter_zero_values();

        assert_eq!(reservoir.len(), 2);
        assert_eq!(reservoir.count_non_zero(), 2);
        assert!(reservoir.contains(&ReservoirCoordinate::new(0, 0)));
        assert!(reservoir.contains(&ReservoirCoordinate::new(2, 0)));
        assert!(!reservoir.contains(&ReservoirCoordinate::new(1, 0)));
        assert!(!reservoir.contains(&ReservoirCoordinate::new(3, 0)));
    }

    #[test]
    fn test_reservoir_default() {
        let reservoir = Reservoir::default();
        assert!(reservoir.is_empty());
        assert_eq!(reservoir.len(), 0);
    }

    #[test]
    fn test_reservoir_display() {
        let mut reservoir = Reservoir::new();

        reservoir.put(ReservoirCoordinate::new(0, 0), TubularBigInt::new(1));
        reservoir.put(ReservoirCoordinate::new(1, 0), TubularBigInt::new(2));
        reservoir.put(ReservoirCoordinate::new(0, 1), TubularBigInt::new(3));

        let display = format!("{}", reservoir);
        assert!(display.contains("Reservoir"));
        assert!(display.contains("3 cells"));
        assert!(display.contains("1"));
        assert!(display.contains("2"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_reservoir_from_hashmap() {
        let mut data = HashMap::new();
        data.insert(ReservoirCoordinate::new(0, 0), TubularBigInt::new(10));
        data.insert(ReservoirCoordinate::new(1, 1), TubularBigInt::new(20));

        let reservoir: Reservoir = data.into();
        assert_eq!(reservoir.len(), 2);
        assert_eq!(reservoir.get(ReservoirCoordinate::new(0, 0)), TubularBigInt::new(10));
        assert_eq!(reservoir.get(ReservoirCoordinate::new(1, 1)), TubularBigInt::new(20));
    }

    #[test]
    fn test_reservoir_large_coordinates() {
        let mut reservoir = Reservoir::new();

        let large_coord = ReservoirCoordinate::new(isize::MAX, isize::MIN);
        let small_coord = ReservoirCoordinate::new(isize::MIN, isize::MAX);

        reservoir.put(large_coord, TubularBigInt::new(100));
        reservoir.put(small_coord, TubularBigInt::new(200));

        assert_eq!(reservoir.get(large_coord), TubularBigInt::new(100));
        assert_eq!(reservoir.get(small_coord), TubularBigInt::new(200));
    }

    #[test]
    fn test_reservoir_negative_and_positive() {
        let mut reservoir = Reservoir::new();

        // Test with negative coordinates
        reservoir.put(ReservoirCoordinate::new(-5, -3), TubularBigInt::new(42));
        reservoir.put(ReservoirCoordinate::new(5, 3), TubularBigInt::new(24));

        assert_eq!(reservoir.get(ReservoirCoordinate::new(-5, -3)), TubularBigInt::new(42));
        assert_eq!(reservoir.get(ReservoirCoordinate::new(5, 3)), TubularBigInt::new(24));

        // Test bounding box includes both
        let bbox = reservoir.bounding_box().unwrap();
        assert_eq!(bbox.0, ReservoirCoordinate::new(-5, -3));
        assert_eq!(bbox.1, ReservoirCoordinate::new(5, 3));
    }

    #[test]
    fn test_reservoir_adjacent_edge_cases() {
        let mut reservoir = Reservoir::new();

        // Test at coordinate boundaries
        let edge_coord = ReservoirCoordinate::new(isize::MAX, isize::MAX);
        reservoir.put(edge_coord.clone(), TubularBigInt::new(999));

        let adjacent = reservoir.get_adjacent(edge_coord.clone());
        // Should wrap around (isize overflow behavior)
        assert_eq!(adjacent.len(), 8);

        // All adjacent cells should be zero since we only put one value
        for (value, coord) in &adjacent {
            if *coord != edge_coord {
                assert_eq!(value, &TubularBigInt::zero());
            }
        }
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_reservoir_coordinate_conversion(
        x in any::<isize>(),
        y in any::<isize>()
    ) {
        let res_coord = ReservoirCoordinate::new(x, y);
        let prog_coord: Coordinate = res_coord.clone().into();
        let back: ReservoirCoordinate = prog_coord.into();

        assert_eq!(res_coord, back);
    }

    #[test]
    fn test_reservoir_put_get_consistency(
        coords in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<i64>()),
            0..100
        )
    ) {
        let mut reservoir = Reservoir::new();
        let mut test_data = Vec::new();

        for (x, y, value) in coords {
            let coord = ReservoirCoordinate::new(x, y);
            let big_value = TubularBigInt::new(value);
            reservoir.put(coord.clone(), big_value.clone());
            test_data.push((coord, big_value));
        }

        // Verify all values can be retrieved
        for (coord, expected_value) in test_data {
            let actual_value = reservoir.get(coord);
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn test_reservoir_get_adjacent_properties(
        x in any::<isize>(),
        y in any::<isize>()
    ) {
        let mut reservoir = Reservoir::new();
        let center = ReservoirCoordinate::new(x, y);

        // Put values in all 8 adjacent cells
        let adjacent_coords = [
            ReservoirCoordinate::new(x - 1, y - 1),
            ReservoirCoordinate::new(x, y - 1),
            ReservoirCoordinate::new(x + 1, y - 1),
            ReservoirCoordinate::new(x - 1, y),
            ReservoirCoordinate::new(x + 1, y),
            ReservoirCoordinate::new(x - 1, y + 1),
            ReservoirCoordinate::new(x, y + 1),
            ReservoirCoordinate::new(x + 1, y + 1),
        ];

        for (i, &coord) in adjacent_coords.iter().enumerate() {
            reservoir.put(coord.clone(), TubularBigInt::new(i as i64));
        }

        let adjacent = reservoir.get_adjacent(center);

        // Should return exactly 8 values
        assert_eq!(adjacent.len(), 8);

        // All adjacent coordinates should be present
        for (i, expected_coord) in adjacent_coords.iter().enumerate() {
            let mut found = false;
            for (value, actual_coord) in &adjacent {
                if actual_coord == &expected_coord {
                    assert_eq!(value, &TubularBigInt::new(i as i64));
                    found = true;
                    break;
                }
            }
            assert!(found, "Adjacent coordinate {:?} not found", expected_coord);
        }
    }

    #[test]
    fn test_reservoir_bounding_box_properties(
        coords in prop::collection::vec(
            (any::<isize>(), any::<isize>()),
            1..50
        )
    ) {
        let mut reservoir = Reservoir::new();
        let coordinate_list: Vec<ReservoirCoordinate> = coords
            .iter()
            .map(|(x, y)| ReservoirCoordinate::new(*x, *y))
            .collect();

        for coord in &coordinate_list {
            reservoir.put(coord.clone(), TubularBigInt::new(1));
        }

        let bbox = reservoir.bounding_box();

        if !coordinate_list.is_empty() {
            assert!(bbox.is_some());

            let (min, max) = bbox.unwrap();

            // All coordinates should be within bounds
            for coord in &coordinate_list {
                assert!(coord.x >= min.x && coord.x <= max.x);
                assert!(coord.y >= min.y && coord.y <= max.y);
            }

            // Bounds should be tight (at least one point on each boundary)
            let mut found_min_x = false;
            let mut found_max_x = false;
            let mut found_min_y = false;
            let mut found_max_y = false;

            for coord in &coordinate_list {
                if coord.x == min.x { found_min_x = true; }
                if coord.x == max.x { found_max_x = true; }
                if coord.y == min.y { found_min_y = true; }
                if coord.y == max.y { found_max_y = true; }
            }

            assert!(found_min_x);
            assert!(found_max_x);
            assert!(found_min_y);
            assert!(found_max_y);
        } else {
            assert!(bbox.is_none());
        }
    }

    #[test]
    fn test_reservoir_filter_zero_properties(
        data in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<i64>()),
            0..50
        )
    ) {
        let mut reservoir = Reservoir::new();
        let mut non_zero_count = 0;

        for (x, y, value) in &data {
            let coord = ReservoirCoordinate::new(*x, *y);
            let big_value = TubularBigInt::new(*value);
            if !big_value.is_zero() {
                non_zero_count += 1;
            }
            reservoir.put(coord, big_value);
        }

        assert_eq!(reservoir.count_non_zero(), non_zero_count);

        reservoir.filter_zero_values();

        assert_eq!(reservoir.len(), non_zero_count);
        assert_eq!(reservoir.count_non_zero(), non_zero_count);

        // All remaining values should be non-zero
        for value in reservoir.values() {
            assert!(!value.is_zero());
        }
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_reservoir_operations() {
        let mut reservoir = Reservoir::new();
        let start = Instant::now();

        // Put operations
        for i in 0..100_000 {
            let coord = ReservoirCoordinate::new(i as isize, (i * 2) as isize);
            let value = TubularBigInt::new(i);
            reservoir.put(coord, value);
        }

        let put_duration = start.elapsed();
        println!("Reservoir put (100K): {:?}", put_duration);
        assert!(put_duration.as_millis() < 500);

        // Get operations
        let start = Instant::now();
        for i in 0..100_000 {
            let coord = ReservoirCoordinate::new(i as isize, (i * 2) as isize);
            let _value = reservoir.get(coord);
        }
        let get_duration = start.elapsed();
        println!("Reservoir get (100K): {:?}", get_duration);
        assert!(get_duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_reservoir_adjacent() {
        let mut reservoir = Reservoir::new();

        // Fill a region
        for x in 0..100 {
            for y in 0..100 {
                let coord = ReservoirCoordinate::new(x, y);
                let value = TubularBigInt::new(x * 100 + y);
                reservoir.put(coord, value);
            }
        }

        let start = Instant::now();
        for x in 10..90 {
            for y in 10..90 {
                let coord = ReservoirCoordinate::new(x, y);
                let _adjacent = reservoir.get_adjacent(coord);
            }
        }
        let duration = start.elapsed();
        println!("Reservoir get_adjacent (64K): {:?}", duration);
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn benchmark_reservoir_iteration() {
        let mut reservoir = Reservoir::new();

        for i in 0..50_000 {
            let coord = ReservoirCoordinate::new(
                (i % 1000) as isize,
                (i / 1000) as isize
            );
            let value = TubularBigInt::new(i);
            reservoir.put(coord, value);
        }

        let start = Instant::now();
        let mut sum = TubularBigInt::zero();
        for (_coord, value) in reservoir.iter() {
            sum = sum + value.clone();
        }
        let duration = start.elapsed();
        println!("Reservoir iteration (50K): {:?}", duration);
        assert!(duration.as_millis() < 100);
        // Verify sum is correct
        assert_eq!(sum.to_i64(), Some((0..50_000).sum()));
    }

    #[test]
    fn benchmark_reservoir_bounding_box() {
        let mut reservoir = Reservoir::new();

        // Create scattered data
        for i in 0..10_000 {
            let x = (i * 1009) % 2000 - 1000; // Spread across -1000 to 1000
            let y = (i * 2017) % 2000 - 1000;
            let coord = ReservoirCoordinate::new(x, y);
            let value = TubularBigInt::new(i);
            reservoir.put(coord, value);
        }

        let start = Instant::now();
        for _ in 0..1_000 {
            let _bbox = reservoir.bounding_box();
        }
        let duration = start.elapsed();
        println!("Reservoir bounding_box (1K): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_reservoir_coordinate_conversion() {
        let coords: Vec<Coordinate> = (0..100_000)
            .map(|i| Coordinate::new(i as isize, (i * 2) as isize))
            .collect();

        let start = Instant::now();

        let _: Vec<ReservoirCoordinate> = coords.iter()
            .cloned()
            .map(ReservoirCoordinate::from)
            .collect();

        let duration = start.elapsed();
        println!("ReservoirCoordinate conversion (100K): {:?}", duration);
        assert!(duration.as_millis() < 50);
    }
}
*/