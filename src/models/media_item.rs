use url::Url;

use crate::models::platform::Platform;

pub struct MediaItem {
    pub platform: Platform,
    pub source_url: Url,
    pub media_url: Url,
    pub author: String,
    pub file_type: String,
}
