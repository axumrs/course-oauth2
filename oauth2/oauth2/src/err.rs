use axum::response::IntoResponse;

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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self);
        axum::response::Json(self.to_string()).into_response()
    }
}
