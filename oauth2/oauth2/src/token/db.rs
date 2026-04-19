use super::model;
use sqlx::{PgExecutor, QueryBuilder, Result, query, query_as};

pub async fn create(e: impl PgExecutor<'_>, m: model::Token) -> Result<model::Token> {
    let sql = r#"INSERT INTO "tokens" ("id", "user_id", "nonce", "token", "created_at", "expired_at", "kind", "application_id") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#;
    query_as(sql)
        .bind(&m.id)
        .bind(&m.user_id)
        .bind(&m.nonce)
        .bind(&m.token)
        .bind(&m.created_at)
        .bind(&m.expired_at)
        .bind(&m.kind)
        .bind(&m.application_id)
        .fetch_one(e)
        .await
}

pub enum FindBy<'a> {
    Id(&'a str),
    Token(&'a str),
}

pub async fn find(e: impl PgExecutor<'_>, f: FindBy<'_>) -> Result<Option<model::Token>> {
    let mut q = QueryBuilder::new(
        r#"SELECT "id", "user_id", "nonce", "token", "created_at", "expired_at", "kind", "application_id" FROM "tokens" WHERE "expired_at" >= CURRENT_TIMESTAMP AND 1=1"#,
    );
    match f {
        FindBy::Id(id) => {
            q.push(r#" AND "id" = "#);
            q.push_bind(id);
        }
        FindBy::Token(token) => {
            q.push(r#" AND "token" = "#);
            q.push_bind(token);
        }
    }
    q.build_query_as().fetch_optional(e).await
}

pub async fn delete_expired(e: impl PgExecutor<'_>) -> Result<u64> {
    let sql = r#"DELETE FROM "tokens" WHERE "expired_at" < CURRENT_TIMESTAMP"#;
    let aff = query(sql).execute(e).await?.rows_affected();
    Ok(aff)
}

pub async fn delete(e: impl PgExecutor<'_>, id: &str) -> Result<u64> {
    let sql = r#"DELETE FROM "tokens" WHERE "id" = $1"#;
    let aff = query(sql).bind(id).execute(e).await?.rows_affected();
    Ok(aff)
}

pub async fn delete_by_user(e: impl PgExecutor<'_>, user_id: &str) -> Result<u64> {
    let sql = r#"DELETE FROM "tokens" WHERE "user_id" = $1"#;
    let aff = query(sql).bind(user_id).execute(e).await?.rows_affected();
    Ok(aff)
}

pub async fn delete_by_token(e: impl PgExecutor<'_>, token: &str) -> Result<u64> {
    let sql = r#"DELETE FROM "tokens" WHERE "token" = $1"#;
    let aff = query(sql).bind(token).execute(e).await?.rows_affected();
    Ok(aff)
}
