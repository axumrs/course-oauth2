use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>,
    pub token: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

impl Token {
    pub fn try_new(user_id: impl Into<String>, token_expired: i64) -> crate::Result<Self> {
        let s = Self {
            id: utils::new_id(),
            user_id: user_id.into(),
            nonce: utils::new_nonce(),
            created_at: chrono::Utc::now(),
            expired_at: chrono::Utc::now() + chrono::Duration::minutes(token_expired),
            ..Default::default()
        };

        let token = utils::sha256_hash_obj(&s)?;
        Ok(Self { token, ..s })
    }
}
