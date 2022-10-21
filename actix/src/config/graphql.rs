use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::app::resolvers::graphql::Query;
use crate::config::database::PgPool;

/**
# Title
 */
pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}
