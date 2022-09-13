use std::ops::DerefMut;

use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::{self, postgres::PgRow, Row};

use crate::config::database::{Pool, Query};

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
struct Post {
  id: i32,
  title: String,
  body: String,
  published: bool,
  created_at: DateTime<Utc>,
}

#[ComplexObject]
impl Post {
  pub async fn posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
    let pool = ctx.data::<Pool>().unwrap();
    let query_str = format!(
      r#"SELECT id, title, body, published, created_at, updated_at FROM main.posts"#
    );
    let result = sqlx::query_as::<_, Post>(query_str.as_str())
      .fetch_all(pool.get().await.unwrap().deref_mut())
      .await
      .unwrap();
    return Ok(result);
  }
}

#[Object]
impl Query {
  async fn posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
    let pool = ctx.data::<Pool>().unwrap();
    let query_str = format!("SELECT id, title, body, published, created_at, updated_at FROM main.posts");
    let result = sqlx::query(query_str.as_str())
      .map(|row: PgRow| Post {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        published: row.get("published"),
        created_at: row.get("created_at"),
      })
      .fetch_all(pool.get().await.unwrap().deref_mut())
      .await
      .unwrap();
    println!("{:#?}", result);
    return Ok(result);
  }
}
