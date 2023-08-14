use super::address::EmailAddress;
use crate::data::{self, Data, DataMap};
use crate::data_map;

#[derive(Debug, PartialEq, Eq)]
pub struct Receiver {
    email: EmailAddress,
    local_context: DataMap,
}

impl std::hash::Hash for Receiver {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

impl ToString for Receiver {
    fn to_string(&self) -> String {
        self.email.to_string()
    }
}

impl Receiver {
    pub fn new(email: EmailAddress, context: Option<DataMap>) -> Self {
        let mut local_context: DataMap = data_map!(
            Data::from("name") => email.name().clone(),
            Data::from("email") => email.email().clone()
        );

        if let Some(context) = context {
            local_context = data::merge_data_maps(local_context, &context);
        }

        Self {
            email,
            local_context,
        }
    }

    pub fn local_context(&self) -> &DataMap {
        &self.local_context
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }
}
