use std::time::{Instant};
use async_graphql::{Context, EmptySubscription, ID, Object, Result, Schema};

use user_repository::get_all;

use crate::app::entities::user_entity::RegisterUserEntity;
use crate::app::repository::user_repository;
use crate::app::repository::user_repository::{check_existing_user, get_user_by_id, register};
use crate::app::requests::login_request::LoginRequest;
use crate::app::requests::register_request::RegisterRequest;
use crate::app::types::context::context;
use crate::app::types::users::query_users::UserTypes;
use crate::utils::password::{hash_password, verify_password};

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
    /**
    ### Login
    A resolver is a function that's responsible for populating the data for a single field in your schema
     */
    async fn login(&self, ctx: &Context<'_>, user: LoginRequest) -> Result<String> {
        let existing_user = check_existing_user(&user.username, &mut context(ctx))?;

        let start = Instant::now();
        if verify_password(&existing_user.password, &user.password)? {
            let duration = start.elapsed();
            println!("Time elapsed: {:?}", duration);
            Ok("123".parse().unwrap())
        } else {
            Err("Loi roi".into())
        }
    }

    /**
    ### Register
    A resolver is a function that's responsible for populating the data for a single field in your schema
     */
    async fn register(&self, ctx: &Context<'_>, user: RegisterRequest) -> Result<UserTypes> {
        let new_user = RegisterUserEntity {
            username: user.username,
            password: hash_password(user.password.as_str())?,
            email: user.email,
        };

        let created_user_entity = register(new_user, &mut context(ctx))?;
        Ok(UserTypes::from(&created_user_entity))
    }
}

