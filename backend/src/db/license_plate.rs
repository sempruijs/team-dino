pub async fn check_license_plate(pool: &PgPool, plate: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM license_plates 
            WHERE license_plate = $1
        )
        "#,
        plate
    )
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap())
}

pub async fn create_license_plate(
    pool: &PgPool,
    user_id: Uuid,
    license_plate: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO license_plates (user_id, license_plate)
        VALUES ($1, $2)
        "#,
        user_id,
        license_plate
    )
    .execute(pool)
    .await?;

    Ok(())
}
