use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{Level, event};

use crate::{app_state::AppState, error::AuthError, jwt::Claims, models::tag::TagRepository};

mod app_state;
mod database;
mod error;
mod jwt;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Get database URL from environment
    event!(Level::INFO, "Starting server");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create app state
    let app_state = AppState::new(pool);

    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/get-forbidden-tags", post(get_forbidden_tags))
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

async fn login(
    _state: State<AppState>,
    Json(body): Json<LoginParameters>,
) -> Result<Json<LoginReturn>, AuthError> {
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

#[derive(Deserialize, Debug)]
struct GetForbiddenTagsParameters {
    tags: Option<Vec<String>>,
}

#[derive(Serialize)]
struct GetForbiddenTagsReturn {
    tags: Vec<String>,
}

#[derive(Debug)]
enum TagError {
    NotFound,
}

impl axum::response::IntoResponse for TagError {
    fn into_response(self) -> axum::response::Response {
        match self {
            TagError::NotFound => {
                (axum::http::StatusCode::NOT_FOUND, "Tag not found").into_response()
            }
        }
    }
}

async fn get_forbidden_tags(
    state: State<AppState>,
    Json(body): Json<GetForbiddenTagsParameters>,
) -> Result<Json<GetForbiddenTagsReturn>, TagError> {
    let tags = body.tags;

    if tags.is_none() {
        let forbidden_tags = TagRepository::get_forbidden_tags(&state.db)
            .await
            .map_err(|_| TagError::NotFound)
            .unwrap_or_default();
        return Ok(Json(GetForbiddenTagsReturn {
            tags: forbidden_tags,
        }));
    }
    let tags = tags.unwrap_or_default();
    let forbidden_tags = TagRepository::get_forbidden_tags_from_collection(tags, &state.db)
        .await
        .map_err(|_| TagError::NotFound)?;
    Ok(Json(GetForbiddenTagsReturn {
        tags: forbidden_tags,
    }))
}
