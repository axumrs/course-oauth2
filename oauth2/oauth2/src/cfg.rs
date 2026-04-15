use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub addr: String,
    pub database_url: String,
    pub database_max_conns: u32,

    pub token_expired: i64,
    pub access_token_expired: i64,
    pub temp_access_token_expired: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let addr = std::env::var("ADDR").unwrap_or("127.0.0.1:9527".into());
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://oauth2:oauth2@127.0.0.1:5432/oauth2".into());
        let database_max_conns = std::env::var("DATABASE_MAX_CONNS")
            .unwrap_or("5".into())
            .parse()
            .unwrap_or(5);

        let token_expired = std::env::var("TOKEN_EXPIRED")
            .unwrap_or("60".into())
            .parse()
            .unwrap_or(60);
        let access_token_expired = std::env::var("ACCESS_TOKEN_EXPIRED")
            .unwrap_or("60".into())
            .parse()
            .unwrap_or(60);
        let temp_access_token_expired = std::env::var("TEMP_ACCESS_TOKEN_EXPIRED")
            .unwrap_or("20".into())
            .parse()
            .unwrap_or(20);

        Self {
            addr,
            database_url,
            database_max_conns,
            token_expired,
            access_token_expired,
            temp_access_token_expired,
        }
    }
}
