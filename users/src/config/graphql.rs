use async_graphql::{EmptySubscription, Schema};
use crate::app::resolvers::user_resolver::{UserMutation, UserQuery};
use crate::config::database::PgPool;

/**
# Title create_schema_with_context User
 */
pub fn user_create_schema_with_context(pool: PgPool) -> Schema<UserQuery, UserMutation, EmptySubscription> {
    Schema::build(UserQuery, UserMutation, EmptySubscription)
        .data(pool)
        .finish()
}
