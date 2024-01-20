use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Response<'a, T = ()>
where
    T: Serialize,
{
    Msg(MsgResponse<'a>),
    Data(DataResponse<T>),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ErrorResponse<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    err_code: Option<usize>,
    msg: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct MsgResponse<'a> {
    msg: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct DataResponse<T>
where
    T: Serialize,
{
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<usize>,
}

impl<'a> MsgResponse<'a> {
    pub fn new<'b: 'a>(msg: &'b str) -> Self {
        Self { msg }
    }
}

impl<'a> ErrorResponse<'a> {
    pub fn new<'b: 'a>(msg: &'b str, err_code: Option<usize>) -> Self {
        Self { msg, err_code }
    }
}

impl<T> DataResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, total: Option<usize>) -> Self {
        Self { data, total }
    }
}

#[cfg_attr(feature = "openapi", derive(IntoParams))]
#[derive(Deserialize)]
pub struct Pagination {
    pub index: i64,
    pub size: i64,
}

impl Pagination {
    pub fn skip(&self) -> i64 {
        self.index.checked_sub(1).unwrap_or(0) * self.size
    }

    pub fn take(&self) -> i64 {
        self.size.max(0)
    }
}


pub mod prelude {
    pub use super::{DataResponse, ErrorResponse, MsgResponse, Pagination};
    pub use actix_web::web::{Json,Redirect,redirect};

    #[macro_export]
    macro_rules! msg {
        ($msg:expr) => {{
            Ok(Json(MsgResponse::new($msg)))
        }};
    }

    #[macro_export]
    macro_rules! error {
        ($msg:expr) => {{
            Ok(Json(ErrorResponse::new($msg, None)))
        }};
        ($msg:expr,$err_code:expr) => {{
            Ok(Json(ErrorResponse::new($msg, Some($err_code))))
        }};
    }

    #[macro_export]
    macro_rules! data {
        ($data:expr) => {{
            Ok(Json(DataResponse::new($data, None)))
        }};
        ($data:expr,$total:expr) => {{
            Ok(Json(DataResponse::new($data, Some($total))))
        }};
    }
}
