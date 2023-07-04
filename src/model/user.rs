use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User{
    pub id: i64,
    pub username: String,
    pub password: String
}

pub struct UserOpt{
    pub id: Option<i64>,
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: Option<String>,
}

