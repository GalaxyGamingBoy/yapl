use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordReset {
    pub(crate) email: String
}