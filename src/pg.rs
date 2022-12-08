#![cfg(feature = "pg")]

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::{Pool, Postgres, Transaction};
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Mutex;
use std::{env, result::Result};

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub type Executor = Pool<Postgres>;

pub async fn tran<'a>() -> SqlResult<Transaction<'a, Postgres>> {
    CONN.get().await.begin().await
}

#[derive(Clone)]
pub struct Config {
    user: String,
    pwd: String,
    host: String,
    port: String,
    db: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Default::default();
}

pub fn init(user: &str, pwd: &str, host: &str, port: u16, db: &str) {
    *CONFIG.lock().unwrap() = Some(Config {
        user: user.to_string(),
        pwd: pwd.to_string(),
        host: host.to_string(),
        port: port.to_string(),
        db: db.to_string(),
    });

    let url = CONFIG.lock().unwrap().clone();

    if let Ok(_) = fs::try_exists("./.env") {
        let mut file = File::create_new(".env").unwrap();
        let bs = format!("DATABADE_URL=\"{}\"", url.unwrap().to_string()).as_bytes();
        file.write(bs).unwrap();
        file.flush().unwrap();
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pwd, self.host, self.port, self.db,
        ))
    }
}

lazy_static! {
    pub static ref CONN: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        sqlx::postgres::PgPoolOptions::new()
            .test_before_acquire(false)
            .connect(&env::var("DATABADE_URL").unwrap())
            .await
            .unwrap()
    });
}
