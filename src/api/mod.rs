pub mod handlers;
use axum::{Router, routing::get, routing::post};
use rust_embed::RustEmbed;
// use rust_embed::axum::RustEmbedLayer;
use crate::api::handlers::AppState;

#[derive(RustEmbed)]
#[folder = "../../web/dist/"]
pub struct WebAssets;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/config", get(handlers::get_config))
        .route("/api/scan/trigger", post(handlers::trigger_scan))
        .with_state(state)
        // static assets layer removed for compatibility with current rust-embed version
        // .layer(RustEmbedLayer::new(WebAssets::fallback_index_html()))
}
