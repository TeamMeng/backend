use crate::{AppError, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: i64,
    name: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

impl AppState {
    pub async fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        let password_hash = hash_password(&input.password)?;

        let user = sqlx::query_as(
            "
            insert into users (name, email, password_hash) values ($1, $2, $3) RETURNING *
            ",
        )
        .bind(input.name)
        .bind(input.email)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn login(&self, input: LoginUser) -> Result<User, AppError> {
        match self.find_user_by_email(&input.email).await? {
            Some(user) => {
                if verify_password(&input.password, &user.password_hash)? {
                    Ok(user)
                } else {
                    Err(AppError::LoginError(format!(
                        "{} user by password error",
                        input.password
                    )))
                }
            }
            None => Err(AppError::LoginError(format!(
                "{} user by email not found",
                input.email
            ))),
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "
            SELECT * FROM users WHERE email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash)?.to_string();

    Ok(parsed_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn hash_password_verify_password_should_work() -> Result<()> {
        let password = "hunter42";

        let password_hash = hash_password(password)?;
        let ret = verify_password(password, &password_hash)?;

        assert!(ret);

        Ok(())
    }

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::test_new().await?;

        let name = "Meng";
        let email = "TeamMeng@123.com";
        let password = "hunter42";

        let input = CreateUser::new(name, email, password);

        let user = state.create_user(input).await?;

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        let ret = verify_password(password, &user.password_hash)?;

        assert!(ret);

        Ok(())
    }

    #[tokio::test]
    async fn find_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::test_new().await?;

        let name = "Meng";
        let email = "TeamMeng@123.com";
        let password = "hunter42";

        let input = CreateUser::new(name, email, password);

        state.create_user(input).await?;

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        let ret = verify_password(password, &user.password_hash)?;

        assert!(ret);

        Ok(())
    }

    #[tokio::test]
    async fn login_should_work() -> Result<()> {
        let (_tdb, state) = AppState::test_new().await?;

        let name = "Meng";
        let email = "TeamMeng@123.com";
        let password = "hunter42";

        let input = CreateUser::new(name, email, password);

        state.create_user(input).await?;

        let input = LoginUser::new(email, password);

        let user = state.login(input).await?;

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        let ret = verify_password(password, &user.password_hash)?;

        assert!(ret);

        Ok(())
    }

    impl CreateUser {
        fn new(
            name: impl Into<String>,
            email: impl Into<String>,
            password: impl Into<String>,
        ) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
                password: password.into(),
            }
        }
    }

    impl LoginUser {
        fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                email: email.into(),
                password: password.into(),
            }
        }
    }
}
