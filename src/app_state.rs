use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        AppState { db }
    }

    pub async fn get_db(&self) -> &PgPool {
        &self.db
    }
}
