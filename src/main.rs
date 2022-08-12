mod routes;
mod config;
mod api;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use crate::config::{AppConfig, read_config};
use crate::routes::{home::*, dynamic_path::*, ignored_segments::*, forwarding::*, user::*};


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        // .attach(api::stage())
        .mount("/", routes![index])
        .mount("/dynamic_path", routes![dynamic_path])
        .mount("/ignored", routes![foo_bar, everything])
        .mount("/forwarding", routes![user_usize, user_isize, user_str])
        .mount("/config", routes![read_config])
        .mount("/", routes![new_user])
        .mount("/", routes![new])
        .attach(AdHoc::config::<AppConfig>())
}
