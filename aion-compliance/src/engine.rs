use aion_core::{
    AionResult, AionError, ComplianceEngine, ComplianceAssessment, ComplianceStatus,
    NormativeFramework, NormativeId, RequirementAssessment, Evidence, Finding, Recommendation,
    GovernanceContext, BusinessRuleEngine
};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

pub struct AdvancedComplianceEngine {
    business_rule_engine: Arc<dyn BusinessRuleEngine + Send + Sync>,
    assessment_cache: HashMap<String, ComplianceAssessment>,
    compliance_rules: Vec<ComplianceRule>,
    evidence_validators: HashMap<String, Box<dyn Fn(&Evidence) -> bool + Send + Sync>>,
}

impl AdvancedComplianceEngine {
    pub fn new(business_rule_engine: Arc<dyn BusinessRuleEngine + Send + Sync>) -> Self {
        let mut engine = Self {
            business_rule_engine,
            assessment_cache: HashMap::new(),
            compliance_rules: Vec::new(),
            evidence_validators: HashMap::new(),
        };

        engine.initialize_compliance_rules();
        engine.initialize_evidence_validators();
        engine
    }

    fn initialize_compliance_rules(&mut self) {
        self.compliance_rules.extend(vec![
            ComplianceRule {
                id: Uuid::new_v4(),
                name: "mandatory_requirement_evidence".to_string(),
                description: "Mandatory requirements must have sufficient evidence".to_string(),
                rule_type: ComplianceRuleType::EvidenceValidation,
                condition: "requirement.mandatory = TRUE".to_string(),
                action: ComplianceAction::RequireEvidence(2),
                severity: ComplianceRuleSeverity::High,
                applicable_frameworks: vec![],
            },
            ComplianceRule {
                id: Uuid::new_v4(),
                name: "evidence_freshness".to_string(),
                description: "Evidence must be collected within the last 12 months".to_string(),
                rule_type: ComplianceRuleType::TemporalValidation,
                condition: "evidence.collected_date >= NOW() - 12_MONTHS".to_string(),
                action: ComplianceAction::ValidateEvidence,
                severity: ComplianceRuleSeverity::Medium,
                applicable_frameworks: vec![],
            },
            ComplianceRule {
                id: Uuid::new_v4(),
                name: "critical_finding_escalation".to_string(),
                description: "Critical findings must be escalated immediately".to_string(),
                rule_type: ComplianceRuleType::FindingEscalation,
                condition: "finding.severity = 'critical'".to_string(),
                action: ComplianceAction::EscalateFinding,
                severity: ComplianceRuleSeverity::Critical,
                applicable_frameworks: vec![],
            },
            ComplianceRule {
                id: Uuid::new_v4(),
                name: "gdpr_breach_notification".to_string(),
                description: "GDPR breaches must be notified within 72 hours".to_string(),
                rule_type: ComplianceRuleType::FrameworkSpecific,
                condition: "framework.title CONTAINS 'GDPR' AND finding.type = 'data_breach'".to_string(),
                action: ComplianceAction::RequireImmediate,
                severity: ComplianceRuleSeverity::Critical,
                applicable_frameworks: vec!["gdpr".to_string()],
            },
            ComplianceRule {
                id: Uuid::new_v4(),
                name: "sox_quarterly_assessment".to_string(),
                description: "SOX controls must be assessed quarterly".to_string(),
                rule_type: ComplianceRuleType::FrameworkSpecific,
                condition: "framework.title CONTAINS 'SOX'".to_string(),
                action: ComplianceAction::RequireQuarterly,
                severity: ComplianceRuleSeverity::High,
                applicable_frameworks: vec!["sox".to_string()],
            },
        ]);
    }

    fn initialize_evidence_validators(&mut self) {
        self.evidence_validators.insert(
            "document".to_string(),
            Box::new(|evidence: &Evidence| {
                !evidence.description.trim().is_empty() &&
                evidence.verification_status == "verified" &&
                evidence.collected_date <= Utc::now()
            })
        );

        self.evidence_validators.insert(
            "certificate".to_string(),
            Box::new(|evidence: &Evidence| {
                evidence.metadata.contains_key("issuer") &&
                evidence.metadata.contains_key("expiry_date") &&
                evidence.verification_status == "verified"
            })
        );

        self.evidence_validators.insert(
            "assessment_report".to_string(),
            Box::new(|evidence: &Evidence| {
                evidence.metadata.contains_key("assessor") &&
                evidence.metadata.contains_key("assessment_date") &&
                evidence.description.len() >= 100
            })
        );

        self.evidence_validators.insert(
            "audit_log".to_string(),
            Box::new(|evidence: &Evidence| {
                evidence.metadata.contains_key("log_source") &&
                evidence.metadata.contains_key("time_range") &&
                evidence.verification_status != "rejected"
            })
        );

        self.evidence_validators.insert(
            "training_record".to_string(),
            Box::new(|evidence: &Evidence| {
                evidence.metadata.contains_key("trainer") &&
                evidence.metadata.contains_key("completion_date") &&
                evidence.metadata.contains_key("participant_count")
            })
        );
    }

    pub fn assess_compliance_comprehensive(
        &mut self,
        entity_id: &str,
        frameworks: &[NormativeFramework],
        context: &GovernanceContext,
    ) -> AionResult<Vec<ComplianceAssessment>> {
        let mut assessments = Vec::new();

        for framework in frameworks {
            let assessment = self.assess_framework_compliance(entity_id, framework, context)?;
            assessments.push(assessment);
        }

        self.apply_cross_framework_rules(&mut assessments)?;
        self.generate_consolidated_recommendations(&mut assessments)?;

        Ok(assessments)
    }

    fn assess_framework_compliance(
        &mut self,
        entity_id: &str,
        framework: &NormativeFramework,
        context: &GovernanceContext,
    ) -> AionResult<ComplianceAssessment> {
        let cache_key = format!("{}:{}", entity_id, framework.id.0);

        if let Some(cached_assessment) = self.assessment_cache.get(&cache_key) {
            if (Utc::now() - cached_assessment.assessment_date).num_hours() < 24 {
                return Ok(cached_assessment.clone());
            }
        }

        let mut requirement_assessments = Vec::new();
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();
        let mut overall_compliant = true;

        for requirement in &framework.requirements {
            let req_assessment = self.assess_requirement_advanced(entity_id, requirement, framework, context)?;

            if req_assessment.status != ComplianceStatus::Compliant && req_assessment.status != ComplianceStatus::NotApplicable {
                overall_compliant = false;

                if requirement.mandatory {
                    findings.push(Finding {
                        id: Uuid::new_v4(),
                        finding_type: "mandatory_non_compliance".to_string(),
                        severity: self.determine_finding_severity(&req_assessment),
                        title: format!("Non-compliance: {}", requirement.title),
                        description: format!("Mandatory requirement not met: {}", requirement.description),
                        affected_requirements: vec![requirement.id],
                        root_cause: Some(self.analyze_root_cause(&req_assessment)),
                        impact_assessment: self.assess_impact(&req_assessment, requirement, context),
                    });
                }
            }

            requirement_assessments.push(req_assessment);
        }

        self.apply_framework_specific_rules(framework, &mut findings, &mut recommendations)?;

        let assessment = ComplianceAssessment {
            id: Uuid::new_v4(),
            entity_id: entity_id.to_string(),
            normative_framework: framework.id.clone(),
            assessment_date: Utc::now(),
            assessor: "AION-CR Advanced Engine".to_string(),
            overall_status: self.determine_overall_status(&requirement_assessments),
            requirement_assessments,
            findings,
            recommendations,
            next_review_date: Some(self.calculate_next_review_date(framework)),
        };

        self.assessment_cache.insert(cache_key, assessment.clone());

        Ok(assessment)
    }

    fn assess_requirement_advanced(
        &self,
        entity_id: &str,
        requirement: &aion_core::Requirement,
        framework: &NormativeFramework,
        context: &GovernanceContext,
    ) -> AionResult<RequirementAssessment> {
        let mut gaps = Vec::new();
        let mut evidence = Vec::new();
        let mut status = ComplianceStatus::NotApplicable;

        if self.is_requirement_applicable(requirement, context)? {
            status = ComplianceStatus::NonCompliant;

            let evidence_collected = self.collect_evidence_for_requirement(entity_id, requirement)?;
            let evidence_valid = self.validate_evidence_collection(&evidence_collected)?;

            if evidence_valid {
                let validation_results = self.validate_requirement_conditions(requirement, context)?;

                if validation_results.iter().all(|&result| result) {
                    status = ComplianceStatus::Compliant;
                } else {
                    status = ComplianceStatus::PartiallyCompliant;
                    gaps.extend(self.identify_compliance_gaps(requirement, &validation_results)?);
                }

                evidence = evidence_collected;
            } else {
                gaps.push("Insufficient or invalid evidence provided".to_string());
            }

            let business_logic_violations = self.business_rule_engine.validate_business_logic(
                entity_id,
                &self.build_requirement_context(requirement, framework, context)
            )?;

            if !business_logic_violations.is_empty() {
                gaps.extend(business_logic_violations);
                if status == ComplianceStatus::Compliant {
                    status = ComplianceStatus::PartiallyCompliant;
                }
            }
        }

        let risk_level = self.assess_requirement_risk(requirement, &gaps, context);

        Ok(RequirementAssessment {
            requirement_id: requirement.id,
            status,
            evidence,
            gaps,
            notes: format!("Assessed for entity: {} in context: {}", entity_id, context.organization),
            risk_level,
        })
    }

    fn is_requirement_applicable(&self, requirement: &aion_core::Requirement, context: &GovernanceContext) -> AionResult<bool> {
        for condition in &requirement.conditions {
            if !self.evaluate_applicability_condition(condition, context)? {
                return Ok(false);
            }
        }

        for exception in &requirement.exceptions {
            if self.check_exception_applies(exception, context)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn evaluate_applicability_condition(&self, condition: &aion_core::Condition, context: &GovernanceContext) -> AionResult<bool> {
        if condition.expression.contains("sector") {
            if let Some(sector_requirement) = self.extract_sector_from_expression(&condition.expression) {
                return Ok(context.sector == sector_requirement);
            }
        }

        if condition.expression.contains("region") {
            if let Some(region_requirement) = self.extract_region_from_expression(&condition.expression) {
                return Ok(context.region == region_requirement);
            }
        }

        if condition.expression.contains("organization_size") {
            if let Some(size_requirement) = context.business_context.get("organization_size") {
                return Ok(self.evaluate_size_condition(&condition.expression, size_requirement));
            }
        }

        Ok(true)
    }

    fn check_exception_applies(&self, exception: &aion_core::Exception, context: &GovernanceContext) -> AionResult<bool> {
        if let Some(valid_until) = exception.valid_until {
            if Utc::now() > valid_until {
                return Ok(false);
            }
        }

        if exception.scope.contains(&context.organization) {
            return Ok(true);
        }

        if exception.scope.contains(&context.sector) {
            return Ok(true);
        }

        Ok(false)
    }

    fn collect_evidence_for_requirement(&self, entity_id: &str, requirement: &aion_core::Requirement) -> AionResult<Vec<Evidence>> {
        let mut evidence = Vec::new();

        for evidence_type in &requirement.evidence_required {
            let mock_evidence = Evidence {
                id: Uuid::new_v4(),
                evidence_type: evidence_type.clone(),
                description: format!("Evidence for requirement: {}", requirement.title),
                source: format!("Entity: {}", entity_id),
                collected_date: Utc::now(),
                verification_status: "pending".to_string(),
                metadata: HashMap::from([
                    ("requirement_id".to_string(), requirement.id.to_string()),
                    ("evidence_type".to_string(), evidence_type.clone()),
                ]),
            };

            evidence.push(mock_evidence);
        }

        Ok(evidence)
    }

    fn validate_evidence_collection(&self, evidence: &[Evidence]) -> AionResult<bool> {
        for evidence_item in evidence {
            if let Some(validator) = self.evidence_validators.get(&evidence_item.evidence_type) {
                if !validator(evidence_item) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn validate_requirement_conditions(&self, requirement: &aion_core::Requirement, context: &GovernanceContext) -> AionResult<Vec<bool>> {
        let mut results = Vec::new();

        for validation_rule in &requirement.validation_rules {
            let result = self.evaluate_validation_rule(validation_rule, context)?;
            results.push(result);
        }

        if results.is_empty() {
            results.push(true);
        }

        Ok(results)
    }

    fn evaluate_validation_rule(&self, rule: &aion_core::ValidationRule, _context: &GovernanceContext) -> AionResult<bool> {
        match rule.rule_type.as_str() {
            "presence" => Ok(true),
            "format" => Ok(true),
            "range" => Ok(true),
            "temporal" => Ok(true),
            _ => Ok(false),
        }
    }

    fn identify_compliance_gaps(&self, requirement: &aion_core::Requirement, validation_results: &[bool]) -> AionResult<Vec<String>> {
        let mut gaps = Vec::new();

        for (idx, &result) in validation_results.iter().enumerate() {
            if !result {
                if let Some(validation_rule) = requirement.validation_rules.get(idx) {
                    gaps.push(validation_rule.error_message.clone());
                } else {
                    gaps.push(format!("Validation failed for requirement: {}", requirement.title));
                }
            }
        }

        Ok(gaps)
    }

    fn build_requirement_context(&self, requirement: &aion_core::Requirement, framework: &NormativeFramework, context: &GovernanceContext) -> HashMap<String, String> {
        let mut req_context = HashMap::new();

        req_context.insert("requirement_id".to_string(), requirement.id.to_string());
        req_context.insert("requirement_title".to_string(), requirement.title.clone());
        req_context.insert("requirement_mandatory".to_string(), requirement.mandatory.to_string());
        req_context.insert("requirement_category".to_string(), requirement.category.clone());
        req_context.insert("framework_id".to_string(), framework.id.0.to_string());
        req_context.insert("framework_title".to_string(), framework.title.clone());
        req_context.insert("organization".to_string(), context.organization.clone());
        req_context.insert("sector".to_string(), context.sector.clone());
        req_context.insert("region".to_string(), context.region.clone());
        req_context.insert("context".to_string(), "requirement_validation".to_string());

        for (key, value) in &context.business_context {
            req_context.insert(format!("business_{}", key), value.clone());
        }

        req_context
    }

    fn assess_requirement_risk(&self, requirement: &aion_core::Requirement, gaps: &[String], context: &GovernanceContext) -> String {
        let mut risk_score = 0;

        if requirement.mandatory {
            risk_score += 3;
        }

        if requirement.priority <= 2 {
            risk_score += 2;
        }

        risk_score += gaps.len();

        if context.risk_profile == "high" {
            risk_score += 2;
        }

        match risk_score {
            0..=2 => "low".to_string(),
            3..=5 => "medium".to_string(),
            6..=8 => "high".to_string(),
            _ => "critical".to_string(),
        }
    }

    fn determine_finding_severity(&self, assessment: &RequirementAssessment) -> String {
        match assessment.risk_level.as_str() {
            "critical" => "critical".to_string(),
            "high" => "high".to_string(),
            "medium" => "medium".to_string(),
            _ => "low".to_string(),
        }
    }

    fn analyze_root_cause(&self, assessment: &RequirementAssessment) -> String {
        if assessment.gaps.is_empty() {
            "No specific gaps identified".to_string()
        } else if assessment.gaps.len() == 1 {
            assessment.gaps[0].clone()
        } else {
            "Multiple compliance gaps identified".to_string()
        }
    }

    fn assess_impact(&self, assessment: &RequirementAssessment, requirement: &aion_core::Requirement, context: &GovernanceContext) -> String {
        let mut impact_factors = Vec::new();

        if requirement.mandatory {
            impact_factors.push("Mandatory requirement non-compliance");
        }

        if assessment.risk_level == "critical" || assessment.risk_level == "high" {
            impact_factors.push("High operational risk");
        }

        if context.risk_profile == "high" {
            impact_factors.push("High-risk organizational profile");
        }

        if requirement.category.contains("security") {
            impact_factors.push("Security implications");
        }

        if requirement.category.contains("privacy") {
            impact_factors.push("Privacy implications");
        }

        if impact_factors.is_empty() {
            "Low impact".to_string()
        } else {
            format!("Impact: {}", impact_factors.join(", "))
        }
    }

    fn apply_framework_specific_rules(&self, framework: &NormativeFramework, findings: &mut Vec<Finding>, recommendations: &mut Vec<Recommendation>) -> AionResult<()> {
        for rule in &self.compliance_rules {
            if rule.rule_type == ComplianceRuleType::FrameworkSpecific {
                if rule.applicable_frameworks.is_empty() ||
                   rule.applicable_frameworks.iter().any(|f| framework.title.to_lowercase().contains(f)) {

                    match &rule.action {
                        ComplianceAction::RequireImmediate => {
                            recommendations.push(Recommendation {
                                id: Uuid::new_v4(),
                                title: format!("Immediate Action Required: {}", rule.name),
                                description: rule.description.clone(),
                                priority: "critical".to_string(),
                                effort_estimate: Some("immediate".to_string()),
                                timeline: Some("24 hours".to_string()),
                                responsible_party: Some("Compliance Team".to_string()),
                                related_findings: findings.iter().map(|f| f.id).collect(),
                            });
                        },
                        ComplianceAction::RequireQuarterly => {
                            recommendations.push(Recommendation {
                                id: Uuid::new_v4(),
                                title: format!("Quarterly Review Required: {}", rule.name),
                                description: rule.description.clone(),
                                priority: "high".to_string(),
                                effort_estimate: Some("medium".to_string()),
                                timeline: Some("quarterly".to_string()),
                                responsible_party: Some("Compliance Team".to_string()),
                                related_findings: Vec::new(),
                            });
                        },
                        _ => {},
                    }
                }
            }
        }

        Ok(())
    }

    fn apply_cross_framework_rules(&self, assessments: &mut [ComplianceAssessment]) -> AionResult<()> {
        let total_frameworks = assessments.len();
        let compliant_frameworks = assessments.iter().filter(|a| a.overall_status == ComplianceStatus::Compliant).count();

        if total_frameworks > 1 && compliant_frameworks < total_frameworks {
            for assessment in assessments.iter_mut() {
                if assessment.overall_status != ComplianceStatus::Compliant {
                    assessment.recommendations.push(Recommendation {
                        id: Uuid::new_v4(),
                        title: "Cross-Framework Compliance Gap".to_string(),
                        description: "Multiple frameworks have compliance issues that may compound risk".to_string(),
                        priority: "high".to_string(),
                        effort_estimate: Some("high".to_string()),
                        timeline: Some("30 days".to_string()),
                        responsible_party: Some("Chief Compliance Officer".to_string()),
                        related_findings: assessment.findings.iter().map(|f| f.id).collect(),
                    });
                }
            }
        }

        Ok(())
    }

    fn generate_consolidated_recommendations(&self, assessments: &mut [ComplianceAssessment]) -> AionResult<()> {
        let mut all_finding_ids: Vec<uuid::Uuid> = assessments.iter().flat_map(|a| a.findings.iter().map(|f| f.id)).collect();
        let finding_count = all_finding_ids.len();

        if finding_count > 5 {
            for assessment in assessments.iter_mut() {
                assessment.recommendations.push(Recommendation {
                    id: Uuid::new_v4(),
                    title: "Comprehensive Compliance Remediation Program".to_string(),
                    description: "Multiple compliance findings require a coordinated remediation approach".to_string(),
                    priority: "high".to_string(),
                    effort_estimate: Some("high".to_string()),
                    timeline: Some("90 days".to_string()),
                    responsible_party: Some("Executive Leadership".to_string()),
                    related_findings: all_finding_ids.clone(),
                });
            }
        }

        Ok(())
    }

    fn determine_overall_status(&self, requirement_assessments: &[RequirementAssessment]) -> ComplianceStatus {
        let mandatory_assessments: Vec<_> = requirement_assessments.iter()
            .filter(|a| a.status != ComplianceStatus::NotApplicable)
            .collect();

        if mandatory_assessments.is_empty() {
            return ComplianceStatus::NotApplicable;
        }

        let compliant_count = mandatory_assessments.iter()
            .filter(|a| a.status == ComplianceStatus::Compliant)
            .count();

        let non_compliant_count = mandatory_assessments.iter()
            .filter(|a| a.status == ComplianceStatus::NonCompliant)
            .count();

        if compliant_count == mandatory_assessments.len() {
            ComplianceStatus::Compliant
        } else if non_compliant_count > 0 {
            ComplianceStatus::NonCompliant
        } else {
            ComplianceStatus::PartiallyCompliant
        }
    }

    fn calculate_next_review_date(&self, framework: &NormativeFramework) -> chrono::DateTime<Utc> {
        let review_interval = if framework.metadata.get("review_frequency").is_some() {
            90 // Quarterly
        } else {
            match framework.jurisdiction {
                aion_core::Jurisdiction::International => 180, // Semi-annual
                aion_core::Jurisdiction::Federal => 90,        // Quarterly
                _ => 365,                                      // Annual
            }
        };

        Utc::now() + chrono::Duration::days(review_interval)
    }

    fn extract_sector_from_expression(&self, expression: &str) -> Option<String> {
        if let Some(start) = expression.find("sector = '") {
            let start = start + 10;
            if let Some(end) = expression[start..].find('\'') {
                return Some(expression[start..start + end].to_string());
            }
        }
        None
    }

    fn extract_region_from_expression(&self, expression: &str) -> Option<String> {
        if let Some(start) = expression.find("region = '") {
            let start = start + 10;
            if let Some(end) = expression[start..].find('\'') {
                return Some(expression[start..start + end].to_string());
            }
        }
        None
    }

    fn evaluate_size_condition(&self, expression: &str, organization_size: &str) -> bool {
        if expression.contains("large") {
            organization_size == "large"
        } else if expression.contains("medium") {
            organization_size == "medium"
        } else if expression.contains("small") {
            organization_size == "small"
        } else {
            true
        }
    }
}

impl ComplianceEngine for AdvancedComplianceEngine {
    fn assess_compliance(&self, entity_id: &str, framework_ids: &[NormativeId]) -> AionResult<ComplianceAssessment> {
        Err(AionError::InternalError {
            message: "Use assess_compliance_comprehensive for advanced functionality".to_string(),
        })
    }

    fn validate_requirements(&self, entity_id: &str, requirement_ids: &[uuid::Uuid]) -> AionResult<Vec<bool>> {
        let mut results = Vec::new();

        for _requirement_id in requirement_ids {
            results.push(true);
        }

        Ok(results)
    }

    fn generate_compliance_report(&self, assessment: &ComplianceAssessment) -> AionResult<String> {
        let mut report = String::new();

        report.push_str("ADVANCED COMPLIANCE ASSESSMENT REPORT\n");
        report.push_str("=====================================\n\n");

        report.push_str(&format!("Entity: {}\n", assessment.entity_id));
        report.push_str(&format!("Framework: {}\n", assessment.normative_framework.0));
        report.push_str(&format!("Assessment Date: {}\n", assessment.assessment_date.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("Assessor: {}\n", assessment.assessor));
        report.push_str(&format!("Overall Status: {:?}\n\n", assessment.overall_status));

        report.push_str("EXECUTIVE SUMMARY\n");
        report.push_str("-----------------\n");
        let compliant_count = assessment.requirement_assessments.iter()
            .filter(|a| a.status == ComplianceStatus::Compliant)
            .count();
        let total_count = assessment.requirement_assessments.len();
        let compliance_percentage = if total_count > 0 { (compliant_count * 100) / total_count } else { 0 };

        report.push_str(&format!("Compliance Rate: {}% ({}/{})\n", compliance_percentage, compliant_count, total_count));
        report.push_str(&format!("Critical Findings: {}\n", assessment.findings.iter().filter(|f| f.severity == "critical").count()));
        report.push_str(&format!("High Priority Recommendations: {}\n\n", assessment.recommendations.iter().filter(|r| r.priority == "high" || r.priority == "critical").count()));

        if !assessment.findings.is_empty() {
            report.push_str("FINDINGS\n");
            report.push_str("--------\n");
            for finding in &assessment.findings {
                report.push_str(&format!("• [{}] {}\n", finding.severity.to_uppercase(), finding.title));
                report.push_str(&format!("  Description: {}\n", finding.description));
                if let Some(root_cause) = &finding.root_cause {
                    report.push_str(&format!("  Root Cause: {}\n", root_cause));
                }
                report.push_str(&format!("  Impact: {}\n\n", finding.impact_assessment));
            }
        }

        if !assessment.recommendations.is_empty() {
            report.push_str("RECOMMENDATIONS\n");
            report.push_str("---------------\n");
            for recommendation in &assessment.recommendations {
                report.push_str(&format!("• [{}] {}\n", recommendation.priority.to_uppercase(), recommendation.title));
                report.push_str(&format!("  Description: {}\n", recommendation.description));
                if let Some(timeline) = &recommendation.timeline {
                    report.push_str(&format!("  Timeline: {}\n", timeline));
                }
                if let Some(responsible_party) = &recommendation.responsible_party {
                    report.push_str(&format!("  Responsible: {}\n", responsible_party));
                }
                report.push_str("\n");
            }
        }

        if let Some(next_review) = assessment.next_review_date {
            report.push_str(&format!("Next Review Date: {}\n", next_review.format("%Y-%m-%d")));
        }

        Ok(report)
    }
}

#[derive(Debug, Clone)]
pub struct ComplianceRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_type: ComplianceRuleType,
    pub condition: String,
    pub action: ComplianceAction,
    pub severity: ComplianceRuleSeverity,
    pub applicable_frameworks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceRuleType {
    EvidenceValidation,
    TemporalValidation,
    FindingEscalation,
    FrameworkSpecific,
}

#[derive(Debug, Clone)]
pub enum ComplianceAction {
    RequireEvidence(usize),
    ValidateEvidence,
    EscalateFinding,
    RequireImmediate,
    RequireQuarterly,
}

#[derive(Debug, Clone)]
pub enum ComplianceRuleSeverity {
    Critical,
    High,
    Medium,
    Low,
}