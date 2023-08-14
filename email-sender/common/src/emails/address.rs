use crate::data::Data;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EmailAddress {
    name: Data,
    email: Data,
}

impl ToString for EmailAddress {
    fn to_string(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

impl EmailAddress {
    pub fn new<T: Into<Data>>(name: T, email: T) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }

    pub fn name(&self) -> &Data {
        &self.name
    }

    pub fn email(&self) -> &Data {
        &self.email
    }
}
