use criterion::{Criterion, criterion_group, criterion_main};

use example::fibonacci::fib_criterion_benchmark;
use repository::check_existing_user_benchmark::check_existing_user_benchmark;
use utils::password::verify_password_benchmark;

mod example;
mod repository;
mod utils;

criterion_group!(
    name = example;
    config = Criterion::default().sample_size(10);
    targets =
        fib_criterion_benchmark,
);

criterion_group!(
    name = check_existing_user_benches;
    config = Criterion::default().sample_size(10);
    targets =
        check_existing_user_benchmark,
);

criterion_group!(
    name = verify_password_benches;
    config = Criterion::default().sample_size(10);
    targets =
        verify_password_benchmark,
);

criterion_main!(example, check_existing_user_benches, verify_password_benches);
