use axum::response::IntoResponse;

use crate::resp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("数据库错误")]
    Database(#[from] sqlx::Error),

    #[error("密码计算错误")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("JSON错误")]
    Json(#[from] serde_json::Error),

    #[error("输入错误")]
    Validations(#[from] validator::ValidationErrors),

    #[error("输入错误")]
    Validation(#[from] validator::ValidationError),

    #[error("{0}")]
    Custom(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("未授权")]
    Unauthorized,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
impl Error {
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn http_code(&self) -> u16 {
        match self {
            &Self::Custom(_)
            | &Self::Validation(_)
            | &Self::Validations(_)
            | &Self::BadRequest(_) => 400,

            &Self::Unauthorized => 401,

            &Self::NotFound(_) => 404,

            _ => 500,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self);
        resp::err(self).as_resp().into_response()
    }
}
