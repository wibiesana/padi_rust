use padi_core::prelude::*;

pub async fn index() -> ApiResult {
    ok(json!({"name": "Padi Rust API Framework", "version": env!("CARGO_PKG_VERSION")}))
}

pub async fn health() -> ApiResult {
    ok(json!({"status": "ok", "queries": Database::query_count()}))
}
