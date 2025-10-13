use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use tubular::types::bigint::TubularBigInt;
use tubular::interpreter::stack::DataStack;

pub fn bench_stack_push_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_push_pop");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("push_only", size), size, |b, &size| {
            b.iter(|| {
                let mut stack = DataStack::new();
                for i in 0..size {
                    stack.push(black_box(TubularBigInt::new(i as i64)));
                }
                black_box(stack);
            })
        });

        group.bench_with_input(BenchmarkId::new("pop_only", size), size, |b, &size| {
            let mut stack = DataStack::with_capacity(size);
            for i in 0..size {
                stack.push(TubularBigInt::new(i as i64));
            }

            b.iter(|| {
                let mut stack = stack.clone();
                for _ in 0..size {
                    let value = stack.pop();
                    black_box(value);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("push_pop", size), size, |b, &size| {
            b.iter(|| {
                let mut stack = DataStack::new();
                for i in 0..size {
                    stack.push(black_box(TubularBigInt::new(i as i64)));
                }
                for _ in 0..size {
                    let value = stack.pop();
                    black_box(value);
                }
            })
        });
    }

    group.finish();
}

pub fn bench_stack_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_operations");

    let mut stack = DataStack::new();
    for i in 0..100 {
        stack.push(TubularBigInt::new(i));
    }

    group.bench_function("peek", |b| {
        b.iter(|| {
            let value = stack.peek();
            black_box(value);
        })
    });

    group.bench_function("peek_depth", |b| {
        b.iter(|| {
            let value = stack.peek_depth(black_box(5));
            black_box(value);
        })
    });

    group.bench_function("swap_top_two", |b| {
        b.iter(|| {
            let mut test_stack = stack.clone();
            let result = test_stack.swap_top_two();
            black_box(result);
        })
    });

    group.bench_function("duplicate", |b| {
        b.iter(|| {
            let mut test_stack = stack.clone();
            let result = test_stack.duplicate();
            black_box(result);
        })
    });

    group.bench_function("is_empty", |b| {
        b.iter(|| {
            let empty = stack.is_empty();
            black_box(empty);
        })
    });

    group.bench_function("len", |b| {
        b.iter(|| {
            let len = stack.len();
            black_box(len);
        })
    });

    group.finish();
}

pub fn bench_stack_peek_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_peek_depth");

    let mut stack = DataStack::new();
    for i in 0..1000 {
        stack.push(TubularBigInt::new(i));
    }

    for depth in [0, 1, 5, 10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("peek_depth", depth), depth, |b, &depth| {
            b.iter(|| {
                let value = stack.peek_depth(black_box(depth));
                black_box(value);
            })
        });
    }

    group.finish();
}

pub fn bench_stack_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_bulk_operations");

    let values: Vec<TubularBigInt> = (0..100).map(|i| TubularBigInt::new(i)).collect();

    group.bench_function("push_n", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            stack.push_n(black_box(values.clone()));
            black_box(stack);
        })
    });

    group.bench_function("pop_n", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            for i in 0..100 {
                stack.push(TubularBigInt::new(i));
            }
            let popped = stack.pop_n(black_box(50));
            black_box(popped);
        })
    });

    group.bench_function("clear", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            for i in 0..100 {
                stack.push(TubularBigInt::new(i));
            }
            stack.clear();
            black_box(stack);
        })
    });

    group.bench_function("truncate", |b| {
        b.iter(|| {
            let mut stack = DataStack::new();
            for i in 0..100 {
                stack.push(TubularBigInt::new(i));
            }
            stack.truncate(black_box(50));
            black_box(stack);
        })
    });

    group.finish();
}

pub fn bench_stack_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_capacity");

    for capacity in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("with_capacity", capacity), capacity, |b, &capacity| {
            b.iter(|| {
                let stack = DataStack::with_capacity(black_box(capacity));
                black_box(stack);
            })
        });

        group.bench_with_input(BenchmarkId::new("fill_capacity", capacity), capacity, |b, &capacity| {
            b.iter(|| {
                let mut stack = DataStack::with_capacity(capacity);
                for i in 0..capacity {
                    stack.push(black_box(TubularBigInt::new(i as i64)));
                }
                black_box(stack);
            })
        });
    }

    group.finish();
}

pub fn bench_stack_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_access_patterns");

    let mut stack = DataStack::new();
    for i in 0..1000 {
        stack.push(TubularBigInt::new(i));
    }

    group.bench_function("get_index", |b| {
        b.iter(|| {
            for i in 0..100 {
                let value = stack.get(black_box(i));
                black_box(value);
            }
        })
    });

    group.bench_function("get_from_top", |b| {
        b.iter(|| {
            for i in 0..100 {
                let value = stack.get_from_top(black_box(i));
                black_box(value);
            }
        })
    });

    group.bench_function("as_slice", |b| {
        b.iter(|| {
            let slice = stack.as_slice();
            black_box(slice);
        })
    });

    group.finish();
}