use axum::{http::StatusCode, response::IntoResponse};
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

    #[error("argon2 error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("login error: {0}")]
    LoginError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::IoError(_) | Self::SerdeYamlError(_) | Self::SqlxError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Argon2Error(_) | Self::LoginError(_) => StatusCode::BAD_REQUEST,
            Self::JwtError(_) => StatusCode::FORBIDDEN,
        };
        status.into_response()
    }
}
