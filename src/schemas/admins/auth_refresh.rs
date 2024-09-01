use serde::{Deserialize, Serialize};
use crate::schemas::admins::Admin;
use crate::schemas::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRefresh {
    pub token: String,
    pub admin: Admin
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthRefreshErrors {
    Unauthorized(ApiError),
    Forbidden(ApiError),
    MissingAdminContext(ApiError)
}