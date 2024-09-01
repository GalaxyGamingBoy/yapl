use std::cell::RefCell;
use std::rc::Rc;
use reqwest::{Client, Proxy};
use crate::apis::admins::Admins;

pub struct Pocketbase {
    #[allow(dead_code)]
    client: Rc<Client>,
    #[allow(dead_code)]
    endpoint: Rc<String>,
    pub admins: Admins
}

impl Pocketbase {
    pub fn new(endpoint: String) -> Self {
        let client = Rc::new(Self::new_client());
        let endpoint = Rc::new(endpoint);

        Self {
            client: client.clone(),
            admins: Admins::new(client, endpoint.clone(), &Self),
            endpoint
        }
    }
}