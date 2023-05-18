pub struct DatabaseData;

impl serenity::prelude::TypeMapKey for DatabaseData {
    type Value = sqlx::SqlitePool;
}

pub async fn setup(config: &crate::config::Config) -> sqlx::SqlitePool {
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(config.database_path.clone())
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    return database;
}
