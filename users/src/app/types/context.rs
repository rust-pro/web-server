use async_graphql::Context;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::config::database::PgPool;

pub fn context(ctx: &Context<'_>) -> PooledConnection<ConnectionManager<PgConnection>> {
    ctx.data::<PgPool>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}
