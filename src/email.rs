#![cfg(feature = "email")]

use lazy_static::lazy_static;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Default::default();
}

#[derive(Clone)]
struct Config {
    from: String,
    pwd: String,
    relay: String,
    port: u16,
}

pub fn init(from: &str, pwd: &str, relay: &str, port: u16) {
    *CONFIG.lock().unwrap() = Some(Config {
        from: from.to_string(),
        pwd: pwd.to_string(),
        relay: relay.to_string(),
        port: port,
    });

    log::info!("email init success")
}

pub async fn send(
    to: &str,
    subject: &str,
    body: &str,
) -> Result<
    <AsyncSmtpTransport<Tokio1Executor> as AsyncTransport>::Ok,
    <AsyncSmtpTransport<Tokio1Executor> as AsyncTransport>::Error,
> {
    let cfg = CONFIG
        .lock()
        .unwrap()
        .clone()
        .expect("please init email first");

    let email = Message::builder()
        .from(format!("evolve.publisher <{}>", &cfg.from).parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(format!("{} <{}>", to, to).parse().unwrap())
        .subject(subject)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(cfg.from, cfg.pwd);

    // Open a remote connection to gmail
    // let mailer = SmtpTransport::relay(&cfg.relay)
    //     .unwrap()
    //     .port(cfg.port)
    //     .credentials(creds)
    //     .build();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.relay)
            .unwrap()
            .port(cfg.port)
            .credentials(creds)
            .build();

    // Send the email
    mailer.send(email).await
}
