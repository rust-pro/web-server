use criterion::{black_box, Criterion};
use dotenv::dotenv;
use users::app::entities::user_entity::UserEntity;
use users::app::repository::user_repository::check_existing_user;
use users::config::database::{create_connection, PgPool};

///Benchmark function check_existing_user()
pub fn check_existing_user_benchmark(c: &mut Criterion) {
    dotenv().ok();
    let pool: PgPool = create_connection();
    c.bench_function("Find User existing in Database", |b| {
        let mut conn = pool.get().unwrap();
        b.iter(|| {
            let _user: UserEntity = check_existing_user(black_box("admin"), &mut conn).unwrap();
        });
    });
}