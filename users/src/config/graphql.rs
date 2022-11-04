use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::app::resolvers::user_resolver::UserQuery;
use crate::config::database::PgPool;

/**
# Title
 */
pub fn create_schema_with_context(pool: PgPool) -> Schema<UserQuery, EmptyMutation, EmptySubscription> {
    Schema::build(UserQuery, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}
