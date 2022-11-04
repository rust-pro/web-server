use async_graphql::*;
use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumString};

pub mod query_users;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}
