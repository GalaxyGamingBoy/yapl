use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod password_auth;
pub mod admins;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    code: i32,
    message: String,
    data: Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiFieldError {
    pub code: String,
    pub message: String
}