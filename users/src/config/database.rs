use std::env::var;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection() -> PgPool {
    let db_url = var("DATABASE_URL").expect("Cant get DB");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}
