use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::QueryRoot;

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub mod connection;