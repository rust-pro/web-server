use rocket::serde::{Deserialize, json::Json};


#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Task<'r> {
    pub description: &'r str,
    pub complete: bool,
}


#[post("/user", format = "application/json", data = "<input>")]
pub fn new_user(input: &str) -> String {
    format!("Thoong tin user = {}", input)
}

#[post("/todo", data = "<task>", format = "json")]
pub fn new(task: Json<Task<'_>>) -> String {
    format!("{:#?}", task)
}