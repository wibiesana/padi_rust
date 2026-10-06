use padi_core::prelude::*;

use crate::controllers::site;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(site::index))
        .route("/health", get(site::health))
}
