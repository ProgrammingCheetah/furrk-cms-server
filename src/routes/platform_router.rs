use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{app_state::AppState, handlers::platform_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get(platform_handler::read)
                .post(platform_handler::create)
                .put(platform_handler::update)
                .delete(platform_handler::delete),
        )
        .with_state(app_state)
}
