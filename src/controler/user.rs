use sqlx::Pool;

use crate::{ctx::Ctx, model::user, Error, Result};

// CRUD Implementation
pub async fn create_user(
    pool: Pool<sqlx::Sqlite>,
    user_fc: user::UserForCreate,
) -> Result<user::UserResponse> {
    sqlx::query_as!(
        user::UserResponse,
        r#"
    INSERT INTO Users (username, password) 
    values (?, ?) 
    RETURNING id, username;
    "#,
        user_fc.username,
        user_fc.password
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("- {:^12} - {:^12} - {:^12} - {:#?}", "MODEL", "USER", "CREATE", e);
        Error::UserExist
    })
}

#[allow(unused)]
pub async fn get_user_by_id(
    pool: Pool<sqlx::Sqlite>,
    id: i64,
) -> Result<user::User> {
    sqlx::query_as!(
        user::User,
        r#"
        SELECT * FROM Users WHERE id = ?;
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("- {:^12} - {:^12} - {:^12} - {:#?}", "MODEL", "USER", "GET_BY_ID", e);
        Error::UserNotFound
    })
}

pub async fn get_user_by_username(
    pool: Pool<sqlx::Sqlite>,
    username: String,
) -> Result<user::UserOpt> {
    sqlx::query_as!(
        user::UserOpt,
        r#"
        SELECT * FROM Users WHERE username = ?;
        "#,
        username
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("- {:^12} - {:^12} - {:^12} - {:#?}", "MODEL", "USER", "GET_BY_USERNAME", e);
        Error::UserNotFound
    })
}

#[allow(unused)]
pub async fn delete_user(
    _ctx: Ctx,
    pool: Pool<sqlx::Sqlite>,
    id: i64,
) -> Result<user::User> {
    sqlx::query_as!(
        user::User,
        r#"
    DELETE FROM Users WHERE id = ? RETURNING *;
    "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("- {:^12} - {:^12} - {:^12} - {:#?}", "MODEL", "USER", "DELETE", e);
        Error::UserDeleteFailedIdNotFound { id }
    })
}
