extern crate post;

use std::env;

use actix_web::{App, HttpServer, web};

use post::config::configure::configure;
use post::config::database::{create_connection_pool, run_migrations};
use post::config::graphql::create_schema_with_context;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("./microservice/post-service/.env").ok();
    let pool = create_connection_pool();
    let app_url: String = env::var("APP_URL").expect("Cant get DB");
    run_migrations(&mut pool.get().expect("Can't get DB connection"));
    let schema = web::Data::new(create_schema_with_context(pool));
    println!("start http://{}", app_url);
    HttpServer::new(move || {
        App::new().configure(configure).app_data(schema.clone())
    }).bind(app_url)?.run().await
}

