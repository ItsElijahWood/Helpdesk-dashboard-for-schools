use std::str::FromStr;

use sqlx::{ConnectOptions, Error, Result, SqliteConnection, sqlite::SqliteConnectOptions};

pub async fn seed_db() {
    let mut conn = connection().await.unwrap();

    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name=?)",
    )
    .bind("freshservice")
    .fetch_one(&mut conn)
    .await
    .unwrap();

    if table_exists {
        return println!("Database setup.");
    }

    sqlx::raw_sql(
        "CREATE TABLE freshservice (
            file_name VARCHAR,
            file_ext VARCHAR,
            file_length INT,
            is_default INT,
            is_selected INT
        )",
    )
    .execute(&mut conn)
    .await
    .expect("failed to create the table.");

    sqlx::query(
        "INSERT INTO freshservice (file_name, file_ext, file_length, is_default, is_selected) VALUES (?, ?, ?, ?, ?)"
    )
    .bind("ut-sound")
    .bind("wav")
    .bind(2)
    .bind(1)
    .bind(1)
    .execute(&mut conn)
    .await
    .expect("failed to seed the table.");

    println!("Seeded the database.");
    println!("Database setup.");
}

pub async fn connection() -> Result<SqliteConnection, Error> {
    let init = SqliteConnectOptions::from_str("sqlite://database.db")?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    Ok(init.connect().await?)
}
