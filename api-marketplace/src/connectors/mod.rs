/*!
 * API Connectors Module
 *
 * Provides standardized connectors for 500+ regulatory APIs worldwide.
 * Each connector implements the same interface for seamless integration.
 */

pub mod government;
pub mod financial;
pub mod healthcare;
pub mod international;
pub mod industry_standards;
pub mod registry;

// Re-export main types
pub use registry::*;

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Result;

/// Connector configuration for external APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    /// Unique connector identifier
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Connector description
    pub description: String,

    /// Connector category
    pub category: ConnectorCategory,

    /// Base URL for the API
    pub base_url: String,

    /// API version
    pub version: String,

    /// Authentication configuration
    pub authentication: AuthenticationConfig,

    /// Rate limiting configuration
    pub rate_limits: RateLimitConfig,

    /// Supported endpoints
    pub endpoints: Vec<EndpointConfig>,

    /// Data transformation rules
    pub transformation_rules: Vec<TransformationRule>,

    /// Webhook configuration
    pub webhook_config: Option<WebhookConfig>,

    /// Metadata
    pub metadata: ConnectorMetadata,

    /// Health check configuration
    pub health_check: HealthCheckConfig,
}

/// Connector categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectorCategory {
    Government,
    Financial,
    Healthcare,
    International,
    IndustryStandards,
    Enterprise,
    Custom(String),
}

/// Authentication configuration for APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub auth_type: AuthenticationType,
    pub credentials: CredentialConfig,
    pub token_refresh: Option<TokenRefreshConfig>,
    pub headers: HashMap<String, String>,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            auth_type: AuthenticationType::None,
            credentials: CredentialConfig::None,
            token_refresh: None,
            headers: HashMap::new(),
        }
    }
}

/// Authentication types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationType {
    None,
    ApiKey,
    Bearer,
    BasicAuth,
    OAuth2,
    JWT,
    Custom(String),
}

/// Credential configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialConfig {
    None,
    ApiKey { key: String, header: String },
    Bearer { token: String },
    BasicAuth { username: String, password: String },
    OAuth2 { client_id: String, client_secret: String, scope: Option<String> },
    JWT { secret: String, algorithm: String },
    Custom { config: HashMap<String, String> },
}

/// Token refresh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshConfig {
    pub refresh_url: String,
    pub refresh_interval: chrono::Duration,
    pub refresh_before_expiry: chrono::Duration,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: Option<u32>,
    pub requests_per_minute: Option<u32>,
    pub requests_per_hour: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub burst_size: Option<u32>,
    pub retry_after_header: Option<String>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: Some(10),
            requests_per_minute: Some(100),
            requests_per_hour: Some(1000),
            requests_per_day: Some(10000),
            burst_size: Some(50),
            retry_after_header: Some("Retry-After".to_string()),
        }
    }
}

/// API endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub name: String,
    pub path: String,
    pub method: HttpMethod,
    pub description: String,
    pub parameters: Vec<ParameterConfig>,
    pub response_format: ResponseFormat,
    pub cache_ttl: Option<chrono::Duration>,
    pub requires_auth: bool,
}

/// HTTP methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// Parameter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConfig {
    pub name: String,
    pub parameter_type: ParameterType,
    pub data_type: DataType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: String,
    pub validation: Option<ValidationRule>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Query,
    Path,
    Header,
    Body,
    FormData,
}

/// Data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Array(Box<DataType>),
    Object,
    Custom(String),
}

/// Response formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseFormat {
    JSON,
    XML,
    CSV,
    Plain,
    Binary,
    Custom(String),
}

/// Validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Range { min: f64, max: f64 },
    Enum(Vec<String>),
    Custom(String),
}

/// Data transformation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRule {
    pub name: String,
    pub rule_type: TransformationType,
    pub source_field: String,
    pub target_field: String,
    pub transformation: String,
}

/// Transformation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    FieldMapping,
    DataConversion,
    Aggregation,
    Filtering,
    Enrichment,
    Custom,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub supported: bool,
    pub subscription_url: Option<String>,
    pub event_types: Vec<String>,
    pub signature_verification: Option<SignatureConfig>,
    pub retry_policy: RetryPolicy,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            supported: false,
            subscription_url: None,
            event_types: vec![],
            signature_verification: None,
            retry_policy: RetryPolicy::default(),
        }
    }
}

/// Signature configuration for webhook verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureConfig {
    pub algorithm: String,
    pub header_name: String,
    pub secret_key: String,
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: chrono::Duration,
    pub max_delay: chrono::Duration,
    pub exponential_backoff: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: chrono::Duration::seconds(1),
            max_delay: chrono::Duration::seconds(60),
            exponential_backoff: true,
        }
    }
}

/// Connector metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorMetadata {
    pub jurisdiction: Vec<String>,
    pub compliance_frameworks: Vec<String>,
    pub data_categories: Vec<String>,
    pub update_frequency: UpdateFrequency,
    pub data_retention: Option<chrono::Duration>,
    pub sla: ServiceLevelAgreement,
    pub documentation_url: Option<String>,
    pub support_contact: Option<String>,
}

/// Update frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    OnDemand,
    Custom(String),
}

/// Service Level Agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLevelAgreement {
    pub availability: f64, // percentage (e.g., 99.9)
    pub response_time: chrono::Duration,
    pub data_freshness: chrono::Duration,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub interval: chrono::Duration,
    pub timeout: chrono::Duration,
    pub expected_status: u16,
    pub expected_response: Option<String>,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
    pub response_time: chrono::Duration,
    pub timestamp: DateTime<Utc>,
    pub cached: bool,
}

/// API connector trait
#[async_trait]
pub trait ApiConnector: Send + Sync {
    /// Get connector configuration
    fn get_config(&self) -> &ConnectorConfig;

    /// Test connection to the API
    async fn test_connection(&self) -> Result<bool>;

    /// Execute a request to the API
    async fn execute_request(
        &self,
        endpoint: &str,
        parameters: &crate::ApiParameters,
    ) -> Result<ApiResponse>;

    /// Subscribe to real-time updates (if supported)
    async fn subscribe_to_updates(
        &self,
        event_types: Vec<String>,
        callback_url: String,
    ) -> Result<String>;

    /// Unsubscribe from updates
    async fn unsubscribe(&self, subscription_id: &str) -> Result<()>;

    /// Get available endpoints
    fn get_endpoints(&self) -> &[EndpointConfig];

    /// Get health status
    async fn get_health(&self) -> Result<HealthStatus>;

    /// Refresh authentication token if needed
    async fn refresh_auth(&self) -> Result<()>;
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub response_time: chrono::Duration,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// API search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSearchResult {
    pub connector_id: String,
    pub name: String,
    pub description: String,
    pub category: ConnectorCategory,
    pub jurisdiction: Vec<String>,
    pub compliance_frameworks: Vec<String>,
    pub real_time_support: bool,
    pub documentation_url: Option<String>,
    pub health_status: HealthStatus,
}

impl ConnectorConfig {
    /// Create configuration for US Federal Register API
    pub fn us_federal_register() -> Self {
        Self {
            id: "us-federal-register".to_string(),
            name: "US Federal Register".to_string(),
            description: "Official journal of the federal government of the United States".to_string(),
            category: ConnectorCategory::Government,
            base_url: "https://www.federalregister.gov/api/v1".to_string(),
            version: "v1".to_string(),
            authentication: AuthenticationConfig {
                auth_type: AuthenticationType::None,
                credentials: CredentialConfig::None,
                token_refresh: None,
                headers: HashMap::new(),
            },
            rate_limits: RateLimitConfig {
                requests_per_second: Some(5),
                requests_per_minute: Some(100),
                requests_per_hour: Some(1000),
                requests_per_day: Some(10000),
                burst_size: Some(20),
                retry_after_header: Some("Retry-After".to_string()),
            },
            endpoints: vec![
                EndpointConfig {
                    name: "documents".to_string(),
                    path: "/documents.json".to_string(),
                    method: HttpMethod::GET,
                    description: "Search and retrieve Federal Register documents".to_string(),
                    parameters: vec![
                        ParameterConfig {
                            name: "conditions".to_string(),
                            parameter_type: ParameterType::Query,
                            data_type: DataType::String,
                            required: false,
                            default_value: None,
                            description: "Search terms".to_string(),
                            validation: None,
                        },
                        ParameterConfig {
                            name: "publication_date".to_string(),
                            parameter_type: ParameterType::Query,
                            data_type: DataType::String,
                            required: false,
                            default_value: None,
                            description: "Publication date range".to_string(),
                            validation: None,
                        },
                    ],
                    response_format: ResponseFormat::JSON,
                    cache_ttl: Some(chrono::Duration::hours(1)),
                    requires_auth: false,
                },
            ],
            transformation_rules: vec![],
            webhook_config: None,
            metadata: ConnectorMetadata {
                jurisdiction: vec!["US".to_string()],
                compliance_frameworks: vec!["Federal".to_string(), "Administrative".to_string()],
                data_categories: vec!["Regulations".to_string(), "Notices".to_string()],
                update_frequency: UpdateFrequency::Daily,
                data_retention: None,
                sla: ServiceLevelAgreement {
                    availability: 99.5,
                    response_time: chrono::Duration::seconds(5),
                    data_freshness: chrono::Duration::hours(24),
                },
                documentation_url: Some("https://www.federalregister.gov/reader-aids/developer-resources/rest-api".to_string()),
                support_contact: Some("support@federalregister.gov".to_string()),
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/documents.json?per_page=1".to_string(),
                interval: chrono::Duration::minutes(5),
                timeout: chrono::Duration::seconds(10),
                expected_status: 200,
                expected_response: None,
            },
        }
    }

    /// Create configuration for SEC EDGAR API
    pub fn sec_edgar() -> Self {
        Self {
            id: "sec-edgar".to_string(),
            name: "SEC EDGAR Database".to_string(),
            description: "Electronic Data Gathering, Analysis, and Retrieval system".to_string(),
            category: ConnectorCategory::Financial,
            base_url: "https://data.sec.gov".to_string(),
            version: "v1".to_string(),
            authentication: AuthenticationConfig {
                auth_type: AuthenticationType::Custom("User-Agent".to_string()),
                credentials: CredentialConfig::Custom({
                    let mut config = HashMap::new();
                    config.insert("User-Agent".to_string(), "AION-CR Compliance Platform contact@aion-cr.com".to_string());
                    config
                }),
                token_refresh: None,
                headers: {
                    let mut headers = HashMap::new();
                    headers.insert("User-Agent".to_string(), "AION-CR Compliance Platform contact@aion-cr.com".to_string());
                    headers
                },
            },
            rate_limits: RateLimitConfig {
                requests_per_second: Some(1), // SEC has strict rate limits
                requests_per_minute: Some(10),
                requests_per_hour: Some(100),
                requests_per_day: Some(1000),
                burst_size: Some(5),
                retry_after_header: Some("Retry-After".to_string()),
            },
            endpoints: vec![
                EndpointConfig {
                    name: "company_filings".to_string(),
                    path: "/submissions/CIK{cik}.json".to_string(),
                    method: HttpMethod::GET,
                    description: "Get company filings by CIK".to_string(),
                    parameters: vec![
                        ParameterConfig {
                            name: "cik".to_string(),
                            parameter_type: ParameterType::Path,
                            data_type: DataType::String,
                            required: true,
                            default_value: None,
                            description: "Central Index Key".to_string(),
                            validation: Some(ValidationRule::Pattern("^[0-9]{10}$".to_string())),
                        },
                    ],
                    response_format: ResponseFormat::JSON,
                    cache_ttl: Some(chrono::Duration::hours(4)),
                    requires_auth: false,
                },
            ],
            transformation_rules: vec![],
            webhook_config: None,
            metadata: ConnectorMetadata {
                jurisdiction: vec!["US".to_string()],
                compliance_frameworks: vec!["SOX".to_string(), "Securities".to_string()],
                data_categories: vec!["10-K".to_string(), "10-Q".to_string(), "8-K".to_string()],
                update_frequency: UpdateFrequency::Daily,
                data_retention: None,
                sla: ServiceLevelAgreement {
                    availability: 99.9,
                    response_time: chrono::Duration::seconds(3),
                    data_freshness: chrono::Duration::hours(24),
                },
                documentation_url: Some("https://www.sec.gov/edgar/sec-api-documentation".to_string()),
                support_contact: Some("webmaster@sec.gov".to_string()),
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/submissions/CIK0000320193.json".to_string(), // Apple Inc.
                interval: chrono::Duration::minutes(10),
                timeout: chrono::Duration::seconds(15),
                expected_status: 200,
                expected_response: None,
            },
        }
    }

    /// Create configuration for EUR-Lex API
    pub fn eur_lex() -> Self {
        Self {
            id: "eur-lex".to_string(),
            name: "EUR-Lex".to_string(),
            description: "Official Journal of the European Union".to_string(),
            category: ConnectorCategory::Government,
            base_url: "https://eur-lex.europa.eu/search-service".to_string(),
            version: "v1".to_string(),
            authentication: AuthenticationConfig::default(),
            rate_limits: RateLimitConfig::default(),
            endpoints: vec![],
            transformation_rules: vec![],
            webhook_config: None,
            metadata: ConnectorMetadata {
                jurisdiction: vec!["EU".to_string()],
                compliance_frameworks: vec!["GDPR".to_string(), "MiFID II".to_string()],
                data_categories: vec!["Directives".to_string(), "Regulations".to_string()],
                update_frequency: UpdateFrequency::Daily,
                data_retention: None,
                sla: ServiceLevelAgreement {
                    availability: 99.5,
                    response_time: chrono::Duration::seconds(5),
                    data_freshness: chrono::Duration::hours(24),
                },
                documentation_url: Some("https://eur-lex.europa.eu/content/help/data-reuse/webservice.html".to_string()),
                support_contact: None,
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/search".to_string(),
                interval: chrono::Duration::minutes(10),
                timeout: chrono::Duration::seconds(10),
                expected_status: 200,
                expected_response: None,
            },
        }
    }

    // Additional connector configurations would be implemented here...
    pub fn cftc_api() -> Self { Self::default_government("cftc-api", "CFTC API", "US") }
    pub fn finra_api() -> Self { Self::default_financial("finra-api", "FINRA API", "US") }
    pub fn fda_api() -> Self { Self::default_healthcare("fda-api", "FDA API", "US") }
    pub fn treasury_ofac() -> Self { Self::default_government("treasury-ofac", "Treasury OFAC", "US") }
    pub fn ema_api() -> Self { Self::default_healthcare("ema-api", "EMA API", "EU") }
    pub fn esma_api() -> Self { Self::default_financial("esma-api", "ESMA API", "EU") }
    pub fn eba_api() -> Self { Self::default_financial("eba-api", "EBA API", "EU") }
    pub fn uk_legislation() -> Self { Self::default_government("uk-legislation", "UK Legislation", "UK") }
    pub fn fca_api() -> Self { Self::default_financial("fca-api", "FCA API", "UK") }
    pub fn bank_of_england() -> Self { Self::default_financial("bank-of-england", "Bank of England", "UK") }
    pub fn canada_gazette() -> Self { Self::default_government("canada-gazette", "Canada Gazette", "CA") }
    pub fn osfi_api() -> Self { Self::default_financial("osfi-api", "OSFI API", "CA") }
    pub fn health_canada() -> Self { Self::default_healthcare("health-canada", "Health Canada", "CA") }
    pub fn australia_legislation() -> Self { Self::default_government("australia-legislation", "Australia Legislation", "AU") }
    pub fn apra_api() -> Self { Self::default_financial("apra-api", "APRA API", "AU") }
    pub fn asic_api() -> Self { Self::default_financial("asic-api", "ASIC API", "AU") }
    pub fn basel_committee() -> Self { Self::default_international("basel-committee", "Basel Committee") }
    pub fn iso_standards() -> Self { Self::default_international("iso-standards", "ISO Standards") }
    pub fn fatf_api() -> Self { Self::default_international("fatf-api", "FATF API") }
    pub fn oecd_api() -> Self { Self::default_international("oecd-api", "OECD API") }
    pub fn pci_standards() -> Self { Self::default_industry("pci-standards", "PCI Standards") }
    pub fn nist_standards() -> Self { Self::default_industry("nist-standards", "NIST Standards") }
    pub fn iso27001_standards() -> Self { Self::default_industry("iso27001-standards", "ISO 27001 Standards") }

    // Helper methods for creating default configurations
    fn default_government(id: &str, name: &str, jurisdiction: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("{} regulatory data", name),
            category: ConnectorCategory::Government,
            base_url: format!("https://api.{}.gov", id),
            version: "v1".to_string(),
            authentication: AuthenticationConfig::default(),
            rate_limits: RateLimitConfig::default(),
            endpoints: vec![],
            transformation_rules: vec![],
            webhook_config: None,
            metadata: ConnectorMetadata {
                jurisdiction: vec![jurisdiction.to_string()],
                compliance_frameworks: vec!["Government".to_string()],
                data_categories: vec!["Regulations".to_string()],
                update_frequency: UpdateFrequency::Daily,
                data_retention: None,
                sla: ServiceLevelAgreement {
                    availability: 99.0,
                    response_time: chrono::Duration::seconds(10),
                    data_freshness: chrono::Duration::hours(24),
                },
                documentation_url: None,
                support_contact: None,
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/health".to_string(),
                interval: chrono::Duration::minutes(10),
                timeout: chrono::Duration::seconds(10),
                expected_status: 200,
                expected_response: None,
            },
        }
    }

    fn default_financial(id: &str, name: &str, jurisdiction: &str) -> Self {
        let mut config = Self::default_government(id, name, jurisdiction);
        config.category = ConnectorCategory::Financial;
        config.metadata.compliance_frameworks = vec!["Financial".to_string(), "Banking".to_string()];
        config
    }

    fn default_healthcare(id: &str, name: &str, jurisdiction: &str) -> Self {
        let mut config = Self::default_government(id, name, jurisdiction);
        config.category = ConnectorCategory::Healthcare;
        config.metadata.compliance_frameworks = vec!["Healthcare".to_string(), "Medical".to_string()];
        config
    }

    fn default_international(id: &str, name: &str) -> Self {
        let mut config = Self::default_government(id, name, "INTL");
        config.category = ConnectorCategory::International;
        config.metadata.jurisdiction = vec!["International".to_string()];
        config
    }

    fn default_industry(id: &str, name: &str) -> Self {
        let mut config = Self::default_government(id, name, "GLOBAL");
        config.category = ConnectorCategory::IndustryStandards;
        config.metadata.jurisdiction = vec!["Global".to_string()];
        config.metadata.compliance_frameworks = vec!["Industry Standards".to_string()];
        config
    }
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default Connector".to_string(),
            description: "Default connector configuration".to_string(),
            category: ConnectorCategory::Custom("Default".to_string()),
            base_url: "https://api.example.com".to_string(),
            version: "v1".to_string(),
            authentication: AuthenticationConfig::default(),
            rate_limits: RateLimitConfig::default(),
            endpoints: vec![],
            transformation_rules: vec![],
            webhook_config: Some(WebhookConfig::default()),
            metadata: ConnectorMetadata {
                jurisdiction: vec!["GLOBAL".to_string()],
                compliance_frameworks: vec!["General".to_string()],
                data_categories: vec!["General".to_string()],
                update_frequency: UpdateFrequency::Daily,
                data_retention: None,
                sla: ServiceLevelAgreement {
                    availability: 99.0,
                    response_time: chrono::Duration::seconds(5),
                    data_freshness: chrono::Duration::hours(24),
                },
                documentation_url: None,
                support_contact: None,
            },
            health_check: HealthCheckConfig {
                enabled: false,
                endpoint: "/health".to_string(),
                interval: chrono::Duration::minutes(5),
                timeout: chrono::Duration::seconds(10),
                expected_status: 200,
                expected_response: None,
            },
        }
    }
}