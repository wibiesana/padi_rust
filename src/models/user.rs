use padi_core::prelude::*;

pub const USERS: Model = Model::new("users")
    .fillable(&["name", "email", "password", "role"])
    .hidden(&["password"]);
