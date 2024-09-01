use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordAuth {
    pub identity: String,
    pub password: String
}