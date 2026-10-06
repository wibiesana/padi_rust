use padi_core::prelude::*;

use crate::controllers::users;

/// Mounted at `/users`. The `AuthUser` extractor in each handler acts as AuthMiddleware.
pub fn routes() -> Router {
    Router::new()
        .route("/", get(users::index).post(users::store))
        .route("/all", get(users::all))
        .route("/{id}", get(users::show).put(users::update).delete(users::destroy))
}
