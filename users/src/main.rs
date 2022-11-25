extern crate users;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use web::Data;

use users::app::schema::user_schema::user_route;
use users::config::database::{create_connection, PgPool};
use users::config::graphql::create_schema_with_context;
use users::database::migrate::run_migrations;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool: PgPool = create_connection();
    run_migrations(&mut pool.get().expect("Can't run migrate"));
    let schema = Data::new(create_schema_with_context(pool));
    println!("Server on start: http://localhost:1405/");
    println!("Thiên di");
    HttpServer::new(move || {
        App::new().configure(user_route).app_data(schema.clone())
    }).bind("0.0.0.0:80")?.run().await
}
