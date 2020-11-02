use sqlx::sqlite::{SqliteQueryAs, SqlitePool};

const DB_NAME: &str = "local/test.db";
const REQUEST_SCHEMA: &str =
    r#"create table if not exists requests (
        id integer primary key,
        timestamp integer not null,
        cache_key text not null unique,
        body text not null
    )"#;

#[derive(Debug, sqlx::FromRow)]
struct Request {
    id: i64,
    timestamp: i64,
    cache_key: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePool::builder()
        .max_size(5)
        .build(format!("sqlite:{}", DB_NAME).as_str()).await?;
    sqlx::query(REQUEST_SCHEMA).execute(&pool).await?;
    sqlx::query("replace into requests (timestamp, cache_key, body) values (?, ?, ?)")
        .bind(4i64).bind("key1").bind("body1")
        .execute(&pool)
        .await?;
    sqlx::query("replace into requests (timestamp, cache_key, body) values (?, ?, ?)")
        .bind(5i64).bind("key2").bind("body2")
        .execute(&pool)
        .await?;
    sqlx::query("replace into requests (timestamp, cache_key, body) values (?, ?, ?)")
        .bind(6i64).bind("key3").bind("body3")
        .execute(&pool)
        .await?;
    let result: (i64,) = sqlx::query_as("select last_insert_rowid()")
        .fetch_one(&pool)
        .await?;
    println!("{:?}", result);
    let result: Request = sqlx::query_as("select * from requests where cache_key = ?")
        .bind("key1")
        .fetch_one(&pool)
        .await?;
    println!("{:?}", result);
    Ok(())
}
