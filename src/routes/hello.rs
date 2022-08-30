use rocket::State;

use crate::MySchema;

#[get("/hello")]
pub async fn hello(_schema: &State<MySchema>) -> String {
  "🚀 says hello!".to_string()
}