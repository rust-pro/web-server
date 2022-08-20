use rocket::{Config, State};
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
  pub key: String,
  pub port: u16,
}

#[get("/")]
pub fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
  format!("{:#?}\n{:#?}", app_config, rocket_config)
}

