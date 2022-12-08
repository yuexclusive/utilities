#![cfg(feature = "pg")]

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::{Pool, Postgres, Transaction};
use std::result::Result;

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub type Executor = Pool<Postgres>;

pub async fn tran<'a>() -> SqlResult<Transaction<'a, Postgres>> {
    CONN.get().await.begin().await
}

lazy_static! {
    pub static ref CONN: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        sqlx::postgres::PgPoolOptions::new()
            .test_before_acquire(false)
            .connect(std::env!("DATABASE_URL").unwrap())
            .await
            .unwrap()
    });
}
