use super::model;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]

pub struct NewAuthorize {
    #[validate(length(min = 20, max = 20, message = "请输入客户ID"))]
    pub client_id: String,

    pub scope: model::AuthorizeScope,
}

#[derive(Deserialize, Validate)]

pub struct NewAccessToken {
    #[validate(length(min = 20, max = 20, message = "请输入客户ID"))]
    pub client_id: String,

    #[validate(length(min = 64, max = 64, message = "请输入客户密钥"))]
    pub client_secret: String,

    #[validate(length(min = 64, max = 64, message = "请输入临时令牌"))]
    pub code: String,
}
