pub use thiserror::Error;
pub type BasicResult<T, E = ErrorKind> = Result<T, E>;

pub enum ErrCode {
    Business = 500000,
    Validate = 400000,
    Unauthorized = 401000,
    // Hint = 452000,
    Timeout = 408000,
    Other = 600000,
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("[ err_code: {} ] business error: {}",.err_code,.msg)]
    Business { msg: String, err_code: usize },

    #[error("[ err_code: {} ] validate error: {}",.err_code,.msg)]
    Validate { msg: String, err_code: usize },

    #[error("[ err_code: {} ] unauthorized: {}",.err_code,.msg)]
    Unauthorized { msg: String, err_code: usize },

    #[error("hint: {}",.0)]
    Hint(String),

    #[error("timeout")]
    Timeout,

    #[cfg(feature = "postgres")]
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[cfg(feature = "redis")]
    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    // #[error(transparent)]
    // Bincode(#[from] bincode::Error),
    #[cfg(feature = "actix-web")]
    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    // #[cfg(feature = "actix-web")]
    // #[error(transparent)]
    // SendError(#[from] tokio::sync::mpsc::error::SendError),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),

    #[cfg(feature = "regex")]
    #[error(transparent)]
    Regex(#[from] fancy_regex::Error),

    #[cfg(feature = "meilisearch")]
    #[error(transparent)]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),

    #[cfg(feature = "email")]
    #[error(transparent)]
    Lettre(#[from] lettre::transport::smtp::Error),
}

#[cfg(feature = "actix-web")]
use actix_web::{http::header::ContentType, HttpResponse};
#[cfg(feature = "actix-web")]
impl actix_web::error::ResponseError for ErrorKind {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ErrorKind::Business {
                msg: _,
                err_code: _,
            } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Validate {
                msg: _,
                err_code: _,
            } => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorKind::Unauthorized {
                msg: _,
                err_code: _,
            } => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorKind::Timeout => actix_web::http::StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Hint(_) => actix_web::http::StatusCode::from_u16(452).unwrap(),
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .insert_header(("access-control-allow-origin", "*"))
            .insert_header(("access-control-allow-headers", "authorization,content-type"))
            .insert_header((
                "access-control-allow-methods",
                "PATCH, POST, CONNECT, GET, TRACE, PUT, OPTIONS, DELETE, HEAD",
            ))
            .insert_header(("access-control-max-age", "3600"))
            .body(match self {
                ErrorKind::Business { msg, err_code }
                | ErrorKind::Validate { msg, err_code }
                | ErrorKind::Unauthorized { msg, err_code } => serde_json::to_string(
                    &util_response::ErrorResponse::new(&format!("{}", msg), Some(*err_code)),
                )
                .unwrap(),
                ErrorKind::Hint(msg) => {
                    serde_json::to_string(&util_response::MsgResponse::new(msg)).unwrap()
                }
                _ => serde_json::to_string(&util_response::MsgResponse::new(&self.to_string()))
                    .unwrap(),
            })
    }
}

impl<T> From<ErrorKind> for Result<T, ErrorKind> {
    fn from(value: ErrorKind) -> Self {
        Err(value)
    }
}

#[macro_export]
macro_rules! business_error {
    ($msg: expr) => {{
        business_error!($msg, util_error::ErrCode::Business as usize)
    }};

    ($msg: expr, $err_code: expr) => {{
        let res = util_error::ErrorKind::Business {
            msg: $msg.to_string(),
            err_code: $err_code,
        };
        log::error!("{}", res);
        res
    }};
}

#[macro_export]
macro_rules! validate_error {
    ($msg: expr) => {{
        validate_error!($msg, util_error::ErrCode::Validate as usize)
    }};

    ($msg: expr, $err_code: expr) => {{
        let res = util_error::ErrorKind::Validate {
            msg: $msg.to_string(),
            err_code: $err_code,
        };
        log::error!("{}", res);
        res
    }};
}

#[macro_export]
macro_rules! hint {
    ($msg: expr) => {{
        let res = util_error::ErrorKind::Hint($msg.to_string());
        log::warn!("{}", res);
        res
    }};
}

#[macro_export]
macro_rules! unauthorized {
    ($msg: expr) => {{
        unauthorized!($msg, util_error::ErrCode::Unauthorized as usize)
    }};

    ($msg: expr, $err_code: expr) => {{
        let res = util_error::ErrorKind::Unauthorized {
            msg: $msg.to_string(),
            err_code: $err_code,
        };
        log::error!("{}", res);
        res
    }};
}
