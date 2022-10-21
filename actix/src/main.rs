extern crate actix;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

use actix::config::configure::configure;
use actix::config::database::{create_connection_pool, run_migrations};
use actix::config::graphql::create_schema_with_context;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool();
    run_migrations(&mut pool.get().expect("Can't get DB connection"));
    let schema = web::Data::new(create_schema_with_context(pool));
    println!("Actix Web đang khởi động http://127.0.0.1:99");
    HttpServer::new(move || {
        App::new().configure(configure).app_data(schema.clone())
    }).bind("0.0.0.0:99")?.run().await
}

