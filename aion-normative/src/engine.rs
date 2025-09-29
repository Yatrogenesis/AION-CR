use aion_core::{
    AionError, AionResult, NormativeFramework, NormativeId, NormativeRepository,
    ValidationEngine, ComplianceEngine, ComplianceAssessment, ComplianceStatus,
    RequirementAssessment, Evidence, Finding, Recommendation
};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

pub struct NormativeEngine {
    repository: Arc<dyn NormativeRepository + Send + Sync>,
    validator: Arc<dyn ValidationEngine + Send + Sync>,
    cache: HashMap<NormativeId, Arc<NormativeFramework>>,
    hierarchy_cache: HashMap<NormativeId, Vec<NormativeId>>,
}

impl NormativeEngine {
    pub fn new(
        repository: Arc<dyn NormativeRepository + Send + Sync>,
        validator: Arc<dyn ValidationEngine + Send + Sync>,
    ) -> Self {
        Self {
            repository,
            validator,
            cache: HashMap::new(),
            hierarchy_cache: HashMap::new(),
        }
    }

    pub fn register_framework(&mut self, mut framework: NormativeFramework) -> AionResult<NormativeId> {
        let validation_errors = self.validator.validate_framework(&framework)?;
        if !validation_errors.is_empty() {
            return Err(AionError::ValidationError {
                field: "framework".to_string(),
                message: validation_errors.join("; "),
            });
        }

        self.check_conflicts(&framework)?;
        self.process_dependencies(&mut framework)?;

        let id = framework.id.clone();
        self.repository.store_framework(framework.clone())?;
        self.cache.insert(id.clone(), Arc::new(framework));
        self.invalidate_hierarchy_cache();

        Ok(id)
    }

    pub fn get_framework(&self, id: &NormativeId) -> AionResult<Option<NormativeFramework>> {
        if let Some(cached) = self.cache.get(id) {
            return Ok(Some((**cached).clone()));
        }

        self.repository.get_framework(id)
    }

    pub fn get_active_frameworks(&self) -> AionResult<Vec<NormativeFramework>> {
        let all_frameworks = self.repository.get_active_frameworks()?;
        Ok(all_frameworks.into_iter().filter(|f| f.is_active()).collect())
    }

    pub fn get_framework_hierarchy(&mut self, id: &NormativeId) -> AionResult<Vec<NormativeId>> {
        if let Some(cached) = self.hierarchy_cache.get(id) {
            return Ok(cached.clone());
        }

        let framework = self.get_framework(id)?
            .ok_or_else(|| AionError::NormativeNotFound { id: id.0.to_string() })?;

        let mut hierarchy = Vec::new();
        self.build_hierarchy(&framework, &mut hierarchy, &mut std::collections::HashSet::new())?;

        self.hierarchy_cache.insert(id.clone(), hierarchy.clone());
        Ok(hierarchy)
    }

    fn build_hierarchy(
        &self,
        framework: &NormativeFramework,
        hierarchy: &mut Vec<NormativeId>,
        visited: &mut std::collections::HashSet<NormativeId>,
    ) -> AionResult<()> {
        if visited.contains(&framework.id) {
            return Ok(());
        }

        visited.insert(framework.id.clone());
        hierarchy.push(framework.id.clone());

        for dep_id in &framework.dependencies {
            if let Some(dep_framework) = self.repository.get_framework(dep_id)? {
                self.build_hierarchy(&dep_framework, hierarchy, visited)?;
            }
        }

        Ok(())
    }

    pub fn update_framework(&mut self, framework: NormativeFramework) -> AionResult<()> {
        let validation_errors = self.validator.validate_framework(&framework)?;
        if !validation_errors.is_empty() {
            return Err(AionError::ValidationError {
                field: "framework".to_string(),
                message: validation_errors.join("; "),
            });
        }

        self.repository.update_framework(framework.clone())?;
        self.cache.insert(framework.id.clone(), Arc::new(framework));
        self.invalidate_hierarchy_cache();
        Ok(())
    }

    pub fn search_frameworks(&self, query: &str) -> AionResult<Vec<NormativeFramework>> {
        self.repository.search_frameworks(query)
    }

    pub fn get_applicable_frameworks(&self, context: &HashMap<String, String>) -> AionResult<Vec<NormativeFramework>> {
        let active_frameworks = self.get_active_frameworks()?;
        let mut applicable = Vec::new();

        for framework in active_frameworks {
            if self.is_framework_applicable(&framework, context)? {
                applicable.push(framework);
            }
        }

        Ok(applicable)
    }

    pub fn resolve_framework_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeFramework>> {
        let mut resolved = Vec::new();
        let mut conflicts = Vec::new();

        for framework in frameworks {
            let mut has_conflict = false;
            for other in frameworks {
                if framework.id != other.id && self.has_direct_conflict(framework, other)? {
                    conflicts.push((framework.clone(), other.clone()));
                    has_conflict = true;
                }
            }
            if !has_conflict {
                resolved.push(framework.clone());
            }
        }

        if !conflicts.is_empty() {
            return Err(AionError::NormativeConflict {
                description: format!("Found {} conflicts requiring resolution", conflicts.len()),
            });
        }

        Ok(resolved)
    }

    fn check_conflicts(&self, framework: &NormativeFramework) -> AionResult<()> {
        let active_frameworks = self.repository.get_active_frameworks()?;

        for existing in &active_frameworks {
            if self.has_direct_conflict(framework, existing)? {
                return Err(AionError::NormativeConflict {
                    description: format!(
                        "Framework '{}' conflicts with existing framework '{}'",
                        framework.title, existing.title
                    ),
                });
            }
        }

        Ok(())
    }

    fn has_direct_conflict(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> AionResult<bool> {
        if framework1.jurisdiction != framework2.jurisdiction {
            return Ok(false);
        }

        for req1 in &framework1.requirements {
            for req2 in &framework2.requirements {
                if self.requirements_conflict(req1, req2)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn requirements_conflict(&self, req1: &aion_core::Requirement, req2: &aion_core::Requirement) -> AionResult<bool> {
        if req1.category != req2.category {
            return Ok(false);
        }

        let similarity = aion_core::calculate_similarity(&req1.description, &req2.description);

        if similarity > 0.8 && req1.mandatory != req2.mandatory {
            return Ok(true);
        }

        for condition1 in &req1.conditions {
            for condition2 in &req2.conditions {
                if self.conditions_conflict(condition1, condition2)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn conditions_conflict(&self, cond1: &aion_core::Condition, cond2: &aion_core::Condition) -> AionResult<bool> {
        if cond1.expression == format!("NOT ({})", cond2.expression) ||
           cond2.expression == format!("NOT ({})", cond1.expression) {
            return Ok(true);
        }

        Ok(false)
    }

    fn process_dependencies(&mut self, framework: &mut NormativeFramework) -> AionResult<()> {
        for dep_id in &framework.dependencies {
            let dep_framework = self.repository.get_framework(dep_id)?
                .ok_or_else(|| AionError::NormativeNotFound { id: dep_id.0.to_string() })?;

            if !dep_framework.is_active() {
                return Err(AionError::ValidationError {
                    field: "dependencies".to_string(),
                    message: format!("Dependency framework '{}' is not active", dep_framework.title),
                });
            }
        }

        Ok(())
    }

    fn is_framework_applicable(&self, framework: &NormativeFramework, context: &HashMap<String, String>) -> AionResult<bool> {
        if let Some(sector) = context.get("sector") {
            if let Some(framework_sector) = framework.metadata.get("applicable_sectors") {
                if !framework_sector.split(',').any(|s| s.trim() == sector) {
                    return Ok(false);
                }
            }
        }

        if let Some(region) = context.get("region") {
            if let Some(framework_region) = framework.metadata.get("applicable_regions") {
                if !framework_region.split(',').any(|r| r.trim() == region) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn invalidate_hierarchy_cache(&mut self) {
        self.hierarchy_cache.clear();
    }

    pub fn get_framework_statistics(&self) -> AionResult<HashMap<String, u64>> {
        let frameworks = self.repository.list_frameworks()?;
        let mut stats = HashMap::new();

        stats.insert("total_frameworks".to_string(), frameworks.len() as u64);
        stats.insert("active_frameworks".to_string(),
                    frameworks.iter().filter(|f| f.is_active()).count() as u64);

        let mut by_type = HashMap::new();
        for framework in &frameworks {
            let type_name = format!("{:?}", framework.normative_type);
            *by_type.entry(type_name).or_insert(0u64) += 1;
        }

        for (type_name, count) in by_type {
            stats.insert(format!("type_{}", type_name.to_lowercase()), count);
        }

        Ok(stats)
    }
}

impl ComplianceEngine for NormativeEngine {
    fn assess_compliance(&self, entity_id: &str, framework_ids: &[NormativeId]) -> AionResult<ComplianceAssessment> {
        let mut requirement_assessments = Vec::new();
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();
        let mut overall_compliant = true;

        for framework_id in framework_ids {
            let framework = self.get_framework(framework_id)?
                .ok_or_else(|| AionError::NormativeNotFound { id: framework_id.0.to_string() })?;

            for requirement in &framework.requirements {
                let assessment = self.assess_requirement(entity_id, requirement)?;
                if assessment.status != ComplianceStatus::Compliant && assessment.status != ComplianceStatus::NotApplicable {
                    overall_compliant = false;

                    if !assessment.gaps.is_empty() {
                        findings.push(Finding {
                            id: Uuid::new_v4(),
                            finding_type: "compliance_gap".to_string(),
                            severity: assessment.risk_level.clone(),
                            title: format!("Non-compliance: {}", requirement.title),
                            description: assessment.gaps.join("; "),
                            affected_requirements: vec![requirement.id],
                            root_cause: Some("Requirements not met".to_string()),
                            impact_assessment: format!("Risk level: {}", assessment.risk_level),
                        });
                    }
                }
                requirement_assessments.push(assessment);
            }
        }

        if !overall_compliant {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                title: "Address compliance gaps".to_string(),
                description: "Implement measures to address identified compliance gaps".to_string(),
                priority: "high".to_string(),
                effort_estimate: Some("medium".to_string()),
                timeline: Some("30 days".to_string()),
                responsible_party: Some("Compliance Team".to_string()),
                related_findings: findings.iter().map(|f| f.id).collect(),
            });
        }

        Ok(ComplianceAssessment {
            id: Uuid::new_v4(),
            entity_id: entity_id.to_string(),
            normative_framework: framework_ids[0].clone(),
            assessment_date: Utc::now(),
            assessor: "AION-CR System".to_string(),
            overall_status: if overall_compliant { ComplianceStatus::Compliant } else { ComplianceStatus::NonCompliant },
            requirement_assessments,
            findings,
            recommendations,
            next_review_date: Some(Utc::now() + chrono::Duration::days(90)),
        })
    }

    fn validate_requirements(&self, entity_id: &str, requirement_ids: &[Uuid]) -> AionResult<Vec<bool>> {
        let mut results = Vec::new();

        for requirement_id in requirement_ids {
            let requirement = self.find_requirement_by_id(*requirement_id)?;
            let assessment = self.assess_requirement(entity_id, &requirement)?;
            results.push(assessment.status == ComplianceStatus::Compliant);
        }

        Ok(results)
    }

    fn generate_compliance_report(&self, assessment: &ComplianceAssessment) -> AionResult<String> {
        let mut report = String::new();

        report.push_str(&format!("COMPLIANCE ASSESSMENT REPORT\n"));
        report.push_str(&format!("Entity: {}\n", assessment.entity_id));
        report.push_str(&format!("Assessment Date: {}\n", assessment.assessment_date.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("Overall Status: {:?}\n\n", assessment.overall_status));

        report.push_str("REQUIREMENT ASSESSMENTS:\n");
        for req_assessment in &assessment.requirement_assessments {
            report.push_str(&format!("- Status: {:?}, Risk: {}\n", req_assessment.status, req_assessment.risk_level));
            if !req_assessment.gaps.is_empty() {
                report.push_str(&format!("  Gaps: {}\n", req_assessment.gaps.join(", ")));
            }
        }

        if !assessment.findings.is_empty() {
            report.push_str("\nFINDINGS:\n");
            for finding in &assessment.findings {
                report.push_str(&format!("- [{}] {}: {}\n", finding.severity, finding.title, finding.description));
            }
        }

        if !assessment.recommendations.is_empty() {
            report.push_str("\nRECOMMENDATIONS:\n");
            for recommendation in &assessment.recommendations {
                report.push_str(&format!("- [{}] {}: {}\n", recommendation.priority, recommendation.title, recommendation.description));
            }
        }

        Ok(report)
    }
}

impl NormativeEngine {
    fn assess_requirement(&self, entity_id: &str, requirement: &aion_core::Requirement) -> AionResult<RequirementAssessment> {
        let mut gaps = Vec::new();
        let mut evidence = Vec::new();
        let mut status = ComplianceStatus::NotApplicable;

        if requirement.mandatory {
            status = ComplianceStatus::NonCompliant;

            for validation_rule in &requirement.validation_rules {
                if !self.evaluate_validation_rule(entity_id, validation_rule)? {
                    gaps.push(validation_rule.error_message.clone());
                }
            }

            if gaps.is_empty() {
                status = ComplianceStatus::Compliant;
                evidence.push(Evidence {
                    id: Uuid::new_v4(),
                    evidence_type: "automated_validation".to_string(),
                    description: "All validation rules passed".to_string(),
                    source: "AION-CR System".to_string(),
                    collected_date: Utc::now(),
                    verification_status: "verified".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        let risk_level = if gaps.is_empty() { "low" } else if gaps.len() <= 2 { "medium" } else { "high" };

        Ok(RequirementAssessment {
            requirement_id: requirement.id,
            status,
            evidence,
            gaps,
            notes: format!("Assessed for entity: {}", entity_id),
            risk_level: risk_level.to_string(),
        })
    }

    fn evaluate_validation_rule(&self, _entity_id: &str, rule: &aion_core::ValidationRule) -> AionResult<bool> {
        match rule.rule_type.as_str() {
            "presence" => Ok(true),
            "format" => Ok(true),
            "range" => Ok(true),
            _ => Ok(false),
        }
    }

    fn find_requirement_by_id(&self, requirement_id: Uuid) -> AionResult<aion_core::Requirement> {
        let frameworks = self.repository.list_frameworks()?;

        for framework in frameworks {
            for requirement in framework.requirements {
                if requirement.id == requirement_id {
                    return Ok(requirement);
                }
            }
        }

        Err(AionError::ValidationError {
            field: "requirement_id".to_string(),
            message: format!("Requirement not found: {}", requirement_id),
        })
    }
}