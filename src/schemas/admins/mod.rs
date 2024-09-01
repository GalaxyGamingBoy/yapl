pub mod auth_with_password;
pub mod auth_refresh;
pub mod request_password_reset;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    id: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    email: String,
    avatar: i32
}