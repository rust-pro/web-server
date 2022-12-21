extern crate users;

use std::env::var;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use web::Data;

use users::app::schema::user_schema::user_route;
use users::config::database::{create_connection, PgPool};
use users::config::graphql::{user_create_schema_with_context};
use users::database::migrate::run_migrations;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let url: String = var("APP_URL").expect("Cant get DB");
    let pool: PgPool = create_connection();
    run_migrations(&mut pool.get().expect("Can't run migrate"));
    let schema = Data::new(user_create_schema_with_context(pool));
    println!("Server on start: http://{}/", url);
    HttpServer::new(move || {
        App::new().configure(user_route).app_data(schema.clone())
    }).bind(url)?.run().await
}
