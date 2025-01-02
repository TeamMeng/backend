use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("serde yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
