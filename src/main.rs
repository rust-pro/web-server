mod routes;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use crate::routes::{home::*, dynamic_path::*, ignored_segments::*, forwarding::*};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/dynamic_path", routes![dynamic_path])
        .mount("/ignored", routes![foo_bar, everything])
        .mount("/forwarding", routes![user_usize, user_isize, user_str])
}
