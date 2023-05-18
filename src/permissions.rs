use sqlx::sqlite::SqliteRow;

pub async fn is_operator(
    user_id: &u64,
    guild_id: &u64,
    dbcon: &sqlx::SqlitePool,
    config: &crate::config::Config,
) -> bool {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;
    if user_id == config.interaction.owner {
        return true;
    }

    let rows = sqlx::query!("SELECT guild_id FROM operators WHERE user_id = ?", user_id)
        .fetch_all(dbcon)
        .await
        .unwrap();
    for row in rows.iter() {
        if row.guild_id == guild_id {
            return true;
        }
    }

    return false;
}

pub async fn add_operator(user_id: &u64, guild_id: &u64, dbcon: &sqlx::SqlitePool) {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;
    sqlx::query!(
        "INSERT INTO OPERATORS (user_id, guild_id) VALUES (?1, ?2)",
        user_id,
        guild_id
    )
    .execute(dbcon)
    .await
    .expect(
        format!(
            "Failed to add operator {} for guild {} to database",
            user_id, guild_id
        )
        .as_str(),
    );
}

pub async fn remove_operator(user_id: &u64, guild_id: &u64, dbcon: &sqlx::SqlitePool) {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;
    sqlx::query!(
        "DELETE FROM operators WHERE user_id = ?1 AND guild_id = ?2",
        user_id,
        guild_id
    )
    .execute(dbcon)
    .await
    .expect(
        format!(
            "Failed to remove operator {} for guild {} from database",
            user_id, guild_id
        )
        .as_str(),
    );
}

pub async fn is_banned(user_id: &u64, guild_id: &u64, dbcon: &sqlx::SqlitePool) -> bool {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;

    let rows = sqlx::query!("SELECT guild_id FROM bans WHERE user_id = ?", user_id)
        .fetch_all(dbcon)
        .await
        .unwrap();
    for row in rows.iter() {
        if row.guild_id == Some(guild_id) {
            return true;
        }
    }

    return false;
}

pub async fn ban(user_id: &u64, guild_id: &u64, dbcon: &sqlx::SqlitePool) {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;
    sqlx::query!(
        "INSERT INTO BANS (user_id, guild_id) VALUES (?1, ?2)",
        user_id,
        guild_id
    )
    .execute(dbcon)
    .await
    .expect(
        format!(
            "Failed to add ban {} for guild {} to database",
            user_id, guild_id
        )
        .as_str(),
    );
}

pub async fn unban(user_id: &u64, guild_id: &u64, dbcon: &sqlx::SqlitePool) {
    let user_id = user_id.clone() as i64;
    let guild_id = guild_id.clone() as i64;
    sqlx::query!(
        "DELETE FROM BANS WHERE user_id = ?1 AND guild_id = ?2",
        user_id,
        guild_id
    )
    .execute(dbcon)
    .await
    .expect(
        format!(
            "Failed to remove ban {} for guild {} from database",
            user_id, guild_id
        )
        .as_str(),
    );
}
