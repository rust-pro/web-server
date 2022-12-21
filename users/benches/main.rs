use criterion::{Criterion, criterion_group, criterion_main};

use example::fibonacci::fib_criterion_benchmark;
use repository::check_existing_user_benchmark::check_existing_user_benchmark;

mod example;
mod repository;

criterion_group!(
    name = example;
    config = Criterion::default().sample_size(10);
    targets =
        fib_criterion_benchmark,
);

criterion_group!(
    name = benches_2;
    config = Criterion::default().sample_size(10);
    targets =
        check_existing_user_benchmark,
);

criterion_main!(example, benches_2);
