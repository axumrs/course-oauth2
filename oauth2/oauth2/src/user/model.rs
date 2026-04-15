use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Default, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_status")]
pub enum UserStatus {
    #[default]
    Pending,
    Active,
    Freeze,
}

#[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub status: UserStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn try_new<S: Into<String>>(
        username: S,
        email: S,
        password: S,
        status: UserStatus,
    ) -> crate::Result<Self> {
        let password = utils::hash_password(password.into().as_str())?;

        Ok(Self {
            id: utils::new_id(),
            username: username.into(),
            email: email.into(),
            password,
            status,
            created_at: chrono::Utc::now(),
        })
    }
}
