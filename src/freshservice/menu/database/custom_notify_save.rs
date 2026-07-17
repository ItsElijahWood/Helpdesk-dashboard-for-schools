use crate::database::connection;

pub async fn db(filename: &String) {
    let mut conn = connection().await.unwrap();

    let ext = filename.rsplitn(2, '.').nth(0).unwrap();

    sqlx::query(
        "INSERT INTO freshservice (file_name, file_ext, file_length, is_default, is_selected) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(filename)
    .bind(ext)
    .bind(4)
    .bind(0)
    .bind(0)
    .execute(&mut conn)
    .await.unwrap();
}
