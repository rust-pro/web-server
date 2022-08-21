#[macro_use]
extern crate rocket;

use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenv::dotenv;
use rocket::{Build, Rocket, State};
use rocket::fairing::AdHoc;
use rocket::response::content;

use crate::config::{AppConfig, read_config};
use crate::database::{Pool, PoolManager};
use crate::models::QueryRoot;
use crate::routes::{dynamic_path::*, forwarding::*, home::*, ignored_segments::*, user::*};

mod routes;
mod config;
mod database;
mod models;

#[launch]
fn rocket() -> Rocket<Build> {
  // Ok(dotenv).expect("TODO: panic message");
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::new(mgr, 16);
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(db_pool).finish();
  rocket::build()
    .manage(schema)
    .mount("/", routes![index])
    .attach(AdHoc::config::<AppConfig>())
}
