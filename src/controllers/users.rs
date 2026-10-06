use padi_core::{hash, prelude::*};

use crate::{models::USERS, resources::UserResource};

pub async fn index(_auth: AuthUser, p: Params) -> ApiResult {
    let (page, per) = p.pagination();
    ok(UserResource::wraps(&USERS.paginate(page, per).await?))
}

pub async fn all(_auth: AuthUser) -> ApiResult {
    ok(UserResource::wraps(&json!(USERS.all().await?)))
}

pub async fn show(_auth: AuthUser, Path(id): Path<i64>) -> ApiResult {
    ok(UserResource::wrap(&USERS.find_or_fail(id).await?))
}

pub async fn store(auth: AuthUser, Body(body): Body) -> ApiResult {
    auth.require_role(&["admin"])?;
    let mut d = Validator::new(
        &body,
        &[
            ("name", "required|string|min:2|max:100"),
            ("email", "required|email|unique:users,email"),
            ("password", "required|string|min:8"),
            ("role", "sometimes|in:admin,user"),
        ],
    )
    .validate()
    .await?;
    d["password"] = json!(hash::make(d["password"].as_str().unwrap_or("")));
    created(UserResource::wrap(&USERS.create(&d).await?))
}

pub async fn update(auth: AuthUser, Path(id): Path<i64>, Body(body): Body) -> ApiResult {
    if auth.user_id() != Some(id) {
        auth.require_role(&["admin"])?;
    }
    let ignore = id.to_string();
    let email_rule = format!("sometimes|email|unique:users,email,{ignore}");
    let mut d = Validator::new(
        &body,
        &[
            ("name", "sometimes|string|min:2|max:100"),
            ("email", email_rule.as_str()),
            ("password", "sometimes|string|min:8"),
        ],
    )
    .validate()
    .await?;
    if let Some(pw) = d.get("password").and_then(|v| v.as_str()).map(hash::make) {
        d["password"] = json!(pw);
    }
    ok(UserResource::wrap(&USERS.update(id, &d).await?))
}

pub async fn destroy(auth: AuthUser, Path(id): Path<i64>) -> ApiResult {
    auth.require_role(&["admin"])?;
    USERS.delete(id).await?;
    no_content()
}
