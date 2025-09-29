use crate::{AionResult, ComplianceAssessment, NormativeConflict, NormativeFramework, NormativeId};
use std::collections::HashMap;

pub trait NormativeRepository {
    fn store_framework(&mut self, framework: NormativeFramework) -> AionResult<()>;
    fn get_framework(&self, id: &NormativeId) -> AionResult<Option<NormativeFramework>>;
    fn list_frameworks(&self) -> AionResult<Vec<NormativeFramework>>;
    fn update_framework(&mut self, framework: NormativeFramework) -> AionResult<()>;
    fn delete_framework(&mut self, id: &NormativeId) -> AionResult<()>;
    fn search_frameworks(&self, query: &str) -> AionResult<Vec<NormativeFramework>>;
    fn get_active_frameworks(&self) -> AionResult<Vec<NormativeFramework>>;
}

pub trait ConflictDetector {
    fn detect_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>>;
    fn analyze_conflict_severity(&self, conflict: &NormativeConflict) -> AionResult<()>;
    fn get_conflicting_frameworks(&self, id: &NormativeId) -> AionResult<Vec<NormativeId>>;
}

pub trait ConflictResolver {
    fn resolve_conflict(&self, conflict: &NormativeConflict) -> AionResult<NormativeFramework>;
    fn suggest_resolution_strategies(&self, conflict: &NormativeConflict) -> AionResult<Vec<String>>;
    fn apply_resolution_strategy(&self, conflict: &NormativeConflict, strategy: &str) -> AionResult<()>;
}

pub trait ComplianceEngine {
    fn assess_compliance(&self, entity_id: &str, frameworks: &[NormativeId]) -> AionResult<ComplianceAssessment>;
    fn validate_requirements(&self, entity_id: &str, requirements: &[uuid::Uuid]) -> AionResult<Vec<bool>>;
    fn generate_compliance_report(&self, assessment: &ComplianceAssessment) -> AionResult<String>;
}

pub trait BusinessRuleEngine {
    fn evaluate_rule(&self, rule_id: &uuid::Uuid, context: &HashMap<String, String>) -> AionResult<bool>;
    fn get_applicable_rules(&self, context: &HashMap<String, String>) -> AionResult<Vec<uuid::Uuid>>;
    fn validate_business_logic(&self, entity_id: &str, context: &HashMap<String, String>) -> AionResult<Vec<String>>;
}

pub trait AuditSystem {
    fn record_action(&mut self, entity_type: &str, entity_id: &str, action: &str, actor: &str, details: HashMap<String, String>) -> AionResult<()>;
    fn get_audit_trail(&self, entity_id: &str) -> AionResult<Vec<crate::AuditTrail>>;
    fn verify_integrity(&self) -> AionResult<bool>;
}

pub trait GovernanceFramework {
    fn initialize_governance(&mut self, context: &crate::GovernanceContext) -> AionResult<()>;
    fn assess_maturity(&self, entity_id: &str) -> AionResult<String>;
    fn recommend_improvements(&self, entity_id: &str) -> AionResult<Vec<crate::Recommendation>>;
    fn generate_governance_report(&self, entity_id: &str) -> AionResult<String>;
}

pub trait ValidationEngine {
    fn validate_framework(&self, framework: &NormativeFramework) -> AionResult<Vec<String>>;
    fn validate_requirement(&self, requirement: &crate::Requirement) -> AionResult<Vec<String>>;
    fn validate_evidence(&self, evidence: &crate::Evidence) -> AionResult<bool>;
}

pub trait NotificationSystem {
    fn notify_conflict_detected(&self, conflict: &NormativeConflict) -> AionResult<()>;
    fn notify_compliance_change(&self, assessment: &ComplianceAssessment) -> AionResult<()>;
    fn notify_framework_update(&self, framework: &NormativeFramework) -> AionResult<()>;
}

pub trait MetricsCollector {
    fn collect_compliance_metrics(&self, entity_id: &str) -> AionResult<HashMap<String, f64>>;
    fn collect_conflict_metrics(&self) -> AionResult<HashMap<String, u64>>;
    fn collect_performance_metrics(&self) -> AionResult<HashMap<String, f64>>;
}

pub trait ReportGenerator {
    fn generate_executive_summary(&self, entity_id: &str) -> AionResult<String>;
    fn generate_detailed_report(&self, entity_id: &str) -> AionResult<String>;
    fn generate_trend_analysis(&self, entity_id: &str, days: u32) -> AionResult<String>;
}

pub trait CacheManager {
    fn get<T>(&self, key: &str) -> AionResult<Option<T>>
    where
        T: serde::de::DeserializeOwned;
    fn set<T>(&mut self, key: &str, value: &T, ttl_seconds: u64) -> AionResult<()>
    where
        T: serde::Serialize;
    fn invalidate(&mut self, key: &str) -> AionResult<()>;
    fn clear(&mut self) -> AionResult<()>;
}