use async_once::AsyncOnce;
use once_cell::sync::OnceCell;
use sqlx::{MySql, Pool, Transaction};
use std::result::Result;
pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;
static mut POOL: OnceCell<AsyncOnce<Pool<MySql>>> = OnceCell::new();

pub async fn tran<'a>() -> SqlResult<Transaction<'a, MySql>> {
    conn().await.begin().await
}

pub async fn conn() -> &'static Pool<MySql> {
    unsafe {
        POOL.get_or_init(|| -> AsyncOnce<Pool<MySql>> {
            AsyncOnce::new(async {
                sqlx::mysql::MySqlPoolOptions::new()
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
    log::info!("mysql init success");
}
