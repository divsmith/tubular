//! Property-based tests using proptest

use proptest::prelude::*;
use tubular::types::*;
use tubular::interpreter::*;

proptest! {
    #[test]
    fn test_coordinate_manhattan_distance_symmetry(
        x1 in any::<isize>(),
        y1 in any::<isize>(),
        x2 in any::<isize>(),
        y2 in any::<isize>()
    ) {
        let coord1 = Coordinate::new(x1, y1);
        let coord2 = Coordinate::new(x2, y2);

        assert_eq!(
            coord1.manhattan_distance(&coord2),
            coord2.manhattan_distance(&coord1)
        );
    }

    #[test]
    fn test_direction_turn_properties(
        direction in prop::sample::select(&[
            Direction::Up, Direction::Down, Direction::Left, Direction::Right
        ])
    ) {
        // Two left turns should equal two right turns
        assert_eq!(
            direction.turn_left().turn_left(),
            direction.turn_right().turn_right()
        );

        // Four turns in any direction should return to original
        assert_eq!(
            direction.turn_left().turn_left().turn_left().turn_left(),
            direction
        );
        assert_eq!(
            direction.turn_right().turn_right().turn_right().turn_right(),
            direction
        );
    }

    #[test]
    fn test_bigint_arithmetic_properties(
        a in any::<i64>(),
        b in any::<i64>(),
        c in any::<i64>()
    ) {
        use tubular::types::bigint::TubularBigInt;

        let big_a = TubularBigInt::new(a);
        let big_b = TubularBigInt::new(b);
        let big_c = TubularBigInt::new(c);

        // Test commutativity of addition and multiplication
        assert_eq!(big_a.clone() + big_b.clone(), big_b.clone() + big_a.clone());
        assert_eq!(big_a.clone() * big_b.clone(), big_b.clone() * big_a.clone());

        // Test associativity
        assert_eq!(
            (big_a.clone() + big_b.clone()) + big_c.clone(),
            big_a.clone() + (big_b.clone() + big_c.clone())
        );
        assert_eq!(
            (big_a.clone() * big_b.clone()) * big_c.clone(),
            big_a.clone() * (big_b.clone() * big_c.clone())
        );

        // Test distributivity
        assert_eq!(
            big_a.clone() * (big_b.clone() + big_c.clone()),
            big_a.clone() * big_b.clone() + big_a.clone() * big_c.clone()
        );
    }

    #[test]
    fn test_stack_lifo_properties(
        values in prop::collection::vec(any::<i64>(), 0..100)
    ) {
        use tubular::interpreter::stack::DataStack;
        use tubular::types::bigint::TubularBigInt;

        let mut stack = DataStack::new();
        let bigint_values: Vec<TubularBigInt> = values
            .iter()
            .map(|&v| TubularBigInt::new(v))
            .collect();

        // Push all values
        for value in &bigint_values {
            stack.push(value.clone());
        }

        // Pop all values - should be reverse order
        let mut popped = Vec::new();
        while !stack.is_empty() {
            popped.push(stack.pop());
        }

        let expected: Vec<TubularBigInt> = bigint_values.iter().rev().cloned().collect();
        assert_eq!(popped, expected);
    }

    #[test]
    fn test_memory_reservoir_consistency(
        operations in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<i64>()),
            0..50
        )
    ) {
        use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
        use tubular::types::bigint::TubularBigInt;

        let mut reservoir = Reservoir::new();
        let mut test_data = Vec::new();

        for (x, y, value) in operations {
            let coord = ReservoirCoordinate::new(x, y);
            let big_value = TubularBigInt::new(value);
            reservoir.put(coord.clone(), big_value.clone());
            test_data.push((coord, big_value));
        }

        // Verify all values can be retrieved consistently
        for (coord, expected_value) in &test_data {
            let actual_value = reservoir.get(coord.clone());
            assert_eq!(actual_value, *expected_value);
        }

        // Test that contains matches get results
        for (coord, _) in &test_data {
            assert!(reservoir.contains(coord));
            assert_eq!(reservoir.get(coord.clone()), reservoir.get_or_zero(coord.clone()));
        }
    }

    #[test]
    fn test_droplet_collision_symmetry(
        x1 in any::<isize>(),
        y1 in any::<isize>(),
        x2 in any::<isize>(),
        y2 in any::<isize>(),
        dir1 in prop::sample::select(&[
            Direction::Up, Direction::Down, Direction::Left, Direction::Right
        ]),
        dir2 in prop::sample::select(&[
            Direction::Up, Direction::Down, Direction::Left, Direction::Right
        ])
    ) {
        use tubular::interpreter::droplet::Droplet;

        let pos1 = Coordinate::new(x1, y1);
        let pos2 = Coordinate::new(x2, y2);
        let droplet1 = Droplet::new(1, pos1, dir1);
        let droplet2 = Droplet::new(2, pos2, dir2);

        // Collision detection should be symmetric
        assert_eq!(
            droplet1.will_collide_with(&droplet2),
            droplet2.will_collide_with(&droplet1)
        );
    }
}