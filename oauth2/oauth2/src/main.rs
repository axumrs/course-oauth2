use oauth2::{Config, init, new_arc_state};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();
    init::log();

    let listener = tokio::net::TcpListener::bind(&cfg.addr).await?;
    tracing::info!("listening on {}", cfg.addr);

    let pool = init::pg(&cfg.database_url, cfg.database_max_conns).await?;
    let state = new_arc_state(cfg, pool);

    let app = init::router(state);

    axum::serve(listener, app).await?;
    Ok(())
}
