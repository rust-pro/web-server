use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_trait::async_trait;
use deadpool::managed::{Manager, RecycleResult};
use dotenv::dotenv;
use sqlx::{Connection, Error as SqlxError, PgConnection};

pub struct Query;

pub struct PoolManager {
  pub url: String,
}

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub type Pool = deadpool::managed::Pool<PoolManager>;

#[async_trait]
impl Manager for PoolManager {
  type Type = PgConnection;
  type Error = SqlxError;
  async fn create(&self) -> Result<PgConnection, SqlxError> {
    PgConnection::connect(&self.url).await
  }
  async fn recycle(&self, obj: &mut PgConnection) -> RecycleResult<SqlxError> {
    Ok(obj.ping().await?)
  }
  fn detach(&self, _obj: &mut Self::Type) {}
}

pub fn connection() -> Schema<Query, EmptyMutation, EmptySubscription> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::builder(mgr).build().unwrap();
  Schema::build(Query, EmptyMutation, EmptySubscription).data(db_pool).finish()
}



