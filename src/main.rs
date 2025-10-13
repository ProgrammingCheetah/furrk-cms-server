use std::collections::BTreeMap;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use yiffy_corner_server::{
    app_state::AppState, config::Config, errors::auth_error::AuthError, jwt::Claims,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("Failed to load config");
    let hostname = config.hostname();

    // Create database connection pool
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Create app state
    let app_state = AppState::new(pool, config);

    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(hostname)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn root(_state: State<AppState>) -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct LoginParameters {
    id: u64,
    first_name: String,
    auth_date: u64,
    hash: String,
    last_name: Option<String>,
    username: Option<String>,
    photo_url: Option<String>,
}

#[derive(Serialize)]
struct LoginReturn {
    token: String,
}

async fn login(
    _state: State<AppState>,
    Json(_body): Json<LoginParameters>,
) -> Result<Json<LoginReturn>, AuthError> {
    unimplemented!()
    // println!("Login requested: {:?}", body);

    // // Check the hash
    // let hash = &body.hash;
    // let mut fields: BTreeMap<&str, &str> = BTreeMap::new();
    // fields.insert("id", &body.id.to_string());
    // fields.insert("first_name", &body.first_name);
    // fields.insert("auth_date", &body.auth_date.to_string());

    // if let Some(ref last_name) = body.last_name {
    //     fields.insert("last_name", last_name);
    // }
    // if let Some(ref username) = body.username {
    //     fields.insert("username", username);
    // }
    // if let Some(ref photo_url) = body.photo_url {
    //     fields.insert("photo_url", photo_url);
    // }

    // let check_string = fields
    //     .into_iter()
    //     .map(|(key, value)| format!("{}={}", key, value))
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // let claims = Claims::from(telegram_id.clone());
    // let token = claims
    //     .token()
    //     .map(|s| s)
    //     .ok_or(AuthError::TokenNotGenerated)?;

    // Ok(Json(LoginReturn { token }))
}
