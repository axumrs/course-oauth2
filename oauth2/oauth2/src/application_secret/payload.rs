use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct New {
    #[validate(length(min = 20, max = 20, message = "请输入应用ID"))]
    pub application_id: String,
}
