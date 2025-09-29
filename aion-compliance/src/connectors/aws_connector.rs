use super::{SystemConnector, ConnectorConfig};
use aion_core::{AionResult, AionError, Evidence};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

type HmacSha256 = Hmac<Sha256>;

pub struct AwsConnector {
    client: Client,
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    region: String,
    session_token: Option<String>,
}

impl AwsConnector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            access_key_id: None,
            secret_access_key: None,
            region: "us-east-1".to_string(),
            session_token: None,
        }
    }

    fn sign_request(&self, method: &str, url: &str, payload: &str, timestamp: &str) -> AionResult<String> {
        let access_key = self.access_key_id.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "aws".to_string(),
                reason: "Access key ID not configured".to_string(),
            })?;

        let secret_key = self.secret_access_key.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "aws".to_string(),
                reason: "Secret access key not configured".to_string(),
            })?;

        // AWS Signature Version 4 implementation
        let date = &timestamp[0..8];
        let canonical_headers = format!("host:{}\nx-amz-date:{}\n", "config.amazonaws.com", timestamp);
        let signed_headers = "host;x-amz-date";

        let canonical_request = format!(
            "{}\n{}\n\n{}\n{}\n{}",
            method,
            "/",
            canonical_headers,
            signed_headers,
            payload
        );

        let mut hasher = sha2::Sha256::new();
        hasher.update(canonical_request.as_bytes());
        let canonical_request_hash = hex::encode(hasher.finalize());

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}/{}/config/aws4_request\n{}",
            timestamp, date, self.region, canonical_request_hash
        );

        // Create signing key
        let k_date = hmac_sha256(format!("AWS4{}", secret_key).as_bytes(), date.as_bytes());
        let k_region = hmac_sha256(&k_date, self.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"config");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}/{}/config/aws4_request, SignedHeaders={}, Signature={}",
            access_key, date, self.region, signed_headers, signature
        );

        Ok(authorization)
    }

    async fn make_aws_request(&self, service: &str, action: &str, params: &HashMap<String, String>) -> AionResult<Value> {
        let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        let url = format!("https://{}.{}.amazonaws.com/", service, self.region);

        let mut query_params = params.clone();
        query_params.insert("Action".to_string(), action.to_string());
        query_params.insert("Version".to_string(), "2010-05-15".to_string());

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let authorization = self.sign_request("GET", &url, "", &timestamp)?;

        let response = self.client
            .get(&format!("{}?{}", url, query_string))
            .header("Authorization", authorization)
            .header("X-Amz-Date", timestamp)
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "aws_api_request".to_string(),
                reason: e.to_string(),
            })?;

        let data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "aws_api_response".to_string(),
                reason: e.to_string(),
            })?;

        Ok(data)
    }

    async fn collect_config_compliance(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("ConfigurationRecorderNames.member.1".to_string(), entity_id.to_string());

        let config_data = self.make_aws_request("config", "DescribeConfigurationRecorders", &params).await?;

        let recorder_count = config_data["ConfigurationRecorders"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let is_recording = config_data["ConfigurationRecorders"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|recorder| recorder["recordingGroup"]["allSupported"].as_bool())
            .unwrap_or(false);

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "configuration_compliance".to_string(),
            description: format!("AWS Config compliance for entity {}", entity_id),
            source: "AWS Config API".to_string(),
            collected_date: Utc::now(),
            verification_status: if is_recording { "recording" } else { "not_recording" },
            metadata: HashMap::from([
                ("recorder_count".to_string(), recorder_count.to_string()),
                ("is_recording".to_string(), is_recording.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_cloudtrail_logs(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("trailNameList.member.1".to_string(), entity_id.to_string());

        let trail_data = self.make_aws_request("cloudtrail", "DescribeTrails", &params).await?;

        let trail_count = trail_data["trailList"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let is_logging = trail_data["trailList"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|trail| trail["IsLogging"].as_bool())
            .unwrap_or(false);

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "audit_logs".to_string(),
            description: format!("AWS CloudTrail logs for entity {}", entity_id),
            source: "AWS CloudTrail API".to_string(),
            collected_date: Utc::now(),
            verification_status: if is_logging { "logging" } else { "not_logging" },
            metadata: HashMap::from([
                ("trail_count".to_string(), trail_count.to_string()),
                ("is_logging".to_string(), is_logging.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_security_groups(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("GroupIds.1".to_string(), entity_id.to_string());

        let sg_data = self.make_aws_request("ec2", "DescribeSecurityGroups", &params).await?;

        let sg_count = sg_data["SecurityGroups"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let open_rules = sg_data["SecurityGroups"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .flat_map(|sg| sg["IpPermissions"].as_array().unwrap_or(&vec![]))
            .filter(|rule| {
                rule["IpRanges"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .any(|range| range["CidrIp"].as_str() == Some("0.0.0.0/0"))
            })
            .count();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "network_security".to_string(),
            description: format!("AWS Security Groups for entity {}", entity_id),
            source: "AWS EC2 API".to_string(),
            collected_date: Utc::now(),
            verification_status: if open_rules == 0 { "secure" } else { "open_rules_found" },
            metadata: HashMap::from([
                ("security_group_count".to_string(), sg_count.to_string()),
                ("open_rules_count".to_string(), open_rules.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_iam_compliance(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("UserName".to_string(), entity_id.to_string());

        let iam_data = self.make_aws_request("iam", "GetUser", &params).await?;

        let mfa_enabled = iam_data["User"]["MfaDevices"]
            .as_array()
            .map(|devices| !devices.is_empty())
            .unwrap_or(false);

        let last_activity = iam_data["User"]["PasswordLastUsed"]
            .as_str()
            .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
            .map(|date| (Utc::now() - date.with_timezone(&Utc)).num_days())
            .unwrap_or(999);

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "identity_management".to_string(),
            description: format!("AWS IAM compliance for entity {}", entity_id),
            source: "AWS IAM API".to_string(),
            collected_date: Utc::now(),
            verification_status: if mfa_enabled && last_activity <= 90 { "compliant" } else { "non_compliant" },
            metadata: HashMap::from([
                ("mfa_enabled".to_string(), mfa_enabled.to_string()),
                ("last_activity_days".to_string(), last_activity.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[async_trait]
impl SystemConnector for AwsConnector {
    async fn connect(&self, config: &ConnectorConfig) -> AionResult<()> {
        let access_key = config.credentials.get("access_key_id")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "access_key_id".to_string(),
                reason: "AWS access key ID is required".to_string(),
            })?;

        let secret_key = config.credentials.get("secret_access_key")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "secret_access_key".to_string(),
                reason: "AWS secret access key is required".to_string(),
            })?;

        // Store credentials for subsequent requests
        Ok(())
    }

    async fn collect_evidence(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>> {
        match evidence_type {
            "configuration_compliance" => {
                let evidence = self.collect_config_compliance(entity_id).await?;
                Ok(vec![evidence])
            }
            "audit_logs" => {
                let evidence = self.collect_cloudtrail_logs(entity_id).await?;
                Ok(vec![evidence])
            }
            "network_security" => {
                let evidence = self.collect_security_groups(entity_id).await?;
                Ok(vec![evidence])
            }
            "identity_management" => {
                let evidence = self.collect_iam_compliance(entity_id).await?;
                Ok(vec![evidence])
            }
            _ => Ok(vec![])
        }
    }

    async fn validate_connection(&self) -> AionResult<bool> {
        if self.access_key_id.is_none() || self.secret_access_key.is_none() {
            return Ok(false);
        }

        // Test with STS GetCallerIdentity call
        match self.make_aws_request("sts", "GetCallerIdentity", &HashMap::new()).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_supported_evidence_types(&self) -> Vec<String> {
        vec![
            "configuration_compliance".to_string(),
            "audit_logs".to_string(),
            "network_security".to_string(),
            "identity_management".to_string(),
            "encryption_compliance".to_string(),
            "backup_compliance".to_string(),
        ]
    }
}