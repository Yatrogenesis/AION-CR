use thiserror::Error;

#[derive(Error, Debug)]
pub enum AionError {
    #[error("Normative framework not found: {id}")]
    NormativeNotFound { id: String },

    #[error("Normative conflict detected: {description}")]
    NormativeConflict { description: String },

    #[error("Compliance validation failed: {reason}")]
    ComplianceValidationError { reason: String },

    #[error("Business rule validation failed: {rule_name}: {message}")]
    BusinessRuleViolation { rule_name: String, message: String },

    #[error("Conflict resolution failed: {strategy}: {reason}")]
    ConflictResolutionError { strategy: String, reason: String },

    #[error("Audit trail corruption detected: {details}")]
    AuditTrailCorruption { details: String },

    #[error("Jurisdiction authority conflict: {details}")]
    JurisdictionConflict { details: String },

    #[error("Invalid normative framework structure: {field}: {reason}")]
    InvalidFrameworkStructure { field: String, reason: String },

    #[error("Database operation failed: {operation}: {reason}")]
    DatabaseError { operation: String, reason: String },

    #[error("Serialization error: {reason}")]
    SerializationError { reason: String },

    #[error("Configuration error: {parameter}: {reason}")]
    ConfigurationError { parameter: String, reason: String },

    #[error("Access denied: {resource}: {reason}")]
    AccessDenied { resource: String, reason: String },

    #[error("Resource locked: {resource}: {holder}")]
    ResourceLocked { resource: String, holder: String },

    #[error("Version conflict: {entity}: expected {expected}, found {found}")]
    VersionConflict {
        entity: String,
        expected: String,
        found: String,
    },

    #[error("Validation error: {field}: {message}")]
    ValidationError { field: String, message: String },

    #[error("Network error: {operation}: {reason}")]
    NetworkError { operation: String, reason: String },

    #[error("Timeout error: {operation}: {duration_ms}ms")]
    TimeoutError { operation: String, duration_ms: u64 },

    #[error("Internal error: {message}")]
    InternalError { message: String },
}

pub type AionResult<T> = Result<T, AionError>;

impl From<serde_json::Error> for AionError {
    fn from(err: serde_json::Error) -> Self {
        AionError::SerializationError {
            reason: err.to_string(),
        }
    }
}

impl From<uuid::Error> for AionError {
    fn from(err: uuid::Error) -> Self {
        AionError::ValidationError {
            field: "uuid".to_string(),
            message: err.to_string(),
        }
    }
}