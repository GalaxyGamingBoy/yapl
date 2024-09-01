use serde::{Deserialize, Serialize};
use crate::schemas::admins::Admin;
use crate::schemas::ApiFieldError;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthWithPassword {
    pub token: String,
    pub admin: Admin
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthWithPasswordErrors {
    ValidationError(AuthWithPassword400)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthWithPassword400 {
    pub code: i32,
    pub message: String,
    pub data: AuthWithPassword400Data
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthWithPassword400Data {
    pub identity: Option<ApiFieldError>,
    pub password: Option<ApiFieldError>
}