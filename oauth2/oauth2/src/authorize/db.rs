use sqlx::PgExecutor;

use super::model;

pub async fn create_or_update(
    e: impl PgExecutor<'_>,
    authorize: model::Authorize,
) -> sqlx::Result<model::Authorize> {
    sqlx::query_as(
        r#"
        INSERT INTO "authorizes" ("id", "application_id", "user_id", "scope", "created_at")
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (application_id, user_id)
        DO UPDATE SET scope = $4
        RETURNING "id", "application_id", "user_id", "scope", "created_at"
        "#,
    )
    .bind(&authorize.id)
    .bind(&authorize.application_id)
    .bind(&authorize.user_id)
    .bind(&authorize.scope)
    .bind(&authorize.created_at)
    .fetch_one(e)
    .await
}

pub async fn find(
    e: impl PgExecutor<'_>,
    application_id: &str,
    user_id: &str,
) -> sqlx::Result<Option<model::Authorize>> {
    sqlx::query_as(
        r#"
        SELECT "id", "application_id", "user_id", "scope", "created_at"
        FROM "authorizes"
        WHERE "application_id" = $1 AND "user_id" = $2
        "#,
    )
    .bind(application_id)
    .bind(user_id)
    .fetch_optional(e)
    .await
}
