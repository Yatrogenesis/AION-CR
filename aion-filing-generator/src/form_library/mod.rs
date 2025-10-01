/*!
 * Form Library Module
 *
 * Comprehensive library of 500+ regulatory form templates covering
 * major jurisdictions and compliance frameworks worldwide.
 */

pub mod us_forms;
pub mod eu_forms;
pub mod uk_forms;
pub mod canada_forms;
pub mod australia_forms;
pub mod international_forms;
pub mod form_registry;

// Re-export main types
pub use form_registry::*;

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, debug};
use uuid::Uuid;

/// Form library managing all regulatory form templates
pub struct FormLibrary {
    /// Form registry
    registry: Arc<FormRegistry>,

    /// Form templates by ID
    templates: Arc<RwLock<HashMap<String, FormTemplate>>>,

    /// Form metadata index
    metadata_index: Arc<RwLock<HashMap<String, FormMetadata>>>,

    /// Template cache
    template_cache: Arc<RwLock<HashMap<String, CachedTemplate>>>,

    /// Configuration
    config: FormLibraryConfig,
}

/// Form library configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormLibraryConfig {
    pub cache_size: usize,
    pub auto_update: bool,
    pub update_interval: chrono::Duration,
    pub supported_languages: Vec<String>,
    pub default_jurisdiction: String,
}

impl Default for FormLibraryConfig {
    fn default() -> Self {
        Self {
            cache_size: 1000,
            auto_update: true,
            update_interval: chrono::Duration::hours(24),
            supported_languages: vec![
                "en".to_string(), "es".to_string(), "fr".to_string(),
                "de".to_string(), "ja".to_string(), "zh".to_string(),
                "pt".to_string(), "it".to_string(), "ru".to_string(),
                "ko".to_string(), "ar".to_string(), "hi".to_string(),
            ],
            default_jurisdiction: "GLOBAL".to_string(),
        }
    }
}

/// Form template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub jurisdiction: String,
    pub compliance_framework: String,
    pub category: FormCategory,
    pub fields: Vec<FormField>,
    pub sections: Vec<FormSection>,
    pub validation_rules: Vec<ValidationRule>,
    pub template_content: TemplateContent,
    pub metadata: FormMetadata,
    pub localization: HashMap<String, LocalizedContent>,
}

/// Form categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormCategory {
    Financial,
    Tax,
    Healthcare,
    Environmental,
    Labor,
    Securities,
    Banking,
    Insurance,
    Technology,
    Manufacturing,
    Custom(String),
}

/// Form field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub field_id: String,
    pub name: String,
    pub description: String,
    pub field_type: FieldType,
    pub required: bool,
    pub validation: Option<FieldValidation>,
    pub default_value: Option<serde_json::Value>,
    pub help_text: Option<String>,
    pub conditional_logic: Option<ConditionalLogic>,
}

/// Field types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Date,
    DateTime,
    Boolean,
    Select { options: Vec<SelectOption> },
    MultiSelect { options: Vec<SelectOption> },
    File,
    Currency,
    Percentage,
    Email,
    Phone,
    Address,
    TaxId,
    Custom(String),
}

/// Select option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

/// Field validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub custom_validator: Option<String>,
}

/// Conditional logic for fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalLogic {
    pub condition: String,
    pub action: ConditionalAction,
}

/// Conditional actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalAction {
    Show,
    Hide,
    Require,
    Optional,
    SetValue(serde_json::Value),
}

/// Form section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSection {
    pub section_id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<String>,
    pub subsections: Vec<FormSection>,
    pub conditional_logic: Option<ConditionalLogic>,
}

/// Template content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContent {
    pub template_type: TemplateType,
    pub content: String,
    pub variables: HashMap<String, VariableDefinition>,
    pub layouts: HashMap<String, LayoutDefinition>,
}

/// Template types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    Handlebars,
    Tera,
    Jinja,
    Liquid,
    Custom(String),
}

/// Variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub name: String,
    pub var_type: String,
    pub default_value: Option<serde_json::Value>,
    pub description: String,
}

/// Layout definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDefinition {
    pub name: String,
    pub description: String,
    pub template: String,
    pub styles: Option<String>,
}

/// Form metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormMetadata {
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub updated_date: chrono::DateTime<chrono::Utc>,
    pub version_history: Vec<VersionInfo>,
    pub tags: Vec<String>,
    pub regulatory_authority: String,
    pub submission_method: SubmissionMethod,
    pub filing_frequency: FilingFrequency,
    pub deadline_rules: Vec<DeadlineRule>,
    pub dependencies: Vec<String>,
    pub related_forms: Vec<String>,
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub changes: String,
    pub author: String,
}

/// Submission methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionMethod {
    Electronic,
    Paper,
    Both,
    OnlinePortal { url: String },
    Email { address: String },
    Api { endpoint: String },
}

/// Filing frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilingFrequency {
    Annual,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
    OnDemand,
    EventBased,
    Custom(String),
}

/// Deadline rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineRule {
    pub description: String,
    pub calculation: DeadlineCalculation,
    pub exceptions: Vec<DeadlineException>,
}

/// Deadline calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlineCalculation {
    DaysAfterPeriodEnd(i32),
    MonthsAfterPeriodEnd(i32),
    SpecificDate { month: u32, day: u32 },
    BusinessDays(i32),
    Custom(String),
}

/// Deadline exceptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineException {
    pub condition: String,
    pub adjustment: DeadlineAdjustment,
}

/// Deadline adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlineAdjustment {
    ExtendDays(i32),
    NextBusinessDay,
    PreviousBusinessDay,
    Custom(String),
}

/// Localized content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedContent {
    pub language: String,
    pub name: String,
    pub description: String,
    pub field_labels: HashMap<String, String>,
    pub help_texts: HashMap<String, String>,
    pub validation_messages: HashMap<String, String>,
}

/// Cached template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTemplate {
    pub template: FormTemplate,
    pub cached_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub rule_type: ValidationRuleType,
    pub severity: ValidationSeverity,
    pub condition: String,
    pub error_message: String,
}

/// Validation rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    Format,
    Range,
    Dependency,
    BusinessRule,
    Custom,
}

/// Validation severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

impl FormLibrary {
    /// Create new form library
    pub async fn new(config: FormLibraryConfig) -> Result<Self> {
        info!("📚 Initializing form library");

        let registry = Arc::new(FormRegistry::new().await?);

        Ok(Self {
            registry,
            templates: Arc::new(RwLock::new(HashMap::new())),
            metadata_index: Arc::new(RwLock::new(HashMap::new())),
            template_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Start the form library
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting form library");

        // Start registry
        self.registry.start().await?;

        info!("✅ Form library started");
        Ok(())
    }

    /// Stop the form library
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping form library");

        // Stop registry
        self.registry.stop().await?;

        info!("✅ Form library stopped");
        Ok(())
    }

    /// Get template by ID
    pub async fn get_template(&self, template_id: &str) -> Result<FormTemplate> {
        debug!("📋 Getting template: {}", template_id);

        // Check cache first
        {
            let cache = self.template_cache.read().await;
            if let Some(cached) = cache.get(template_id) {
                // Update access statistics
                let mut cache_write = self.template_cache.write().await;
                if let Some(cached_mut) = cache_write.get_mut(template_id) {
                    cached_mut.access_count += 1;
                    cached_mut.last_accessed = chrono::Utc::now();
                }
                return Ok(cached.template.clone());
            }
        }

        // Load from registry
        let template = self.registry.get_template(template_id).await?;

        // Cache the template
        {
            let mut cache = self.template_cache.write().await;
            cache.insert(template_id.to_string(), CachedTemplate {
                template: template.clone(),
                cached_at: chrono::Utc::now(),
                access_count: 1,
                last_accessed: chrono::Utc::now(),
            });
        }

        Ok(template)
    }

    /// Register a new template
    pub async fn register_template(&self, config: FormTemplateConfig) -> Result<String> {
        info!("📝 Registering template: {}", config.name);

        let template_id = self.registry.register_template(config).await?;

        // Clear relevant cache entries
        self.invalidate_cache().await?;

        info!("✅ Template registered: {}", template_id);
        Ok(template_id)
    }

    /// Search forms
    pub async fn search_forms(&self, filter: crate::FormFilter) -> Result<Vec<FormTemplate>> {
        debug!("🔍 Searching forms with filter: {:?}", filter);

        let templates = self.registry.search_templates(filter).await?;

        Ok(templates)
    }

    /// Get form count
    pub async fn get_form_count(&self) -> Result<u32> {
        self.registry.get_template_count().await
    }

    /// Get status
    pub async fn get_status(&self) -> Result<FormLibraryStatus> {
        let template_count = self.get_form_count().await?;
        let cache_size = {
            let cache = self.template_cache.read().await;
            cache.len()
        };

        Ok(FormLibraryStatus {
            total_templates: template_count,
            cached_templates: cache_size as u32,
            supported_languages: self.config.supported_languages.len() as u32,
            last_update: chrono::Utc::now(),
        })
    }

    /// Invalidate cache
    async fn invalidate_cache(&self) -> Result<()> {
        let mut cache = self.template_cache.write().await;
        cache.clear();
        Ok(())
    }
}

/// Form library status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormLibraryStatus {
    pub total_templates: u32,
    pub cached_templates: u32,
    pub supported_languages: u32,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

/// Form template configuration for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormTemplateConfig {
    pub name: String,
    pub description: String,
    pub jurisdiction: String,
    pub compliance_framework: String,
    pub category: FormCategory,
    pub template_source: TemplateSource,
    pub metadata: FormTemplateMetadata,
}

/// Template source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateSource {
    Builtin { resource_path: String },
    File { file_path: String },
    Url { url: String },
    Inline { content: String },
}

/// Form template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormTemplateMetadata {
    pub version: String,
    pub author: String,
    pub regulatory_authority: String,
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    pub tags: Vec<String>,
}

impl FormTemplateConfig {
    /// Create SEC 10-K form configuration
    pub fn sec_10k() -> Self {
        Self {
            name: "SEC Form 10-K".to_string(),
            description: "Annual report required by the SEC".to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Securities".to_string(),
            category: FormCategory::Financial,
            template_source: TemplateSource::Builtin {
                resource_path: "templates/us/sec/form-10k.hbs".to_string(),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Securities and Exchange Commission".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["SEC".to_string(), "Annual".to_string(), "Public Company".to_string()],
            },
        }
    }

    /// Create SEC 10-Q form configuration
    pub fn sec_10q() -> Self {
        Self {
            name: "SEC Form 10-Q".to_string(),
            description: "Quarterly report required by the SEC".to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Securities".to_string(),
            category: FormCategory::Financial,
            template_source: TemplateSource::Builtin {
                resource_path: "templates/us/sec/form-10q.hbs".to_string(),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Securities and Exchange Commission".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["SEC".to_string(), "Quarterly".to_string(), "Public Company".to_string()],
            },
        }
    }

    /// Create SEC 8-K form configuration
    pub fn sec_8k() -> Self {
        Self {
            name: "SEC Form 8-K".to_string(),
            description: "Current report filed for major corporate events".to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Securities".to_string(),
            category: FormCategory::Financial,
            template_source: TemplateSource::Builtin {
                resource_path: "templates/us/sec/form-8k.hbs".to_string(),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Securities and Exchange Commission".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["SEC".to_string(), "Current Report".to_string(), "Events".to_string()],
            },
        }
    }

    // Additional form configurations would be implemented here...
    pub fn sec_form_d() -> Self { Self::default_sec("Form D", "Notice of Exempt Offering") }
    pub fn sec_form_s1() -> Self { Self::default_sec("Form S-1", "Registration Statement") }
    pub fn irs_form_1120() -> Self { Self::default_tax("Form 1120", "U.S. Corporation Income Tax Return") }
    pub fn irs_form_941() -> Self { Self::default_tax("Form 941", "Employer's Quarterly Federal Tax Return") }
    pub fn irs_form_990() -> Self { Self::default_tax("Form 990", "Return of Organization Exempt From Income Tax") }
    pub fn ffiec_call_report() -> Self { Self::default_banking("FFIEC Call Report", "Consolidated Reports of Condition and Income") }
    pub fn occ_sar() -> Self { Self::default_banking("SAR", "Suspicious Activity Report") }
    pub fn fda_510k() -> Self { Self::default_healthcare("FDA 510(k)", "Premarket Notification") }
    pub fn fda_nda() -> Self { Self::default_healthcare("FDA NDA", "New Drug Application") }
    pub fn epa_tier_ii() -> Self { Self::default_environmental("EPA Tier II", "Emergency and Hazardous Chemical Inventory") }
    pub fn epa_form_r() -> Self { Self::default_environmental("EPA Form R", "Toxic Chemical Release Inventory") }
    pub fn dol_form_5500() -> Self { Self::default_labor("DOL Form 5500", "Annual Return/Report of Employee Benefit Plan") }
    pub fn osha_form_300() -> Self { Self::default_labor("OSHA Form 300", "Log of Work-Related Injuries and Illnesses") }
    pub fn gdpr_dpia() -> Self { Self::default_eu("GDPR DPIA", "Data Protection Impact Assessment") }
    pub fn gdpr_breach_notification() -> Self { Self::default_eu("GDPR Breach", "Personal Data Breach Notification") }
    pub fn mifid_ii_transaction_report() -> Self { Self::default_eu("MiFID II TR", "Transaction Report") }
    pub fn emir_trade_report() -> Self { Self::default_eu("EMIR TR", "Trade Report") }
    pub fn reach_registration() -> Self { Self::default_eu("REACH Registration", "Chemical Registration") }
    pub fn ets_monitoring_report() -> Self { Self::default_eu("EU ETS", "Emissions Monitoring Report") }

    // Helper methods for creating default configurations
    fn default_sec(name: &str, description: &str) -> Self {
        Self {
            name: format!("SEC {}", name),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Securities".to_string(),
            category: FormCategory::Securities,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/sec/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Securities and Exchange Commission".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["SEC".to_string()],
            },
        }
    }

    fn default_tax(name: &str, description: &str) -> Self {
        Self {
            name: format!("IRS {}", name),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Tax".to_string(),
            category: FormCategory::Tax,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/irs/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Internal Revenue Service".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["IRS".to_string(), "Tax".to_string()],
            },
        }
    }

    fn default_banking(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Banking".to_string(),
            category: FormCategory::Banking,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/banking/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Federal Financial Institutions Examination Council".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["Banking".to_string()],
            },
        }
    }

    fn default_healthcare(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Healthcare".to_string(),
            category: FormCategory::Healthcare,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/fda/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Food and Drug Administration".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["FDA".to_string(), "Healthcare".to_string()],
            },
        }
    }

    fn default_environmental(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Environmental".to_string(),
            category: FormCategory::Environmental,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/epa/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Environmental Protection Agency".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["EPA".to_string(), "Environmental".to_string()],
            },
        }
    }

    fn default_labor(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            jurisdiction: "US".to_string(),
            compliance_framework: "Labor".to_string(),
            category: FormCategory::Labor,
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/us/dol/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "Department of Labor".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["DOL".to_string(), "Labor".to_string()],
            },
        }
    }

    fn default_eu(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            jurisdiction: "EU".to_string(),
            compliance_framework: "European Union".to_string(),
            category: FormCategory::Custom("EU".to_string()),
            template_source: TemplateSource::Builtin {
                resource_path: format!("templates/eu/{}.hbs", name.to_lowercase().replace(" ", "-")),
            },
            metadata: FormTemplateMetadata {
                version: "2024.1".to_string(),
                author: "AION-CR Team".to_string(),
                regulatory_authority: "European Commission".to_string(),
                effective_date: chrono::Utc::now(),
                expiration_date: None,
                tags: vec!["EU".to_string()],
            },
        }
    }
}