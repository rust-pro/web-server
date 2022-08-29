mod routes;
mod config;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use crate::config::{AppConfig, read_config};
use crate::routes::{home::*};


#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build()
    .mount("/", routes![index])
    .mount("/config", routes![read_config])
    .attach(AdHoc::config::<AppConfig>())
}
