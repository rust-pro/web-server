use chrono::{DateTime, Local};
use diesel::prelude::*;

use crate::app::types::users::query_users::UserTypes;
use crate::database::schema::main::users;

#[derive(Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl From<&UserEntity> for UserTypes {
    fn from(entity: &UserEntity) -> Self {
        UserTypes {
            id: entity.id.into(),
            username: entity.username.clone(),
            email: entity.email.clone(),
            role: entity.role.clone(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}
