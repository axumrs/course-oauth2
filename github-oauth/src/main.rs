use std::collections::HashMap;

use axum::{
    Router,
    extract::{Path, Query},
    response::Html,
    routing::get,
    serve,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let listener = TcpListener::bind("127.0.0.1:9527").await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/callback", get(callback))
        .route("/user-info/{access_token}/{token_type}", get(user_info));

    serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<String> {
    let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();
    let html_content = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>OAuth 2.0</title>
        </head>
        <body>
            <div style="margin: 5rem">
                <a href="https://github.com/login/oauth/authorize?scope=read:user&client_id={client_id}" style="background: blueviolet; border-radius: 0.5rem; color: #fff; padding: 0.5rem;text-decoration: none;">使用 Github 登录</a>
            </div>
        </body>
        </html>
        "#,
    );
    Html(html_content.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenRespose {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}
async fn callback(Query(q): Query<HashMap<String, String>>) -> Html<String> {
    let code = q.get("code").unwrap();

    let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();
    let secert_key = std::env::var("GITHUB_SECERT_KEY").unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let cli = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let mut data = HashMap::new();
    data.insert("client_id", client_id);
    data.insert("client_secret", secert_key);
    data.insert("code", code.into());

    let resp: AccessTokenRespose = cli
        .post("https://github.com/login/oauth/access_token")
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{resp:?}");

    Html(format!(
        r#"<a href="/user-info/{}/{}">用户信息</a>"#,
        &resp.access_token, &resp.token_type
    ))
}

async fn user_info(Path((access_token, token_type)): Path<(String, String)>) -> String {
    let auth = format!("{token_type} {access_token}");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&auth).unwrap(),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("AXUM.EU.ORG"),
    );

    let cli = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
    let resp = cli
        .get("https://api.github.com/user")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    resp
}
