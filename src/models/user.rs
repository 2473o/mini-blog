use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, types::chrono};

use crate::errors::AppError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub display_name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateUser {
    pub username: String,
    pub password_hash: String,
    pub display_name: String,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

#[derive(Debug)]
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, AppError> {
        let user = sqlx::query_as(
            r#"
            INSERT INTO users (username, password_hash, display_name)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, display_name, created_at, updated_at
            "#,
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.display_name)
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<Option<User>, AppError> {
        if user.id == 0 {
            return Err(AppError::InvalidArgument("user id is invalid".to_string()));
        }

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET 
                display_name = COALESCE($2, display_name),
            WHERE id = $1
            RETURNING id, username, display_name
            "#,
        )
        .bind(user.id)
        .bind(&user.display_name)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            r#"
            SELECT id, username, password_hash,display_name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_username(&self, user_name: &String) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            r#"
            SELECT id, username, password_hash,display_name,created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(user_name)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }
}
