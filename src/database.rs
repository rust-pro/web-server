use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::Query;

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub mod connection;