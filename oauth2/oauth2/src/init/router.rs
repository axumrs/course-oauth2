use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

use crate::{ArcAppState, application, application_secret, auth, authorize, mw, user};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/api", api_init(state.clone()))
        .nest("/login/oauth", access_token_init(state))
}

fn api_init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/auth", auth_init(state.clone()))
        .nest("/application", application_init(state.clone()))
        .nest("/login/oauth", authorize_init(state.clone()))
        .nest("/user", user_init(state))
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

fn authorize_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/authorize", post(authorize::handler::authorize))
        .route(
            "/authorize/{application_id}",
            get(authorize::handler::get_authorize),
        )
        .layer(middleware::from_extractor_with_state::<
            mw::UserAuth,
            ArcAppState,
        >(state.clone()))
        .with_state(state)
}

fn access_token_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/access_token", post(authorize::handler::access_token))
        .with_state(state)
}

fn user_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(user::handler::find))
        .layer(middleware::from_extractor_with_state::<
            mw::UserAuth,
            ArcAppState,
        >(state.clone()))
        .with_state(state)
}
