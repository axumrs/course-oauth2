use oauth2::token;

#[derive(serde::Serialize)]
pub struct AccessTokenRequest {
    pub code: String,
    pub client_secret: String,
    pub client_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub kind: token::model::TokenKind,
    pub application_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = std::env::var("CODE")?; // 临时令牌
    let client_secret = std::env::var("CLIENT_SECRET")?; // 应用密钥
    let client_id = std::env::var("CLIENT_ID")?; // 应用ID

    let data = AccessTokenRequest {
        code,
        client_secret,
        client_id,
    };

    let access_token: Token = reqwest::Client::new()
        .post("http://127.0.0.1:9527/login/oauth/access_token")
        .json(&data)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", access_token);

    Ok(())
}
