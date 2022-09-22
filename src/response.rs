#![cfg(feature = "response")]
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Response<'a, T = ()>
where
    T: Serialize,
{
    Msg { msg: &'a str },
    Data { data: T },
    DataWithPagination { data: T, total: usize },
}

pub fn msg<'a>(msg: &'a str) -> Response<'a> {
    Response::Msg { msg }
}

pub fn data<'a, T>(data: T) -> Response<'a, T>
where
    T: Serialize,
{
    Response::Data { data }
}

pub fn data_with_pagination<'a, T>(data: T, total: usize) -> Response<'a, T>
where
    T: Serialize,
{
    Response::DataWithPagination { data, total }
}
