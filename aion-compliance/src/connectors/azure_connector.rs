use super::{SystemConnector, ConnectorConfig};
use aion_core::{AionResult, AionError, Evidence};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

pub struct AzureConnector {
    client: Client,
    tenant_id: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
}

impl AzureConnector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            tenant_id: None,
            client_id: None,
            client_secret: None,
            access_token: None,
        }
    }

    async fn authenticate(&mut self, config: &ConnectorConfig) -> AionResult<()> {
        let tenant_id = config.credentials.get("tenant_id")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "tenant_id".to_string(),
                reason: "Azure tenant ID is required".to_string(),
            })?;

        let client_id = config.credentials.get("client_id")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "client_id".to_string(),
                reason: "Azure client ID is required".to_string(),
            })?;

        let client_secret = config.credentials.get("client_secret")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "client_secret".to_string(),
                reason: "Azure client secret is required".to_string(),
            })?;

        let auth_url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant_id);

        let params = [
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("scope", "https://graph.microsoft.com/.default"),
            ("grant_type", "client_credentials"),
        ];

        let response = self.client
            .post(&auth_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "azure_auth".to_string(),
                reason: e.to_string(),
            })?;

        let auth_data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "azure_auth_response".to_string(),
                reason: e.to_string(),
            })?;

        self.access_token = auth_data["access_token"].as_str().map(|s| s.to_string());
        self.tenant_id = Some(tenant_id.clone());
        self.client_id = Some(client_id.clone());
        self.client_secret = Some(client_secret.clone());

        Ok(())
    }

    async fn query_graph_api(&self, endpoint: &str) -> AionResult<Value> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "azure".to_string(),
                reason: "No access token available".to_string(),
            })?;

        let url = format!("https://graph.microsoft.com/v1.0/{}", endpoint);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "graph_api_query".to_string(),
                reason: e.to_string(),
            })?;

        let data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "graph_api_response".to_string(),
                reason: e.to_string(),
            })?;

        Ok(data)
    }

    async fn collect_security_compliance_data(&self, entity_id: &str) -> AionResult<Evidence> {
        let endpoint = "security/secureScores";
        let security_data = self.query_graph_api(endpoint).await?;

        let current_score = security_data["value"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|obj| obj["currentScore"].as_f64())
            .unwrap_or(0.0);

        let max_score = security_data["value"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|obj| obj["maxScore"].as_f64())
            .unwrap_or(100.0);

        let score_percentage = (current_score / max_score * 100.0) as u32;

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "security_compliance".to_string(),
            description: format!("Azure Security Score for entity {}", entity_id),
            source: "Microsoft Graph Security API".to_string(),
            collected_date: Utc::now(),
            verification_status: if score_percentage >= 80 { "compliant" } else { "non_compliant" },
            metadata: HashMap::from([
                ("current_score".to_string(), current_score.to_string()),
                ("max_score".to_string(), max_score.to_string()),
                ("score_percentage".to_string(), score_percentage.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_audit_logs(&self, entity_id: &str) -> AionResult<Evidence> {
        let endpoint = "auditLogs/directoryAudits?$top=100";
        let audit_data = self.query_graph_api(endpoint).await?;

        let log_count = audit_data["value"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let recent_activities = audit_data["value"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .take(10)
            .filter_map(|item| item["activityDisplayName"].as_str())
            .collect::<Vec<_>>();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "audit_logs".to_string(),
            description: format!("Azure AD audit logs for entity {}", entity_id),
            source: "Microsoft Graph Audit API".to_string(),
            collected_date: Utc::now(),
            verification_status: if log_count > 0 { "available" } else { "no_data" },
            metadata: HashMap::from([
                ("log_count".to_string(), log_count.to_string()),
                ("recent_activities".to_string(), recent_activities.join(", ")),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_conditional_access_policies(&self, entity_id: &str) -> AionResult<Evidence> {
        let endpoint = "identity/conditionalAccess/policies";
        let policies_data = self.query_graph_api(endpoint).await?;

        let enabled_policies = policies_data["value"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|policy| policy["state"].as_str() == Some("enabled"))
            .count();

        let total_policies = policies_data["value"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "access_controls".to_string(),
            description: format!("Azure Conditional Access policies for entity {}", entity_id),
            source: "Microsoft Graph Conditional Access API".to_string(),
            collected_date: Utc::now(),
            verification_status: if enabled_policies > 0 { "configured" } else { "not_configured" },
            metadata: HashMap::from([
                ("enabled_policies".to_string(), enabled_policies.to_string()),
                ("total_policies".to_string(), total_policies.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }
}

#[async_trait]
impl SystemConnector for AzureConnector {
    async fn connect(&self, config: &ConnectorConfig) -> AionResult<()> {
        // Create mutable copy for authentication
        let mut connector = AzureConnector::new();
        connector.authenticate(config).await?;
        Ok(())
    }

    async fn collect_evidence(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>> {
        match evidence_type {
            "security_compliance" => {
                let evidence = self.collect_security_compliance_data(entity_id).await?;
                Ok(vec![evidence])
            }
            "audit_logs" => {
                let evidence = self.collect_audit_logs(entity_id).await?;
                Ok(vec![evidence])
            }
            "access_controls" => {
                let evidence = self.collect_conditional_access_policies(entity_id).await?;
                Ok(vec![evidence])
            }
            _ => Ok(vec![])
        }
    }

    async fn validate_connection(&self) -> AionResult<bool> {
        if self.access_token.is_none() {
            return Ok(false);
        }

        // Test connection with a simple Graph API call
        match self.query_graph_api("organization").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_supported_evidence_types(&self) -> Vec<String> {
        vec![
            "security_compliance".to_string(),
            "audit_logs".to_string(),
            "access_controls".to_string(),
            "conditional_access".to_string(),
            "identity_protection".to_string(),
        ]
    }
}