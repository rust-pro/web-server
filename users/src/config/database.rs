use std::env::var;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection() -> PgPool {
    let db_connection: String = var("DB_CONNECTION").unwrap_or("postgres".into());
    let db_username: String = var("DB_USERNAME").unwrap_or("postgres".into());
    let db_password: String = var("DB_PASSWORD").unwrap_or("password".into());
    let db_host: String = var("DB_HOST").unwrap_or("127.0.0.1".into());
    let db_port: String = var("DB_PORT").unwrap_or("5432".into());
    let db_database: String = var("DB_DATABASE").unwrap_or("users".into());

    let connection_manager = format!("{}://{}:{}@{}:{}/{}", db_connection, db_username, db_password, db_host, db_port, db_database);
    let manager = ConnectionManager::<PgConnection>::new(connection_manager);
    Pool::builder().build(manager).expect("Failed to create pool")
}
