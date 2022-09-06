#[macro_use]
extern crate rocket;

use rocket::{Build, fairing::AdHoc, Rocket, routes};

use crate::config::{app::{AppConfig, read_config}, database::connection};
use crate::routes::{graphql::*, hello::*};

mod models;
mod routes;
mod config;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build()
    .manage(connection()).
    mount("/graphql", routes![graphql_query,graphql_request,graphql_playground,hello])
    .mount("/config", routes![read_config])
    .mount("/hello", routes![hello])
    .attach(AdHoc::config::<AppConfig>())
}