use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    transport::smtp::{response::Response, Error},
    Message, SmtpTransport, Transport,
};
use once_cell::sync::OnceCell;

static mut CONFIG: OnceCell<Config> = OnceCell::new();
static MAILER: OnceCell<SmtpTransport> = OnceCell::new();

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

    match mailer().test_connection() {
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

pub fn mailer() -> &'static SmtpTransport {
    MAILER.get_or_init(|| {
        let cfg = unsafe { CONFIG.get_unchecked() };
        let creds = Credentials::new(cfg.from.clone(), cfg.pwd.clone());
        let res = SmtpTransport::relay(&cfg.relay)
            .unwrap()
            .port(cfg.port)
            .credentials(creds)
            .build();

        res
    })
}

pub async fn send(to: &str, subject: &str, body: &str) -> Result<Response, Error> {
    let from = &unsafe { CONFIG.get_unchecked() }.from;
    let email = Message::builder()
        .from(format!("evolve.publisher <{}>", from).parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(format!("reciver <{}>", to).parse().unwrap())
        .subject(subject)
        .body(String::from(body))
        .unwrap();

    mailer().send(&email)
}
