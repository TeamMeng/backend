use crate::AppError;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: i32,
    pub db_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    pub ek: String,
    pub dk: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        let rdr = File::open("backend.yaml")?;

        let ret = serde_yaml::from_reader(rdr)?;

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn config_new_should_work() -> Result<()> {
        let config = AppConfig::new()?;

        assert_eq!(config.server.port, 6688);
        assert_eq!(
            config.server.db_url,
            "postgres://postgres:postgres@localhost:5432/base"
        );
        assert_eq!(config.auth.ek, "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIPU73KJReh1Lxv+aF/UbPPygRE7Bf3ozqyKsu65+pSCk\n-----END PRIVATE KEY-----\n");
        assert_eq!(config.auth.dk, "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAzrRHbn3NvA3c3oSoAY4h/BZ49VzYV+d1UVi7tCz6o20=\n-----END PUBLIC KEY-----\n");

        Ok(())
    }
}
