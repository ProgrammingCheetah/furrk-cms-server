use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
    extract::State,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{error::AuthError, jwt::Claims, app_state::AppState};

mod app_state;
mod database;
mod error;
mod jwt;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // Create app state
    let app_state = AppState::new(pool);
    
    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root(_state: State<AppState>) -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct LoginParameters {
    telegram_id: String,
}

#[derive(Serialize)]
struct LoginReturn {
    token: String,
}

async fn login(_state: State<AppState>, Json(body): Json<LoginParameters>) -> Result<Json<LoginReturn>, AuthError> {
    println!("Login requested: {:?}", body);
    let telegram_id = body.telegram_id;

    if telegram_id.is_empty() {
        return Err(AuthError::NoData);
    }

    let claims = Claims::from(telegram_id.clone());
    let token = claims
        .token()
        .map(|s| s)
        .ok_or(AuthError::TokenNotGenerated)?;

    Ok(Json(LoginReturn { token }))
}
