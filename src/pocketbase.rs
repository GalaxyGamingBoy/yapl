use std::rc::Rc;
use reqwest::{Client, Proxy};
use reqwest::header::HeaderMap;
use crate::apis::admins::Admins;
use crate::data::password_auth::PasswordAuth;
use crate::schemas::admins::auth_with_password::AuthWithPasswordErrors;

pub struct Pocketbase {
    #[allow(dead_code)]
    client: Rc<Client>,
    #[allow(dead_code)]
    endpoint: Rc<String>,
    #[allow(dead_code)]
    token: String,
    pub admins: Admins
}

impl Pocketbase {
    pub fn new(endpoint: String) -> Self {
        Self::new_with_token(endpoint, String::new())
    }

    pub fn new_with_token(endpoint: String, token: String) -> Self {
        let client = Rc::new(Self::new_client(token.clone()));
        let endpoint = Rc::new(endpoint);

        Self {
            client: client.clone(),
            admins: Admins::new(client, endpoint.clone()),
            token,
            endpoint
        }
    }

    pub async fn auth_with_password(&self, password_auth: PasswordAuth) -> Result<Pocketbase, AuthWithPasswordErrors> {
        match self.admins.auth_with_password(password_auth).await {
            Ok(val) => Ok(Pocketbase::new_with_token(self.endpoint.to_string(), val.token)),
            Err(err) => Err(err)
        }
    }

    fn new_client(token: String) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        Client::builder()
            .proxy(Proxy::https("http://localhost:8080").unwrap())
            .proxy(Proxy::http("http://localhost:8080").unwrap())
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build().unwrap()
    }
}