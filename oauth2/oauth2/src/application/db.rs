use sqlx::{PgExecutor, QueryBuilder, Result, query_as};

use super::model;

pub async fn create(e: impl PgExecutor<'_>, m: model::Application) -> Result<model::Application> {
    let mut q = QueryBuilder::new(
        r#"INSERT INTO "applications" ("id", "user_id", "name", "description", "homepage_url", "callback_url", "status", "created_at", "updated_at") "#,
    );
    q.push_values(&[&m], |mut b, m| {
        b.push_bind(&m.id)
            .push_bind(&m.user_id)
            .push_bind(&m.name)
            .push_bind(&m.description)
            .push_bind(&m.homepage_url)
            .push_bind(&m.callback_url)
            .push_bind(&m.status)
            .push_bind(&m.created_at)
            .push_bind(&m.updated_at);
    });

    q.push(" RETURNING *");

    q.build_query_as().fetch_one(e).await
}

pub async fn find(e: impl PgExecutor<'_>, id: &str) -> Result<Option<model::Application>> {
    let sql = r#"SELECT "id", "user_id", "name", "description", "homepage_url", "callback_url", "status", "created_at", "updated_at" FROM "applications" WHERE "id" = $1"#;
    query_as(sql).bind(id).fetch_optional(e).await
}

pub struct ListOptions {
    pub user_id: Option<String>,
    pub status: Option<model::ApplicationStatus>,
}

pub async fn list(e: impl PgExecutor<'_>, opts: ListOptions) -> Result<Vec<model::Application>> {
    let mut q = QueryBuilder::new(
        r#"SELECT "id", "user_id", "name", "description", "homepage_url", "callback_url", "status", "created_at", "updated_at" FROM "applications" WHERE 1=1"#,
    );
    if let Some(user_id) = &opts.user_id {
        q.push(r#" AND "user_id" = "#);
        q.push_bind(user_id);
    }
    if let Some(status) = &opts.status {
        q.push(r#" AND "status" = "#);
        q.push_bind(status);
    }
    q.push(r#" ORDER BY "id" DESC"#);
    q.build_query_as().fetch_all(e).await
}
