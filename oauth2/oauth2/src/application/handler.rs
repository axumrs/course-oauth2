use axum::{Json, extract::State};
use validator::Validate;

use super::{db, model, payload};
use crate::{ArcAppState, Result, mw::UserAuth, resp};

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
