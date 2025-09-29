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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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