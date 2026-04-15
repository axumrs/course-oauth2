use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct New {
    #[validate(length(min = 3, message = "请输入应用名称"))]
    pub name: String,

    #[validate(url(message = "请输入首页地址"))]
    pub homepage: String,

    pub desc: String,

    #[validate(url(message = "请输入回调地址"))]
    pub callback: String,
}
