use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tubular::types::coordinate::Coordinate;
use tubular::types::direction::Direction;
use tubular::types::bigint::TubularBigInt;
use num_bigint::BigInt;

pub fn bench_coordinate_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_creation");

    group.bench_function("new", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(42), black_box(-17));
            black_box(coord);
        })
    });

    group.bench_function("origin", |b| {
        b.iter(|| {
            let coord = Coordinate::origin();
            black_box(coord);
        })
    });

    group.bench_function("offset", |b| {
        let base = Coordinate::new(100, 200);
        b.iter(|| {
            let coord = base.offset(black_box(5), black_box(-3));
            black_box(coord);
        })
    });

    group.finish();
}

pub fn bench_coordinate_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_operations");

    let coord1 = Coordinate::new(100, 200);
    let coord2 = Coordinate::new(150, 180);

    group.bench_function("manhattan_distance", |b| {
        b.iter(|| {
            let dist = coord1.manhattan_distance(black_box(&coord2));
            black_box(dist);
        })
    });

    group.bench_function("add_direction", |b| {
        let coord = Coordinate::new(50, 50);
        b.iter(|| {
            let new_coord = coord + black_box(Direction::Right);
            black_box(new_coord);
        })
    });

    group.bench_function("sub_direction", |b| {
        let coord = Coordinate::new(50, 50);
        b.iter(|| {
            let new_coord = coord - black_box(Direction::Up);
            black_box(new_coord);
        })
    });

    group.finish();
}

pub fn bench_direction_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("direction_operations");

    group.bench_function("dx_dy", |b| {
        b.iter(|| {
            let direction = black_box(Direction::Right);
            let dx = direction.dx();
            let dy = direction.dy();
            black_box((dx, dy));
        })
    });

    group.bench_function("opposite", |b| {
        b.iter(|| {
            let direction = black_box(Direction::Up);
            let opposite = direction.opposite();
            black_box(opposite);
        })
    });

    group.bench_function("turn_left", |b| {
        b.iter(|| {
            let direction = black_box(Direction::Up);
            let left = direction.turn_left();
            black_box(left);
        })
    });

    group.bench_function("turn_right", |b| {
        b.iter(|| {
            let direction = black_box(Direction::Up);
            let right = direction.turn_right();
            black_box(right);
        })
    });

    group.bench_function("from_char", |b| {
        b.iter(|| {
            let direction = Direction::from_char(black_box('^'));
            black_box(direction);
        })
    });

    group.finish();
}

pub fn bench_bigint_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("bigint_creation");

    group.bench_function("new_i64", |b| {
        b.iter(|| {
            let bigint = TubularBigInt::new(black_box(42));
            black_box(bigint);
        })
    });

    group.bench_function("zero", |b| {
        b.iter(|| {
            let bigint = TubularBigInt::zero();
            black_box(bigint);
        })
    });

    group.bench_function("one", |b| {
        b.iter(|| {
            let bigint = TubularBigInt::one();
            black_box(bigint);
        })
    });

    group.bench_function("from_bigint_large", |b| {
        let large_bigint = BigInt::from(12345678901234567890i128);
        b.iter(|| {
            let bigint = TubularBigInt::from_bigint(black_box(large_bigint.clone()));
            black_box(bigint);
        })
    });

    group.bench_function("from_char", |b| {
        b.iter(|| {
            let bigint = TubularBigInt::from_char(black_box('A'));
            black_box(bigint);
        })
    });

    group.finish();
}

pub fn bench_bigint_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("bigint_arithmetic");

    let a = TubularBigInt::new(1000000);
    let b = TubularBigInt::new(123456);
    let large_a = TubularBigInt::from_bigint(BigInt::from(12345678901234567890i128));
    let large_b = TubularBigInt::from_bigint(BigInt::from(9876543210987654321i128));

    group.bench_function("add_small", |b| {
        b.iter(|| {
            let result = black_box(a.clone()) + black_box(b.clone());
            black_box(result);
        })
    });

    group.bench_function("sub_small", |b| {
        b.iter(|| {
            let result = black_box(a.clone()) - black_box(b.clone());
            black_box(result);
        })
    });

    group.bench_function("mul_small", |b| {
        b.iter(|| {
            let result = black_box(a.clone()) * black_box(b.clone());
            black_box(result);
        })
    });

    group.bench_function("div_small", |b| {
        b.iter(|| {
            let result = black_box(a.clone()) / black_box(b.clone());
            black_box(result);
        })
    });

    group.bench_function("add_large", |b| {
        b.iter(|| {
            let result = black_box(large_a.clone()) + black_box(large_b.clone());
            black_box(result);
        })
    });

    group.bench_function("mul_large", |b| {
        b.iter(|| {
            let result = black_box(large_a.clone()) * black_box(large_b.clone());
            black_box(result);
        })
    });

    group.bench_function("increment", |b| {
        let mut value = TubularBigInt::new(1000000);
        b.iter(|| {
            value.increment();
            black_box(&value);
        })
    });

    group.bench_function("decrement", |b| {
        let mut value = TubularBigInt::new(1000000);
        b.iter(|| {
            value.decrement();
            black_box(&value);
        })
    });

    group.finish();
}