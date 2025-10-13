use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::coordinate::Coordinate;
use tubular::interpreter::grid::{ProgramGrid, ProgramCell};
use std::collections::HashMap;

pub fn bench_grid_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_lookup");

    // Create different sized grids for testing
    let small_grid = create_test_grid(10, 10);
    let medium_grid = create_test_grid(100, 100);
    let large_grid = create_test_grid(1000, 1000);

    group.bench_function("lookup_small", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(5), black_box(5));
            let cell = small_grid.get(coord);
            black_box(cell);
        })
    });

    group.bench_function("lookup_medium", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(50), black_box(50));
            let cell = medium_grid.get(coord);
            black_box(cell);
        })
    });

    group.bench_function("lookup_large", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(500), black_box(500));
            let cell = large_grid.get(coord);
            black_box(cell);
        })
    });

    group.bench_function("lookup_missing", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(9999), black_box(9999));
            let cell = large_grid.get(coord);
            black_box(cell);
        })
    });

    group.finish();
}

pub fn bench_grid_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_iteration");

    let small_grid = create_test_grid(10, 10);
    let medium_grid = create_test_grid(100, 100);
    let large_grid = create_test_grid(1000, 1000);

    for (size_name, grid) in [
        ("small", &small_grid),
        ("medium", &medium_grid),
        ("large", &large_grid),
    ] {
        group.throughput(Throughput::Elements(grid.size() as u64));
        group.bench_with_input(BenchmarkId::new("iter", size_name), size_name, |b, _| {
            b.iter(|| {
                let mut count = 0;
                for (coord, cell) in grid.iter() {
                    black_box(coord);
                    black_box(cell);
                    count += 1;
                }
                black_box(count);
            })
        });
    }

    group.finish();
}

pub fn bench_grid_sparse_vs_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_sparse_vs_dense");

    // Create sparse grid (few cells in large area)
    let sparse_grid = create_sparse_grid(1000, 1000, 100);

    // Create dense grid (many cells in small area)
    let dense_grid = create_test_grid(10, 10);

    group.bench_function("sparse_lookup", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(500), black_box(500));
            let cell = sparse_grid.get(coord);
            black_box(cell);
        })
    });

    group.bench_function("dense_lookup", |b| {
        b.iter(|| {
            let coord = Coordinate::new(black_box(5), black_box(5));
            let cell = dense_grid.get(coord);
            black_box(cell);
        })
    });

    group.throughput(Throughput::Elements(sparse_grid.size() as u64));
    group.bench_function("sparse_iteration", |b| {
        b.iter(|| {
            for (coord, cell) in sparse_grid.iter() {
                black_box(coord);
                black_box(cell);
            }
        })
    });

    group.throughput(Throughput::Elements(dense_grid.size() as u64));
    group.bench_function("dense_iteration", |b| {
        b.iter(|| {
            for (coord, cell) in dense_grid.iter() {
                black_box(coord);
                black_box(cell);
            }
        })
    });

    group.finish();
}

pub fn bench_grid_cell_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_cell_creation");

    group.bench_function("program_cell_new", |b| {
        b.iter(|| {
            let cell = ProgramCell::new(black_box('+'));
            black_box(cell);
        })
    });

    group.bench_function("is_flow_control_symbol", |b| {
        b.iter(|| {
            let result = ProgramCell::is_flow_control_symbol(black_box('^'));
            black_box(result);
        })
    });

    group.bench_function("is_operator_symbol", |b| {
        b.iter(|| {
            let result = ProgramCell::is_operator_symbol(black_box('+'));
            black_box(result);
        })
    });

    group.bench_function("is_valid_symbol", |b| {
        b.iter(|| {
            let result = ProgramCell::is_valid_symbol(black_box('A'));
            black_box(result);
        })
    });

    group.finish();
}

pub fn bench_grid_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_operations");

    group.bench_function("add_cell", |b| {
        b.iter(|| {
            let mut grid = ProgramGrid::new();
            for i in 0..100 {
                let coord = Coordinate::new(i as isize, i as isize);
                let _ = grid.add_cell(coord, '+');
            }
            black_box(grid);
        })
    });

    group.bench_function("dimensions", |b| {
        let grid = create_test_grid(100, 100);
        b.iter(|| {
            let (width, height) = grid.dimensions();
            black_box((width, height));
        })
    });

    group.bench_function("symbols_in_bounds", |b| {
        let grid = create_test_grid(50, 50);
        b.iter(|| {
            let symbols = grid.symbols_in_bounds();
            black_box(symbols);
        })
    });

    group.finish();
}

// Helper functions to create test grids
fn create_test_grid(width: usize, height: usize) -> ProgramGrid {
    let mut grid = ProgramGrid::new();

    for y in 0..height {
        for x in 0..width {
            let coord = Coordinate::new(x as isize, y as isize);
            let symbol = match (x + y) % 4 {
                0 => '+',
                1 => '|',
                2 => '-',
                _ => 'v',
            };
            let _ = grid.add_cell(coord, symbol);
        }
    }

    // Add start symbol
    let _ = grid.add_cell(Coordinate::new(0, 0), '@');

    grid
}

fn create_sparse_grid(width: usize, height: usize, cell_count: usize) -> ProgramGrid {
    let mut grid = ProgramGrid::new();

    for i in 0..cell_count {
        let x = (i * 7) % width;
        let y = (i * 13) % height;
        let coord = Coordinate::new(x as isize, y as isize);
        let symbol = match i % 4 {
            0 => '+',
            1 => '|',
            2 => '-',
            _ => 'v',
        };
        let _ = grid.add_cell(coord, symbol);
    }

    grid
}