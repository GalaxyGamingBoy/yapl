use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schemas::ApiFieldError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    id: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    email: String,
    avatar: i32
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthWithPasswordErrors {
    BadRequest(AuthWithPassword400)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthWithPassword {
    pub token: String,
    pub admin: Admin
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