use serde::{Deserialize, Serialize};
use crate::schemas::ApiFieldError;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordReset {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestPasswordResetErrors {
    ValidationError(RequestPasswordReset400)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordReset400 {
    code: i32,
    message: String,
    data: RequestPasswordReset400Data
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordReset400Data {
    email: ApiFieldError
}