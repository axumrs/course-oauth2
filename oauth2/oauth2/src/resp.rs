use axum::{Json, http::StatusCode};
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Resp<T> {
    #[serde(skip)]
    pub code: StatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: StatusCode::OK,
            data: Some(data),
            error: None,
        }
    }

    pub fn as_resp(self) -> JsonResp<T> {
        (self.code, Json(self))
    }

    pub fn err(e: Error) -> Self {
        Self {
            code: StatusCode::from_u16(e.http_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            data: None,
            error: Some(e.to_string()),
        }
    }

    pub fn no_data_ok() -> Self {
        Self {
            code: StatusCode::OK,
            data: None,
            error: None,
        }
    }
}

pub type JsonResp<T> = (StatusCode, Json<Resp<T>>);

pub fn ok<T: Serialize>(data: T) -> Resp<T> {
    Resp::ok(data)
}

pub fn err(e: Error) -> Resp<()> {
    Resp::err(e)
}

pub fn no_data_ok() -> Resp<()> {
    Resp::no_data_ok()
}
