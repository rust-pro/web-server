use std::env;

use async_graphql::Context;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::MigrationHarness;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./src/database/migrations");

pub fn create_connection_pool() -> PgPool {
    dotenv::from_filename("./microservice/post-service/.env").ok();
    let db_url = env::var("DATABASE_URL").expect("Cant get DB");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn run_migrations(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) {
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run database migrations");
}

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> PooledConnection<ConnectionManager<PgConnection>> {
    ctx.data::<PgPool>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}
