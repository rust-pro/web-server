#[macro_use]
extern crate rocket;

use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenv::dotenv;
use rocket::{Build, fairing::AdHoc, Rocket, routes};

use crate::config::{AppConfig, read_config};
use crate::database::{connection::{Pool, PoolManager}, MySchema};
use crate::models::starword::QueryRoot;
use crate::routes::{graphql::*, hello::*};

mod database;
mod models;
mod routes;
mod config;

#[launch]
fn rocket() -> Rocket<Build> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::builder(mgr).build().unwrap();
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(db_pool).finish();
  rocket::build()
    .manage(schema).
    mount("/", routes![
        graphql_query,
        graphql_request,
        graphql_playground,
        hello
      ])
    .mount("/config", routes![read_config])
    .mount("/hello", routes![hello])
    .attach(AdHoc::config::<AppConfig>())
}