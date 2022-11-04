use async_graphql::*;
use chrono::{DateTime, Local};

#[derive(Debug, SimpleObject)]
pub struct UserTypes {
    pub id: ID,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>
}
