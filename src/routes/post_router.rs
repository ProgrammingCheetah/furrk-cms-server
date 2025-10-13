use axum::{Router, routing::get};

use crate::{app_state::AppState, handlers::media_handler};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(media_handler::read)
                .post(media_handler::create)
                .put(media_handler::update)
                .delete(media_handler::delete),
        )
        .with_state(app_state)
}
