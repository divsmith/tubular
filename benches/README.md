# Tubular Performance Benchmark Suite

This directory contains comprehensive performance benchmarks for the Tubular interpreter using the Criterion benchmarking framework.

## Overview

The benchmark suite covers all major components of the Tubular interpreter:

1. **Core Data Structures** (`core_data_structures.rs`)
   - Coordinate creation and operations
   - Direction operations and transformations
   - TubularBigInt arithmetic and comparisons

2. **Grid Operations** (`grid_operations.rs`)
   - Grid lookup and access patterns
   - Sparse vs dense grid performance
   - Grid iteration and traversal
   - Program cell operations

3. **Stack Operations** (`stack_operations.rs`)
   - Push/pop operations at various depths
   - Stack access patterns and bulk operations
   - Memory management and growth patterns

4. **Memory Operations** (`memory_operations.rs`)
   - Reservoir access patterns and iteration
   - Adjacent cell access
   - Memory pressure and fragmentation tests

5. **Droplet Simulation** (`droplet_simulation.rs`)
   - Single and multiple droplet creation
   - Movement and collision detection
   - Complex scenario simulations

6. **Parser Performance** (`parser_performance.rs`)
   - Small, medium, and large program parsing
   - Validation performance
   - Character processing and grid construction

7. **Program Execution** (`program_execution.rs`)
   - Simple, arithmetic, memory, and complex programs
   - Execution scaling with program size
   - Performance target validation (1000+ droplets, 1000x1000 grids)

8. **Memory Usage** (`memory_usage.rs`)
   - Allocation pattern analysis
   - Memory pressure testing
   - Growth and fragmentation patterns

## Running Benchmarks

### Run all benchmarks:
```bash
cargo bench
```

### Run specific benchmark groups:
```bash
# Core data structures
cargo bench --bench performance coordinate_creation

# Grid operations
cargo bench --bench performance grid_lookup

# Stack operations
cargo bench --bench performance stack_push_pop

# Memory operations
cargo bench --bench performance reservoir_access

# Parser performance
cargo bench --bench performance parser_small_programs

# Program execution
cargo bench --bench performance simple_programs

# Droplet simulation
cargo bench --bench performance droplet_creation

# Memory usage
cargo bench --bench performance memory_allocation_patterns
```

### View benchmark results:
```bash
# Generate HTML report
cargo bench --bench performance -- --output-format html

# Save results to file
cargo bench --bench performance | tee benchmark_results.txt
```

## Performance Targets

The benchmarks validate these performance targets:

- **1000+ Droplets**: Can simulate 1000+ active droplets efficiently
- **1000x1000 Grids**: Can handle large program grids (1M cells)
- **Sub-millisecond Operations**: Core operations complete in microseconds
- **Memory Efficiency**: Memory usage scales appropriately with program size

## Benchmark Categories

### 1. Core Data Structure Benchmarks

**Coordinate Operations:**
- `coordinate_creation/new`: Creating new coordinates
- `coordinate_creation/origin`: Creating origin coordinates
- `coordinate_creation/offset`: Coordinate offset operations
- `coordinate_operations/manhattan_distance`: Distance calculations
- `coordinate_operations/add_direction`: Direction-based movement

**Direction Operations:**
- `direction_operations/dx_dy`: Getting coordinate offsets
- `direction_operations/opposite`: Getting opposite directions
- `direction_operations/turn_left/right`: Direction transformations

**BigInt Operations:**
- `bigint_creation/new_i64`: Creating from i64 values
- `bigint_creation/from_bigint_large`: Creating from large BigInt values
- `bigint_arithmetic/add_small`: Small integer addition
- `bigint_arithmetic/mul_large`: Large integer multiplication
- `bigint_arithmetic/increment_decrement`: Increment/decrement operations

### 2. Grid Operation Benchmarks

**Lookup Performance:**
- `grid_lookup/lookup_small`: Small grid (10x10) lookups
- `grid_lookup/lookup_large`: Large grid (1000x1000) lookups
- `grid_lookup/lookup_missing`: Missing cell lookups

**Iteration Performance:**
- `grid_iteration/iter`: Grid iteration with throughput metrics
- `grid_iteration/sparse_iteration`: Sparse grid traversal
- `grid_iteration/dense_iteration`: Dense grid traversal

**Grid Construction:**
- `grid_operations/add_cell`: Adding cells to grids
- `grid_operations/dimensions`: Calculating grid dimensions
- `grid_sparse_vs_dense`: Sparse vs dense grid comparisons

### 3. Stack Operation Benchmarks

**Basic Operations:**
- `stack_push_pop/push_only`: Various sizes of push operations
- `stack_push_pop/pop_only`: Various sizes of pop operations
- `stack_push_pop/push_pop`: Combined push/pop cycles

**Advanced Operations:**
- `stack_operations/peek_depth`: Peeking at different stack depths
- `stack_operations/swap_top_two`: Stack top swapping
- `stack_operations/duplicate`: Stack duplication

**Bulk Operations:**
- `stack_bulk_operations/push_n`: Bulk push operations
- `stack_bulk_operations/pop_n`: Bulk pop operations
- `stack_bulk_operations/clear_truncate`: Stack clearing and truncation

### 4. Memory Operation Benchmarks

**Reservoir Access:**
- `reservoir_access_patterns/get_small_medium_large`: Various size lookups
- `reservoir_access_patterns/put_small_large`: Various size storage
- `reservoir_adjacent_access`: Adjacent cell retrieval

**Reservoir Operations:**
- `reservoir_iteration/iter_values_keys`: Different iteration patterns
- `reservoir_operations/contains_remove`: Basic operations
- `reservoir_filter_operations/count_non_zero`: Zero-value filtering

### 5. Droplet Simulation Benchmarks

**Creation & Movement:**
- `droplet_creation/new_basic_with_value`: Droplet creation patterns
- `droplet_movement/next_position`: Position calculation
- `droplet_movement/move_to_set_direction`: Movement operations

**Multi-Droplet:**
- `multiple_droplets/create_many`: Creating many droplets
- `multiple_droplets/collision_check_all`: Collision detection
- `droplet_complex_scenarios`: Complex simulation scenarios

### 6. Parser Performance Benchmarks

**Program Parsing:**
- `parser_small_programs`: Small programs (1-10 lines)
- `parser_medium_programs`: Medium programs (10-50 lines)
- `parser_large_programs`: Large programs (50+ lines)

**Validation & Character Processing:**
- `parser_validation/validate_valid_invalid`: Validation performance
- `parser_character_processing`: Symbol recognition and validation

**Grid Construction:**
- `parser_grid_construction/construct_dense_sparse`: Grid building patterns
- `parser_error_handling`: Error handling performance

### 7. Program Execution Benchmarks

**Program Types:**
- `simple_programs`: Hello world, basic arithmetic, constants
- `arithmetic_programs`: Mathematical computations (factorial, fibonacci)
- `memory_programs`: Reservoir operations and patterns
- `complex_programs`: Multi-droplet and control flow programs

**Execution Scaling:**
- `execution_scaling`: Performance with increasing program size
- `performance_targets`: 1000+ droplets and 1000x1000 grid targets

### 8. Memory Usage Benchmarks

**Allocation Patterns:**
- `memory_allocation_patterns`: Core structure allocation
- `memory_pressure`: Large data structure pressure testing
- `memory_growth_patterns`: Exponential growth scenarios

**Memory Reuse:**
- `memory_reuse_patterns`: Stack and reservoir reuse patterns
- `memory_fragmentation`: Fragmentation testing

## Benchmark Results Interpretation

### Throughput Metrics
Many benchmarks use `Throughput::Elements` to measure operations per second. This helps compare efficiency across different input sizes.

### Timing Metrics
All benchmarks measure average execution time with statistical confidence intervals. Look for:
- **Mean**: Average execution time
- **StdDev**: Variability in execution time
- **Median**: Typical execution time
- **Min/Max**: Best and worst case performance

### Performance Regression Detection
The Criterion framework automatically detects performance regressions by comparing new results against saved baselines.

## Benchmark Configuration

The benchmarks are configured in `Cargo.toml`:

```toml
[[bench]]
name = "performance"
harness = false
```

This tells Cargo to use Criterion's harness instead of the default test harness.

## Adding New Benchmarks

To add new benchmarks:

1. Create or modify the appropriate module file
2. Add the benchmark function to the module
3. Include the function in the `criterion_group!` macro in `performance.rs`
4. Follow the naming conventions and patterns established

### Benchmark Template

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_new_feature(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_feature");

    group.bench_function("basic_operation", |b| {
        b.iter(|| {
            // Your benchmark code here
            let result = your_operation(black_box(input_data));
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_new_feature);
criterion_main!(benches);
```

## Best Practices

1. **Use `black_box()`** to prevent compiler optimizations
2. **Group related benchmarks** for logical organization
3. **Use meaningful benchmark names** that describe what's being measured
4. **Test with realistic data sizes** that represent actual usage
5. **Include throughput metrics** for operations that scale with input size
6. **Document benchmarks** with comments explaining what they measure
7. **Run benchmarks on consistent hardware** for reliable comparisons

## Performance Monitoring

Regularly run benchmarks to:

- Detect performance regressions
- Identify optimization opportunities
- Validate performance targets
- Track performance improvements over time
- Establish performance baselines for new features

## Integration with CI

These benchmarks can be integrated into CI pipelines to automatically detect performance regressions. Consider:

- Running benchmarks on a dedicated performance testing machine
- Using performance budgets to prevent regressions
- Storing benchmark results for historical comparison
- Setting up alerts for significant performance changes