use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AuthError {
    TokenNotGenerated,
    NoData,
    ExpiredToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::TokenNotGenerated => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't generate token")
            }
            Self::NoData => (StatusCode::BAD_REQUEST, "No data provided"),
            Self::ExpiredToken => (StatusCode::BAD_REQUEST, "Expired token"),
        };

        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}
