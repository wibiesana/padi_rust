use padi_core::{hash, prelude::*};

use crate::{models::USERS, resources::UserResource};

const TOKEN_TTL: i64 = 3600;

fn token_for(user: &Value) -> Result<String, ApiError> {
    Auth::generate_token(
        json!({"user_id": user["id"], "email": user["email"], "role": user["role"]}),
        TOKEN_TTL,
    )
}

pub async fn register(Body(body): Body) -> ApiResult {
    let mut data = Validator::new(
        &body,
        &[
            ("name", "required|string|min:2|max:100"),
            ("email", "required|email|unique:users,email"),
            ("password", "required|string|min:8"),
        ],
    )
    .validate()
    .await?;
    data["password"] = json!(hash::make(data["password"].as_str().unwrap_or("")));
    data["role"] = json!("user");

    let user = USERS.create(&data).await?;
    let token = token_for(&user)?;
    created(json!({"user": UserResource::wrap(&user), "token": token, "expires_in": TOKEN_TTL}))
}

pub async fn login(Body(body): Body) -> ApiResult {
    let d = Validator::new(&body, &[("email", "required|email"), ("password", "required|string")])
        .validate()
        .await?;
    let row = USERS.find_by("email", d["email"].as_str().unwrap_or("")).await?;
    let ok_pw = row
        .as_ref()
        .map(|u| hash::verify(d["password"].as_str().unwrap_or(""), u["password"].as_str().unwrap_or("")))
        .unwrap_or(false);
    let user = match row {
        Some(u) if ok_pw => USERS.present(u),
        _ => return Err(ApiError::unauthorized("Invalid credentials")),
    };
    let token = token_for(&user)?;
    ok(json!({"user": UserResource::wrap(&user), "token": token, "expires_in": TOKEN_TTL}))
}

pub async fn refresh(auth: AuthUser) -> ApiResult {
    let user = USERS.find_or_fail(auth.user_id().unwrap_or(0)).await?;
    ok(json!({"token": token_for(&user)?, "expires_in": TOKEN_TTL}))
}

/// Stateless JWT: client discards the token.
pub async fn logout(_auth: AuthUser) -> ApiResult {
    Ok(ApiResponse::new(200, Value::Null).message("Logged out"))
}

pub async fn me(auth: AuthUser) -> ApiResult {
    ok(UserResource::wrap(&USERS.find_or_fail(auth.user_id().unwrap_or(0)).await?))
}
