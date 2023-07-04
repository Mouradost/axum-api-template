use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::error::{Error, Result};

// database connection
pub async fn connect_db(durl: String, max_connections: u32) -> Result<Pool<Sqlite>> {
    // Check if database exist
    if !Sqlite::database_exists(&durl).await.unwrap_or(false) {
        tracing::debug!("Creating database {}", &durl);
        match Sqlite::create_database(&durl).await {
            Ok(_) => tracing::debug!("Create db success"),
            Err(error) => tracing::error!("error: {error}"),
        }
    } else {
        tracing::debug!("Database already exists");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect(&durl)
        .await
        .map_err(|_| Error::DatabaseConnectionFailed)?;
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            Error::DatabaseInternalError
        })?;

    Ok(pool)
}
