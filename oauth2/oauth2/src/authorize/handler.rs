use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use super::{db, model, payload};
use crate::{
    ArcAppState, Error, Result, application_secret,
    mw::UserAuth,
    token::{db as token_db, model as token_model},
};

pub async fn authorize(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Json(frm): Json<payload::NewAuthorize>,
) -> Result<Json<token_model::Token>> {
    frm.validate()?;

    let mut tx = state.pool.begin().await?;

    // 创建或更新授权
    let m = model::Authorize::new(&frm.client_id, &user_auth.user.id, frm.scope);
    let m = match db::create_or_update(&mut *tx, m).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 创建临时令牌
    let t = token_model::Token::try_with_application(
        &m.user_id,
        state.cfg.temp_access_token_expired,
        token_model::TokenKind::TempAccessToken,
        &m.application_id,
    )?;

    let t = match token_db::create(&mut *tx, t).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    tx.commit().await?;

    Ok(Json(t))
}

pub async fn access_token(
    State(state): State<ArcAppState>,
    Json(frm): Json<payload::NewAccessToken>,
) -> Result<Json<token_model::Token>> {
    frm.validate()?;

    let mut tx = state.pool.begin().await?;

    // 临时令牌
    let tmp_token = match token_db::find(&mut *tx, token_db::FindBy::Token(&frm.code)).await {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                return Err(Error::not_found("授权码不存在"));
            }
        },
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 应用密钥
    let app_secret =
        match application_secret::db::find_by_secret(&mut *tx, &frm.client_secret, &frm.client_id)
            .await
        {
            Ok(v) => match v {
                Some(v) => v,
                None => {
                    return Err(Error::not_found("客户ID/密钥错误"));
                }
            },
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };
    if app_secret.application_id != tmp_token.application_id {
        return Err(Error::forbidden("授权应用不匹配"));
    }
    if app_secret.secret != frm.client_secret {
        return Err(Error::not_found("客户ID/密钥错误"));
    }

    // 访问令牌
    let t = token_model::Token::try_with_application(
        &tmp_token.user_id,
        state.cfg.access_token_expired,
        token_model::TokenKind::AccessToken,
        &tmp_token.application_id,
    )?;

    let t = match token_db::create(&mut *tx, t).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 删除临时令牌
    if let Err(e) = token_db::delete_by_token(&mut *tx, &frm.code).await {
        tx.rollback().await?;
        return Err(e.into());
    }
    tx.commit().await?;

    Ok(Json(t))
}

#[derive(serde::Serialize)]
pub struct GetAuthorizeResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub authorize: Option<model::Authorize>,
    pub is_authorized: bool,
}
pub async fn get_authorize(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Path(application_id): Path<String>,
) -> Result<Json<GetAuthorizeResp>> {
    let m = db::find(&state.pool, &application_id, &user_auth.user.id).await?;

    Ok(Json(GetAuthorizeResp {
        is_authorized: m.is_some(),
        authorize: m,
    }))
}
