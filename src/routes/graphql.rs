use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::response::content;
use rocket::State;
use crate::MySchema;

#[rocket::get("/")]
pub fn graphql_playground() -> content::RawHtml<String> {
  content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<MySchema>, query: GraphQLQuery) -> GraphQLResponse {
  query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(schema: &State<MySchema>, request: GraphQLRequest) -> GraphQLResponse {
  request.execute(schema).await
}