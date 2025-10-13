use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    config: Config,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        AppState { db, config }
    }

    pub fn get_db(&self) -> &PgPool {
        &self.db
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}
