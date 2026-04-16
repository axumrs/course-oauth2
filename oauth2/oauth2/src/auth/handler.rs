use axum::{Json, extract::State};
use validator::Validate;

use crate::{
    ArcAppState, Error, Result,
    auth::payload,
    resp,
    token::{db as token_db, model as token_model},
    user::{db as user_db, model as user_model},
    utils,
};

pub async fn user_login(
    State(state): State<ArcAppState>,
    Json(frm): Json<payload::UserLogin>,
) -> Result<resp::JsonResp<token_model::Token>> {
    frm.validate()?;

    let mut tx = state.pool.begin().await?;

    let user = match user_db::find(&mut *tx, user_db::FindBy::Username(&frm.username)).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    let user = match user {
        Some(v) => v,
        None => return Err(Error::not_found("用户名/密码错误")),
    };

    if !utils::verify_password(&frm.password, &user.password)? {
        return Err(Error::not_found("用户名/密码错误"));
    }

    let m = token_model::Token::try_new(&user.id, state.cfg.token_expired)?;
    let token = match token_db::create(&mut *tx, m).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    tx.commit().await?;

    Ok(resp::ok(token).as_resp())
}

pub async fn user_register(
    State(state): State<ArcAppState>,
    Json(frm): Json<payload::UserRegister>,
) -> Result<resp::JsonResp<user_model::User>> {
    frm.validate()?;

    let mut tx = state.pool.begin().await?;

    let exists = match user_db::is_exist(&mut *tx, &frm.username, &frm.email, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    if exists {
        tx.rollback().await?;
        return Err(Error::bad_request("用户已存在"));
    }

    let m = user_model::User::try_new(
        &frm.username,
        &frm.email,
        &frm.password,
        user_model::UserStatus::Active,
    )?;
    let user = match user_db::create(&mut *tx, m).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    tx.commit().await?;

    Ok(resp::ok(user).as_resp())
}
