use axum::{extract::FromRequestParts, http};

use crate::{ArcAppState, token, user};

pub struct UserAuth {
    pub user: user::model::User,
    pub token: String,
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

        let finded_token =
            match token::db::find(&state.pool, token::db::FindBy::Token(&token)).await? {
                Some(v) => v,
                None => return Err(crate::Error::not_found("令牌错误")),
            };

        let user =
            match user::db::find(&state.pool, user::db::FindBy::Id(&finded_token.user_id)).await? {
                Some(v) => v,
                None => return Err(crate::Error::not_found("用户不存在")),
            };

        Ok(Self { user, token })
    }
}
