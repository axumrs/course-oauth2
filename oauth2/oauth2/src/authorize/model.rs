use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize, Default, sqlx::Type, Clone)]
#[sqlx(type_name = "authorize_scope")]
#[serde(rename_all = "snake_case")]
pub enum AuthorizeScope {
    User,

    #[serde(rename = "read:user")]
    ReadUser,

    #[default]
    #[serde(rename = "user:email")]
    UserEmail,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Authorize {
    pub id: String,
    pub application_id: String,
    pub user_id: String,
    pub scope: AuthorizeScope,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Authorize {
    pub fn new<S: Into<String>>(application_id: S, user_id: S, scope: AuthorizeScope) -> Self {
        Self {
            id: utils::new_id(),
            application_id: application_id.into(),
            user_id: user_id.into(),
            scope,
            created_at: chrono::Utc::now(),
        }
    }
}
