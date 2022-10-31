#![cfg(feature = "redis")]
use lazy_static::lazy_static;
use redis::{aio::Connection, AsyncCommands, Commands, FromRedisValue, ToRedisArgs};
use serde::ser::Serialize;
use std::sync::Mutex;
pub mod derive {
    pub use redis_encoding_derive::{FromRedisValue, ToRedisArgs};
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Default::default();
}

#[derive(Clone, Default)]
struct Config {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl redis::IntoConnectionInfo for Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        Ok(redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(self.host, self.port),
            redis: redis::RedisConnectionInfo {
                username: self.username,
                password: self.password,
                ..Default::default()
            },
        })
    }
}

pub fn init(host: impl AsRef<str>, port: u16, username: Option<String>, password: Option<String>) {
    *CONFIG.lock().unwrap() = Some(Config {
        host: host.as_ref().to_owned(),
        port: port,
        username: username,
        password: password,
    });

    log::info!("email init success")
}

fn get_config() -> Config {
    CONFIG.lock().unwrap().clone().expect("please init redis")
}

pub async fn conn() -> redis::RedisResult<Connection> {
    let client = redis::Client::open(get_config())?;
    client.get_async_connection().await
}

async fn pubsub() -> redis::RedisResult<redis::aio::PubSub> {
    let client = redis::Client::open(get_config())?;
    let res = client.get_async_connection().await?.into_pubsub();
    Ok(res)
}

pub async fn subscribe(
    channel_name: &str,
) -> redis::RedisResult<impl futures::Stream<Item = redis::Msg>> {
    let mut pubsub = pubsub().await?;
    pubsub.subscribe(channel_name).await?;
    let stream = pubsub.into_on_message();
    Ok(stream)
}

pub async fn set<'a, K, V>(k: K, v: V) -> redis::RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set(k, v).await
}

pub async fn publish<'a, K, V>(channel: K, v: V) -> redis::RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.publish(channel, v).await
}

pub async fn set_ex<'a, K, V>(k: K, v: V, seconds: usize) -> redis::RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set_ex(k, v, seconds).await
}

pub async fn get<'a, K, V>(k: K) -> Result<V, redis::RedisError>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
    V: FromRedisValue,
{
    conn().await?.get::<_, V>(k).await
}

pub async fn ttl<'a, K>(k: K) -> Result<u32, redis::RedisError>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.ttl::<_, u32>(k).await
}

pub async fn exists<'a, K>(k: K) -> Result<bool, redis::RedisError>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.exists::<_, bool>(k).await
}

pub mod sync {
    use super::*;
    fn conn_sync() -> redis::RedisResult<redis::Connection> {
        let client = redis::Client::open(get_config())?;
        client.get_connection()
    }

    pub fn publish<'a, K, V>(channel: K, v: V) -> redis::RedisResult<()>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        V: Serialize + ToRedisArgs + Send + Sync + 'a,
    {
        conn_sync()?.publish(channel, v)
    }

    pub fn set<'a, K, V>(k: K, v: V) -> redis::RedisResult<()>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        V: Serialize + ToRedisArgs + Send + Sync + 'a,
    {
        conn_sync()?.set(k, v)
    }

    pub fn get<'a, K, V>(k: K) -> Result<V, redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
        V: FromRedisValue,
    {
        conn_sync()?.get::<_, V>(k)
    }

    pub fn set_ex<'a, K, V>(k: K, v: V, seconds: usize) -> redis::RedisResult<()>
    where
        K: ToRedisArgs + Send + Sync + 'a,
        V: Serialize + ToRedisArgs + Send + Sync + 'a,
    {
        conn_sync()?.set_ex(k, v, seconds)
    }

    pub fn ttl<'a, K>(k: K) -> Result<u32, redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
    {
        conn_sync()?.ttl::<_, u32>(k)
    }

    pub fn exists<'a, K>(k: K) -> Result<bool, redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
    {
        conn_sync()?.exists::<_, bool>(k)
    }

    pub fn subscribe(channel_name: &str) -> redis::RedisResult<redis::Msg> {
        let mut conn = conn_sync()?;
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(channel_name)?;
        let stream = pubsub.get_message()?;
        Ok(stream)
    }
}
