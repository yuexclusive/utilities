#[cfg(feature = "response")]
use crate::response;
use std::error;
use std::fmt::Display;

// pub type BasicResult<T, E = Box<dyn Error>> = Result<T, E>;
pub type BasicResult<T, E = ErrorKind> = Result<T, E>;

#[derive(Debug)]
pub enum ErrorKind {
    BusinessError(String),
    ValidationError(String),
    Unauthorized(String),
    Timeout(String),
    Hint(String),
    OtherError(Box<dyn error::Error>),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::BusinessError(msg) => f.write_str(msg),
            ErrorKind::ValidationError(msg) => f.write_str(msg),
            ErrorKind::Unauthorized(msg) => f.write_str(msg),
            ErrorKind::Timeout(msg) => f.write_str(msg),
            ErrorKind::Hint(msg) => f.write_str(msg),
            ErrorKind::OtherError(err) => f.write_fmt(format_args!("other error: {}", err)),
        }
    }
}

impl std::error::Error for ErrorKind {}

#[cfg(feature = "pg")]
impl From<sqlx::Error> for ErrorKind {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => "can not find data in db".to_validation_error(),
            err => ErrorKind::OtherError(Box::new(err)),
        }
    }
}

#[cfg(feature = "redis")]
impl From<redis::RedisError> for ErrorKind {
    fn from(err: redis::RedisError) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

// impl From<bincode::Error> for ErrorKind {
//     fn from(err: bincode::Error) -> Self {
//         ErrorKind::OtherError(Box::new(err))
//     }
// }

#[cfg(feature = "actix-web")]
impl From<jsonwebtoken::errors::Error> for ErrorKind {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

impl From<std::time::SystemTimeError> for ErrorKind {
    fn from(err: std::time::SystemTimeError) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

#[cfg(feature = "regex")]
impl From<regex::Error> for ErrorKind {
    fn from(err: regex::Error) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

#[cfg(feature = "meilisearch")]
impl From<meilisearch_sdk::errors::Error> for ErrorKind {
    fn from(err: meilisearch_sdk::errors::Error) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

#[cfg(feature = "email")]
impl From<lettre::transport::smtp::Error> for ErrorKind {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        ErrorKind::OtherError(Box::new(err))
    }
}

#[cfg(feature = "actix-web")]
use actix_web::{http::header::ContentType, HttpResponse};
#[cfg(feature = "actix-web")]
impl actix_web::error::ResponseError for ErrorKind {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ErrorKind::BusinessError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorKind::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorKind::Timeout(_) => actix_web::http::StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Hint(_) => actix_web::http::StatusCode::from_u16(452).unwrap(),
            ErrorKind::OtherError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
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
            .body(serde_json::to_string(&response::msg(&self.to_string())).unwrap())
    }
}

pub trait ToError {
    fn to_business_error(&self) -> ErrorKind;
    fn to_validation_error(&self) -> ErrorKind;
    fn to_hint(&self) -> ErrorKind;
    fn to_unauthorized(&self) -> ErrorKind;
}

impl<T> ToError for T
where
    T: AsRef<str>,
{
    fn to_business_error(&self) -> ErrorKind {
        ErrorKind::BusinessError(self.as_ref().to_string())
    }

    fn to_validation_error(&self) -> ErrorKind {
        ErrorKind::ValidationError(self.as_ref().to_string())
    }

    fn to_hint(&self) -> ErrorKind {
        ErrorKind::Hint(self.as_ref().to_string())
    }

    fn to_unauthorized(&self) -> ErrorKind {
        ErrorKind::Unauthorized(self.as_ref().to_string())
    }
}
