use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use rocket::response::content;
use rocket::response::content::RawHtml;

#[get("/")]
pub fn index() -> RawHtml<String> {
  content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}