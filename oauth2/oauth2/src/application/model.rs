use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Default, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "application_status")]
pub enum ApplicationStatus {
    #[default]
    Pending,
    Active,
    Reject,
}

#[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
    pub callback_url: String,
    pub status: ApplicationStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Application {
    pub fn new<S: Into<String>>(
        user_id: S,
        name: S,
        description: S,
        homepage_url: S,
        callback_url: S,
        status: ApplicationStatus,
    ) -> Self {
        Self {
            id: utils::new_id(),
            user_id: user_id.into(),
            name: name.into(),
            description: description.into(),
            homepage_url: homepage_url.into(),
            callback_url: callback_url.into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status,
            ..Default::default()
        }
    }
}
