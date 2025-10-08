use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, types::chrono};

use crate::errors::AppError;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Blog {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateBlog {
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug)]
pub struct UpdateBlog {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct BlogRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> BlogRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, blog: &CreateBlog) -> Result<Blog, AppError> {
        let user = sqlx::query_as(
            r#"
            INSERT INTO blogs (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(&blog.title)
        .bind(&blog.content)
        .bind(&blog.author_id)
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(
        &self,
        author_id: i64,
        blog: &UpdateBlog,
    ) -> Result<Option<Blog>, AppError> {
        if blog.id == 0 {
            return Err(AppError::InvalidArgument("blog id is invalid".to_string()));
        }

        let result = sqlx::query_as::<_, Blog>(
            r#"
            UPDATE blogs
            SET 
                title = COALESCE($2, title),
                content = COALESCE($3, content),
            WHERE id = $1 AND author_id = $4
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(blog.id)
        .bind(&blog.title)
        .bind(&blog.content)
        .bind(author_id)
        .fetch_optional(self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Blog>, AppError> {
        let result = sqlx::query_as(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM blogs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_blogs_by_author_id(&self, id: i64) -> Result<Vec<Blog>, AppError> {
        let blogs = sqlx::query_as(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM blogs
            WHERE author_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(self.pool)
        .await?;

        Ok(blogs)
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<(), AppError> {
        let _ = sqlx::query(
            r#"
            DELETE FROM blogs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
