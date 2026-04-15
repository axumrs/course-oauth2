use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserLogin {
    #[validate(length(min = 3, message = "请输入用户名"))]
    pub username: String,

    #[validate(length(min = 6, message = "请输入密码"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserRegister {
    #[validate(length(min = 3, message = "请输入用户名"))]
    pub username: String,

    #[validate(length(min = 6, message = "请输入密码"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "两次输入的密码不一致"))]
    pub re_password: String,

    #[validate(email(message = "请输入邮箱"))]
    pub email: String,
}
