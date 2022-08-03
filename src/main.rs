mod routes;

#[cfg(test)]
mod test;

#[macro_use]
extern crate rocket;

use rocket::{Build, Config, Rocket, State};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use crate::routes::{home::*, dynamic_path::*, ignored_segments::*, forwarding::*};


#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    key: String,
    port: u16,
}

#[get("/")]
fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
    format!("{:#?}\n{:#?}", app_config, rocket_config)
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/dynamic_path", routes![dynamic_path])
        .mount("/ignored", routes![foo_bar, everything])
        .mount("/forwarding", routes![user_usize, user_isize, user_str])
        .mount("/config", routes![read_config])
        .attach(AdHoc::config::<AppConfig>())
}
