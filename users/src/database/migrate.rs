use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::MigrationHarness;

const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./src/database/migrations");

/**
Execute all unapplied migrations for a given migration source
*/
pub fn run_migrations(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) {
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run database migrations");
    println!("Run migrations done");
}
