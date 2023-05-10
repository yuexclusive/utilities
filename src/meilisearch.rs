#![cfg(feature = "meilisearch")]

use meilisearch_sdk::client::Client;
pub use meilisearch_sdk::settings::Settings;
use once_cell::sync::OnceCell;

#[derive(Clone)]
struct Config {
    address: String,
    api_key: String,
}

static mut CONFIG: OnceCell<Config> = OnceCell::new();
static CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn init(address: &str, api_key: &str) {
    unsafe {
        CONFIG.get_or_init(|| Config {
            address: address.to_string(),
            api_key: api_key.to_string(),
        })
    };

    match client().get_stats().await {
        Ok(_status) => {
            log::info!("✅meilisearch init success")
        }
        Err(e) => {
            panic!("get_stats failed, error: {e}")
        }
    }
}

pub fn client() -> &'static Client {
    CLIENT.get_or_init(|| {
        let cfg = unsafe { CONFIG.get_unchecked() };
        Client::new(cfg.address.clone(), cfg.api_key.clone())
    })
}
