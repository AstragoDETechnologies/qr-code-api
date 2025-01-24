use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum Identification {
    Sct,  // SEPA Credit Transfer
    Inst, // SEPA Instant Credit Transfer
}

impl Identification {
    pub fn value(&self) -> String {
        match self {
            Identification::Sct => return String::from("SCT"),
            Identification::Inst => return String::from("INST"),
        }
    }
}

impl TryFrom<&str> for Identification {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: String = value.trim().to_uppercase();
        match value.as_str() {
            "SCT" => return Ok(Identification::Sct),
            "INST" => return Ok(Identification::Inst),
            _ => Err("Unsupported Value".into()),
        }
    }
}
