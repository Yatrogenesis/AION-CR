pub mod azure_connector;
pub mod aws_connector;
pub mod splunk_connector;
pub mod servicenow_connector;
pub mod sharepoint_connector;
pub mod confluence_connector;
pub mod jira_connector;
pub mod okta_connector;

pub use azure_connector::AzureConnector;
pub use aws_connector::AwsConnector;
pub use splunk_connector::SplunkConnector;
pub use servicenow_connector::ServiceNowConnector;
pub use sharepoint_connector::SharePointConnector;
pub use confluence_connector::ConfluenceConnector;
pub use jira_connector::JiraConnector;
pub use okta_connector::OktaConnector;

use aion_core::{AionResult, Evidence};
use std::collections::HashMap;
use async_trait::async_trait;

#[async_trait]
pub trait SystemConnector: Send + Sync {
    async fn connect(&self, config: &ConnectorConfig) -> AionResult<()>;
    async fn collect_evidence(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>>;
    async fn validate_connection(&self) -> AionResult<bool>;
    fn get_supported_evidence_types(&self) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub struct ConnectorConfig {
    pub endpoint: String,
    pub credentials: HashMap<String, String>,
    pub timeout_ms: u64,
    pub retry_count: u32,
}

pub struct EvidenceCollectionManager {
    connectors: HashMap<String, Box<dyn SystemConnector>>,
}

impl EvidenceCollectionManager {
    pub fn new() -> Self {
        Self {
            connectors: HashMap::new(),
        }
    }

    pub fn register_connector(&mut self, name: String, connector: Box<dyn SystemConnector>) {
        self.connectors.insert(name, connector);
    }

    pub async fn collect_evidence_by_type(&self, evidence_type: &str, entity_id: &str) -> AionResult<Vec<Evidence>> {
        let mut all_evidence = Vec::new();

        for (name, connector) in &self.connectors {
            if connector.get_supported_evidence_types().contains(&evidence_type.to_string()) {
                match connector.collect_evidence(evidence_type, entity_id).await {
                    Ok(mut evidence) => {
                        for item in &mut evidence {
                            item.metadata.insert("connector".to_string(), name.clone());
                        }
                        all_evidence.extend(evidence);
                    }
                    Err(e) => {
                        tracing::error!("Failed to collect evidence from {}: {}", name, e);
                    }
                }
            }
        }

        Ok(all_evidence)
    }
}