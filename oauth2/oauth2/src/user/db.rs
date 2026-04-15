use super::model;
use sqlx::{PgExecutor, QueryBuilder, Result};

pub enum FindBy<'a> {
    Id(&'a str),
    Username(&'a str),
    Email(&'a str),
}

pub async fn is_exist(
    e: impl PgExecutor<'_>,
    username: &str,
    email: &str,
    id: Option<&str>,
) -> Result<bool> {
    let mut q = QueryBuilder::new(r#"SELECT COUNT(*) FROM "users" WHERE 1=1"#);
    q.push(r#" AND ("username" = "#).push_bind(username);
    q.push(r#" OR "email" = "#).push_bind(email).push(r#")"#);

    if let Some(id) = id {
        q.push(r#" AND "id" <> "#).push_bind(id);
    }

    let count: i64 = q.build_query_scalar().fetch_one(e).await?;
    Ok(count > 0)
}

pub async fn create(e: impl PgExecutor<'_>, m: model::User) -> Result<model::User> {
    let mut q = QueryBuilder::new(
        r#"INSERT INTO "users" ("id", "username", "email", "password", "status", "created_at") "#,
    );

    q.push_values(&[&m], |mut b, m| {
        b.push_bind(&m.id)
            .push_bind(&m.username)
            .push_bind(&m.email)
            .push_bind(&m.password)
            .push_bind(&m.status)
            .push_bind(&m.created_at);
    });

    q.push(" RETURNING *");

    q.build_query_as().fetch_one(e).await
}

pub async fn find<'a>(e: impl PgExecutor<'a>, f: FindBy<'a>) -> Result<Option<model::User>> {
    let mut q = QueryBuilder::new(
        r#"SELECT "id", "username", "email", "password", "status", "created_at" FROM "users" WHERE 1=1"#,
    );

    match f {
        FindBy::Id(id) => {
            q.push(r#" AND "id" ="#);
            q.push_bind(id);
        }
        FindBy::Username(username) => {
            q.push(r#" AND "username" = "#);
            q.push_bind(username);
        }
        FindBy::Email(email) => {
            q.push(r#" AND "email" = "#);
            q.push_bind(email);
        }
    }

    q.build_query_as().fetch_optional(e).await
}

#[cfg(test)]
mod tests {
    async fn get_pool() -> sqlx::Result<sqlx::PgPool> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        sqlx::postgres::PgPool::connect(&database_url).await
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = get_pool().await.unwrap();
        let user = super::model::User::try_new(
            "test",
            "test@axum.eu.org",
            "test",
            super::model::UserStatus::Active,
        )
        .unwrap();
        let user = super::create(&pool, user).await.unwrap();
        assert_eq!(user.username, "test");
    }

    #[tokio::test]
    async fn test_find_user() {
        let pool = get_pool().await.unwrap();
        let user = super::find(&pool, super::FindBy::Username("test"))
            .await
            .unwrap();
        assert_eq!(user.unwrap().username, "test");
    }
}
