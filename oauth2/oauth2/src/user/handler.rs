use axum::{Json, extract::State};

use super::{db, model};
use crate::{ArcAppState, Error, Result, mw::UserAuth};

pub async fn find(
    State(state): State<ArcAppState>,
    user_auth: UserAuth,
) -> Result<Json<model::User>> {
    // Ok(Json(user_auth.user))
    let user = match db::find(&state.pool, db::FindBy::Id(&user_auth.user.id)).await? {
        Some(v) => v,
        None => return Err(Error::not_found("用户不存在")),
    };

    Ok(Json(user))
}
