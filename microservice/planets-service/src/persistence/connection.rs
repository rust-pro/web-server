use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    dotenv::from_filename("./microservice/planets-service/.env").ok();
    let db_url = env::var("DATABASE_URL").expect("Cant get DB");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
