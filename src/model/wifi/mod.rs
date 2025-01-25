pub mod encryption;

use super::verification::VerificationResult;
use encryption::EncryptionType;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct WifiQrCode {
    ssid: String,
    encryption: Option<EncryptionType>,
    password: Option<String>,
    hidden: Option<bool>,
}

impl WifiQrCode {
    pub fn verify(&self) -> VerificationResult {
        // Empty SSID
        if self.ssid == "" {
            return VerificationResult {
                success: false,
                error_msg: Some(String::from("The SSID must not be empty.")),
            };
        }

        // Encryption, but no passowrd
        if let Some(_) = &self.encryption {
            if self.password == None {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "A password must be provided when using an encrypted network.",
                    )),
                };
            }
        }

        // Password, but no encryption
        if let Some(password) = &self.password {
            if self.encryption == None {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "An encrytion method must be provided when using a password.",
                    )),
                };
            }

            if password.len() < 8 {
                return VerificationResult {
                    success: false,
                    error_msg: Some(String::from(
                        "The password must be at least 8 characters long.",
                    )),
                };
            }
        }

        VerificationResult {
            success: true,
            error_msg: None,
        }
    }

    pub fn generate_wifi_string(&self) -> Result<String, Box<dyn Error>> {
        let verification_result = self.verify();
        if verification_result.success == false {
            return Err(format!(
                "Could not generate WIFI-String due to incorrect Data: {}",
                verification_result
                    .error_msg
                    .unwrap_or(String::from("No further Information."))
            )
            .into());
        }

        let mut output = String::new();

        // WIFI Info
        output += "WIFI:";

        // SSID
        output += &format!("S:{};", self.ssid);

        // Encryption
        match &self.encryption {
            Some(encryption_type) => output += &format!("T:{};", encryption_type.value()),
            None => output += "T:nopass;",
        }

        // Password
        match &self.password {
            Some(password) => output += &format!("P:{};", password),
            None => output += "P:;",
        }

        // Hidden
        match &self.hidden {
            Some(hidden) => output += &format!("H:{};", hidden),
            None => output += "H:false;",
        }

        Ok(output)
    }
}
