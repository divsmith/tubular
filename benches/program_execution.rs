use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::parser::grid_parser::GridParser;
use tubular::interpreter::execution::TubularInterpreter;
use tubular::interpreter::grid::ProgramGrid;

pub fn bench_simple_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_programs");

    let programs = [
        ("hello_world", create_hello_world_program()),
        ("simple_arith", create_simple_arithmetic_program()),
        ("constant_output", create_constant_output_program()),
        ("short_loop", create_short_loop_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.bench_with_input(BenchmarkId::new("execute", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone()))
                    .unwrap()
                    .with_options(false, false, Some(10000));
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_arithmetic_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic_programs");

    let programs = [
        ("addition_chain", create_addition_chain_program()),
        ("factorial", create_factorial_program()),
        ("fibonacci", create_fibonacci_program()),
        ("complex_math", create_complex_math_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.throughput(Throughput::Elements(grid.size() as u64));

        group.bench_with_input(BenchmarkId::new("execute", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone()))
                    .unwrap()
                    .with_options(false, false, Some(10000));
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_memory_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_programs");

    let programs = [
        ("memory_store_load", create_memory_store_load_program()),
        ("memory_patterns", create_memory_patterns_program()),
        ("large_reservoir", create_large_reservoir_program()),
        ("memory_intensive", create_memory_intensive_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.bench_with_input(BenchmarkId::new("execute", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_complex_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_programs");

    let programs = [
        ("multi_droplet", create_multi_droplet_program()),
        ("grid_wide", create_grid_wide_program()),
        ("deep_stack", create_deep_stack_program()),
        ("control_flow", create_control_flow_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.throughput(Throughput::Elements(grid.size() as u64));

        group.bench_with_input(BenchmarkId::new("execute", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_execution_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("execution_scaling");

    for size in [10, 50, 100, 500].iter() {
        let program = create_scaling_program(*size);
        let grid = parse_program(&program);

        group.throughput(Throughput::Elements((size * size) as u64));

        group.bench_with_input(BenchmarkId::new("scaled_execution", size), size, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_droplet_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("droplet_simulation");

    let programs = [
        ("single_path", create_single_path_program()),
        ("branching", create_branching_program()),
        ("collision_course", create_collision_course_program()),
        ("many_droplets", create_many_droplets_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.bench_with_input(BenchmarkId::new("simulate", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_io_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("io_operations");

    let programs = [
        ("heavy_output", create_heavy_output_program()),
        ("input_processing", create_input_processing_program()),
        ("mixed_io", create_mixed_io_program()),
    ];

    for (name, program_source) in programs.iter() {
        let grid = parse_program(program_source);

        group.bench_with_input(BenchmarkId::new("execute", name), name, |b, _| {
            b.iter(|| {
                let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
                let result = interpreter.run().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_performance_targets(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_targets");

    // Test for 1000+ droplets performance target
    let thousand_droplets_program = create_thousand_droplets_program();
    let grid = parse_program(&thousand_droplets_program);

    group.bench_function("thousand_droplets", |b| {
        b.iter(|| {
            let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
            let result = interpreter.run().unwrap();
            black_box(result);
        })
    });

    // Test for 1000x1000 grid performance target
    let large_grid_program = create_large_grid_target_program();
    let grid = parse_program(&large_grid_program);

    group.bench_function("large_grid_1000x1000", |b| {
        b.iter(|| {
            let mut interpreter = TubularInterpreter::new(black_box(grid.clone())).unwrap();
            let result = interpreter.run().unwrap();
            black_box(result);
        })
    });

    group.finish();
}

// Helper functions to create test programs
fn parse_program(source: &str) -> ProgramGrid {
    let parser = GridParser::new();
    parser.parse_string(source).expect("Failed to parse test program")
}

fn create_hello_world_program() -> String {
    r#"
@
H
e
l
l
o
,
W
o
r
l
d
!
,
!
"#.to_string()
}

fn create_simple_arithmetic_program() -> String {
    r#"
@
|
5
:
3
:
+
n
,
!
"#.to_string()
}

fn create_constant_output_program() -> String {
    r#"
@
42
n
,
42
n
,
42
n
,
!
"#.to_string()
}

fn create_short_loop_program() -> String {
    r#"
@>
|
v
1
+
n
,
^
<
!
"#.to_string()
}

fn create_addition_chain_program() -> String {
    r#"
@
|
1
+
+
+
+
+
+
+
+
+
n
,
!
"#.to_string()
}

fn create_factorial_program() -> String {
    r#"
@
5
>
1
S
:
1
-
:
S
C
*
n
,
!
"#.to_string()
}

fn create_fibonacci_program() -> String {
    r#"
@
1
:
1
:
+
n
,
S
C
+
n
,
S
C
+
n
,
!
"#.to_string()
}

fn create_complex_math_program() -> String {
    r#"
@
|
5
*
3
+
2
/
7
%
n
,
!
"#.to_string()
}

fn create_memory_store_load_program() -> String {
    r#"
@
|
10
:
20
:
42
P
|
10
:
20
:
G
n
,
!
"#.to_string()
}

fn create_memory_patterns_program() -> String {
    r#"
@
|
0
:
0
:
1
P
|
1
:
0
:
2
P
|
0
:
1
:
3
P
|
1
:
1
:
4
P
0
:
0
:
G
n
,
1
:
0
:
G
n
,
0
:
1
:
G
n
,
1
:
1
:
G
n
,
!
"#.to_string()
}

fn create_large_reservoir_program() -> String {
    let mut program = String::new();
    program.push_str("@\n|\n");
    for i in 0..100 {
        program.push_str(&format!("{}:{}:{}P\n", i, i, i * 2));
    }
    for i in 0..100 {
        program.push_str(&format!("{}:{}:Gn,\n", i, i));
    }
    program.push_str("!\n");
    program
}

fn create_memory_intensive_program() -> String {
    r#"
@
|
100
:
100
:
999
P
|
200
:
200
:
888
P
|
300
:
300
:
777
P
|
100
:
100
:
G
n
,
200
:
200
:
G
n
,
300
:
300
:
G
n
,
!
"#.to_string()
}

fn create_multi_droplet_program() -> String {
    r#"
@>+
|
1
v
2
v
3
v
!
"#.to_string()
}

fn create_grid_wide_program() -> String {
    let mut program = String::new();
    program.push_str("@");
    for _ in 0..100 {
        program.push('>');
    }
    program.push_str("12345n,\n");
    for _ in 0..100 {
        program.push('v');
    }
    program.push_str("!\n");
    program
}

fn create_deep_stack_program() -> String {
    let mut program = String::new();
    program.push_str("@\n");
    for i in 1..=100 {
        program.push_str(&format!("{}\n", i));
    }
    for _ in 0..100 {
        program.push_str("+\n");
    }
    program.push_str("n,\n!\n");
    program
}

fn create_control_flow_program() -> String {
    r#"
@>1<
|
v
2
v
3
v
4
v
5
v
!
"#.to_string()
}

fn create_scaling_program(size: usize) -> String {
    let mut program = String::new();
    program.push_str("@");
    for _ in 0..size {
        program.push('>');
    }
    program.push_str("123+n,\n");
    for _ in 0..size {
        program.push('v');
    }
    program.push_str("!\n");
    program
}

fn create_single_path_program() -> String {
    r#"
@>v
  |
  1
  +
  n
  ,
  !
"#.to_string()
}

fn create_branching_program() -> String {
    r#"
@>+v
   |
   1v
  / \
  2 3
  n n
  , ,
  v v
  \ /
   !
"#.to_string()
}

fn create_collision_course_program() -> String {
    r#"
@    @
>    <
   +
  n ,
  !
"#.to_string()
}

fn create_many_droplets_program() -> String {
    let mut program = String::new();
    for _i in 0..10 {
        program.push_str("@>\n");
    }
    program.push_str("123+n,\n");
    for _ in 0..10 {
        program.push_str("v\n");
    }
    program.push_str("!\n");
    program
}

fn create_heavy_output_program() -> String {
    let mut program = String::new();
    program.push_str("@\n");
    for i in 1..=100 {
        program.push_str(&format!("{}n,\n", i));
    }
    program.push_str("!\n");
    program
}

fn create_input_processing_program() -> String {
    r#"
@?
:
?
:
+
n
,
!
"#.to_string()
}

fn create_mixed_io_program() -> String {
    r#"
@?
+
n
,
?
*
n
,
?
+
?
*
n
,
!
"#.to_string()
}

fn create_thousand_droplets_program() -> String {
    let mut program = String::new();
    // Create a grid that spawns many droplets
    for _ in 0..32 {
        for _ in 0..32 {
            program.push_str("@>");
        }
        program.push('\n');
    }
    program.push_str("123+n,\n");
    for _ in 0..32 {
        for _ in 0..32 {
            program.push('v');
        }
        program.push('\n');
    }
    program.push_str("!\n");
    program
}

fn create_large_grid_target_program() -> String {
    let mut program = String::new();
    // Create a 1000x1000 grid with sparse content
    program.push_str("@");
    for _ in 0..999 {
        program.push(' ');
    }
    program.push('\n');

    for _ in 0..998 {
        program.push(' ');
        for _ in 0..998 {
            program.push(' ');
        }
        program.push_str("1+\n");
    }

    for _ in 0..999 {
        program.push(' ');
    }
    program.push_str("n,\n");

    for _ in 0..1000 {
        program.push('v');
    }
    program.push('\n');

    program.push(' ');
    for _ in 0..999 {
        program.push(' ');
    }
    program.push_str("!\n");

    program
}