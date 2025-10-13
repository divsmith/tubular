use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::coordinate::Coordinate;
use tubular::types::direction::Direction;
use tubular::types::bigint::TubularBigInt;
use tubular::interpreter::grid::{ProgramGrid, ProgramCell};
use tubular::interpreter::stack::DataStack;
use tubular::interpreter::memory::{Reservoir, ReservoirCoordinate};
use tubular::interpreter::droplet::{Droplet, DropletId};
use tubular::parser::grid_parser::GridParser;
use num_bigint::BigInt;
use std::collections::HashMap;

mod core_data_structures;
mod grid_operations;
mod stack_operations;
mod memory_operations;
mod droplet_simulation;
mod parser_performance;
mod program_execution;
mod memory_usage;

use core_data_structures::*;
use grid_operations::*;
use stack_operations::*;
use memory_operations::*;
use droplet_simulation::*;
use parser_performance::*;
use program_execution::*;
use memory_usage::*;

criterion_group!(
    benches,
    // Core data structures
    bench_coordinate_creation,
    bench_coordinate_operations,
    bench_direction_operations,
    bench_bigint_creation,
    bench_bigint_arithmetic,

    // Grid operations
    bench_grid_lookup,
    bench_grid_iteration,
    bench_grid_sparse_vs_dense,
    bench_grid_cell_creation,

    // Stack operations
    bench_stack_push_pop,
    bench_stack_operations,
    bench_stack_peek_depth,

    // Memory operations
    bench_reservoir_access_patterns,
    bench_reservoir_adjacent_access,
    bench_reservoir_iteration,

    // Droplet simulation
    bench_droplet_creation,
    bench_droplet_movement,
    bench_droplet_collision_detection,
    bench_multiple_droplets,

    // Parser performance
    bench_parser_small_programs,
    bench_parser_medium_programs,
    bench_parser_large_programs,
    bench_parser_validation,

    // Program execution
    bench_simple_programs,
    bench_arithmetic_programs,
    bench_memory_programs,
    bench_complex_programs,

    // Memory usage
    bench_memory_allocation_patterns,
    bench_memory_pressure
);

criterion_main!(benches);