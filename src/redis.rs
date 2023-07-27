#![cfg(feature = "redis")]

use once_cell::sync::OnceCell;
pub use redis;
use redis::{
    aio::{Connection, ConnectionLike, PubSub},
    AsyncCommands, Client, ConnectionAddr, ConnectionInfo, FromRedisValue, IntoConnectionInfo,
    RedisConnectionInfo, RedisResult, ToRedisArgs,
};
use serde::ser::Serialize;
pub mod derive {
    pub use redis_encoding_derive::{from_redis, to_redis};
}

static CLIENT: OnceCell<Client> = OnceCell::new();

static mut CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Default)]
struct Config {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl IntoConnectionInfo for Config {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        Ok(ConnectionInfo {
            addr: ConnectionAddr::Tcp(self.host, self.port),
            redis: RedisConnectionInfo {
                username: self.username,
                password: self.password,
                ..Default::default()
            },
        })
    }
}

pub async fn init(
    host: impl AsRef<str>,
    port: u16,
    username: Option<String>,
    password: Option<String>,
) {
    unsafe {
        CONFIG.get_or_init(|| Config {
            host: host.as_ref().to_owned(),
            port,
            username,
            password,
        });
    }

    match ping().await {
        Ok(redis::Value::Status(ref v)) => {
            if v == "PONG" {
                log::info!("redis init success");
            } else {
                panic!("redis init failed, status: {}", v);
            }
        }
        Err(e) => {
            panic!("redis init failed, error: {}", e)
        }
        other => {
            panic!("redis init failed, other: {:?}", other)
        }
    }
}

pub async fn conn() -> RedisResult<Connection> {
    let cfg = unsafe { CONFIG.get_unchecked() };
    let client = CLIENT.get_or_init(|| redis::Client::open(cfg.clone()).unwrap());
    client.get_async_connection().await
}

async fn pubsub() -> RedisResult<PubSub> {
    let res = conn().await?.into_pubsub();
    Ok(res)
}

pub async fn subscribe(channel_name: &str) -> RedisResult<impl futures::Stream<Item = redis::Msg>> {
    let mut pubsub = pubsub().await?;
    pubsub.subscribe(channel_name).await?;
    let stream = pubsub.into_on_message();
    Ok(stream)
}

pub async fn set<'a, K, V>(k: K, v: V) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set(k, v).await
}

pub async fn del<'a, K>(k: K) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.del(k).await
}

pub async fn publish<'a, K, V>(channel: K, v: V) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.publish(channel, v).await
}

pub async fn set_nx<'a, K, V>(k: K, v: V) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set_nx(k, v).await
}

pub async fn set_ex<'a, K, V>(k: K, v: V, seconds: usize) -> RedisResult<()>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set_ex(k, v, seconds).await
}

pub async fn get<'a, K, V>(k: K) -> RedisResult<V>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
    V: FromRedisValue,
{
    conn().await?.get::<_, V>(k).await
}

pub async fn ttl<'a, K>(k: K) -> RedisResult<u32>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.ttl::<_, u32>(k).await
}

pub async fn exists<'a, K>(k: K) -> RedisResult<bool>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.exists::<_, bool>(k).await
}

pub async fn ping() -> RedisResult<redis::Value> {
    conn().await?.req_packed_command(&redis::cmd("ping")).await
}
