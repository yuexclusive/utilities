#![cfg(feature = "meilisearch")]

use lazy_static::lazy_static;
use meilisearch_sdk::client::Client;
pub use meilisearch_sdk::settings::Settings;
use std::sync::Mutex;

#[derive(Clone)]
struct Config {
    address: String,
    api_key: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Default::default();
}

lazy_static! {
    pub static ref CONN: Client = {
        let cfg = CONFIG
            .lock()
            .unwrap()
            .clone()
            .expect("please init config first");
        Client::new(cfg.address, cfg.api_key)
    };
}

pub fn init(address: &str, api_key: &str) {
    *CONFIG.lock().unwrap() = Some(Config {
        address: address.to_string(),
        api_key: api_key.to_string(),
    });

    log::info!("meilisearch init success")
}
