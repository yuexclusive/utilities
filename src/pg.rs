#![cfg(feature = "pg")]
use async_once::AsyncOnce;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres, Transaction};
use std::result::Result;

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub type Executor = Pool<Postgres>;

static mut POOL: OnceCell<AsyncOnce<Pool<Postgres>>> = OnceCell::new();

pub async fn tran<'a>() -> SqlResult<Transaction<'a, Postgres>> {
    conn().await.begin().await
}

pub async fn conn() -> &'static Pool<Postgres> {
    unsafe {
        POOL.get_or_init(|| -> AsyncOnce<Pool<Postgres>> {
            AsyncOnce::new(async {
                sqlx::postgres::PgPoolOptions::new()
                    .test_before_acquire(false)
                    .connect(&std::env::var("DATABASE_URL").unwrap())
                    .await
                    .unwrap()
            })
        })
    }
    .await
}

// If something wrong, it will be show at compile time
pub fn init() {
    dotenv::dotenv().unwrap();
    log::info!("✅pg init success");
}
