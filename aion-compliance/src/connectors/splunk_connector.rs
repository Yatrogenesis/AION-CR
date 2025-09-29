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

pub struct SplunkConnector {
    client: Client,
    base_url: Option<String>,
    session_key: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl SplunkConnector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: None,
            session_key: None,
            username: None,
            password: None,
        }
    }

    async fn authenticate(&mut self, config: &ConnectorConfig) -> AionResult<()> {
        let username = config.credentials.get("username")
            .ok_or_else(|| AionError::AuthenticationError {
                service: "splunk".to_string(),
                reason: "Username is required".to_string(),
            })?;

        let password = config.credentials.get("password")
            .ok_or_else(|| AionError::AuthenticationError {
                service: "splunk".to_string(),
                reason: "Password is required".to_string(),
            })?;

        self.base_url = Some(config.endpoint.clone());
        self.username = Some(username.clone());
        self.password = Some(password.clone());

        let auth_url = format!("{}/services/auth/login", config.endpoint);

        let params = [
            ("username", username.as_str()),
            ("password", password.as_str()),
            ("output_mode", "json"),
        ];

        let response = self.client
            .post(&auth_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "splunk_auth".to_string(),
                reason: e.to_string(),
            })?;

        let auth_data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "splunk_auth_response".to_string(),
                reason: e.to_string(),
            })?;

        self.session_key = auth_data["sessionKey"].as_str().map(|s| s.to_string());

        if self.session_key.is_none() {
            return Err(AionError::AuthenticationError {
                service: "splunk".to_string(),
                reason: "Failed to obtain session key".to_string(),
            });
        }

        Ok(())
    }

    async fn execute_search(&self, query: &str, earliest_time: &str, latest_time: &str) -> AionResult<Value> {
        let base_url = self.base_url.as_ref()
            .ok_or_else(|| AionError::ConfigurationError {
                field: "base_url".to_string(),
                reason: "Splunk base URL not configured".to_string(),
            })?;

        let session_key = self.session_key.as_ref()
            .ok_or_else(|| AionError::AuthenticationError {
                service: "splunk".to_string(),
                reason: "Not authenticated to Splunk".to_string(),
            })?;

        // Create search job
        let search_url = format!("{}/services/search/jobs", base_url);

        let params = [
            ("search", query),
            ("earliest_time", earliest_time),
            ("latest_time", latest_time),
            ("output_mode", "json"),
        ];

        let response = self.client
            .post(&search_url)
            .header("Authorization", format!("Splunk {}", session_key))
            .form(&params)
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "splunk_search".to_string(),
                reason: e.to_string(),
            })?;

        let job_data: Value = response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "splunk_job_response".to_string(),
                reason: e.to_string(),
            })?;

        let job_id = job_data["sid"].as_str()
            .ok_or_else(|| AionError::ParsingError {
                source: "splunk_job_id".to_string(),
                reason: "No job ID returned".to_string(),
            })?;

        // Wait for job completion and get results
        let results_url = format!("{}/services/search/jobs/{}/results", base_url, job_id);

        // Poll for completion (simplified - in production would implement proper polling)
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let results_response = self.client
            .get(&results_url)
            .header("Authorization", format!("Splunk {}", session_key))
            .query(&[("output_mode", "json")])
            .send()
            .await
            .map_err(|e| AionError::NetworkError {
                operation: "splunk_results".to_string(),
                reason: e.to_string(),
            })?;

        let results: Value = results_response.json().await
            .map_err(|e| AionError::ParsingError {
                source: "splunk_results".to_string(),
                reason: e.to_string(),
            })?;

        Ok(results)
    }

    async fn collect_security_events(&self, entity_id: &str) -> AionResult<Evidence> {
        let query = format!(
            "search index=security host=\"{}\" earliest=-24h@h latest=now | stats count by sourcetype | head 100",
            entity_id
        );

        let results = self.execute_search(&query, "-24h@h", "now").await?;

        let event_count = results["results"]
            .as_array()
            .map(|arr| arr.len())
            .unwrap_or(0);

        let critical_events = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|event| {
                event["sourcetype"].as_str()
                    .map(|st| st.contains("threat") || st.contains("alert"))
                    .unwrap_or(false)
            })
            .count();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "security_events".to_string(),
            description: format!("Security events from Splunk for entity {}", entity_id),
            source: "Splunk Security Index".to_string(),
            collected_date: Utc::now(),
            verification_status: if critical_events == 0 { "clean" } else { "alerts_found" },
            metadata: HashMap::from([
                ("total_events".to_string(), event_count.to_string()),
                ("critical_events".to_string(), critical_events.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
                ("search_query".to_string(), query),
            ]),
        })
    }

    async fn collect_compliance_logs(&self, entity_id: &str) -> AionResult<Evidence> {
        let query = format!(
            "search index=compliance host=\"{}\" earliest=-7d@d latest=now | eval compliance_status=case(like(_raw, \"%PASS%\"), \"PASS\", like(_raw, \"%FAIL%\"), \"FAIL\", 1=1, \"UNKNOWN\") | stats count by compliance_status",
            entity_id
        );

        let results = self.execute_search(&query, "-7d@d", "now").await?;

        let total_checks = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        let failed_checks = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|result| result["compliance_status"].as_str() == Some("FAIL"))
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        let compliance_rate = if total_checks > 0 {
            ((total_checks - failed_checks) as f64 / total_checks as f64 * 100.0) as u32
        } else {
            0
        };

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "compliance_monitoring".to_string(),
            description: format!("Compliance monitoring logs from Splunk for entity {}", entity_id),
            source: "Splunk Compliance Index".to_string(),
            collected_date: Utc::now(),
            verification_status: if compliance_rate >= 95 { "compliant" } else { "non_compliant" },
            metadata: HashMap::from([
                ("total_checks".to_string(), total_checks.to_string()),
                ("failed_checks".to_string(), failed_checks.to_string()),
                ("compliance_rate".to_string(), compliance_rate.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_access_logs(&self, entity_id: &str) -> AionResult<Evidence> {
        let query = format!(
            "search index=access host=\"{}\" earliest=-24h@h latest=now | eval access_result=case(like(_raw, \"%SUCCESS%\"), \"SUCCESS\", like(_raw, \"%FAILED%\"), \"FAILED\", 1=1, \"UNKNOWN\") | stats count by access_result, user",
            entity_id
        );

        let results = self.execute_search(&query, "-24h@h", "now").await?;

        let total_attempts = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        let failed_attempts = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|result| result["access_result"].as_str() == Some("FAILED"))
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        let unique_users = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|result| result["user"].as_str())
            .collect::<std::collections::HashSet<_>>()
            .len();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "access_logs".to_string(),
            description: format!("Access logs from Splunk for entity {}", entity_id),
            source: "Splunk Access Index".to_string(),
            collected_date: Utc::now(),
            verification_status: if failed_attempts < total_attempts / 10 { "normal" } else { "suspicious" },
            metadata: HashMap::from([
                ("total_attempts".to_string(), total_attempts.to_string()),
                ("failed_attempts".to_string(), failed_attempts.to_string()),
                ("unique_users".to_string(), unique_users.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }

    async fn collect_data_loss_prevention(&self, entity_id: &str) -> AionResult<Evidence> {
        let query = format!(
            "search index=dlp host=\"{}\" earliest=-7d@d latest=now | eval severity=case(like(_raw, \"%HIGH%\"), \"HIGH\", like(_raw, \"%MEDIUM%\"), \"MEDIUM\", like(_raw, \"%LOW%\"), \"LOW\", 1=1, \"UNKNOWN\") | stats count by severity",
            entity_id
        );

        let results = self.execute_search(&query, "-7d@d", "now").await?;

        let high_severity = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter(|result| result["severity"].as_str() == Some("HIGH"))
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        let total_incidents = results["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|result| result["count"].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
            .sum::<i32>();

        Ok(Evidence {
            id: Uuid::new_v4(),
            evidence_type: "data_loss_prevention".to_string(),
            description: format!("DLP incidents from Splunk for entity {}", entity_id),
            source: "Splunk DLP Index".to_string(),
            collected_date: Utc::now(),
            verification_status: if high_severity == 0 { "secure" } else { "incidents_detected" },
            metadata: HashMap::from([
                ("total_incidents".to_string(), total_incidents.to_string()),
                ("high_severity_incidents".to_string(), high_severity.to_string()),
                ("entity_id".to_string(), entity_id.to_string()),
            ]),
        })
    }
}

#[async_trait]
impl SystemConnector for SplunkConnector {
    async fn connect(&self, config: &ConnectorConfig) -> AionResult<()> {
        let mut connector = SplunkConnector::new();
        connector.authenticate(config).await?;
        Ok(())
    }

    async fn collect_evidence(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>> {
        match evidence_type {
            "security_events" => {
                let evidence = self.collect_security_events(entity_id).await?;
                Ok(vec![evidence])
            }
            "compliance_monitoring" => {
                let evidence = self.collect_compliance_logs(entity_id).await?;
                Ok(vec![evidence])
            }
            "access_logs" => {
                let evidence = self.collect_access_logs(entity_id).await?;
                Ok(vec![evidence])
            }
            "data_loss_prevention" => {
                let evidence = self.collect_data_loss_prevention(entity_id).await?;
                Ok(vec![evidence])
            }
            _ => Ok(vec![])
        }
    }

    async fn validate_connection(&self) -> AionResult<bool> {
        if self.session_key.is_none() {
            return Ok(false);
        }

        // Test with a simple search
        match self.execute_search("| stats count", "-1m", "now").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_supported_evidence_types(&self) -> Vec<String> {
        vec![
            "security_events".to_string(),
            "compliance_monitoring".to_string(),
            "access_logs".to_string(),
            "data_loss_prevention".to_string(),
            "network_monitoring".to_string(),
            "threat_intelligence".to_string(),
        ]
    }
}