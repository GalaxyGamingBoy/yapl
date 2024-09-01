use std::fmt::format;
use std::rc::Rc;
use reqwest::{Client, StatusCode};
use crate::data::password_auth::PasswordAuth;
use crate::schemas::admins::auth_refresh::{AuthRefresh, AuthRefreshErrors};
use crate::schemas::admins::auth_with_password::{AuthWithPassword, AuthWithPassword400, AuthWithPasswordErrors};
use crate::schemas::ApiError;

pub struct Admins {
    client: Rc<Client>,
    endpoint: Rc<String>
}

impl Admins {
    pub fn new(client: Rc<Client>, endpoint: Rc<String>) -> Self {
        Self {client, endpoint}
    }

    pub async fn auth_with_password(&self, password_auth: PasswordAuth) -> Result<AuthWithPassword, AuthWithPasswordErrors> {
        let req =
            self.client.post(format!("{}/api/admins/auth-with-password", self.endpoint))
                .json(&password_auth).send().await.unwrap();

        match req.status() {
            StatusCode::OK => Ok(req.json::<AuthWithPassword>().await.unwrap()),
            StatusCode::BAD_REQUEST => Err(AuthWithPasswordErrors::BadRequest(
                req.json::<AuthWithPassword400>().await.unwrap()
            )),
            _ => unreachable!()
        }
    }

    pub async fn auth_refresh(&self) -> Result<AuthRefresh, AuthRefreshErrors> {
        let req =
            self.client.post(format!("{}/api/admins/auth-refresh", self.endpoint))
                .send().await.unwrap();

        match req.status() {
            StatusCode::OK => Ok(req.json::<AuthRefresh>().await.unwrap()),
            StatusCode::UNAUTHORIZED => Err(AuthRefreshErrors::Unauthorized(req.json::<ApiError>().await.unwrap())),
            StatusCode::FORBIDDEN => Err(AuthRefreshErrors::Forbidden(req.json::<ApiError>().await.unwrap())),
            StatusCode::NOT_FOUND => Err(AuthRefreshErrors::MissingAdminContext(req.json::<ApiError>().await.unwrap())),
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::password_auth::PasswordAuth;
    use crate::pocketbase::Pocketbase;

    async fn pocketbase() -> Pocketbase {
        let user = PasswordAuth {identity: "test@example.com".to_string(), password: "testexamplecom".to_string()};
        let pb = Pocketbase::new("http://127.0.0.1:8090".to_string());
        pb.auth_with_password(user).await.unwrap()
    }

    #[tokio::test]
    async fn auth_with_password() {
        let pocketbase = Pocketbase::new("http://127.0.0.1:8090".to_string());
        let user = PasswordAuth {identity: "test@example.com".to_string(), password: "testexamplecom".to_string()};
        let req = pocketbase.admins.auth_with_password(user).await;

        assert!(req.is_ok());
        assert!(!req.unwrap().token.is_empty())
    }

    #[tokio::test]
    async fn auth_refresh() {
        let req = pocketbase().await.admins.auth_refresh().await;


        assert!(req.is_ok());
        assert!(!req.unwrap().token.is_empty())
    }
}