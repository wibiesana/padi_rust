use padi_core::prelude::*;

/// Shapes a users row for API output.
pub struct UserResource;

impl Resource for UserResource {
    fn to_array(u: &Value) -> Value {
        json!({
            "id": u["id"],
            "name": u["name"],
            "email": u["email"],
            "role": u["role"],
            "created_at": u["created_at"],
            "updated_at": u["updated_at"],
        })
    }
}
