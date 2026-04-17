use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use super::{db, model, payload};
use crate::{ArcAppState, Error, Result, mw::UserAuth, resp};

pub async fn new(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Json(frm): Json<payload::New>,
) -> Result<resp::JsonResp<model::Application>> {
    frm.validate()?;

    let m = model::Application::new(
        &user_auth.user.id,
        &frm.name,
        &frm.desc,
        &frm.homepage,
        &frm.callback,
        model::ApplicationStatus::Active,
    );
    let app = db::create(&state.pool, m).await?;

    Ok(resp::ok(app).as_resp())
}

pub async fn list(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
) -> Result<Json<Vec<model::Application>>> {
    let data = db::list(
        &state.pool,
        db::ListOptions {
            user_id: Some(user_auth.user.id.clone()),
            status: Some(model::ApplicationStatus::Active),
        },
    )
    .await?;

    Ok(Json(data))
}

pub async fn find(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<model::Application>> {
    let data = db::find(&state.pool, &id).await?;

    let data = match data {
        Some(v) => v,
        None => return Err(Error::not_found("不存在的应用")),
    };

    if data.user_id != user_auth.user.id {
        return Err(Error::forbidden("没有权限"));
    }

    Ok(resp::ok(data).as_resp())
}
