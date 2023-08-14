use super::address::EmailAddress;
use crate::data::{AccessToken, Data};

#[derive(Debug)]
pub struct Sender {
    email: EmailAddress,
    token: AccessToken,
    server: Data,
}

impl ToString for Sender {
    fn to_string(&self) -> String {
        self.email.to_string()
    }
}

impl Sender {
    pub fn new<T: Into<Data>>(email: EmailAddress, token: AccessToken, server: T) -> Self {
        Self {
            email,
            token,
            server: server.into(),
        }
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn token(&self) -> &AccessToken {
        &self.token
    }

    pub fn set_token(&mut self, token: AccessToken) {
        self.token = token;
    }

    pub fn set_server(&mut self, server: Data) {
        self.server = server;
    }

    pub fn server(&self) -> &Data {
        &self.server
    }
}
