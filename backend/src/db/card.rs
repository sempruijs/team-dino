pub async fn check_card(pool: &PgPool, card_id: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM cards 
            WHERE card_id = $1
        )
        "#,
        card_id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap())
}

pub async fn create_card(pool: &PgPool, user_id: Uuid, card_id: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO cards (user_id, card_id)
        VALUES ($1, $2)
        "#,
        user_id,
        card_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
