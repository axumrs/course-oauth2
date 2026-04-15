pub async fn init(dsn: &str, max_conn: u32) -> sqlx::Result<sqlx::PgPool> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(dsn)
        .await
}
