use oauth2::user::model;

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub status: model::UserStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let access_token = std::env::var("ACCESS_TOKEN")?; // 访问令牌

    let client = reqwest::Client::new();
    let user: User = client
        .get("http://127.0.0.1:9527/api/user")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", user);
    Ok(())
}
