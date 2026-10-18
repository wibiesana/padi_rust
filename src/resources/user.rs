use padi_core::prelude::*;

/// Shapes a users row for API output.
pub struct UserResource;

impl Resource for UserResource {
    fn to_array(u: &Value) -> Value {
        let mut obj = json!({
            "id": u["id"],
            "email": u["email"],
            "role": u["role"],
            "created_at": u["created_at"],
            "updated_at": u["updated_at"],
        });
        if let Some(name) = u.get("name").filter(|v| !v.is_null()) {
            obj["name"] = name.clone();
        }
        if let Some(username) = u.get("username").filter(|v| !v.is_null()) {
            obj["username"] = username.clone();
        }
        if let Some(status) = u.get("status").filter(|v| !v.is_null()) {
            obj["status"] = status.clone();
        }
        obj
    }
}
