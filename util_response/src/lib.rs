use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema)]
pub struct MsgResponse {
    pub msg: String,
}

#[derive(ToSchema)]
pub struct MsgResponseWithErrCode {
    pub msg: String,
    pub err_code: usize,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct Response<D, M>
where
    D: Serialize,
    M: AsRef<str>,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<M>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_code: Option<usize>,
}

#[derive(Deserialize, IntoParams)]
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

impl<D, M> Response<D, M>
where
    D: Serialize,
    M: AsRef<str>,
{
    pub fn new(
        data: Option<D>,
        total: Option<usize>,
        msg: Option<M>,
        err_code: Option<usize>,
    ) -> Self {
        Self {
            data,
            total,
            msg,
            err_code,
        }
    }
}

pub mod prelude {
    pub use super::{Pagination, Response};
    pub use actix_web::web::{redirect, Json, Redirect};

    #[macro_export]
    macro_rules! msg {
        ($msg:expr) => {{
            Response::new(None::<()>, None, Some($msg), None)
        }};
        ($msg:expr,$err_code:expr) => {{
            Response::new(None::<()>, None, Some($msg), Some($err_code))
        }};
    }

    #[macro_export]
    macro_rules! data {
        ($data:expr) => {{
            Response::new(Some($data), None, None::<String>, None)
        }};
        ($data:expr,$total:expr) => {{
            Response::new(Some($data), Some($total), None::<String>, None)
        }};
    }
}
