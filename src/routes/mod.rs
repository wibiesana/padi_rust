use padi_core::prelude::*;

mod auth;
mod site;
mod users;

/// Mirrors app/Routes/api.php of the PHP template.
pub fn api() -> Router {
    Router::new()
        .merge(site::routes())
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
}
