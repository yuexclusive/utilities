use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use once_cell::sync::OnceCell;

static mut CONFIG: OnceCell<Config> = OnceCell::new();
static MAILER: OnceCell<AsyncSmtpTransport<Tokio1Executor>> = OnceCell::new();

#[derive(Clone)]
struct Config {
    from: String,
    pwd: String,
    relay: String,
    port: u16,
}

pub async fn init(from: &str, pwd: &str, relay: &str, port: u16) {
    unsafe {
        CONFIG.get_or_init(|| Config {
            from: from.to_string(),
            pwd: pwd.to_string(),
            relay: relay.to_string(),
            port,
        });
    }

    match mailer().test_connection().await {
        Ok(v) => {
            if v {
                log::info!("email init success");
            } else {
                panic!("mail init failed, the connection not connected");
            }
        }
        Err(e) => {
            panic!("mail init failed, error: {}", e)
        }
    }
}

pub fn mailer() -> &'static AsyncSmtpTransport<Tokio1Executor> {
    MAILER.get_or_init(|| {
        let cfg = unsafe { CONFIG.get_unchecked() };
        let creds = Credentials::new(cfg.from.clone(), cfg.pwd.clone());
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.relay)
                .unwrap()
                .port(cfg.port)
                .credentials(creds)
                .build();

        mailer
    })
}

pub async fn send(
    to: &str,
    subject: &str,
    body: &str,
) -> Result<
    <AsyncSmtpTransport<Tokio1Executor> as AsyncTransport>::Ok,
    <AsyncSmtpTransport<Tokio1Executor> as AsyncTransport>::Error,
> {
    let from = &unsafe { CONFIG.get_unchecked() }.from;
    let email = Message::builder()
        .from(format!("evolve.publisher <{}>", from).parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(format!("{} <{}>", to, to).parse().unwrap())
        .subject(subject)
        .body(String::from(body))
        .unwrap();

    mailer().send(email).await
}
