use actix_web::{HttpRequest, web};
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

async fn user_schema(schema: web::Data<UserSchema>, http_req: HttpRequest, req: GraphQLRequest) -> GraphQLResponse {
    let mut query = req.into_inner();
    let getting_role_result = common_utils::get_role(http_req);
    query = query.data(getting_role_result);
    schema.execute(query).await.into()
}
