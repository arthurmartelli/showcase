mod macros;

pub use crate::data::{self, AccessToken, Auth, Data, DataMap, DataSet};
pub use crate::emails::{self, Email, EmailAddress, Receiver, Sender};
pub use crate::template::{self, Template};
pub use macros::*;

pub type DynError = Box<dyn std::error::Error>;
