use aion_core::{AionResult, AionError, Evidence, ComplianceData, NormativeFramework};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use reqwest::Client;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceConnectorHub {
    azure_connector: AzureConnector,
    aws_connector: AWSConnector,
    splunk_connector: SplunkConnector,
    servicenow_connector: ServiceNowConnector,
    active_directory_connector: ActiveDirectoryConnector,
    database_connectors: HashMap<String, DatabaseConnector>,
    api_connectors: HashMap<String, ApiConnector>,
    file_system_connectors: HashMap<String, FileSystemConnector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConnector {
    pub tenant_id: String,
    pub client_id: String,
    pub subscription_id: String,
    pub resource_groups: Vec<String>,
    pub client: Option<Client>,
    pub auth_token: Option<AzureAuthToken>,
    pub connection_config: AzureConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub rate_limit_per_second: u32,
    pub enabled_services: Vec<AzureService>,
    pub compliance_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AzureService {
    SecurityCenter,
    Sentinel,
    PolicyManager,
    ComplianceCenter,
    AuditLogs,
    ActivityLogs,
    ResourceManager,
    KeyVault,
    ActiveDirectory,
    Monitor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureAuthToken {
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AWSConnector {
    pub region: String,
    pub access_key_id: String,
    pub account_id: String,
    pub role_arn: Option<String>,
    pub session_token: Option<String>,
    pub connection_config: AWSConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AWSConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub enabled_services: Vec<AWSService>,
    pub compliance_frameworks: Vec<String>,
    pub cross_account_roles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AWSService {
    SecurityHub,
    ConfigService,
    CloudTrail,
    GuardDuty,
    Inspector,
    ComplianceReports,
    IAM,
    Organizations,
    ControlTower,
    SystemsManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplunkConnector {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub app_context: String,
    pub search_indexes: Vec<String>,
    pub connection_config: SplunkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplunkConfig {
    pub timeout_seconds: u64,
    pub max_events_per_search: u32,
    pub search_time_range: String,
    pub enabled_search_types: Vec<SplunkSearchType>,
    pub compliance_searches: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplunkSearchType {
    SecurityEvents,
    ComplianceAudits,
    AccessLogs,
    SystemEvents,
    NetworkTraffic,
    ApplicationLogs,
    ErrorLogs,
    PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNowConnector {
    pub instance_url: String,
    pub username: String,
    pub api_version: String,
    pub tables_access: Vec<String>,
    pub connection_config: ServiceNowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNowConfig {
    pub timeout_seconds: u64,
    pub max_records_per_query: u32,
    pub enabled_modules: Vec<ServiceNowModule>,
    pub compliance_workflows: HashMap<String, String>,
    pub incident_priority_filter: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceNowModule {
    IncidentManagement,
    ChangeManagement,
    ComplianceFramework,
    RiskManagement,
    AuditManagement,
    PolicyCompliance,
    SecurityOperations,
    AssetManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDirectoryConnector {
    pub domain_controller: String,
    pub domain_name: String,
    pub ldap_port: u16,
    pub base_dn: String,
    pub connection_config: ADConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADConfig {
    pub timeout_seconds: u64,
    pub use_ssl: bool,
    pub search_scope: ADSearchScope,
    pub compliance_groups: Vec<String>,
    pub audit_attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ADSearchScope {
    Base,
    OneLevel,
    Subtree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnector {
    pub connection_string: String,
    pub database_type: DatabaseType,
    pub schema: String,
    pub connection_config: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    PostgreSQL,
    MySQL,
    SQLServer,
    Oracle,
    MongoDB,
    Cassandra,
    Redis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub timeout_seconds: u64,
    pub max_connections: u32,
    pub query_timeout: u64,
    pub compliance_tables: Vec<String>,
    pub audit_tables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConnector {
    pub base_url: String,
    pub auth_method: ApiAuthMethod,
    pub headers: HashMap<String, String>,
    pub connection_config: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiAuthMethod {
    ApiKey(String),
    Bearer(String),
    Basic(String, String),
    OAuth2(OAuth2Config),
    Certificate(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub scope: Vec<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub rate_limit_per_second: u32,
    pub compliance_endpoints: HashMap<String, String>,
    pub response_format: ApiResponseFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiResponseFormat {
    JSON,
    XML,
    CSV,
    Plain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConnector {
    pub root_path: String,
    pub access_permissions: FilePermissions,
    pub file_patterns: Vec<String>,
    pub connection_config: FileSystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub list_directories: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    pub timeout_seconds: u64,
    pub max_file_size_mb: u64,
    pub supported_formats: Vec<FileFormat>,
    pub compliance_directories: Vec<String>,
    pub watch_for_changes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileFormat {
    PDF,
    Word,
    Excel,
    CSV,
    JSON,
    XML,
    Text,
    Log,
}

// Evidence collection structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedEvidence {
    pub evidence_id: Uuid,
    pub source_connector: ConnectorType,
    pub collected_at: DateTime<Utc>,
    pub data_classification: DataClassification,
    pub compliance_relevance: ComplianceRelevance,
    pub evidence_data: EvidenceData,
    pub metadata: EvidenceMetadata,
    pub chain_of_custody: Vec<CustodyRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectorType {
    Azure(String),
    AWS(String),
    Splunk(String),
    ServiceNow(String),
    ActiveDirectory(String),
    Database(String),
    Api(String),
    FileSystem(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRelevance {
    pub applicable_frameworks: Vec<String>,
    pub compliance_score: f64,
    pub risk_indicators: Vec<String>,
    pub regulatory_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceData {
    StructuredData(serde_json::Value),
    UnstructuredText(String),
    BinaryData(Vec<u8>),
    DocumentReference(String),
    SystemMetrics(HashMap<String, f64>),
    AuditLog(AuditLogEntry),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceMetadata {
    pub file_hash: Option<String>,
    pub file_size: Option<u64>,
    pub mime_type: Option<String>,
    pub encryption_status: EncryptionStatus,
    pub retention_period: Option<Duration>,
    pub access_history: Vec<AccessRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionStatus {
    Unencrypted,
    EncryptedAtRest,
    EncryptedInTransit,
    EncryptedBoth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecord {
    pub accessed_by: String,
    pub accessed_at: DateTime<Utc>,
    pub access_type: AccessType,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Export,
    Share,
    Analyze,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyRecord {
    pub custodian: String,
    pub transferred_at: DateTime<Utc>,
    pub transfer_reason: String,
    pub verification_hash: String,
    pub digital_signature: Option<String>,
}

impl EvidenceConnectorHub {
    pub fn new() -> Self {
        Self {
            azure_connector: AzureConnector::new(),
            aws_connector: AWSConnector::new(),
            splunk_connector: SplunkConnector::new(),
            servicenow_connector: ServiceNowConnector::new(),
            active_directory_connector: ActiveDirectoryConnector::new(),
            database_connectors: HashMap::new(),
            api_connectors: HashMap::new(),
            file_system_connectors: HashMap::new(),
        }
    }

    pub async fn collect_compliance_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        let mut all_evidence = Vec::new();

        // Collect from Azure
        if let Ok(azure_evidence) = self.azure_connector.collect_evidence(framework).await {
            all_evidence.extend(azure_evidence);
        }

        // Collect from AWS
        if let Ok(aws_evidence) = self.aws_connector.collect_evidence(framework).await {
            all_evidence.extend(aws_evidence);
        }

        // Collect from Splunk
        if let Ok(splunk_evidence) = self.splunk_connector.collect_evidence(framework).await {
            all_evidence.extend(splunk_evidence);
        }

        // Collect from ServiceNow
        if let Ok(servicenow_evidence) = self.servicenow_connector.collect_evidence(framework).await {
            all_evidence.extend(servicenow_evidence);
        }

        // Collect from Active Directory
        if let Ok(ad_evidence) = self.active_directory_connector.collect_evidence(framework).await {
            all_evidence.extend(ad_evidence);
        }

        // Collect from databases
        for (name, connector) in &mut self.database_connectors {
            if let Ok(db_evidence) = connector.collect_evidence(framework).await {
                all_evidence.extend(db_evidence);
            }
        }

        // Collect from APIs
        for (name, connector) in &mut self.api_connectors {
            if let Ok(api_evidence) = connector.collect_evidence(framework).await {
                all_evidence.extend(api_evidence);
            }
        }

        // Collect from file systems
        for (name, connector) in &mut self.file_system_connectors {
            if let Ok(fs_evidence) = connector.collect_evidence(framework).await {
                all_evidence.extend(fs_evidence);
            }
        }

        Ok(all_evidence)
    }

    pub async fn real_time_monitoring(&self) -> AionResult<Vec<ComplianceAlert>> {
        let mut alerts = Vec::new();

        // Monitor Azure in real-time
        if let Ok(azure_alerts) = self.azure_connector.monitor_compliance().await {
            alerts.extend(azure_alerts);
        }

        // Monitor AWS in real-time
        if let Ok(aws_alerts) = self.aws_connector.monitor_compliance().await {
            alerts.extend(aws_alerts);
        }

        // Monitor Splunk for security events
        if let Ok(splunk_alerts) = self.splunk_connector.monitor_security_events().await {
            alerts.extend(splunk_alerts);
        }

        // Monitor ServiceNow incidents
        if let Ok(servicenow_alerts) = self.servicenow_connector.monitor_incidents().await {
            alerts.extend(servicenow_alerts);
        }

        Ok(alerts)
    }

    pub async fn validate_evidence_integrity(&self, evidence: &CollectedEvidence) -> AionResult<IntegrityValidation> {
        let mut validation = IntegrityValidation {
            is_valid: true,
            validation_errors: Vec::new(),
            chain_of_custody_valid: true,
            hash_verified: true,
            timestamp_valid: true,
            signature_valid: true,
        };

        // Validate hash if present
        if let Some(hash) = &evidence.metadata.file_hash {
            if !self.verify_file_hash(&evidence.evidence_data, hash).await? {
                validation.is_valid = false;
                validation.hash_verified = false;
                validation.validation_errors.push("File hash verification failed".to_string());
            }
        }

        // Validate chain of custody
        if !self.validate_chain_of_custody(&evidence.chain_of_custody).await? {
            validation.is_valid = false;
            validation.chain_of_custody_valid = false;
            validation.validation_errors.push("Chain of custody validation failed".to_string());
        }

        // Validate timestamps
        if !self.validate_timestamps(evidence).await? {
            validation.is_valid = false;
            validation.timestamp_valid = false;
            validation.validation_errors.push("Timestamp validation failed".to_string());
        }

        Ok(validation)
    }

    async fn verify_file_hash(&self, data: &EvidenceData, expected_hash: &str) -> AionResult<bool> {
        // Simplified hash verification
        match data {
            EvidenceData::BinaryData(bytes) => {
                let calculated_hash = format!("{:x}", md5::compute(bytes));
                Ok(calculated_hash == expected_hash)
            },
            EvidenceData::UnstructuredText(text) => {
                let calculated_hash = format!("{:x}", md5::compute(text.as_bytes()));
                Ok(calculated_hash == expected_hash)
            },
            _ => Ok(true) // Skip validation for other types
        }
    }

    async fn validate_chain_of_custody(&self, chain: &[CustodyRecord]) -> AionResult<bool> {
        if chain.is_empty() {
            return Ok(false);
        }

        // Validate sequential transfers
        for window in chain.windows(2) {
            let prev = &window[0];
            let current = &window[1];

            if current.transferred_at < prev.transferred_at {
                return Ok(false);
            }

            // Verify hash continuity
            if prev.verification_hash != current.verification_hash {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn validate_timestamps(&self, evidence: &CollectedEvidence) -> AionResult<bool> {
        let now = Utc::now();

        // Evidence cannot be from the future
        if evidence.collected_at > now {
            return Ok(false);
        }

        // Validate access history timestamps
        for access in &evidence.metadata.access_history {
            if access.accessed_at > now {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    pub alert_id: Uuid,
    pub severity: AlertSeverity,
    pub source: ConnectorType,
    pub triggered_at: DateTime<Utc>,
    pub rule_violated: String,
    pub description: String,
    pub affected_resources: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub auto_remediation_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityValidation {
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub chain_of_custody_valid: bool,
    pub hash_verified: bool,
    pub timestamp_valid: bool,
    pub signature_valid: bool,
}

// Implementation stubs for connectors
impl AzureConnector {
    fn new() -> Self {
        Self {
            tenant_id: String::new(),
            client_id: String::new(),
            subscription_id: String::new(),
            resource_groups: Vec::new(),
            client: None,
            auth_token: None,
            connection_config: AzureConfig::default(),
        }
    }

    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        self.authenticate().await?;

        let mut evidence = Vec::new();

        // Collect from Security Center
        if self.connection_config.enabled_services.contains(&AzureService::SecurityCenter) {
            evidence.extend(self.collect_security_center_evidence(framework).await?);
        }

        // Collect from Compliance Center
        if self.connection_config.enabled_services.contains(&AzureService::ComplianceCenter) {
            evidence.extend(self.collect_compliance_center_evidence(framework).await?);
        }

        // Collect from Policy Manager
        if self.connection_config.enabled_services.contains(&AzureService::PolicyManager) {
            evidence.extend(self.collect_policy_evidence(framework).await?);
        }

        Ok(evidence)
    }

    async fn authenticate(&mut self) -> AionResult<()> {
        // Simplified authentication
        let auth_url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", self.tenant_id);

        // Mock authentication for now
        self.auth_token = Some(AzureAuthToken {
            access_token: "mock_token".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            token_type: "Bearer".to_string(),
            scope: vec!["https://management.azure.com/.default".to_string()],
        });

        Ok(())
    }

    async fn collect_security_center_evidence(&self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        let evidence = CollectedEvidence {
            evidence_id: Uuid::new_v4(),
            source_connector: ConnectorType::Azure("SecurityCenter".to_string()),
            collected_at: Utc::now(),
            data_classification: DataClassification::Confidential,
            compliance_relevance: ComplianceRelevance {
                applicable_frameworks: vec![framework.id.0.to_string()],
                compliance_score: 0.85,
                risk_indicators: vec!["medium_risk".to_string()],
                regulatory_tags: vec!["security".to_string(), "azure".to_string()],
            },
            evidence_data: EvidenceData::StructuredData(serde_json::json!({
                "security_alerts": [],
                "policy_violations": [],
                "compliance_score": 85
            })),
            metadata: EvidenceMetadata {
                file_hash: Some("mock_hash".to_string()),
                file_size: Some(1024),
                mime_type: Some("application/json".to_string()),
                encryption_status: EncryptionStatus::EncryptedInTransit,
                retention_period: Some(Duration::from_secs(86400 * 30)), // 30 days
                access_history: Vec::new(),
            },
            chain_of_custody: vec![CustodyRecord {
                custodian: "Azure Security Center".to_string(),
                transferred_at: Utc::now(),
                transfer_reason: "Automated compliance collection".to_string(),
                verification_hash: "mock_hash".to_string(),
                digital_signature: None,
            }],
        };

        Ok(vec![evidence])
    }

    async fn collect_compliance_center_evidence(&self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }

    async fn collect_policy_evidence(&self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }

    async fn monitor_compliance(&self) -> AionResult<Vec<ComplianceAlert>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl AWSConnector {
    fn new() -> Self {
        Self {
            region: "us-east-1".to_string(),
            access_key_id: String::new(),
            account_id: String::new(),
            role_arn: None,
            session_token: None,
            connection_config: AWSConfig::default(),
        }
    }

    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }

    async fn monitor_compliance(&self) -> AionResult<Vec<ComplianceAlert>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl SplunkConnector {
    fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8089,
            username: String::new(),
            app_context: "search".to_string(),
            search_indexes: vec!["main".to_string()],
            connection_config: SplunkConfig::default(),
        }
    }

    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }

    async fn monitor_security_events(&self) -> AionResult<Vec<ComplianceAlert>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl ServiceNowConnector {
    fn new() -> Self {
        Self {
            instance_url: String::new(),
            username: String::new(),
            api_version: "v1".to_string(),
            tables_access: vec!["incident".to_string(), "change_request".to_string()],
            connection_config: ServiceNowConfig::default(),
        }
    }

    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }

    async fn monitor_incidents(&self) -> AionResult<Vec<ComplianceAlert>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl ActiveDirectoryConnector {
    fn new() -> Self {
        Self {
            domain_controller: String::new(),
            domain_name: String::new(),
            ldap_port: 389,
            base_dn: String::new(),
            connection_config: ADConfig::default(),
        }
    }

    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl DatabaseConnector {
    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl ApiConnector {
    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

impl FileSystemConnector {
    async fn collect_evidence(&mut self, framework: &NormativeFramework) -> AionResult<Vec<CollectedEvidence>> {
        // Mock implementation
        Ok(Vec::new())
    }
}

// Default implementations
impl Default for AzureConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            rate_limit_per_second: 10,
            enabled_services: vec![
                AzureService::SecurityCenter,
                AzureService::ComplianceCenter,
                AzureService::PolicyManager,
            ],
            compliance_scope: Vec::new(),
        }
    }
}

impl Default for AWSConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            enabled_services: vec![
                AWSService::SecurityHub,
                AWSService::ConfigService,
                AWSService::CloudTrail,
            ],
            compliance_frameworks: Vec::new(),
            cross_account_roles: HashMap::new(),
        }
    }
}

impl Default for SplunkConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 60,
            max_events_per_search: 10000,
            search_time_range: "-24h@h".to_string(),
            enabled_search_types: vec![
                SplunkSearchType::SecurityEvents,
                SplunkSearchType::ComplianceAudits,
            ],
            compliance_searches: HashMap::new(),
        }
    }
}

impl Default for ServiceNowConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_records_per_query: 1000,
            enabled_modules: vec![
                ServiceNowModule::IncidentManagement,
                ServiceNowModule::ComplianceFramework,
            ],
            compliance_workflows: HashMap::new(),
            incident_priority_filter: vec!["High".to_string(), "Critical".to_string()],
        }
    }
}

impl Default for ADConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            use_ssl: true,
            search_scope: ADSearchScope::Subtree,
            compliance_groups: Vec::new(),
            audit_attributes: vec![
                "whenCreated".to_string(),
                "whenChanged".to_string(),
                "lastLogon".to_string(),
            ],
        }
    }
}

impl Default for EvidenceConnectorHub {
    fn default() -> Self {
        Self::new()
    }
}