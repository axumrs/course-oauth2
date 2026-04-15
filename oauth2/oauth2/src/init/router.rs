use axum::{Router, middleware, routing::post};

use crate::{ArcAppState, application, auth, mw};

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
        .route("/", post(application::handler::new))
        .layer(middleware::from_extractor_with_state::<
            mw::UserAuth,
            ArcAppState,
        >(state.clone()))
        .with_state(state)
}
