use actix_web::{HttpResponse, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::app::resolvers::graphql::AppSchema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}
