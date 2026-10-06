use padi_core::prelude::*;

use crate::controllers::auth;

/// Mounted at `/auth`.
pub fn routes() -> Router {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
}
