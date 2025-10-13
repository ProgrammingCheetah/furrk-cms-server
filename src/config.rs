use anyhow::{Context, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(Environment::Production),
            "testing" => Ok(Environment::Testing),
            _ => Ok(Environment::Development),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let environment = std::env::var("ENV")
            .unwrap_or("development".to_string())
            .parse::<Environment>()?;

        if environment == Environment::Development {
            dotenv::dotenv().ok();
        }

        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let jwt_secret = std::env::var("JWT_SECRET").context("JWT_SECRET must be set")?;
        let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .context("PORT must be set")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .context("PORT must be a valid port number")?;

        Ok(Self {
            database_url,
            jwt_secret,
            host,
            port,
        })
    }

    pub fn hostname(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
