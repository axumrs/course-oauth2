use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct ApplicationSecret {
    pub id: String,
    pub application_id: String,
    pub secret: String,
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ApplicationSecret {
    pub fn try_new(application_id: impl Into<String>) -> crate::Result<Self> {
        let s = Self {
            id: utils::new_id(),
            application_id: application_id.into(),
            nonce: utils::new_nonce(),
            created_at: chrono::Utc::now(),
            ..Default::default()
        };

        let secret = utils::sha256_hash_obj(&s)?;

        Ok(Self { secret, ..s })
    }
}
