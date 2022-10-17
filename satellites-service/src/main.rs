extern crate satellites_service;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

use satellites_service::{configure_service, create_schema_with_context, run_migrations};
use satellites_service::persistence::connection::create_connection_pool;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool();
    run_migrations(&mut pool.get().expect("Can't get DB connection"));

    let schema = web::Data::new(create_schema_with_context(pool));

    println!("Actix Web has launched from http://127.0.0.1:8002");
    HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    }).bind("0.0.0.0:8002")?.run().await
}
