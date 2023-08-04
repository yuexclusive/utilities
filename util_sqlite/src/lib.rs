use async_once::AsyncOnce;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, Transaction};
use std::result::Result;
pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;
static mut POOL: OnceCell<AsyncOnce<Pool<Sqlite>>> = OnceCell::new();

pub async fn tran<'a>() -> SqlResult<Transaction<'a, Sqlite>> {
    conn().await.begin().await
}

pub async fn conn() -> &'static Pool<Sqlite> {
    unsafe {
        POOL.get_or_init(|| -> AsyncOnce<Pool<Sqlite>> {
            AsyncOnce::new(async {
                sqlx::sqlite::SqlitePoolOptions::new()
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
    log::info!("sqlite init success");
}