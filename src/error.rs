use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Error {
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}