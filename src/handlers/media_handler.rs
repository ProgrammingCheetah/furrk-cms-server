use axum::extract::State;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{app_state::AppState, models::platform::Platform};

#[derive(Serialize, Deserialize)]
pub struct MediaItem {
    pub platform: Platform,
    pub source_url: Url,
    pub media_url: Url,
    pub author: String,
    pub file_type: String,
}

impl MediaItem {
    pub fn new(
        platform: Platform,
        source_url: Url,
        media_url: Url,
        author: String,
        file_type: String,
    ) -> Self {
        Self {
            platform,
            source_url,
            media_url,
            author,
            file_type,
        }
    }
}

/// Creates a post
pub async fn create(state: State<AppState>) {
    let db = state.get_db();
    // let result = sqlx::query!(
    //     "INSERT INTO media_item (platform, source_url, media_url, author, file_type) VALUES ($1, $2, $3, $4, $5)",
    //     media_item.platform,
    //     media_item.source_url,
    //     media_item.media_url,
    //     media_item.author,
    //     media_item.file_type,
    // )
    // .execute(db)
    // .await;
}
/// Reads a post
pub async fn read() {}
/// Updates a post
pub async fn update() {}
/// Deletes a post
pub async fn delete() {}
