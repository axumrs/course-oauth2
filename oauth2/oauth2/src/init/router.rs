use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

use crate::{ArcAppState, application, application_secret, auth, mw};

pub fn init(state: ArcAppState) -> Router {
    Router::new().nest("/api", api_init(state))
}

fn api_init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/auth", auth_init(state.clone()))
        .nest("/application", application_init(state.clone()))
}

fn auth_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/login", post(auth::handler::user_login))
        .route("/register", post(auth::handler::user_register))
        .with_state(state)
}

fn application_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/",
            post(application::handler::new).get(application::handler::list),
        )
        .route("/{id}", get(application::handler::find))
        .route("/secret", post(application_secret::handler::create))
        .route(
            "/secret/{application_id}",
            get(application_secret::handler::list),
        )
        .route(
            "/secret/{application_id}/{id}",
            delete(application_secret::handler::del),
        )
        .layer(middleware::from_extractor_with_state::<
            mw::UserAuth,
            ArcAppState,
        >(state.clone()))
        .with_state(state)
}
