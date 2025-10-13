use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::coordinate::Coordinate;
use tubular::types::bigint::TubularBigInt;
use tubular::interpreter::grid::{ProgramGrid, ProgramCell};
use tubular::interpreter::stack::DataStack;
use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
use tubular::interpreter::droplet::{Droplet, DropletId};
use tubular::parser::grid_parser::GridParser;
use std::alloc::{System, GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

// Simple memory tracker for benchmarks
struct MemoryTracker {
    allocations: AtomicUsize,
    deallocations: AtomicUsize,
    current_usage: AtomicUsize,
    peak_usage: AtomicUsize,
}

impl MemoryTracker {
    const fn new() -> Self {
        Self {
            allocations: AtomicUsize::new(0),
            deallocations: AtomicUsize::new(0),
            current_usage: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
        }
    }

    fn allocate(&self, size: usize) {
        let old = self.current_usage.fetch_add(size, Ordering::SeqCst);
        let _peak = self.peak_usage.fetch_max(old + size, Ordering::SeqCst);
        self.allocations.fetch_add(1, Ordering::SeqCst);
    }

    fn deallocate(&self, size: usize) {
        self.current_usage.fetch_sub(size, Ordering::SeqCst);
        self.deallocations.fetch_add(1, Ordering::SeqCst);
    }

    fn stats(&self) -> MemoryStats {
        MemoryStats {
            allocations: self.allocations.load(Ordering::SeqCst),
            deallocations: self.deallocations.load(Ordering::SeqCst),
            current_usage: self.current_usage.load(Ordering::SeqCst),
            peak_usage: self.peak_usage.load(Ordering::SeqCst),
        }
    }

    fn reset(&self) {
        self.allocations.store(0, Ordering::SeqCst);
        self.deallocations.store(0, Ordering::SeqCst);
        self.current_usage.store(0, Ordering::SeqCst);
        self.peak_usage.store(0, Ordering::SeqCst);
    }
}

#[derive(Debug)]
struct MemoryStats {
    allocations: usize,
    deallocations: usize,
    current_usage: usize,
    peak_usage: usize,
}

static MEMORY_TRACKER: MemoryTracker = MemoryTracker::new();

pub fn bench_memory_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation_patterns");

    group.bench_function("coordinate_allocation", |b| {
        MEMORY_TRACKER.reset();
        b.iter(|| {
            let coords: Vec<Coordinate> = (0..1000)
                .map(|i| Coordinate::new(i as isize, (i * 2) as isize))
                .collect();
            black_box(coords);
        });
        let stats = MEMORY_TRACKER.stats();
        println!("Coordinate allocation: {:?}", stats);
    });

    group.bench_function("bigint_allocation", |b| {
        MEMORY_TRACKER.reset();
        b.iter(|| {
            let bigints: Vec<TubularBigInt> = (0..1000)
                .map(|i| TubularBigInt::new(i as i64))
                .collect();
            black_box(bigints);
        });
        let stats = MEMORY_TRACKER.stats();
        println!("BigInt allocation: {:?}", stats);
    });

    group.bench_function("grid_cell_allocation", |b| {
        MEMORY_TRACKER.reset();
        b.iter(|| {
            let cells: Vec<ProgramCell> = (0..1000)
                .map(|i| ProgramCell::new(match i % 4 {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    _ => '/',
                }))
                .collect();
            black_box(cells);
        });
        let stats = MEMORY_TRACKER.stats();
        println!("Grid cell allocation: {:?}", stats);
    });

    group.bench_function("droplet_allocation", |b| {
        MEMORY_TRACKER.reset();
        b.iter(|| {
            let droplets: Vec<Droplet> = (0..1000)
                .map(|i| Droplet::new(
                    i as DropletId,
                    Coordinate::new(i as isize, i as isize),
                    tubular::types::direction::Direction::Right
                ))
                .collect();
            black_box(droplets);
        });
        let stats = MEMORY_TRACKER.stats();
        println!("Droplet allocation: {:?}", stats);
    });

    group.finish();
}

pub fn bench_memory_pressure(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pressure");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("large_grid", size), size, |b, &size| {
            b.iter(|| {
                let mut grid = ProgramGrid::new();
                for i in 0..size {
                    let coord = Coordinate::new(i as isize, (i * 3) % 1000);
                    let _ = grid.add_cell(coord, '+');
                }
                black_box(grid);
            });
        });

        group.bench_with_input(BenchmarkId::new("large_stack", size), size, |b, &size| {
            b.iter(|| {
                let mut stack = DataStack::with_capacity(size as usize);
                for i in 0..size {
                    stack.push(TubularBigInt::new(i as i64));
                }
                black_box(stack);
            });
        });

        group.bench_with_input(BenchmarkId::new("large_reservoir", size), size, |b, &size| {
            b.iter(|| {
                let mut reservoir = Reservoir::with_capacity(size as usize);
                for i in 0..size {
                    let coord = ReservoirCoordinate::new(i as isize, (i * 7) % 1000);
                    let value = TubularBigInt::new(i as i64);
                    reservoir.put(coord, value);
                }
                black_box(reservoir);
            });
        });
    }

    group.finish();
}

pub fn bench_memory_reuse_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_reuse_patterns");

    group.bench_function("stack_push_pop_reuse", |b| {
        b.iter(|| {
            let mut stack = DataStack::with_capacity(1000);
            // Fill and empty stack multiple times
            for _ in 0..10 {
                for i in 0..100 {
                    stack.push(TubularBigInt::new(i));
                }
                for _ in 0..100 {
                    stack.pop();
                }
            }
            black_box(stack);
        });
    });

    group.bench_function("reservoir_put_remove_reuse", |b| {
        b.iter(|| {
            let mut reservoir = Reservoir::with_capacity(1000);
            let coords: Vec<ReservoirCoordinate> = (0..100)
                .map(|i| ReservoirCoordinate::new(i as isize, i as isize))
                .collect();

            // Put and remove values multiple times
            for _ in 0..10 {
                for (i, coord) in coords.iter().enumerate() {
                    let value = TubularBigInt::new(i as i64);
                    reservoir.put(coord.clone(), value);
                }
                for coord in &coords {
                    reservoir.remove(coord);
                }
            }
            black_box(reservoir);
        });
    });

    group.finish();
}

pub fn bench_memory_growth_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_growth_patterns");

    group.bench_function("exponential_stack_growth", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            let mut size = 1;
            for _ in 0..10 {
                for _ in 0..size {
                    stack.push(TubularBigInt::new(1));
                }
                size *= 2;
            }
            black_box(stack);
        });
    });

    group.bench_function("exponential_reservoir_growth", |b| {
        b.iter(|| {
            let mut reservoir = Reservoir::new();
            let mut size = 1;
            for iteration in 0..10 {
                for i in 0..size {
                    let coord = ReservoirCoordinate::new((size * iteration + i) as isize, 0);
                    let value = TubularBigInt::new(i as i64);
                    reservoir.put(coord, value);
                }
                size *= 2;
            }
            black_box(reservoir);
        });
    });

    group.finish();
}

pub fn bench_memory_fragmentation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_fragmentation");

    group.bench_function("fragmented_stack_access", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            // Create fragmented pattern
            for i in 0..1000 {
                if i % 3 == 0 {
                    stack.push(TubularBigInt::new(i));
                }
            }
            // Access random elements
            for i in [0, 100, 200, 300, 400, 500, 600, 700, 800, 900] {
                let value = stack.get_from_top(i);
                black_box(value);
            }
        });
    });

    group.bench_function("fragmented_reservoir_access", |b| {
        b.iter(|| {
            let mut reservoir = Reservoir::new();
            // Create fragmented pattern
            for i in 0..1000 {
                if i % 3 == 0 {
                    let coord = ReservoirCoordinate::new(i as isize, i as isize);
                    let value = TubularBigInt::new(i as i64);
                    reservoir.put(coord, value);
                }
            }
            // Access random elements
            for i in [0, 100, 200, 300, 400, 500, 600, 700, 800, 900] {
                let coord = ReservoirCoordinate::new(i as isize, i as isize);
                let value = reservoir.get(coord);
                black_box(value);
            }
        });
    });

    group.finish();
}

pub fn bench_memory_vs_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_vs_performance");

    // Test memory-efficient vs memory-intensive approaches
    group.bench_function("efficient_grid_iteration", |b| {
        let grid = create_test_grid(1000, 1000);
        b.iter(|| {
            let mut count = 0;
            for (coord, cell) in grid.iter() {
                black_box(coord);
                black_box(cell);
                count += 1;
                if count >= 1000 { break; } // Limit iteration
            }
        });
    });

    group.bench_function("intensive_grid_iteration", |b| {
        let grid = create_test_grid(1000, 1000);
        b.iter(|| {
            let mut collected = Vec::new();
            for (coord, cell) in grid.iter() {
                collected.push((coord.clone(), cell.clone()));
            }
            black_box(collected);
        });
    });

    group.finish();
}

pub fn bench_parser_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_memory_usage");

    let programs = [
        ("small", create_small_parser_program()),
        ("medium", create_medium_parser_program()),
        ("large", create_large_parser_program()),
    ];

    for (name, program) in programs.iter() {
        group.bench_with_input(BenchmarkId::new("parse_memory", name), name, |b, _| {
            b.iter(|| {
                let parser = GridParser::new();
                let grid = parser.parse_string(black_box(program.as_str()));
                black_box(grid);
            });
        });
    }

    group.finish();
}

// Helper functions
fn create_test_grid(width: usize, height: usize) -> ProgramGrid {
    let mut grid = ProgramGrid::new();
    for y in 0..height {
        for x in 0..width {
            let coord = Coordinate::new(x as isize, y as isize);
            let symbol = match (x + y) % 4 {
                0 => '+',
                1 => '-',
                2 => '*',
                _ => '/',
            };
            let _ = grid.add_cell(coord, symbol);
        }
    }
    grid
}

fn create_small_parser_program() -> String {
    "@\n|\n5 3 + A n,\n!\n".to_string()
}

fn create_medium_parser_program() -> String {
    let mut program = String::new();
    for y in 0..50 {
        for x in 0..50 {
            if x == 0 && y == 0 {
                program.push('@');
            } else {
                program.push(match (x + y) % 4 {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    _ => '/',
                });
            }
        }
        program.push('\n');
    }
    program
}

fn create_large_parser_program() -> String {
    let mut program = String::new();
    for y in 0..200 {
        for x in 0..200 {
            if x == 0 && y == 0 {
                program.push('@');
            } else {
                program.push(match (x + y) % 8 {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    3 => '/',
                    4 => '|',
                    5 => '\\',
                    6 => '/',
                    _ => (x % 10).to_string().chars().next().unwrap(),
                });
            }
        }
        program.push('\n');
    }
    program
}