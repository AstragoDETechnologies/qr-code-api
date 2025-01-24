use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Version {
    NonEee, // EEE with non-EEE countries
    Eee,    // EEE with EEE countries
}

impl Version {
    pub fn value(&self) -> i32 {
        match self {
            Version::NonEee => return 1,
            Version::Eee => return 2,
        }
    }
}

impl TryFrom<i32> for Version {
    type Error = Box<dyn Error>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => return Ok(Version::NonEee),
            2 => return Ok(Version::Eee),
            _ => return Err("Unsupported Value".into()),
        }
    }
}
