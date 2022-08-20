#[macro_use]
extern crate rocket;

use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenv::dotenv;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;

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
  Ok(dotenv).expect("TODO: panic message");
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::new(mgr, 16);
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(db_pool).finish();
  rocket::build()
    .manage(schema)
    .mount("/", routes![index])
    .mount("/dynamic_path", routes![dynamic_path])
    .mount("/ignored", routes![foo_bar, everything])
    .mount("/forwarding", routes![user_usize, user_isize, user_str])
    .mount("/config", routes![read_config])
    .mount("/", routes![new_user])
    .mount("/", routes![new])
    .attach(AdHoc::config::<AppConfig>())
}
