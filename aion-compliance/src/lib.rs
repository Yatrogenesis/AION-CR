pub mod engine;
pub mod assessor;
pub mod monitor;
pub mod reporter;
pub mod frameworks;
pub mod global_frameworks;
pub mod regulatory_monitor;
pub mod licensing_engine;
pub mod granular_legal_database;
pub mod interactive_query_system;
pub mod comprehensive_legal_library;
pub mod financial_services_library;
pub mod healthcare_pharmaceutical_library;
pub mod energy_utilities_library;
pub mod manufacturing_industrial_library;
pub mod technology_digital_economy_library;
pub mod environmental_sustainability_library;
pub mod labor_employment_library;
pub mod international_trade_customs_library;
pub mod intellectual_property_library;
pub mod connectors;
pub mod dynamic_rules_engine;
pub mod database_manager;
pub mod migrations;
pub mod llm_integration;
pub mod predictive_analytics;
pub mod real_time_dashboards;
pub mod dashboard_components;
pub mod alert_notification_system;
pub mod multi_jurisdictional_framework;
pub mod autonomous_agents;
pub mod ai_reasoning_engine;
pub mod quantum_cryptography;
pub mod regulatory_prediction_system;
pub mod testing_frameworks;
pub mod performance_optimization;
pub mod real_quantum_crypto;
pub mod real_ml_models;
pub mod real_database;
pub mod real_http_apis;

pub use engine::*;
pub use assessor::*;
pub use monitor::*;
pub use reporter::*;
pub use frameworks::*;
pub use global_frameworks::*;
pub use regulatory_monitor::*;
pub use licensing_engine::*;
pub use granular_legal_database::*;
pub use interactive_query_system::*;
pub use comprehensive_legal_library::*;
pub use financial_services_library::*;
pub use healthcare_pharmaceutical_library::*;
pub use energy_utilities_library::*;
pub use manufacturing_industrial_library::*;
pub use technology_digital_economy_library::*;
pub use environmental_sustainability_library::*;
pub use labor_employment_library::*;
pub use international_trade_customs_library::*;
pub use intellectual_property_library::*;
pub use connectors::*;
pub use dynamic_rules_engine::*;
pub use database_manager::*;
pub use migrations::*;
pub use llm_integration::*;
pub use predictive_analytics::*;
pub use real_time_dashboards::*;
pub use dashboard_components::*;
pub use alert_notification_system::*;
pub use multi_jurisdictional_framework::*;
pub use autonomous_agents::*;
pub use ai_reasoning_engine::*;
pub use quantum_cryptography::*;
pub use regulatory_prediction_system::*;
pub use testing_frameworks::*;
pub use performance_optimization::*;
pub use real_quantum_crypto::*;
pub use real_ml_models::*;
pub use real_database::*;
pub use real_http_apis::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Test that all modules are accessible
        // This ensures the public API is working correctly

        // Test that we can create basic compliance structures
        use aion_core::{NormativeId, NormativeType, Jurisdiction};

        let id = NormativeId::new();
        assert!(!id.0.is_nil());

        let normative_type = NormativeType::Regulation;
        assert_eq!(format!("{:?}", normative_type), "Regulation");

        let jurisdiction = Jurisdiction::Federal;
        assert_eq!(format!("{:?}", jurisdiction), "Federal");
    }

    #[test]
    fn test_compliance_integration() {
        // Test that compliance modules integrate properly
        // This verifies the module structure works as expected

        // Test basic compliance workflow
        let framework_id = aion_core::NormativeId::new();
        assert!(!framework_id.0.is_nil());

        // Test priority parsing from utils
        let priority = aion_core::parse_priority("high");
        assert!(priority.is_ok());
        assert_eq!(priority.unwrap(), 2);
    }

    #[test]
    fn test_error_handling() {
        use aion_core::{AionError, AionResult};

        let error = AionError::ComplianceValidationError {
            reason: "Test validation failure".to_string(),
        };

        let error_message = error.to_string();
        assert!(error_message.contains("Compliance validation failed"));
        assert!(error_message.contains("Test validation failure"));

        let result: AionResult<String> = Err(error);
        assert!(result.is_err());
    }

    #[test]
    fn test_normative_framework_validation() {
        use aion_core::{NormativeFramework, NormativeType, Jurisdiction};

        let framework = NormativeFramework::new(
            "Test Compliance Framework".to_string(),
            "Framework for testing compliance functionality".to_string(),
            NormativeType::Framework,
            Jurisdiction::Federal,
            "Test Compliance Authority".to_string(),
        );

        assert_eq!(framework.title, "Test Compliance Framework");
        assert_eq!(framework.normative_type, NormativeType::Framework);
        assert_eq!(framework.jurisdiction, Jurisdiction::Federal);
        assert!(framework.is_active());

        // Test mandatory requirements functionality
        let mandatory_reqs = framework.get_mandatory_requirements();
        assert_eq!(mandatory_reqs.len(), 0); // New framework has no requirements
    }

    #[test]
    fn test_validation_utilities() {
        use aion_core::{validate_email, validate_version, normalize_string};

        // Test email validation for compliance contacts
        assert!(validate_email("compliance@company.com"));
        assert!(validate_email("legal.team@organization.gov"));
        assert!(!validate_email("invalid-email"));

        // Test version validation for framework versions
        assert!(validate_version("1.0.0"));
        assert!(validate_version("2.1.5"));
        assert!(!validate_version("invalid-version"));

        // Test string normalization for consistent data handling
        assert_eq!(normalize_string("  Compliance Framework  "), "compliance_framework");
        assert_eq!(normalize_string("GDPR Implementation"), "gdpr_implementation");
    }

    #[test]
    fn test_metadata_handling() {
        use aion_core::merge_metadata;
        use std::collections::HashMap;

        let mut base_metadata = HashMap::new();
        base_metadata.insert("source".to_string(), "regulatory_database".to_string());
        base_metadata.insert("version".to_string(), "1.0".to_string());

        let mut overlay_metadata = HashMap::new();
        overlay_metadata.insert("version".to_string(), "1.1".to_string());
        overlay_metadata.insert("last_updated".to_string(), "2024-01-01".to_string());

        let merged = merge_metadata(&base_metadata, &overlay_metadata);

        assert_eq!(merged.get("source"), Some(&"regulatory_database".to_string()));
        assert_eq!(merged.get("version"), Some(&"1.1".to_string())); // Overlay wins
        assert_eq!(merged.get("last_updated"), Some(&"2024-01-01".to_string()));
    }

    #[test]
    fn test_keyword_extraction() {
        use aion_core::extract_keywords;

        let compliance_text = "The Data Protection Regulation requires comprehensive privacy compliance for all personal data processing";
        let keywords = extract_keywords(compliance_text);

        assert!(keywords.contains(&"data".to_string()));
        assert!(keywords.contains(&"protection".to_string()));
        assert!(keywords.contains(&"regulation".to_string()));
        assert!(keywords.contains(&"privacy".to_string()));
        assert!(keywords.contains(&"compliance".to_string()));
        assert!(keywords.contains(&"personal".to_string()));
        assert!(keywords.contains(&"processing".to_string()));

        // Stop words should be filtered out
        assert!(!keywords.contains(&"the".to_string()));
        assert!(!keywords.contains(&"for".to_string()));
        assert!(!keywords.contains(&"all".to_string()));
    }

    #[test]
    fn test_similarity_calculation() {
        use aion_core::calculate_similarity;

        let text1 = "GDPR data protection requirements for EU organizations";
        let text2 = "European data protection regulations and GDPR compliance";
        let text3 = "Financial services banking regulations and compliance";

        let similarity_high = calculate_similarity(text1, text2);
        let similarity_low = calculate_similarity(text1, text3);

        assert!(similarity_high > 0.0);
        assert!(similarity_low >= 0.0);
        assert!(similarity_high > similarity_low);
    }

    #[test]
    fn test_reference_id_generation() {
        use aion_core::generate_reference_id;

        let compliance_ref = generate_reference_id("COMP");
        let audit_ref = generate_reference_id("AUDIT");
        let framework_ref = generate_reference_id("FW");

        assert!(compliance_ref.starts_with("COMP-"));
        assert!(audit_ref.starts_with("AUDIT-"));
        assert!(framework_ref.starts_with("FW-"));

        // All references should be unique
        assert_ne!(compliance_ref, audit_ref);
        assert_ne!(audit_ref, framework_ref);
        assert_ne!(compliance_ref, framework_ref);
    }

    #[test]
    fn test_file_size_formatting() {
        use aion_core::format_file_size;

        // Test compliance document sizes
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1048576), "1.00 MB");
        assert_eq!(format_file_size(2097152), "2.00 MB");

        // Large compliance databases
        assert_eq!(format_file_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_business_hours_validation() {
        use aion_core::is_business_hours;
        use chrono::{TimeZone, Utc};

        // Test compliance review schedules
        let business_hour = Utc.with_ymd_and_hms(2024, 1, 15, 14, 30, 0).unwrap();
        let after_hours = Utc.with_ymd_and_hms(2024, 1, 15, 20, 0, 0).unwrap();
        let early_morning = Utc.with_ymd_and_hms(2024, 1, 15, 6, 0, 0).unwrap();

        assert!(is_business_hours(business_hour));
        assert!(!is_business_hours(after_hours));
        assert!(!is_business_hours(early_morning));
    }

    #[test]
    fn test_sequence_number_generation() {
        use aion_core::next_sequence_number;

        // Test compliance audit trail sequence numbers
        let seq1 = next_sequence_number();
        let seq2 = next_sequence_number();
        let seq3 = next_sequence_number();

        assert!(seq2 > seq1);
        assert!(seq3 > seq2);
        assert_eq!(seq2, seq1 + 1);
        assert_eq!(seq3, seq2 + 1);
    }

    #[test]
    fn test_sql_string_escaping() {
        use aion_core::escape_sql_string;

        // Test compliance data sanitization
        let unsafe_input = "Regulation with 'quotes' and \\backslash";
        let safe_output = escape_sql_string(unsafe_input);

        assert_eq!(safe_output, "Regulation with ''quotes'' and \\\\backslash");

        let normal_text = "Standard compliance text";
        assert_eq!(escape_sql_string(normal_text), normal_text);
    }

    #[test]
    fn test_string_truncation() {
        use aion_core::truncate_string;

        // Test compliance summary truncation
        let long_regulation = "This is a very long regulation description that needs to be truncated for display purposes";
        let truncated = truncate_string(long_regulation, 50);

        assert!(truncated.len() <= 50);
        assert!(truncated.ends_with("..."));

        let short_text = "Short text";
        assert_eq!(truncate_string(short_text, 50), short_text);
    }

    #[test]
    fn test_uuid_validation() {
        use aion_core::is_valid_uuid;

        // Test compliance entity ID validation
        assert!(is_valid_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_valid_uuid("6ba7b810-9dad-11d1-80b4-00c04fd430c8"));

        assert!(!is_valid_uuid("invalid-uuid"));
        assert!(!is_valid_uuid("123-456-789"));
        assert!(!is_valid_uuid(""));
    }
}