use crate::{AppError, AppState, CreateUser, LoginUser};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn signup(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    state.create_user(input).await?;

    Ok((StatusCode::CREATED, "created"))
}

pub async fn signin(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let ret = state.login(input).await?;
    Ok((StatusCode::OK, Json(ret)))
}
