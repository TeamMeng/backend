mod config;
mod error;
mod model;
mod util;

use sqlx::PgPool;
use sqlx_db_tester::TestPg;
use std::{fmt::Debug, ops::Deref, path::Path, sync::Arc};

pub use config::{AppConfig, AuthConfig, ServerConfig};
pub use error::AppError;
pub use model::{CreateUser, User};
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

    pub async fn test_new() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::new()?;

        let post = config.server.db_url.rfind("/");
        let url = match post {
            Some(post) => &config.server.db_url[..post],
            None => "postgres://postgres:postgres@localhost:5432",
        };

        let tdb = TestPg::new(url.to_string(), Path::new("./migrations"));
        let pool = tdb.get_pool().await;
        let ek = EncodingKey::new(&config.auth.ek)?;
        let dk = DecodingKey::new(&config.auth.dk)?;

        Ok((
            tdb,
            Self {
                inner: Arc::new(AppStateInner {
                    config,
                    pool,
                    ek,
                    dk,
                }),
            },
        ))
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
