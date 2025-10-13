use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::coordinate::Coordinate;
use tubular::types::direction::Direction;
use tubular::types::bigint::TubularBigInt;
use tubular::interpreter::droplet::{Droplet, DropletId};
use std::collections::HashMap;

pub fn bench_droplet_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_creation");

    group.bench_function("new_basic", |b| {
        b.iter(|| {
            let droplet = Droplet::new(
                black_box(1),
                black_box(Coordinate::new(0, 0)),
                black_box(Direction::Right)
            );
            black_box(droplet);
        })
    });

    group.bench_function("with_value", |b| {
        b.iter(|| {
            let droplet = Droplet::with_value(
                black_box(1),
                black_box(TubularBigInt::new(42)),
                black_box(Coordinate::new(0, 0)),
                black_box(Direction::Right)
            );
            black_box(droplet);
        })
    });

    group.finish();
}

pub fn bench_droplet_movement(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_movement");

    let droplet = Droplet::new(1, Coordinate::new(50, 50), Direction::Right);

    group.bench_function("next_position", |b| {
        b.iter(|| {
            let next_pos = droplet.next_position();
            black_box(next_pos);
        })
    });

    group.bench_function("move_to", |b| {
        b.iter(|| {
            let mut test_droplet = droplet.clone();
            let new_pos = Coordinate::new(black_box(75), black_box(25));
            test_droplet.move_to(new_pos);
            black_box(test_droplet);
        })
    });

    group.bench_function("set_direction", |b| {
        b.iter(|| {
            let mut test_droplet = droplet.clone();
            test_droplet.set_direction(black_box(Direction::Down));
            black_box(test_droplet);
        })
    });

    group.bench_function("set_value", |b| {
        b.iter(|| {
            let mut test_droplet = droplet.clone();
            let new_value = TubularBigInt::new(black_box(999));
            test_droplet.set_value(new_value);
            black_box(test_droplet);
        })
    });

    group.finish();
}

pub fn bench_droplet_collision_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_collision_detection");

    let droplet1 = Droplet::new(1, Coordinate::new(10, 10), Direction::Right);
    let droplet2 = Droplet::new(2, Coordinate::new(11, 10), Direction::Left); // Will collide
    let droplet3 = Droplet::new(3, Coordinate::new(20, 20), Direction::Up); // Won't collide

    group.bench_function("will_collide_true", |b| {
        b.iter(|| {
            let will_collide = droplet1.will_collide_with(black_box(&droplet2));
            black_box(will_collide);
        })
    });

    group.bench_function("will_collide_false", |b| {
        b.iter(|| {
            let will_collide = droplet1.will_collide_with(black_box(&droplet3));
            black_box(will_collide);
        })
    });

    group.bench_function("is_at_same_position", |b| {
        let same_pos_droplet = Droplet::new(4, Coordinate::new(10, 10), Direction::Down);
        b.iter(|| {
            let same_pos = droplet1.is_at_same_position(black_box(&same_pos_droplet));
            black_box(same_pos);
        })
    });

    group.finish();
}

pub fn bench_multiple_droplets(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_droplets");

    for count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));

        group.bench_with_input(BenchmarkId::new("create_many", count), count, |b, &count| {
            b.iter(|| {
                let mut droplets = Vec::new();
                for i in 0..count {
                    let x = (i * 7) % 1000;
                    let y = (i * 13) % 1000;
                    let coord = Coordinate::new(x as isize, y as isize);
                    let droplet = Droplet::new(i as DropletId, coord, Direction::Right);
                    droplets.push(droplet);
                }
                black_box(droplets);
            })
        });

        group.bench_with_input(BenchmarkId::new("collision_check_all", count), count, |b, &count| {
            let droplets: Vec<Droplet> = (0..count).map(|i| {
                let x = (i * 7) % 100;
                let y = (i * 13) % 100;
                let coord = Coordinate::new(x as isize, y as isize);
                Droplet::new(i as DropletId, coord, Direction::Right)
            }).collect();

            b.iter(|| {
                let mut collision_count = 0;
                for i in 0..droplets.len() {
                    for j in (i + 1)..droplets.len() {
                        if droplets[i].will_collide_with(&droplets[j]) {
                            collision_count += 1;
                        }
                    }
                }
                black_box(collision_count);
            })
        });

        group.bench_with_input(BenchmarkId::new("hash_collection", count), count, |b, &count| {
            b.iter(|| {
                let mut droplet_set = std::collections::HashSet::new();
                for i in 0..count {
                    let x = (i * 7) % 1000;
                    let y = (i * 13) % 1000;
                    let coord = Coordinate::new(x as isize, y as isize);
                    let droplet = Droplet::new(i as DropletId, coord, Direction::Right);
                    droplet_set.insert(droplet);
                }
                black_box(droplet_set);
            })
        });
    }

    group.finish();
}

pub fn bench_droplet_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_lifecycle");

    group.bench_function("activate_deactivate", |b| {
        b.iter(|| {
            let mut droplet = Droplet::new(1, Coordinate::new(0, 0), Direction::Right);
            droplet.deactivate();
            black_box(droplet.is_active());
        })
    });

    group.bench_function("status_checks", |b| {
        let droplet = Droplet::new(1, Coordinate::new(0, 0), Direction::Right);
        b.iter(|| {
            let active = droplet.is_active();
            black_box(active);
        })
    });

    group.finish();
}

pub fn bench_droplet_cloning(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_cloning");

    let droplet = Droplet::with_value(
        1,
        TubularBigInt::new(123456789),
        Coordinate::new(100, 200),
        Direction::Left
    );

    group.bench_function("clone_single", |b| {
        b.iter(|| {
            let cloned = black_box(droplet.clone());
            black_box(cloned);
        })
    });

    for count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::new("clone_many", count), count, |b, &count| {
            b.iter(|| {
                let mut clones = Vec::with_capacity(count);
                for _ in 0..count {
                    clones.push(droplet.clone());
                }
                black_box(clones);
            })
        });
    }

    group.finish();
}

pub fn bench_droplet_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_hashing");

    let droplet = Droplet::new(42, Coordinate::new(10, 20), Direction::Up);

    group.bench_function("hash_single", |b| {
        b.iter(|| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            black_box(droplet.clone()).hash(&mut hasher);
            black_box(hasher.finish());
        })
    });

    group.bench_function("partial_eq", |b| {
        let other_droplet = Droplet::new(42, Coordinate::new(99, 99), Direction::Down);
        b.iter(|| {
            let equal = black_box(droplet.clone()) == black_box(other_droplet.clone());
            black_box(equal);
        })
    });

    group.finish();
}

pub fn bench_droplet_complex_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_complex_scenarios");

    // Scenario: Many droplets moving in different directions
    group.bench_function("mixed_directions", |b| {
        let mut droplets = Vec::new();
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for i in 0..1000 {
            let x = (i * 7) % 1000;
            let y = (i * 13) % 1000;
            let coord = Coordinate::new(x as isize, y as isize);
            let direction = directions[i % directions.len()];
            let droplet = Droplet::new(i as DropletId, coord, direction);
            droplets.push(droplet);
        }

        b.iter(|| {
            let mut next_positions = Vec::new();
            for droplet in &droplets {
                let next_pos = droplet.next_position();
                next_positions.push(next_pos);
            }
            black_box(next_positions);
        })
    });

    // Scenario: Droplets converging to a point
    group.bench_function("converging_droplets", |b| {
        let target = Coordinate::new(500, 500);
        let mut droplets = Vec::new();

        for i in 0..100 {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / 100.0;
            let radius = 100.0;
            let x = target.x + (radius * angle.cos()) as isize;
            let y = target.y + (radius * angle.sin()) as isize;
            let coord = Coordinate::new(x, y);

            // Point towards target
            let direction = if x < target.x { Direction::Right }
                           else if x > target.x { Direction::Left }
                           else if y < target.y { Direction::Down }
                           else { Direction::Up };

            let droplet = Droplet::new(i as DropletId, coord, direction);
            droplets.push(droplet);
        }

        b.iter(|| {
            let mut at_target = 0;
            for droplet in &droplets {
                if droplet.position == target {
                    at_target += 1;
                }
            }
            black_box(at_target);
        })
    });

    group.finish();
}