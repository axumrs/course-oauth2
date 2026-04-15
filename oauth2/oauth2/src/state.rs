use std::sync::Arc;

use sqlx::PgPool;

use crate::Config;

pub struct AppState {
    pub cfg: Arc<Config>,
    pub pool: PgPool,
}

pub type ArcAppState = Arc<AppState>;

pub fn new_arc_state(cfg: Config, pool: PgPool) -> ArcAppState {
    Arc::new(AppState {
        cfg: Arc::new(cfg),
        pool,
    })
}
