use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormativeId(pub Uuid);

impl NormativeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NormativeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NormativeType {
    Regulation,
    Policy,
    Standard,
    Guideline,
    Procedure,
    Protocol,
    Framework,
    Principle,
    Rule,
    Directive,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Jurisdiction {
    International,
    Federal,
    State,
    Regional,
    Local,
    Sectoral,
    Organizational,
    Departmental,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    PartiallyCompliant,
    NotApplicable,
    UnderReview,
    Pending,
    Exempt,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictType {
    DirectContradiction,
    ImplicitConflict,
    JurisdictionalOverlap,
    TemporalInconsistency,
    ScopeAmbiguity,
    AuthorityConflict,
    RequirementConflict,
    ImplementationConflict,
    PriorityDispute,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    LexSpecialis,
    LexPosterior,
    LexSuperior,
    Harmonization,
    Contextualization,
    Delegation,
    Arbitration,
    Mediation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormativeFramework {
    pub id: NormativeId,
    pub title: String,
    pub description: String,
    pub normative_type: NormativeType,
    pub jurisdiction: Jurisdiction,
    pub authority: String,
    pub effective_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub version: String,
    pub status: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub requirements: Vec<Requirement>,
    pub dependencies: Vec<NormativeId>,
    pub supersedes: Vec<NormativeId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub mandatory: bool,
    pub conditions: Vec<Condition>,
    pub exceptions: Vec<Exception>,
    pub evidence_required: Vec<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub priority: u8,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub id: Uuid,
    pub description: String,
    pub expression: String,
    pub context_variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    pub id: Uuid,
    pub description: String,
    pub conditions: Vec<Condition>,
    pub scope: String,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: Uuid,
    pub name: String,
    pub rule_type: String,
    pub expression: String,
    pub error_message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormativeConflict {
    pub id: Uuid,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub normative_a: NormativeId,
    pub normative_b: NormativeId,
    pub involved_frameworks: Vec<NormativeId>,
    pub description: String,
    pub affected_requirements: Vec<Uuid>,
    pub context: HashMap<String, String>,
    pub discovered_at: DateTime<Utc>,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    pub id: Uuid,
    pub entity_id: String,
    pub normative_framework: NormativeId,
    pub assessment_date: DateTime<Utc>,
    pub assessor: String,
    pub overall_status: ComplianceStatus,
    pub requirement_assessments: Vec<RequirementAssessment>,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub next_review_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementAssessment {
    pub requirement_id: Uuid,
    pub status: ComplianceStatus,
    pub evidence: Vec<Evidence>,
    pub gaps: Vec<String>,
    pub notes: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub evidence_type: String,
    pub description: String,
    pub source: String,
    pub collected_date: DateTime<Utc>,
    pub verification_status: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: Uuid,
    pub finding_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub affected_requirements: Vec<Uuid>,
    pub root_cause: Option<String>,
    pub impact_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub effort_estimate: Option<String>,
    pub timeline: Option<String>,
    pub responsible_party: Option<String>,
    pub related_findings: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrail {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
    pub details: HashMap<String, String>,
    pub previous_state: Option<String>,
    pub new_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_expression: String,
    pub context: String,
    pub priority: u8,
    pub active: bool,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceContext {
    pub organization: String,
    pub sector: String,
    pub region: String,
    pub applicable_jurisdictions: Vec<Jurisdiction>,
    pub business_context: HashMap<String, String>,
    pub risk_profile: String,
    pub maturity_level: String,
}

impl NormativeFramework {
    pub fn new(
        title: String,
        description: String,
        normative_type: NormativeType,
        jurisdiction: Jurisdiction,
        authority: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: NormativeId::new(),
            title,
            description,
            normative_type,
            jurisdiction,
            authority,
            effective_date: now,
            expiration_date: None,
            version: "1.0.0".to_string(),
            status: "active".to_string(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            requirements: Vec::new(),
            dependencies: Vec::new(),
            supersedes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.effective_date
            && self.expiration_date.map_or(true, |exp| now < exp)
            && self.status == "active"
    }

    pub fn add_requirement(&mut self, requirement: Requirement) {
        self.requirements.push(requirement);
        self.updated_at = Utc::now();
    }

    pub fn get_mandatory_requirements(&self) -> Vec<&Requirement> {
        self.requirements.iter().filter(|r| r.mandatory).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normative_id_creation() {
        let id1 = NormativeId::new();
        let id2 = NormativeId::new();
        assert_ne!(id1, id2, "Each NormativeId should be unique");
    }

    #[test]
    fn test_normative_id_default() {
        let id = NormativeId::default();
        assert!(!id.0.is_nil(), "Default NormativeId should not be nil");
    }

    #[test]
    fn test_normative_id_serialization() {
        let id = NormativeId::new();
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: NormativeId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized, "NormativeId should serialize/deserialize correctly");
    }

    #[test]
    fn test_normative_type_variants() {
        let types = vec![
            NormativeType::Regulation,
            NormativeType::Policy,
            NormativeType::Standard,
            NormativeType::Guideline,
            NormativeType::Procedure,
            NormativeType::Protocol,
            NormativeType::Framework,
            NormativeType::Principle,
            NormativeType::Rule,
            NormativeType::Directive,
        ];

        for normative_type in types {
            let serialized = serde_json::to_string(&normative_type).unwrap();
            let deserialized: NormativeType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(normative_type, deserialized);
        }
    }

    #[test]
    fn test_jurisdiction_variants() {
        let jurisdictions = vec![
            Jurisdiction::International,
            Jurisdiction::Federal,
            Jurisdiction::State,
            Jurisdiction::Regional,
            Jurisdiction::Local,
            Jurisdiction::Sectoral,
            Jurisdiction::Organizational,
            Jurisdiction::Departmental,
        ];

        for jurisdiction in jurisdictions {
            let serialized = serde_json::to_string(&jurisdiction).unwrap();
            let deserialized: Jurisdiction = serde_json::from_str(&serialized).unwrap();
            assert_eq!(jurisdiction, deserialized);
        }
    }

    #[test]
    fn test_compliance_status_variants() {
        let statuses = vec![
            ComplianceStatus::Compliant,
            ComplianceStatus::NonCompliant,
            ComplianceStatus::PartiallyCompliant,
            ComplianceStatus::NotApplicable,
            ComplianceStatus::UnderReview,
            ComplianceStatus::Pending,
            ComplianceStatus::Exempt,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: ComplianceStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_conflict_severity_ordering() {
        assert!(ConflictSeverity::Critical > ConflictSeverity::High);
        assert!(ConflictSeverity::High > ConflictSeverity::Medium);
        assert!(ConflictSeverity::Medium > ConflictSeverity::Low);
        assert!(ConflictSeverity::Low > ConflictSeverity::Informational);
    }

    #[test]
    fn test_normative_framework_creation() {
        let framework = NormativeFramework::new(
            "Test Framework".to_string(),
            "A test framework for validation".to_string(),
            NormativeType::Framework,
            Jurisdiction::Federal,
            "Test Authority".to_string(),
        );

        assert_eq!(framework.title, "Test Framework");
        assert_eq!(framework.normative_type, NormativeType::Framework);
        assert_eq!(framework.jurisdiction, Jurisdiction::Federal);
        assert_eq!(framework.authority, "Test Authority");
        assert_eq!(framework.version, "1.0.0");
        assert!(framework.is_active());
    }

    #[test]
    fn test_normative_framework_is_active() {
        let mut framework = NormativeFramework::new(
            "Test".to_string(),
            "Test".to_string(),
            NormativeType::Framework,
            Jurisdiction::Federal,
            "Authority".to_string(),
        );

        // Active framework
        assert!(framework.is_active());

        // Inactive framework (status)
        framework.status = "inactive".to_string();
        assert!(!framework.is_active());

        // Reset status
        framework.status = "active".to_string();

        // Future effective date
        framework.effective_date = Utc::now() + chrono::Duration::days(1);
        assert!(!framework.is_active());

        // Past expiration date
        framework.effective_date = Utc::now() - chrono::Duration::days(2);
        framework.expiration_date = Some(Utc::now() - chrono::Duration::days(1));
        assert!(!framework.is_active());
    }

    #[test]
    fn test_normative_framework_add_requirement() {
        let mut framework = NormativeFramework::new(
            "Test".to_string(),
            "Test".to_string(),
            NormativeType::Framework,
            Jurisdiction::Federal,
            "Authority".to_string(),
        );

        let requirement = Requirement {
            id: Uuid::new_v4(),
            title: "Test Requirement".to_string(),
            description: "Test requirement description".to_string(),
            mandatory: true,
            priority: "high".to_string(),
            deadline: None,
            verification_method: "audit".to_string(),
            compliance_evidence: Vec::new(),
            status: "active".to_string(),
            metadata: HashMap::new(),
        };

        let initial_count = framework.requirements.len();
        framework.add_requirement(requirement);
        assert_eq!(framework.requirements.len(), initial_count + 1);
    }

    #[test]
    fn test_normative_framework_get_mandatory_requirements() {
        let mut framework = NormativeFramework::new(
            "Test".to_string(),
            "Test".to_string(),
            NormativeType::Framework,
            Jurisdiction::Federal,
            "Authority".to_string(),
        );

        let mandatory_req = Requirement {
            id: Uuid::new_v4(),
            title: "Mandatory Requirement".to_string(),
            description: "This is mandatory".to_string(),
            mandatory: true,
            priority: "high".to_string(),
            deadline: None,
            verification_method: "audit".to_string(),
            compliance_evidence: Vec::new(),
            status: "active".to_string(),
            metadata: HashMap::new(),
        };

        let optional_req = Requirement {
            id: Uuid::new_v4(),
            title: "Optional Requirement".to_string(),
            description: "This is optional".to_string(),
            mandatory: false,
            priority: "low".to_string(),
            deadline: None,
            verification_method: "review".to_string(),
            compliance_evidence: Vec::new(),
            status: "active".to_string(),
            metadata: HashMap::new(),
        };

        framework.add_requirement(mandatory_req);
        framework.add_requirement(optional_req);

        let mandatory_reqs = framework.get_mandatory_requirements();
        assert_eq!(mandatory_reqs.len(), 1);
        assert!(mandatory_reqs[0].mandatory);
    }

    #[test]
    fn test_business_rule_serialization() {
        let rule = BusinessRule {
            id: Uuid::new_v4(),
            name: "Test Rule".to_string(),
            description: "Test rule description".to_string(),
            rule_expression: "age > 18".to_string(),
            context: "user_validation".to_string(),
            priority: 1,
            active: true,
            version: "1.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: BusinessRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule.name, deserialized.name);
        assert_eq!(rule.rule_expression, deserialized.rule_expression);
        assert_eq!(rule.active, deserialized.active);
    }

    #[test]
    fn test_governance_context_creation() {
        let context = GovernanceContext {
            organization: "Test Org".to_string(),
            sector: "Technology".to_string(),
            region: "North America".to_string(),
            applicable_jurisdictions: vec![Jurisdiction::Federal, Jurisdiction::State],
            business_context: HashMap::new(),
            risk_profile: "Medium".to_string(),
            maturity_level: "Advanced".to_string(),
        };

        assert_eq!(context.organization, "Test Org");
        assert_eq!(context.sector, "Technology");
        assert_eq!(context.applicable_jurisdictions.len(), 2);
    }

    #[test]
    fn test_audit_trail_creation() {
        let trail = AuditTrail {
            id: Uuid::new_v4(),
            entity_type: "NormativeFramework".to_string(),
            entity_id: Uuid::new_v4().to_string(),
            action: "CREATE".to_string(),
            actor: "test_user".to_string(),
            timestamp: Utc::now(),
            details: HashMap::new(),
            previous_state: None,
            new_state: Some("active".to_string()),
        };

        assert_eq!(trail.entity_type, "NormativeFramework");
        assert_eq!(trail.action, "CREATE");
        assert_eq!(trail.actor, "test_user");
        assert!(trail.new_state.is_some());
    }
}