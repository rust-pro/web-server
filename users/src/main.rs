extern crate users;

use std::env::{var_os};
use actix_web::{App, HttpServer, web};
use dotenv::{dotenv, var};
use web::Data;

use users::app::schema::user_schema::user_route;
use users::config::database::{create_connection, PgPool};
use users::config::graphql::{user_create_schema_with_context};
use users::database::migrate::run_migrations;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_url = var("APP_URL").unwrap_or("0.0.0.0".into()); // Docker: 0.0.0.0
    let port = var_os("APP_PORT").unwrap_or("80".into());
    let port = port.to_str().unwrap().parse::<u16>().unwrap();
    let pool: PgPool = create_connection();
    run_migrations(&mut pool.get().expect("Can't run migrate"));
    let schema = Data::new(user_create_schema_with_context(pool));
    println!("Server on start: http://{}:{}", app_url, port);

    HttpServer::new(move || {
        App::new().configure(user_route).app_data(schema.clone())
    }).bind((app_url, port))?.run().await
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
