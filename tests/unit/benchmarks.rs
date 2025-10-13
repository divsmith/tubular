//! Performance benchmarks for critical components

use std::time::Instant;

#[cfg(test)]
mod benchmarks {
    use super::*;

    #[test]
    fn benchmark_overall_performance() {
        println!("=== Tubular Performance Benchmarks ===");

        // Benchmark basic operations
        benchmark_coordinate_operations();
        benchmark_direction_operations();
        benchmark_bigint_operations();
        benchmark_stack_operations();
        benchmark_memory_operations();
        benchmark_droplet_operations();
    }

    fn benchmark_coordinate_operations() {
        use tubular::types::Coordinate;

        let start = Instant::now();
        for i in 0..1_000_000 {
            let coord1 = Coordinate::new(i, i * 2);
            let coord2 = Coordinate::new(i + 1, i * 2 + 1);
            let _distance = coord1.manhattan_distance(&coord2);
            let _offset = coord1.offset(1, 1);
        }
        let duration = start.elapsed();
        println!("Coordinate operations (3M): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    fn benchmark_direction_operations() {
        use tubular::types::direction::Direction;

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
        println!("Direction operations (50M): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    fn benchmark_bigint_operations() {
        use tubular::types::bigint::TubularBigInt;

        let a = TubularBigInt::new(1000);
        let b = TubularBigInt::new(42);
        let start = Instant::now();

        for _ in 0..1_000_000 {
            let _sum = a.clone() + b.clone();
            let _diff = a.clone() - b.clone();
            let _prod = a.clone() * b.clone();
            let _div = a.safe_div(&b);
            let _mod = a.safe_mod(&b);
        }

        let duration = start.elapsed();
        println!("BigInt operations (5M): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    fn benchmark_stack_operations() {
        use tubular::interpreter::stack::DataStack;
        use tubular::types::bigint::TubularBigInt;

        let mut stack = DataStack::new();
        let start = Instant::now();

        for i in 0..1_000_000 {
            stack.push(TubularBigInt::new(i));
            if i % 10 == 0 {
                stack.pop();
            }
        }

        let duration = start.elapsed();
        println!("Stack operations (1M): {:?}", duration);
        assert!(duration.as_millis() < 300);
    }

    fn benchmark_memory_operations() {
        use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
        use tubular::types::bigint::TubularBigInt;

        let mut reservoir = Reservoir::new();
        let start = Instant::now();

        for i in 0..100_000 {
            let coord = ReservoirCoordinate::new(i % 1000, i / 1000);
            reservoir.put(coord, TubularBigInt::new(i));
        }

        for i in 0..100_000 {
            let coord = ReservoirCoordinate::new(i % 1000, i / 1000);
            let _value = reservoir.get(coord);
        }

        let duration = start.elapsed();
        println!("Memory operations (200K): {:?}", duration);
        assert!(duration.as_millis() < 400);
    }

    fn benchmark_droplet_operations() {
        use tubular::interpreter::droplet::Droplet;
        use tubular::types::{Coordinate, Direction};
        use tubular::types::bigint::TubularBigInt;

        let mut droplets: Vec<Droplet> = (0..10_000)
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
}