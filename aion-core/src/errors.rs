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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normative_not_found_error() {
        let error = AionError::NormativeNotFound {
            id: "test-id-123".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Normative framework not found"));
        assert!(error_message.contains("test-id-123"));
    }

    #[test]
    fn test_normative_conflict_error() {
        let error = AionError::NormativeConflict {
            description: "Conflicting regulations detected".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Normative conflict detected"));
        assert!(error_message.contains("Conflicting regulations detected"));
    }

    #[test]
    fn test_compliance_validation_error() {
        let error = AionError::ComplianceValidationError {
            reason: "Missing required documentation".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Compliance validation failed"));
        assert!(error_message.contains("Missing required documentation"));
    }

    #[test]
    fn test_business_rule_violation() {
        let error = AionError::BusinessRuleViolation {
            rule_name: "age_validation".to_string(),
            message: "Age must be greater than 18".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Business rule validation failed"));
        assert!(error_message.contains("age_validation"));
        assert!(error_message.contains("Age must be greater than 18"));
    }

    #[test]
    fn test_conflict_resolution_error() {
        let error = AionError::ConflictResolutionError {
            strategy: "LexSpecialis".to_string(),
            reason: "Cannot determine specific law priority".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Conflict resolution failed"));
        assert!(error_message.contains("LexSpecialis"));
        assert!(error_message.contains("Cannot determine specific law priority"));
    }

    #[test]
    fn test_audit_trail_corruption() {
        let error = AionError::AuditTrailCorruption {
            details: "Hash mismatch detected in audit log".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Audit trail corruption detected"));
        assert!(error_message.contains("Hash mismatch detected in audit log"));
    }

    #[test]
    fn test_jurisdiction_conflict() {
        let error = AionError::JurisdictionConflict {
            details: "Federal and state authority overlap".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Jurisdiction authority conflict"));
        assert!(error_message.contains("Federal and state authority overlap"));
    }

    #[test]
    fn test_invalid_framework_structure() {
        let error = AionError::InvalidFrameworkStructure {
            field: "effective_date".to_string(),
            reason: "Date cannot be in the future".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Invalid normative framework structure"));
        assert!(error_message.contains("effective_date"));
        assert!(error_message.contains("Date cannot be in the future"));
    }

    #[test]
    fn test_database_error() {
        let error = AionError::DatabaseError {
            operation: "SELECT".to_string(),
            reason: "Connection timeout".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Database operation failed"));
        assert!(error_message.contains("SELECT"));
        assert!(error_message.contains("Connection timeout"));
    }

    #[test]
    fn test_serialization_error() {
        let error = AionError::SerializationError {
            reason: "Invalid JSON format".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Serialization error"));
        assert!(error_message.contains("Invalid JSON format"));
    }

    #[test]
    fn test_configuration_error() {
        let error = AionError::ConfigurationError {
            parameter: "database_url".to_string(),
            reason: "URL format is invalid".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Configuration error"));
        assert!(error_message.contains("database_url"));
        assert!(error_message.contains("URL format is invalid"));
    }

    #[test]
    fn test_access_denied() {
        let error = AionError::AccessDenied {
            resource: "confidential_documents".to_string(),
            reason: "Insufficient permissions".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Access denied"));
        assert!(error_message.contains("confidential_documents"));
        assert!(error_message.contains("Insufficient permissions"));
    }

    #[test]
    fn test_resource_locked() {
        let error = AionError::ResourceLocked {
            resource: "normative_framework_123".to_string(),
            holder: "user_456".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Resource locked"));
        assert!(error_message.contains("normative_framework_123"));
        assert!(error_message.contains("user_456"));
    }

    #[test]
    fn test_version_conflict() {
        let error = AionError::VersionConflict {
            entity: "framework".to_string(),
            expected: "2.0".to_string(),
            found: "1.5".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Version conflict"));
        assert!(error_message.contains("framework"));
        assert!(error_message.contains("expected 2.0"));
        assert!(error_message.contains("found 1.5"));
    }

    #[test]
    fn test_validation_error() {
        let error = AionError::ValidationError {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Validation error"));
        assert!(error_message.contains("email"));
        assert!(error_message.contains("Invalid email format"));
    }

    #[test]
    fn test_network_error() {
        let error = AionError::NetworkError {
            operation: "HTTP_GET".to_string(),
            reason: "Host unreachable".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Network error"));
        assert!(error_message.contains("HTTP_GET"));
        assert!(error_message.contains("Host unreachable"));
    }

    #[test]
    fn test_timeout_error() {
        let error = AionError::TimeoutError {
            operation: "database_query".to_string(),
            duration_ms: 5000,
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Timeout error"));
        assert!(error_message.contains("database_query"));
        assert!(error_message.contains("5000ms"));
    }

    #[test]
    fn test_internal_error() {
        let error = AionError::InternalError {
            message: "Unexpected system state".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Internal error"));
        assert!(error_message.contains("Unexpected system state"));
    }

    #[test]
    fn test_aion_result_type() {
        let success: AionResult<String> = Ok("success".to_string());
        assert!(success.is_ok());
        assert_eq!(success.unwrap(), "success");

        let failure: AionResult<String> = Err(AionError::InternalError {
            message: "test error".to_string(),
        });
        assert!(failure.is_err());
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let aion_error: AionError = json_error.into();

        match aion_error {
            AionError::SerializationError { reason } => {
                assert!(reason.contains("expected"));
            }
            _ => panic!("Expected SerializationError"),
        }
    }

    #[test]
    fn test_uuid_error_conversion() {
        let uuid_error = uuid::Uuid::parse_str("invalid-uuid").unwrap_err();
        let aion_error: AionError = uuid_error.into();

        match aion_error {
            AionError::ValidationError { field, message } => {
                assert_eq!(field, "uuid");
                assert!(message.contains("invalid"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_error_debug_format() {
        let error = AionError::NormativeNotFound {
            id: "test".to_string(),
        };

        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("NormativeNotFound"));
        assert!(debug_output.contains("test"));
    }

    #[test]
    fn test_error_chain_compatibility() {
        let error = AionError::DatabaseError {
            operation: "INSERT".to_string(),
            reason: "Constraint violation".to_string(),
        };

        // Test that error implements standard error traits
        let error_source = std::error::Error::source(&error);
        assert!(error_source.is_none()); // Our errors don't have sources by default

        let error_string = error.to_string();
        assert!(!error_string.is_empty());
    }
}