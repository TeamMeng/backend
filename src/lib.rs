mod config;
mod error;
mod util;

use sqlx::PgPool;
use std::{fmt::Debug, ops::Deref, sync::Arc};

pub use config::{AppConfig, AuthConfig, ServerConfig};
pub use error::AppError;
pub use util::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub config: AppConfig,
    pub pool: PgPool,
    pub ek: EncodingKey,
    pub dk: DecodingKey,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        let ek = EncodingKey::new(&config.auth.ek)?;
        let dk = DecodingKey::new(&config.auth.dk)?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                pool,
                ek,
                dk,
            }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
