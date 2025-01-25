use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EncryptionType {
    Wpa,
    Wep,
}

impl EncryptionType {
    pub fn value(&self) -> String {
        match self {
            EncryptionType::Wep => return String::from("WEP"),
            EncryptionType::Wpa => return String::from("WPA"),
        }
    }
}

impl TryFrom<&str> for EncryptionType {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let binding = value.trim().to_uppercase();
        let value = binding.as_str();

        match value {
            "WEP" => return Ok(EncryptionType::Wep),
            "WPA" => return Ok(EncryptionType::Wpa),
            _ => return Err("Error parsing Data.".into()),
        }
    }
}
