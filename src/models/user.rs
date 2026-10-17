use padi_core::prelude::*;

pub const USERS: Model = Model::new("users")
    .fillable(&["name", "email", "password", "role", "remember_token"])
    .hidden(&["password", "remember_token"]);
