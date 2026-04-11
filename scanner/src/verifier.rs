use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Valid,
    Invalid,
    Unknown(String),
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub status: VerificationStatus,
    pub message: String,
}

pub struct Verifier {
    client: Client,
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .user_agent("fsesc-scanner/0.2.0")
                .build()
                .unwrap(),
        }
    }

    pub async fn verify(&self, pattern_name: &str, secret: &str) -> VerificationResult {
        match pattern_name {
            "GitHub Personal Access Token" => self.verify_github(secret).await,
            "Stripe API Key" => self.verify_stripe(secret).await,
            _ => VerificationResult {
                status: VerificationStatus::Skipped,
                message: "No verification logic for this pattern".to_string(),
            },
        }
    }

    async fn verify_github(&self, token: &str) -> VerificationResult {
        let res = self.client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => VerificationResult {
                status: VerificationStatus::Valid,
                message: "Valid GitHub Token".to_string(),
            },
            Ok(resp) if resp.status().as_u16() == 401 => VerificationResult {
                status: VerificationStatus::Invalid,
                message: "Invalid/Revoked GitHub Token".to_string(),
            },
            Ok(resp) => VerificationResult {
                status: VerificationStatus::Unknown(resp.status().to_string()),
                message: format!("GitHub API returned {}", resp.status()),
            },
            Err(e) => VerificationResult {
                status: VerificationStatus::Unknown("Network Error".to_string()),
                message: e.to_string(),
            },
        }
    }

    async fn verify_stripe(&self, key: &str) -> VerificationResult {
        let res = self.client
            .get("https://api.stripe.com/v1/accounts")
            .basic_auth(key, Some(""))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => VerificationResult {
                status: VerificationStatus::Valid,
                message: "Valid Stripe Key".to_string(),
            },
            Ok(resp) if resp.status().as_u16() == 401 => VerificationResult {
                status: VerificationStatus::Invalid,
                message: "Invalid/Unauthorized Stripe Key".to_string(),
            },
            Ok(resp) => VerificationResult {
                status: VerificationStatus::Unknown(resp.status().to_string()),
                message: format!("Stripe API returned {}", resp.status()),
            },
            Err(e) => VerificationResult {
                status: VerificationStatus::Unknown("Network Error".to_string()),
                message: e.to_string(),
            },
        }
    }
}
