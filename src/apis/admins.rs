use std::rc::Rc;
use reqwest::{Client, StatusCode};
use crate::pocketbase::Pocketbase;
use crate::schemas::admins::{AuthWithPassword, AuthWithPassword400, AuthWithPasswordErrors};
use crate::schemas::password_auth::PasswordAuth;

pub struct Admins {
    client: Rc<Client>,
    endpoint: Rc<String>
}

impl Admins {
    pub fn new(client: Rc<Client>, endpoint: Rc<String>, test: &Pocketbase) -> Self {
        Self {client, endpoint}
    }
    pub async fn auth_with_password(&self, password_auth: PasswordAuth) -> Result<AuthWithPassword, AuthWithPasswordErrors> {
        let req = self.client.post(format!("{}/api/admins/auth-with-password", self.endpoint)).json(&password_auth).send().await.unwrap();
        match req.status() {
            StatusCode::OK => Ok(req.json::<AuthWithPassword>().await.unwrap()),
            StatusCode::BAD_REQUEST => Err(AuthWithPasswordErrors::BadRequest(
                req.json::<AuthWithPassword400>().await.unwrap()
            )),
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schemas::password_auth::PasswordAuth;
    use crate::pocketbase::Pocketbase;

    fn pocketbase() -> Pocketbase {
        Pocketbase::new("http://127.0.0.1:8090".to_string())
    }
    #[tokio::test]
    async fn auth_with_password() {
        let user = PasswordAuth {identity: "test@example.com".to_string(), password: "testexamplecom".to_string()};
        let req = pocketbase().admins.auth_with_password(user).await;

        assert!(req.is_ok());
        assert!(!req.unwrap().token.is_empty())
    }
}