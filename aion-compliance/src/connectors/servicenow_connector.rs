use super::{SystemConnector, ConnectorConfig};
use aion_core::{AionResult, AionError, Evidence};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

pub struct ServiceNowConnector {
    client: Client,
    base_url: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl ServiceNowConnector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: None,
            username: None,
            password: None,
        }
    }

    async fn make_request(&self, endpoint: &str, params: Option<&HashMap<String, String>>) -> AionResult<Value> {
        let base_url = self.base_url.as_ref()
            .ok_or_else(|| AionError::ConfigurationError {
                field: "base_url".to_string(),
                reason: "ServiceNow instance URL not configured".to_string(),
            })?;

        let username = self.username.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "servicenow".to_string(),
                reason: "Username not configured".to_string(),
            })?;

        let password = self.password.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "servicenow".to_string(),
                reason: "Password not configured".to_string(),
            })?;

        let auth_header = STANDARD.encode(format!("{}:{}", username, password));
        let url = format!("{}/api/now/table/{}", base_url, endpoint);

        let mut request = self.client
            .get(&url)
            .header("Authorization", format!("Basic {}", auth_header))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        if let Some(query_params) = params {
            request = request.query(query_params);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "servicenow_request".to_string(),
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(AionError::NetworkError {
                operation: "servicenow_request".to_string(),
                reason: format!("HTTP {}: {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown")),
            });
        }

        let data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "servicenow_response".to_string(),
                reason: e.to_string(),
            })?;

        Ok(data)
    }

    async fn collect_incidents(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("sysparm_query".to_string(), format!("caller_id.name={}^ORcmdb_ci.name={}", entity_id, entity_id));
        params.insert("sysparm_limit".to_string(), "1000".to_string());

        let incidents_data = self.make_request("incident", Some(&params)).await?;

        let total_incidents = incidents_data["result"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let open_incidents = incidents_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|incident| {
                let state = incident["state"].as_str().unwrap_or("");
                state == "1" || state == "2" || state == "3" // New, In Progress, On Hold
            })
            .count();

        let critical_incidents = incidents_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|incident| {
                let priority = incident["priority"].as_str().unwrap_or("");
                priority == "1" || priority == "2" // Critical or High
            })
            .count();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "incident_management".to_string(),
            description: format!("ServiceNow incidents for entity {}", entity_id),
            source: "ServiceNow Incident Management".to_string(),
            collected_date: Utc::now(),
            verification_status: if critical_incidents == 0 { "stable" } else { "critical_incidents_open" },
            metadata: HashMap::from([
                ("total_incidents".to_string(), total_incidents.to_string()),
                ("open_incidents".to_string(), open_incidents.to_string()),
                ("critical_incidents".to_string(), critical_incidents.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_change_requests(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("sysparm_query".to_string(), format!("cmdb_ci.name={}^ORrequested_by.name={}", entity_id, entity_id));
        params.insert("sysparm_limit".to_string(), "500".to_string());

        let changes_data = self.make_request("change_request", Some(&params)).await?;

        let total_changes = changes_data["result"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let emergency_changes = changes_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|change| {
                let change_type = change["type"].as_str().unwrap_or("");
                change_type == "emergency"
            })
            .count();

        let successful_changes = changes_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|change| {
                let state = change["state"].as_str().unwrap_or("");
                state == "3" // Implemented
            })
            .count();

        let success_rate = if total_changes > 0 {
            (successful_changes as f64 / total_changes as f64 * 100.0) as u32
        } else {
            100
        };

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "change_management".to_string(),
            description: format!("ServiceNow change requests for entity {}", entity_id),
            source: "ServiceNow Change Management".to_string(),
            collected_date: Utc::now(),
            verification_status: if success_rate >= 95 && emergency_changes == 0 { "controlled" } else { "high_risk" },
            metadata: HashMap::from([
                ("total_changes".to_string(), total_changes.to_string()),
                ("emergency_changes".to_string(), emergency_changes.to_string()),
                ("success_rate".to_string(), success_rate.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_vulnerability_assessments(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("sysparm_query".to_string(), format!("cmdb_ci.name={}", entity_id));
        params.insert("sysparm_limit".to_string(), "1000".to_string());

        let vulns_data = self.make_request("sn_vul_vulnerable_item", Some(&params)).await?;

        let total_vulns = vulns_data["result"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let critical_vulns = vulns_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|vuln| {
                let severity = vuln["severity"].as_str().unwrap_or("");
                severity == "1" || severity == "2" // Critical or High
            })
            .count();

        let remediated_vulns = vulns_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|vuln| {
                let state = vuln["state"].as_str().unwrap_or("");
                state == "4" // Remediated
            })
            .count();

        let remediation_rate = if total_vulns > 0 {
            (remediated_vulns as f64 / total_vulns as f64 * 100.0) as u32
        } else {
            100
        };

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "vulnerability_management".to_string(),
            description: format!("ServiceNow vulnerability assessments for entity {}", entity_id),
            source: "ServiceNow Vulnerability Response".to_string(),
            collected_date: Utc::now(),
            verification_status: if critical_vulns == 0 && remediation_rate >= 90 { "secure" } else { "vulnerabilities_found" },
            metadata: HashMap::from([
                ("total_vulnerabilities".to_string(), total_vulns.to_string()),
                ("critical_vulnerabilities".to_string(), critical_vulns.to_string()),
                ("remediation_rate".to_string(), remediation_rate.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_security_incidents(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("sysparm_query".to_string(), format!("u_affected_ci.name={}^category=security", entity_id));
        params.insert("sysparm_limit".to_string(), "500".to_string());

        let sec_incidents_data = self.make_request("sn_si_incident", Some(&params)).await?;

        let total_sec_incidents = sec_incidents_data["result"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let open_sec_incidents = sec_incidents_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|incident| {
                let state = incident["state"].as_str().unwrap_or("");
                state != "6" && state != "7" // Not Resolved or Closed
            })
            .count();

        let high_impact_incidents = sec_incidents_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|incident| {
                let impact = incident["impact"].as_str().unwrap_or("");
                impact == "1" || impact == "2" // High or Medium impact
            })
            .count();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "security_incidents".to_string(),
            description: format!("ServiceNow security incidents for entity {}", entity_id),
            source: "ServiceNow Security Incident Response".to_string(),
            collected_date: Utc::now(),
            verification_status: if open_sec_incidents == 0 { "secure" } else { "security_incidents_open" },
            metadata: HashMap::from([
                ("total_security_incidents".to_string(), total_sec_incidents.to_string()),
                ("open_security_incidents".to_string(), open_sec_incidents.to_string()),
                ("high_impact_incidents".to_string(), high_impact_incidents.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_compliance_assessments(&self, entity_id: &str) -> AionResult<Evidence> {
        let mut params = HashMap::new();
        params.insert("sysparm_query".to_string(), format!("u_business_service.name={}", entity_id));
        params.insert("sysparm_limit".to_string(), "100".to_string());

        let compliance_data = self.make_request("sn_compliance_assessment", Some(&params)).await?;

        let total_assessments = compliance_data["result"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let passed_assessments = compliance_data["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|assessment| {
                let result = assessment["u_result"].as_str().unwrap_or("");
                result == "pass" || result == "compliant"
            })
            .count();

        let compliance_score = if total_assessments > 0 {
            (passed_assessments as f64 / total_assessments as f64 * 100.0) as u32
        } else {
            0
        };

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "compliance_assessments".to_string(),
            description: format!("ServiceNow compliance assessments for entity {}", entity_id),
            source: "ServiceNow GRC".to_string(),
            collected_date: Utc::now(),
            verification_status: if compliance_score >= 90 { "compliant" } else { "non_compliant" },
            metadata: HashMap::from([
                ("total_assessments".to_string(), total_assessments.to_string()),
                ("passed_assessments".to_string(), passed_assessments.to_string()),
                ("compliance_score".to_string(), compliance_score.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }
}

#[async_trait]
impl SystemConnector for ServiceNowConnector {
    async fn connect(&self, config: &ConnectorConfig) -> AionResult<()> {
        let username = config.credentials.get("username")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "username".to_string(),
                reason: "ServiceNow username is required".to_string(),
            })?;

        let password = config.credentials.get("password")
            .ok_or_else(|| AionError::ConfigurationError {
                field: "password".to_string(),
                reason: "ServiceNow password is required".to_string(),
            })?;

        // Test connection with a simple API call
        let auth_header = STANDARD.encode(format!("{}:{}", username, password));
        let test_url = format!("{}/api/now/table/sys_user?sysparm_limit=1", config.endpoint);

        let response = self.client
            .get(&test_url)
            .header("Authorization", format!("Basic {}", auth_header))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "servicenow_connection_test".to_string(),
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(AionError::AuthenticationError {
                service: "servicenow".to_string(),
                reason: format!("Authentication failed: HTTP {}", response.status()),
            });
        }

        Ok(())
    }

    async fn collect_evidence(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>> {
        match evidence_type {
            "incident_management" => {
                let evidence = self.collect_incidents(entity_id).await?;
                Ok(vec![evidence])
            }
            "change_management" => {
                let evidence = self.collect_change_requests(entity_id).await?;
                Ok(vec![evidence])
            }
            "vulnerability_management" => {
                let evidence = self.collect_vulnerability_assessments(entity_id).await?;
                Ok(vec![evidence])
            }
            "security_incidents" => {
                let evidence = self.collect_security_incidents(entity_id).await?;
                Ok(vec![evidence])
            }
            "compliance_assessments" => {
                let evidence = self.collect_compliance_assessments(entity_id).await?;
                Ok(vec![evidence])
            }
            _ => Ok(vec![])
        }
    }

    async fn validate_connection(&self) -> AionResult<bool> {
        match self.make_request("sys_user?sysparm_limit=1", None).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_supported_evidence_types(&self) -> Vec<String> {
        vec![
            "incident_management".to_string(),
            "change_management".to_string(),
            "vulnerability_management".to_string(),
            "security_incidents".to_string(),
            "compliance_assessments".to_string(),
            "risk_assessments".to_string(),
        ]
    }
}