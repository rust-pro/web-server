use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};

use user_repository::get_all;

use crate::app::repository::user_repository;
use crate::app::repository::user_repository::get_user;
use crate::app::types::context::context;
use crate::app::types::users::query_users::UserTypes;

pub struct UserQuery;

pub type UserSchema = Schema<UserQuery, EmptyMutation, EmptySubscription>;

#[Object]
impl UserQuery {
    /// Get all users
    async fn get_users(&self, ctx: &Context<'_>) -> Vec<UserTypes> {
        get_all(&mut context(ctx))
            .expect("Can't get users")
            .iter()
            .map(UserTypes::from)
            .collect()
    }

    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Option<UserTypes> {
        let id = id
            .to_string()
            .parse::<i32>()
            .expect("ddd");
        get_user(id, &mut context(ctx)).ok().map(|e| UserTypes::from(&e))
    }
}

