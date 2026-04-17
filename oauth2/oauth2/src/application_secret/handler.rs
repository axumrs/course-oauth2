use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use super::{db, model, payload};
use crate::{ArcAppState, Error, Result, application::db as app_db, mw::UserAuth};

pub async fn create(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Json(frm): Json<payload::New>,
) -> Result<Json<model::ApplicationSecret>> {
    frm.validate()?;

    let app = app_db::find(&state.pool, &frm.application_id).await?;
    let app = match app {
        Some(v) => v,
        None => return Err(Error::not_found("不存在的应用")),
    };

    if app.user_id != user_auth.user.id {
        return Err(Error::forbidden("没有权限"));
    }

    let m = model::ApplicationSecret::try_new(&frm.application_id)?;
    let m = db::create(&state.pool, m).await?;
    Ok(Json(m))
}

pub async fn list(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Path(application_id): Path<String>,
) -> Result<Json<Vec<model::ApplicationSecret>>> {
    let app = app_db::find(&state.pool, &application_id).await?;
    let app = match app {
        Some(v) => v,
        None => return Err(Error::not_found("不存在的应用")),
    };

    if app.user_id != user_auth.user.id {
        return Err(Error::forbidden("没有权限"));
    }

    let m = db::list(&state.pool, &application_id).await?;
    let m = m
        .into_iter()
        .map(|k| {
            let secret = format!("{}***", utf8_slice::till(&k.secret, 10));
            model::ApplicationSecret { secret, ..k }
        })
        .collect();
    Ok(Json(m))
}

pub async fn del(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Path((application_id, id)): Path<(String, String)>,
) -> Result<Json<u64>> {
    let app = app_db::find(&state.pool, &application_id).await?;
    let app = match app {
        Some(v) => v,
        None => return Err(Error::not_found("不存在的应用")),
    };

    if app.user_id != user_auth.user.id {
        return Err(Error::forbidden("没有权限"));
    }

    let count = db::count(&state.pool, &application_id).await?;
    if count <= 1 {
        return Err(Error::bad_request("至少保留一个密钥"));
    }

    let aff = db::delete(&state.pool, &id, &application_id).await?;
    Ok(Json(aff))
}
