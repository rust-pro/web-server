use async_graphql::{Context, EmptySubscription, ID, Object, Schema, Result};

use user_repository::get_all;

use crate::app::repository::user_repository;
use crate::app::repository::user_repository::{check_existing_user, get_user_by_id};
use crate::app::requests::login_request::LoginRequest;
use crate::app::types::context::context;
use crate::app::types::users::query_users::UserTypes;
use crate::utils::password::verify_password;

/**
 * Type UserSchema
 */
pub type UserSchema = Schema<UserQuery, UserMutation, EmptySubscription>;

/**
 * Declare the Structure UserQuery
 */
pub struct UserQuery;

/**
 * Declare the Structure UserQuery
 */
pub struct UserMutation;

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
    ///Get user by ID
    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Option<UserTypes> {
        let id = id
            .to_string()
            .parse::<i32>()
            .expect("ddd");
        get_user_by_id(id, &mut context(ctx)).ok().map(|e| UserTypes::from(&e))
    }
}

#[Object]
impl UserMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginRequest) -> Result<String> {
        let existing_user = check_existing_user(&input.username, &mut context(ctx))?;
        if verify_password(&existing_user.password, &input.password)? {
            Ok("123".parse().unwrap())
        } else {
            Err("Loi roi".into())
        }
    }
}

