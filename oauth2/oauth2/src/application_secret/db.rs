use sqlx::{PgExecutor, Result, query, query_as, query_scalar};

use super::model;

pub async fn count(e: impl PgExecutor<'_>, application_id: &str) -> Result<i64> {
    let sql = r#"SELECT COUNT(*) FROM "application_secrets" WHERE "application_id" = $1"#;
    query_scalar(sql).bind(application_id).fetch_one(e).await
}
pub async fn create(
    e: impl PgExecutor<'_>,
    m: model::ApplicationSecret,
) -> Result<model::ApplicationSecret> {
    let sql = r#"INSERT INTO "application_secrets" ("id", "application_id", "secret", "nonce", "created_at") VALUES ($1, $2, $3, $4, $5) RETURNING *"#;
    query_as(sql)
        .bind(&m.id)
        .bind(&m.application_id)
        .bind(&m.secret)
        .bind(&m.nonce)
        .bind(&m.created_at)
        .fetch_one(e)
        .await
}

pub async fn find(
    e: impl PgExecutor<'_>,
    id: &str,
    application_id: &str,
) -> Result<Option<model::ApplicationSecret>> {
    let sql = r#"SELECT "id", "application_id", "secret", "nonce", "created_at" FROM "application_secrets" WHERE "id" = $1 AND "application_id" = $2"#;
    query_as(sql)
        .bind(id)
        .bind(application_id)
        .fetch_optional(e)
        .await
}

pub async fn find_by_secret(
    e: impl PgExecutor<'_>,
    secret: &str,
    application_id: &str,
) -> Result<Option<model::ApplicationSecret>> {
    let sql = r#"SELECT "id", "application_id", "secret", "nonce", "created_at" FROM "application_secrets" WHERE "secret" = $1 AND "application_id" = $2"#;
    query_as(sql)
        .bind(secret)
        .bind(application_id)
        .fetch_optional(e)
        .await
}

pub async fn list(
    e: impl PgExecutor<'_>,
    application_id: &str,
) -> Result<Vec<model::ApplicationSecret>> {
    let sql = r#"SELECT "id", "application_id", "secret", "nonce", "created_at" FROM "application_secrets" WHERE "application_id" = $1 ORDER BY "id" DESC"#;
    query_as(sql).bind(application_id).fetch_all(e).await
}

pub async fn delete(e: impl PgExecutor<'_>, id: &str, application_id: &str) -> Result<u64> {
    let sql = r#"DELETE FROM "application_secrets" WHERE "id" = $1 AND "application_id" = $2"#;
    let aff = query(sql)
        .bind(id)
        .bind(application_id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}
