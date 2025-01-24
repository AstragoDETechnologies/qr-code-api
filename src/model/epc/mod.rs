use iban::Iban;
use identification::Identification;
use purpose::Purpose;
use serde::{Deserialize, Serialize};
use std::error::Error;
use version::Version;

pub mod identification;
pub mod purpose;
pub mod version;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
    pub error_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpcQrCode {
    version: Version,
    identification: Identification,
    bic: Option<String>,
    name: String,
    iban: String,
    amount: Option<f32>,
    purpose: Option<Purpose>,
    remittance_ref: Option<String>,
    remittance_txt: Option<String>,
    information: Option<String>,
}

impl EpcQrCode {
    pub fn verify(&self) -> VerificationResult {
        // Verify existence of BIC
        if self.version == Version::NonEea && self.bic == None {
            return VerificationResult {
                success: false,
                error_msg: Some(String::from(
                    "BIC must be provided for codes in non-EEE regions.",
                )),
            };
        }

        // Verify Name
        if self.name.len() > 70 || self.name.len() <= 0 {
            return VerificationResult {
                success: false,
                error_msg: Some(String::from(
                    "Name must be between 1 and 70 characters long.",
                )),
            };
        }

        // Verify IBAN
        if let Err(error) = self.iban.parse::<Iban>() {
            return VerificationResult {
                success: false,
                error_msg: Some(format!("Iban is incorrect: {}", error)),
            };
        }

        // Verify Amount
        if let Some(amount) = self.amount {
            if amount > 999999999.99 || amount < 0.01 {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "The provided amount must be between 0.01 and 999999999.99.",
                    )),
                };
            }
        }

        // Verify Remittance
        if self.remittance_ref != None && self.remittance_txt != None {
            return VerificationResult {
                success: false,
                error_msg: Some(String::from("Only one Remittance can be used at a time.")),
            };
        }

        // Verify Remittance (Reference)
        if let Some(remittance_ref) = &self.remittance_ref {
            if remittance_ref.len() > 25 || remittance_ref.len() <= 0 {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "If remittance_ref is defined, it must be between 1 and 25 characters long.",
                    )),
                };
            }
        }

        // Verify Remittance (Text)
        if let Some(remittance_txt) = &self.remittance_txt {
            if remittance_txt.len() > 140 || remittance_txt.len() <= 0 {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "If remittance_txt is defined, it must be between 1 and 140 characters long.",
                    )),
                };
            }
        }

        // Verify Information
        if let Some(information) = &self.information {
            if information.len() > 70 || information.len() <= 0 {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "If information is defined, it must be between 1 and 70 characters long.",
                    )),
                };
            }
        }

        // All checks passed
        VerificationResult {
            success: true,
            error_msg: None,
        }
    }

    pub fn generate_epc_string(&self) -> Result<String, Box<dyn Error>> {
        // Exit on Verification
        let verification_result = self.verify();
        if verification_result.success == false {
            return Err(format!(
                "Could not generate EPC-String due to incorrect Data: {}",
                verification_result
                    .error_msg
                    .unwrap_or(String::from("No further information"))
            )
            .into());
        }

        let mut output = String::new();
        // Service Tag
        output += "BCD\n";

        // Version
        output += &format!("{:03}\n", self.version.value());

        // Character Set (UTF-8)
        output += "1\n";

        // Identification
        output += &format!("{}\n", self.identification.value());

        // Bic
        match &self.bic {
            Some(bic) => {
                output += &format!("{}\n", bic);
            }
            None => {
                output += "\n";
            }
        }

        // Name
        output += &format!("{}\n", self.name);

        // IBAN
        output += &format!("{}\n", self.iban);

        // Amount
        if let Some(amount) = self.amount {
            output += &format!("EUR{:.2}\n", amount);
        }

        // Purpose
        match &self.purpose {
            Some(purpose) => output += &format!("{}\n", purpose.value()),
            None => output += "\n",
        }

        // Remittance (Reference)
        match &self.remittance_ref {
            Some(remittance) => output += &format!("{}\n", remittance),
            None => output += "\n",
        }

        // Remittance (Text)
        match &self.remittance_txt {
            Some(remittance) => output += &format!("{}\n", remittance),
            None => output += "\n",
        }

        // Information
        if let Some(information) = &self.information {
            output += information;
        }

        Ok(output)
    }
}
