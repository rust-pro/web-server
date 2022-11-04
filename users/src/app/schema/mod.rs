use actix_web::HttpResponse;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};

pub mod user_schema;

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}
