use criterion::{black_box, Criterion};

fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

pub fn fib_criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
}