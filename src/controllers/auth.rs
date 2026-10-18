use padi_core::{hash, prelude::*};

use crate::{models::USERS, resources::UserResource};

const TOKEN_TTL: i64 = 3600;
const REMEMBER_ME_TTL: i64 = 365 * 24 * 3600; // 31536000 seconds (1 year)

fn token_for_with_ttl(user: &Value, ttl: i64) -> Result<String, ApiError> {
    Auth::generate_token(
        json!({"user_id": user["id"], "email": user["email"], "role": user["role"]}),
        ttl,
    )
}

fn token_for(user: &Value) -> Result<String, ApiError> {
    token_for_with_ttl(user, TOKEN_TTL)
}

fn auth_payload(user: &Value, token: &str, expires_in: i64, remember_token: Option<&str>) -> Value {
    let mut payload = json!({
        "user": UserResource::wrap(user),
        "token": token,
        "expires_in": expires_in,
    });
    if let Some(rt) = remember_token {
        payload["remember_token"] = json!(rt);
    }
    payload
}

fn is_truthy(val: Option<&Value>) -> bool {
    match val {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64() == Some(1),
        Some(Value::String(s)) => {
            let s = s.trim().to_lowercase();
            s == "true" || s == "1" || s == "yes"
        }
        _ => false,
    }
}

fn is_user_active(user: &Value) -> bool {
    if let Some(role) = user.get("role").and_then(|v| v.as_str()) {
        if role.eq_ignore_ascii_case("superadmin") {
            return true;
        }
    }

    match user.get("status") {
        None | Some(Value::Null) => true,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64() == Some(1),
        Some(Value::String(s)) => {
            let s = s.trim().to_lowercase();
            s.is_empty() || s == "active" || s == "1" || s == "true"
        }
        _ => true,
    }
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
    created(auth_payload(&user, &token, TOKEN_TTL, None))
}

pub async fn login(Body(body): Body) -> ApiResult {
    let identifier = body
        .get("email")
        .or_else(|| body.get("username"))
        .or_else(|| body.get("login"))
        .or_else(|| body.get("user"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .unwrap_or("");

    let password = body
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let remember_me = is_truthy(body.get("remember_me"));

    let mut errors: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();

    if identifier.is_empty() {
        let key = if body.get("username").is_some() {
            "username"
        } else if body.get("login").is_some() {
            "login"
        } else if body.get("user").is_some() {
            "user"
        } else {
            "email"
        };
        errors.insert(key.to_string(), vec![format!("The {} or username field is required", key)]);
    }

    if password.is_empty() {
        errors.insert("password".to_string(), vec!["The password field is required".to_string()]);
    }

    if !errors.is_empty() {
        return Err(ApiError::Validation(errors));
    }

    // Try finding user by email first, then username, then name
    let mut row = USERS.find_by("email", identifier).await.ok().flatten();

    if row.is_none() {
        if let Ok(Some(u)) = USERS.find_by("username", identifier).await {
            row = Some(u);
        }
    }

    if row.is_none() {
        if let Ok(Some(u)) = USERS.find_by("name", identifier).await {
            row = Some(u);
        }
    }

    let ok_pw = row
        .as_ref()
        .map(|u| hash::verify(password, u["password"].as_str().unwrap_or("")))
        .unwrap_or(false);

    let raw_user = match row {
        Some(u) if ok_pw => u,
        _ => return Err(ApiError::unauthorized("Invalid credentials")),
    };

    if !is_user_active(&raw_user) {
        return Err(ApiError::unauthorized("Account is not active"));
    }

    let user_id = raw_user.get("id").and_then(|id| {
        id.as_i64().or_else(|| id.as_str().and_then(|s| s.parse().ok()))
    }).unwrap_or(0);

    let mut remember_token_str = None;
    let (ttl, token) = if remember_me {
        let rt = Auth::generate_remember_token();
        if user_id > 0 {
            let _ = USERS.update(user_id, &json!({"remember_token": &rt})).await;
        }
        remember_token_str = Some(rt);
        let tok = token_for_with_ttl(&raw_user, REMEMBER_ME_TTL)?;
        (REMEMBER_ME_TTL, tok)
    } else {
        let tok = token_for_with_ttl(&raw_user, TOKEN_TTL)?;
        (TOKEN_TTL, tok)
    };

    let user = USERS.present(raw_user);
    ok(auth_payload(&user, &token, ttl, remember_token_str.as_deref()))
}

pub async fn refresh(headers: HeaderMap, Body(body): Body) -> ApiResult {
    let remember_token = body
        .get("remember_token")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let (raw_user, is_remember) = if let Some(rt) = remember_token {
        let row = USERS.find_by("remember_token", rt).await.ok().flatten();
        match row {
            Some(u) => (u, true),
            None => return Err(ApiError::unauthorized("Invalid or expired remember token")),
        }
    } else if let Some(auth_header) = headers.get("authorization").and_then(|h| h.to_str().ok()) {
        let token = auth_header
            .strip_prefix("Bearer ")
            .or_else(|| auth_header.strip_prefix("bearer "))
            .unwrap_or("")
            .trim();
        let claims = Auth::verify_token(token).ok_or_else(|| ApiError::unauthorized("Invalid or expired token"))?;
        let user_id = claims
            .get("user_id")
            .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
            .ok_or_else(|| ApiError::unauthorized("Invalid token claims"))?;
        let u = USERS.find_or_fail(user_id).await?;
        (u, false)
    } else {
        return Err(ApiError::validation("remember_token", "The remember_token field is required"));
    };

    if !is_user_active(&raw_user) {
        return Err(ApiError::unauthorized("Your account is inactive. Please contact support."));
    }

    let ttl = if is_remember { REMEMBER_ME_TTL } else { TOKEN_TTL };
    let token = token_for_with_ttl(&raw_user, ttl)?;
    let user = USERS.present(raw_user);
    ok(auth_payload(&user, &token, ttl, remember_token))
}

/// Stateless JWT: client discards the token.
pub async fn logout(_auth: AuthUser) -> ApiResult {
    Ok(ApiResponse::new(200, Value::Null).message("Logged out"))
}

pub async fn me(auth: AuthUser) -> ApiResult {
    let user_id = auth.user_id().ok_or_else(|| ApiError::unauthorized("Unauthenticated"))?;
    ok(UserResource::wrap(&USERS.find_or_fail(user_id).await?))
}
