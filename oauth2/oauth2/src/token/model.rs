use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, Default)]
#[sqlx(type_name = "token_kind")]
pub enum TokenKind {
    #[default]
    Token,
    AccessToken,
    TempAccessToken,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>,
    pub token: String,
    pub kind: TokenKind,
    pub application_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

impl Token {
    pub fn try_new(
        user_id: impl Into<String>,
        token_expired: i64,
        kind: TokenKind,
    ) -> crate::Result<Self> {
        Self::try_with_application(user_id, token_expired, kind, "")
    }

    pub fn try_with_application(
        user_id: impl Into<String>,
        token_expired: i64,
        kind: TokenKind,
        application_id: impl Into<String>,
    ) -> crate::Result<Self> {
        let s = Self {
            id: utils::new_id(),
            user_id: user_id.into(),
            nonce: utils::new_nonce(),
            kind,
            application_id: application_id.into(),
            created_at: chrono::Utc::now(),
            expired_at: chrono::Utc::now() + chrono::Duration::minutes(token_expired),
            ..Default::default()
        };

        let token = utils::sha256_hash_obj(&s)?;
        Ok(Self { token, ..s })
    }
}
