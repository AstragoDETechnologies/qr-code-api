use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Version {
    NonEea, // EEE with non-EEE countries
    Eea,    // EEE with EEE countries
}

impl Version {
    pub fn value(&self) -> i32 {
        match self {
            Version::NonEea => return 1,
            Version::Eea => return 2,
        }
    }
}

impl TryFrom<i32> for Version {
    type Error = Box<dyn Error>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => return Ok(Version::NonEea),
            2 => return Ok(Version::Eea),
            _ => return Err("Unsupported Value".into()),
        }
    }
}
