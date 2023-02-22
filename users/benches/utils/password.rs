use criterion::{black_box, Criterion};
use users::utils::password::{generate_hash, verify_password};

pub fn verify_password_benchmark(c: &mut Criterion) {
    let hash = generate_hash("12345678").unwrap();
    let password = "12345678";
    c.bench_function("Verify password", |b| {
        b.iter(|| {
            verify_password(black_box(&hash), black_box(password)).unwrap();
        })
    });
}