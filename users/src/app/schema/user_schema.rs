use actix_web::web;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::app::resolvers::user_resolver::UserSchema;
use crate::app::schema::index_playground;

pub fn user_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(user_schema))
            .route(web::get().to(index_playground)),
    );
}

async fn user_schema(schema: web::Data<UserSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
