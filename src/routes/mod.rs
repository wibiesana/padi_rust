use padi_core::prelude::*;

mod auth;
mod site;
mod users;

/// Main API router combining public, auth, and resource endpoints.
pub fn api() -> Router {
    Router::new()
        .merge(site::routes())
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
}
