use axum::{extract::FromRequestParts, http};

use crate::{
    ArcAppState,
    token::db as token_db,
    user::{db as user_db, model as user_model},
};

pub enum TokenKind {
    Token,
    AccessToken,
    TempAccessToken,
}
pub struct UserAuth {
    pub user: user_model::User,
    pub token: String,
    pub token_kind: TokenKind,
}

impl FromRequestParts<ArcAppState> for UserAuth {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match parts
            .headers
            .get(http::header::AUTHORIZATION)
            .ok_or_else(|| crate::Error::Unauthorized)?
            .to_str()
            .map_err(|_| crate::Error::Unauthorized)?
            .to_string()
            .strip_prefix("Bearer ")
        {
            Some(t) => t.to_string(),
            None => return Err(crate::Error::Unauthorized),
        };

        // temp access token
        // access token
        // token
        let (user_id, token_kind) =
            match token_db::find(&state.pool, token_db::FindBy::Token(&token)).await? {
                Some(v) => (v.user_id, TokenKind::AccessToken),
                None => return Err(crate::Error::not_found("令牌错误")),
            };

        let user = match user_db::find(&state.pool, user_db::FindBy::Id(&user_id)).await? {
            Some(v) => v,
            None => return Err(crate::Error::not_found("用户不存在")),
        };

        Ok(Self {
            user,
            token,
            token_kind,
        })
    }
}
