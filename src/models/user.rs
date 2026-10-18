use padi_core::prelude::*;

pub const USERS: Model = Model::new("users")
    .fillable(&["name", "username", "email", "password", "role", "status", "remember_token"])
    .hidden(&["password", "remember_token"]);
