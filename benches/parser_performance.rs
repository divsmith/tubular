use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::parser::grid_parser::GridParser;
use tubular::parser::validator;
use tubular::interpreter::grid::ProgramGrid;

pub fn bench_parser_small_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_small_programs");

    // Small programs (1-10 lines)
    let programs = [
        ("single_char", "@\n"),
        ("simple_arith", "5 3 + A n,\n"),
        ("small_loop", "@>\n1v\n<-\n"),
        ("calculator", "@|\n?:\nA n,\n!\n"),
        ("memory_test", "@7:15:42P|10:20:99P7:15Gn,10:20Gn,!\n"),
    ];

    for (name, program) in programs.iter() {
        group.bench_with_input(BenchmarkId::new("parse", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(*program));
                black_box(result);
            })
        });

        group.bench_with_input(BenchmarkId::new("parse_validate", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let grid = parser.parse(black_box(*program)).unwrap();
                let validation = validator::validate_program(&grid);
                black_box(validation);
            })
        });
    }

    group.finish();
}

pub fn bench_parser_medium_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_medium_programs");

    // Medium programs (10-50 lines)
    let programs = [
        ("medium_calculator", create_medium_calculator()),
        ("memory_operations", create_memory_program()),
        ("flow_control", create_flow_control_program()),
        ("subroutine_test", create_subroutine_program()),
    ];

    for (name, program) in programs.iter() {
        group.throughput(Throughput::Bytes(program.len() as u64));

        group.bench_with_input(BenchmarkId::new("parse", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(program.as_str()));
                black_box(result);
            })
        });

        group.bench_with_input(BenchmarkId::new("parse_validate", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let grid = parser.parse_string(black_box(program.as_str())).unwrap();
                let validation = validator::validate_program(&grid);
                black_box(validation);
            })
        });
    }

    group.finish();
}

pub fn bench_parser_large_programs(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_large_programs");

    // Large programs (50+ lines)
    let programs = [
        ("large_arithmetic", create_large_arithmetic_program()),
        ("large_grid", create_large_grid_program()),
        ("complex_simulation", create_complex_simulation_program()),
    ];

    for (name, program) in programs.iter() {
        group.throughput(Throughput::Bytes(program.len() as u64));

        group.bench_with_input(BenchmarkId::new("parse", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(program.as_str()));
                black_box(result);
            })
        });

        group.bench_with_input(BenchmarkId::new("parse_validate", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let grid = parser.parse_string(black_box(program.as_str())).unwrap();
                let validation = validator::validate_program(&grid);
                black_box(validation);
            })
        });
    }

    group.finish();
}

pub fn bench_parser_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_validation");

    let valid_programs = [
        ("simple_valid", "@|\n5 3 + A n,\n!\n"),
        ("complex_valid", &create_valid_complex_program()),
    ];

    let invalid_programs = [
        ("no_start", "5 3 + A n,\n!\n"),
        ("multiple_starts", "@\n@\n5 3 + A n,\n!\n"),
        ("invalid_chars", "@\n5 3 & A n,\n!\n"),
    ];

    for (name, program) in valid_programs.iter() {
        group.bench_with_input(BenchmarkId::new("validate_valid", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let grid = parser.parse_string(black_box(program.as_str())).unwrap();
                let validation = validator::validate_program(&grid);
                black_box(validation);
            })
        });
    }

    for (name, program) in invalid_programs.iter() {
        group.bench_with_input(BenchmarkId::new("validate_invalid", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let grid = parser.parse(black_box(program.as_str())).unwrap_or_default();
                let validation = validator::validate_program(&grid);
                black_box(validation);
            })
        });
    }

    group.finish();
}

pub fn bench_parser_character_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_character_processing");

    let test_strings = [
        ("numeric", "1234567890"),
        ("operators", "+-*/%=<>:;!"),
        ("flow_control", "|/\\^v<>"),
        ("mixed", "123+456-789*0/"),
    ];

    for (name, string) in test_strings.iter() {
        group.bench_with_input(BenchmarkId::new("is_operator_symbol", name), name, |b, _| {
            b.iter(|| {
                for ch in black_box(string).chars() {
                    tubular::interpreter::grid::ProgramCell::is_operator_symbol(ch);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("is_flow_control_symbol", name), name, |b, _| {
            b.iter(|| {
                for ch in black_box(string).chars() {
                    tubular::interpreter::grid::ProgramCell::is_flow_control_symbol(ch);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("is_valid_symbol", name), name, |b, _| {
            b.iter(|| {
                for ch in black_box(string).chars() {
                    tubular::interpreter::grid::ProgramCell::is_valid_symbol(ch);
                }
            })
        });
    }

    group.finish();
}

pub fn bench_parser_grid_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_grid_construction");

    for size in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements((size * size) as u64));

        group.bench_with_input(BenchmarkId::new("construct_dense", size), size, |b, &size| {
            let program = create_dense_program(size);
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(&program));
                black_box(result);
            })
        });

        group.bench_with_input(BenchmarkId::new("construct_sparse", size), size, |b, &size| {
            let program = create_sparse_program(size, size / 10);
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(&program));
                black_box(result);
            })
        });
    }

    group.finish();
}

pub fn bench_parser_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_error_handling");

    let problematic_programs = [
        ("very_wide", &["@".to_string(), " ".repeat(10000), "+\n".to_string()].concat()),
        ("very_tall", &"@+\n".repeat(1000)),
        ("mixed_whitespace", &"@ \t \n  \r + \n"),
        ("empty_lines", &"\n\n@\n\n+\n\n\n"),
    ];

    for (name, program) in problematic_programs.iter() {
        group.throughput(Throughput::Bytes(program.len() as u64));

        group.bench_with_input(BenchmarkId::new("parse_problematic", name), name, |b, _| {
            b.iter(|| {
                let mut parser = GridParser::new();
                let result = parser.parse_string(black_box(program.as_str()));
                black_box(result);
            })
        });
    }

    group.finish();
}

// Helper functions to create test programs
fn create_medium_calculator() -> String {
    r#"
@
|
5
:
3
:
A
n
,
!
"#.to_string()
}

fn create_memory_program() -> String {
    r#"
@
|
7
:
15
:
42
P
|
10
:
20
:
99
P
7
:
15
G
n
,
10
:
20
G
n
,
!
"#.to_string()
}

fn create_flow_control_program() -> String {
    r#"
@>
|
v
<
-
!
"#.to_string()
}

fn create_subroutine_program() -> String {
    r#"
@|
5S
3S
C
n,
!
"#.to_string()
}

fn create_large_arithmetic_program() -> String {
    let mut program = String::new();
    program.push_str("@\n|\n");
    for i in 0..100 {
        program.push_str(&format!("{}\n", i % 10));
        program.push_str("+\n");
    }
    program.push_str("n,\n!\n");
    program
}

fn create_large_grid_program() -> String {
    let mut program = String::new();
    for y in 0..100 {
        for x in 0..100 {
            let ch = match (x + y) % 8 {
                0 => '+',
                1 => '-',
                2 => '*',
                3 => '/',
                4 => '|',
                5 => '\\',
                6 => '/',
                _ => 'v',
            };
            program.push(ch);
        }
        program.push('\n');
    }
    program
}

fn create_complex_simulation_program() -> String {
    let mut program = String::new();
    program.push_str("@>\n");
    for i in 0..200 {
        program.push_str(&format!("{}v\n", i % 10));
        program.push_str("<\n");
    }
    program.push_str("n,\n!\n");
    program
}

fn create_valid_complex_program() -> String {
    r#"
@>|\n
123v\n
456-\n\n
789+\n\n
ABC*\n\n
nnn,\n\n
!\n
"#.to_string()
}

fn create_dense_program(size: usize) -> String {
    let mut program = String::new();
    for y in 0..size {
        for x in 0..size {
            if x == 0 && y == 0 {
                program.push('@');
            } else {
                let ch = match (x + y) % 6 {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    3 => '/',
                    4 => (x % 10).to_string().chars().next().unwrap(),
                    _ => '|',
                };
                program.push(ch);
            }
        }
        program.push('\n');
    }
    program
}

fn create_sparse_program(size: usize, cell_count: usize) -> String {
    let mut program = String::new();
    let mut cells = Vec::new();

    // Add start position
    cells.push((0, 0, '@'));

    // Add random sparse cells
    for i in 1..cell_count {
        let x = (i * 7) % size;
        let y = (i * 13) % size;
        let ch = match i % 6 {
            0 => '+',
            1 => '-',
            2 => '*',
            3 => '/',
            4 => (i % 10).to_string().chars().next().unwrap(),
            _ => '|',
        };
        cells.push((x, y, ch));
    }

    // Build the program string
    for y in 0..size {
        for x in 0..size {
            let mut found = false;
            for (cx, cy, ch) in &cells {
                if *cx == x && *cy == y {
                    program.push(*ch);
                    found = true;
                    break;
                }
            }
            if !found {
                program.push(' ');
            }
        }
        program.push('\n');
    }

    program
}