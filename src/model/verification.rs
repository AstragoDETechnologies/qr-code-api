use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
    pub error_msg: Option<String>,
}
