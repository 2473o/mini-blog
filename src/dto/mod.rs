use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::blog::Blog as BlogDao;
use crate::models::user::User as UserDao;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 6, max = 20))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(min = 1, max = 20))]
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: User,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CreateBlogReq {
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UpdateBlogReq {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BlogResp {
    pub blog: BlogDao,
}

impl From<UserDao> for User {
    fn from(user: UserDao) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
