use sqlx::PgPool;
use std::fmt;
use std::{ops::Deref, sync::Arc};

use crate::{
    auth::{DecodingKey, EncodingKey},
    dto::User,
    errors::AppError,
};

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(pool: PgPool, ek_str: &str, dk_str: &str) -> Result<Self, AppError> {
        let ek = EncodingKey::load(ek_str)?;
        let dk = DecodingKey::load(dk_str)?;
        let inner = Arc::new(AppStateInner { pool, ek, dk });

        Ok(Self { inner })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait TokenVeirfy {
    type Error: fmt::Debug;
    fn vetify(&self, token: &str) -> Result<User, Self::Error>;
}

impl TokenVeirfy for AppState {
    type Error = AppError;
    fn vetify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.dk.verify(token)?)
    }
}

#[derive(Clone)]
pub struct AppStateInner {
    pub pool: PgPool,
    pub ek: EncodingKey,
    pub dk: DecodingKey,
}
