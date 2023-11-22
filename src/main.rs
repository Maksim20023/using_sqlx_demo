use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn display_user(user: &User) {
    println!(
        "User ID: {}, Name: {}, Email: {}",
        user.id, user.name, user.email
    );
}

async fn fetch_and_display_users(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await?;

    for user in rows {
        display_user(&user);
    }

    Ok(())
}
async fn insert_user(pool: &SqlitePool, name: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    sqlx::query(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE)"
    ).execute(&pool).await?;
    let users = vec![
        ("Alice", "alice@example.com"),
        ("Bob", "bob@example.com"),
        ("Carol", "carol@example.com"),
        ("Dave", "dave@example.com"),
        ("Eve", "eve@example.com"),
    ];

    for (name, email) in users {
        insert_user(&pool, name, email).await?;
    }

    fetch_and_display_users(&pool).await?;

    Ok(())
}
