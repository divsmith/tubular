use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::bigint::TubularBigInt;
use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
use tubular::types::coordinate::Coordinate;
use std::collections::HashMap;

pub fn bench_reservoir_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_access_patterns");

    // Create reservoirs with different sizes
    let small_reservoir = create_test_reservoir(100);
    let medium_reservoir = create_test_reservoir(1000);
    let large_reservoir = create_test_reservoir(10000);

    group.bench_function("get_small", |b| {
        b.iter(|| {
            let coord = ReservoirCoordinate::new(black_box(10), black_box(10));
            let value = small_reservoir.get(coord);
            black_box(value);
        })
    });

    group.bench_function("get_medium", |b| {
        b.iter(|| {
            let coord = ReservoirCoordinate::new(black_box(50), black_box(50));
            let value = medium_reservoir.get(coord);
            black_box(value);
        })
    });

    group.bench_function("get_large", |b| {
        b.iter(|| {
            let coord = ReservoirCoordinate::new(black_box(100), black_box(100));
            let value = large_reservoir.get(coord);
            black_box(value);
        })
    });

    group.bench_function("put_small", |b| {
        b.iter(|| {
            let mut reservoir = Reservoir::new();
            for i in 0..100 {
                let coord = ReservoirCoordinate::new(i as isize, i as isize);
                let value = TubularBigInt::new(i as i64);
                reservoir.put(coord, value);
            }
            black_box(reservoir);
        })
    });

    group.bench_function("put_large", |b| {
        b.iter(|| {
            let mut reservoir = Reservoir::with_capacity(1000);
            for i in 0..1000 {
                let coord = ReservoirCoordinate::new(i as isize, i as isize);
                let value = TubularBigInt::new(i as i64);
                reservoir.put(coord, value);
            }
            black_box(reservoir);
        })
    });

    group.finish();
}

pub fn bench_reservoir_adjacent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_adjacent_access");

    let reservoir = create_test_reservoir(1000);

    group.bench_function("get_adjacent", |b| {
        b.iter(|| {
            let coord = ReservoirCoordinate::new(black_box(50), black_box(50));
            let adjacent = reservoir.get_adjacent(coord);
            black_box(adjacent);
        })
    });

    group.bench_function("get_adjacent_boundary", |b| {
        b.iter(|| {
            let coord = ReservoirCoordinate::new(black_box(0), black_box(0));
            let adjacent = reservoir.get_adjacent(coord);
            black_box(adjacent);
        })
    });

    group.finish();
}

pub fn bench_reservoir_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_iteration");

    let small_reservoir = create_test_reservoir(100);
    let medium_reservoir = create_test_reservoir(1000);
    let large_reservoir = create_test_reservoir(10000);

    for (size_name, reservoir) in [
        ("small", &small_reservoir),
        ("medium", &medium_reservoir),
        ("large", &large_reservoir),
    ] {
        group.throughput(Throughput::Elements(reservoir.len() as u64));

        group.bench_with_input(BenchmarkId::new("iter", size_name), size_name, |b, _| {
            b.iter(|| {
                let mut count = 0;
                for (coord, value) in reservoir.iter() {
                    black_box(coord);
                    black_box(value);
                    count += 1;
                }
                black_box(count);
            })
        });

        group.bench_with_input(BenchmarkId::new("iter_mut", size_name), size_name, |b, _| {
            b.iter(|| {
                let mut test_reservoir = reservoir.clone();
                for (coord, value) in test_reservoir.iter_mut() {
                    black_box(coord);
                    black_box(value);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("keys", size_name), size_name, |b, _| {
            b.iter(|| {
                let mut count = 0;
                for coord in reservoir.keys() {
                    black_box(coord);
                    count += 1;
                }
                black_box(count);
            })
        });

        group.bench_with_input(BenchmarkId::new("values", size_name), size_name, |b, _| {
            b.iter(|| {
                let mut count = 0;
                for value in reservoir.values() {
                    black_box(value);
                    count += 1;
                }
                black_box(count);
            })
        });
    }

    group.finish();
}

pub fn bench_reservoir_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_operations");

    let reservoir = create_test_reservoir(1000);

    group.bench_function("contains", |b| {
        let coord = ReservoirCoordinate::new(50, 50);
        b.iter(|| {
            let contains = reservoir.contains(black_box(&coord));
            black_box(contains);
        })
    });

    group.bench_function("remove", |b| {
        b.iter(|| {
            let mut test_reservoir = reservoir.clone();
            for i in 0..100 {
                let coord = ReservoirCoordinate::new(i as isize, i as isize);
                test_reservoir.remove(&coord);
            }
            black_box(test_reservoir);
        })
    });

    group.bench_function("clear", |b| {
        b.iter(|| {
            let mut test_reservoir = reservoir.clone();
            test_reservoir.clear();
            black_box(test_reservoir);
        })
    });

    group.bench_function("is_empty", |b| {
        b.iter(|| {
            let empty = reservoir.is_empty();
            black_box(empty);
        })
    });

    group.bench_function("len", |b| {
        b.iter(|| {
            let len = reservoir.len();
            black_box(len);
        })
    });

    group.finish();
}

pub fn bench_reservoir_bounding_box(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_bounding_box");

    let clustered_reservoir = create_clustered_reservoir(100, 10); // 100 cells clustered in 10x10 area
    let scattered_reservoir = create_scattered_reservoir(100, 1000); // 100 cells scattered in 1000x1000 area

    group.bench_function("bounding_box_clustered", |b| {
        b.iter(|| {
            let bbox = clustered_reservoir.bounding_box();
            black_box(bbox);
        })
    });

    group.bench_function("bounding_box_scattered", |b| {
        b.iter(|| {
            let bbox = scattered_reservoir.bounding_box();
            black_box(bbox);
        })
    });

    group.finish();
}

pub fn bench_reservoir_filter_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_filter_operations");

    let reservoir = create_test_reservoir_with_zeros(1000); // Include some zero values

    group.bench_function("count_non_zero", |b| {
        b.iter(|| {
            let count = reservoir.count_non_zero();
            black_box(count);
        })
    });

    group.bench_function("filter_zero_values", |b| {
        b.iter(|| {
            let mut test_reservoir = reservoir.clone();
            test_reservoir.filter_zero_values();
            black_box(test_reservoir);
        })
    });

    group.finish();
}

pub fn bench_reservoir_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("reservoir_conversion");

    let mut hashmap = HashMap::new();
    for i in 0..1000 {
        let coord = ReservoirCoordinate::new(i as isize, i as isize);
        let value = TubularBigInt::new(i as i64);
        hashmap.insert(coord, value);
    }

    group.bench_function("from_hashmap", |b| {
        b.iter(|| {
            let reservoir = Reservoir::from(black_box(hashmap.clone()));
            black_box(reservoir);
        })
    });

    group.bench_function("coordinate_conversion", |b| {
        let program_coord = Coordinate::new(42, 99);
        b.iter(|| {
            let reservoir_coord = ReservoirCoordinate::from(black_box(program_coord));
            let back_to_program = reservoir_coord.to_program_coordinate();
            black_box((reservoir_coord, back_to_program));
        })
    });

    group.finish();
}

// Helper functions to create test reservoirs
fn create_test_reservoir(size: usize) -> Reservoir {
    let mut reservoir = Reservoir::new();
    for i in 0..size {
        let x = (i * 7) % 1000;
        let y = (i * 13) % 1000;
        let coord = ReservoirCoordinate::new(x as isize, y as isize);
        let value = TubularBigInt::new(i as i64);
        reservoir.put(coord, value);
    }
    reservoir
}

fn create_test_reservoir_with_zeros(size: usize) -> Reservoir {
    let mut reservoir = Reservoir::new();
    for i in 0..size {
        let x = (i * 7) % 1000;
        let y = (i * 13) % 1000;
        let coord = ReservoirCoordinate::new(x as isize, y as isize);
        // Include some zero values
        let value = if i % 5 == 0 {
            TubularBigInt::zero()
        } else {
            TubularBigInt::new(i as i64)
        };
        reservoir.put(coord, value);
    }
    reservoir
}

fn create_clustered_reservoir(cell_count: usize, cluster_size: usize) -> Reservoir {
    let mut reservoir = Reservoir::new();
    for i in 0..cell_count {
        let x = (i % cluster_size) as isize;
        let y = (i / cluster_size) as isize;
        let coord = ReservoirCoordinate::new(x, y);
        let value = TubularBigInt::new(i as i64);
        reservoir.put(coord, value);
    }
    reservoir
}

fn create_scattered_reservoir(cell_count: usize, area_size: usize) -> Reservoir {
    let mut reservoir = Reservoir::new();
    for i in 0..cell_count {
        let x = (i * 73) % area_size;
        let y = (i * 37) % area_size;
        let coord = ReservoirCoordinate::new(x as isize, y as isize);
        let value = TubularBigInt::new(i as i64);
        reservoir.put(coord, value);
    }
    reservoir
}