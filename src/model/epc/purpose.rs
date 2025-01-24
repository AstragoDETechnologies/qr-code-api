use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum Purpose {
    Bene, // Unemployment benefit
    Dept, // Deposit
    Gdds, // Purchase/sale of goods
    Mtup, // Mobile top-up
    Pens, // Pensions
    Sala, // Salary
    Trad, // Trade business
}

impl Purpose {
    pub fn value(&self) -> String {
        match self {
            Purpose::Bene => String::from("BENE"),
            Purpose::Dept => String::from("DEPT"),
            Purpose::Gdds => String::from("GDDS"),
            Purpose::Mtup => String::from("MTUP"),
            Purpose::Pens => String::from("PENS"),
            Purpose::Sala => String::from("SALA"),
            Purpose::Trad => String::from("TRAD"),
        }
    }
}

impl TryFrom<&str> for Purpose {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().to_uppercase();

        match value.as_str() {
            "BENE" => Ok(Purpose::Bene),
            "DEPT" => Ok(Purpose::Dept),
            "GDDS" => Ok(Purpose::Gdds),
            "MTUP" => Ok(Purpose::Mtup),
            "PENS" => Ok(Purpose::Pens),
            "SALA" => Ok(Purpose::Sala),
            "TRAD" => Ok(Purpose::Trad),
            _ => Err("Unsupported Value".into()),
        }
    }
}
