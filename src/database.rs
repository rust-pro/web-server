use async_trait::async_trait;
use deadpool::managed::{Manager, RecycleResult};
use sqlx::{Connection, Error as SqlxError, PgConnection};

pub struct PoolManager {
  pub url: String,
}

pub type Pool = deadpool::managed::Pool<PoolManager>;


// pub struct DbPool {}
//
// impl DbPool {
//   pub fn new(url: String, size: usize) -> Pool {
//     Pool::new(PoolManager { url }, size)
//   }
// }

#[async_trait]
impl Manager for PoolManager {
  type Type = PgConnection;
  type Error = SqlxError;
  async fn create(&self) -> Result<Self::Type, Self::Error> {
    PgConnection::connect(&self.url).await
  }
  async fn recycle(&self, obj: &mut Self::Type) -> RecycleResult<Self::Error> {
    Ok(obj.ping().await?)
  }
  fn detach(&self, _obj: &mut Self::Type) {}
}