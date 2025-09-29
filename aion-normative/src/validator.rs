use aion_core::{
    AionError, AionResult, NormativeFramework, Requirement, Evidence, ValidationEngine,
    BusinessRuleEngine, BusinessRule
};
use std::collections::HashMap;
use regex::Regex;
use chrono::Utc;

pub struct ComprehensiveValidator {
    business_rules: Vec<BusinessRule>,
    validation_cache: HashMap<String, Vec<String>>,
    custom_validators: HashMap<String, Box<dyn Fn(&str) -> bool + Send + Sync>>,
}

impl ComprehensiveValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            business_rules: Vec::new(),
            validation_cache: HashMap::new(),
            custom_validators: HashMap::new(),
        };

        validator.initialize_default_business_rules();
        validator.initialize_custom_validators();
        validator
    }

    fn initialize_default_business_rules(&mut self) {
        let now = Utc::now();

        self.business_rules.extend(vec![
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "framework_title_length".to_string(),
                description: "Framework title must be between 10 and 200 characters".to_string(),
                rule_expression: "title.length >= 10 AND title.length <= 200".to_string(),
                context: "framework_validation".to_string(),
                priority: 1,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "framework_description_required".to_string(),
                description: "Framework description is mandatory and must be meaningful".to_string(),
                rule_expression: "description.length >= 50 AND NOT description.contains('lorem ipsum')".to_string(),
                context: "framework_validation".to_string(),
                priority: 1,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "framework_authority_valid".to_string(),
                description: "Framework authority must be a recognized entity".to_string(),
                rule_expression: "authority.length > 0 AND NOT authority.contains('unknown')".to_string(),
                context: "framework_validation".to_string(),
                priority: 2,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "effective_date_valid".to_string(),
                description: "Effective date cannot be more than 5 years in the future".to_string(),
                rule_expression: "effective_date <= NOW() + 5_YEARS".to_string(),
                context: "framework_validation".to_string(),
                priority: 2,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "expiration_after_effective".to_string(),
                description: "Expiration date must be after effective date".to_string(),
                rule_expression: "expiration_date IS NULL OR expiration_date > effective_date".to_string(),
                context: "framework_validation".to_string(),
                priority: 1,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "version_format_valid".to_string(),
                description: "Version must follow semantic versioning".to_string(),
                rule_expression: "version MATCHES '^\\d+\\.\\d+\\.\\d+$'".to_string(),
                context: "framework_validation".to_string(),
                priority: 3,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "minimum_requirements".to_string(),
                description: "Framework must have at least one requirement".to_string(),
                rule_expression: "requirements.count >= 1".to_string(),
                context: "framework_validation".to_string(),
                priority: 1,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "requirement_title_unique".to_string(),
                description: "Requirement titles must be unique within framework".to_string(),
                rule_expression: "requirements.titles.all_unique()".to_string(),
                context: "requirement_validation".to_string(),
                priority: 2,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "mandatory_requirement_evidence".to_string(),
                description: "Mandatory requirements must specify required evidence".to_string(),
                rule_expression: "mandatory = TRUE IMPLIES evidence_required.count > 0".to_string(),
                context: "requirement_validation".to_string(),
                priority: 2,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
            BusinessRule {
                id: uuid::Uuid::new_v4(),
                name: "validation_rule_syntax".to_string(),
                description: "Validation rules must have valid syntax".to_string(),
                rule_expression: "validation_rules.all(rule => rule.expression.is_valid_syntax())".to_string(),
                context: "requirement_validation".to_string(),
                priority: 1,
                active: true,
                version: "1.0.0".to_string(),
                created_at: now,
                updated_at: now,
            },
        ]);
    }

    fn initialize_custom_validators(&mut self) {
        self.custom_validators.insert(
            "email".to_string(),
            Box::new(|value: &str| {
                let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
                email_regex.is_match(value)
            })
        );

        self.custom_validators.insert(
            "url".to_string(),
            Box::new(|value: &str| {
                let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
                url_regex.is_match(value)
            })
        );

        self.custom_validators.insert(
            "phone".to_string(),
            Box::new(|value: &str| {
                let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
                phone_regex.is_match(value)
            })
        );

        self.custom_validators.insert(
            "iso_date".to_string(),
            Box::new(|value: &str| {
                chrono::DateTime::parse_from_rfc3339(value).is_ok()
            })
        );

        self.custom_validators.insert(
            "uuid".to_string(),
            Box::new(|value: &str| {
                uuid::Uuid::parse_str(value).is_ok()
            })
        );

        self.custom_validators.insert(
            "semantic_version".to_string(),
            Box::new(|value: &str| {
                let semver_regex = Regex::new(r"^\d+\.\d+\.\d+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$").unwrap();
                semver_regex.is_match(value)
            })
        );
    }

    pub fn add_custom_validator<F>(&mut self, name: String, validator: F)
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.custom_validators.insert(name, Box::new(validator));
    }

    pub fn add_business_rule(&mut self, rule: BusinessRule) {
        self.business_rules.push(rule);
    }

    pub fn validate_framework_comprehensive(&self, framework: &NormativeFramework) -> AionResult<ValidationReport> {
        let mut report = ValidationReport::new();

        let cache_key = format!("framework_{}", framework.id.0);
        if let Some(cached_errors) = self.validation_cache.get(&cache_key) {
            report.errors.extend(cached_errors.clone());
            return Ok(report);
        }

        self.validate_framework_structure(framework, &mut report)?;
        self.validate_framework_business_rules(framework, &mut report)?;
        self.validate_requirements_comprehensive(framework, &mut report)?;
        self.validate_framework_consistency(framework, &mut report)?;
        self.validate_framework_completeness(framework, &mut report)?;

        report.overall_valid = report.errors.is_empty();
        report.confidence_score = self.calculate_validation_confidence(&report);

        Ok(report)
    }

    fn validate_framework_structure(&self, framework: &NormativeFramework, report: &mut ValidationReport) -> AionResult<()> {
        if framework.title.trim().is_empty() {
            report.errors.push("Framework title cannot be empty".to_string());
        } else if framework.title.len() < 10 {
            report.errors.push("Framework title too short (minimum 10 characters)".to_string());
        } else if framework.title.len() > 200 {
            report.errors.push("Framework title too long (maximum 200 characters)".to_string());
        }

        if framework.description.trim().is_empty() {
            report.errors.push("Framework description cannot be empty".to_string());
        } else if framework.description.len() < 50 {
            report.errors.push("Framework description too short (minimum 50 characters)".to_string());
        }

        if framework.authority.trim().is_empty() {
            report.errors.push("Framework authority cannot be empty".to_string());
        }

        if !aion_core::validate_version(&framework.version) {
            report.errors.push(format!("Invalid version format: {}", framework.version));
        }

        let now = Utc::now();
        let max_future = now + chrono::Duration::days(5 * 365);
        if framework.effective_date > max_future {
            report.errors.push("Effective date cannot be more than 5 years in the future".to_string());
        }

        if let Some(expiration) = framework.expiration_date {
            if expiration <= framework.effective_date {
                report.errors.push("Expiration date must be after effective date".to_string());
            }
        }

        if framework.requirements.is_empty() {
            report.warnings.push("Framework has no requirements".to_string());
        }

        Ok(())
    }

    fn validate_framework_business_rules(&self, framework: &NormativeFramework, report: &mut ValidationReport) -> AionResult<()> {
        let context = self.build_framework_context(framework);

        for rule in &self.business_rules {
            if !rule.active || rule.context != "framework_validation" {
                continue;
            }

            match self.evaluate_business_rule(rule, &context) {
                Ok(true) => {
                    report.validations_passed += 1;
                },
                Ok(false) => {
                    let message = format!("Business rule violation: {}", rule.description);
                    if rule.priority <= 2 {
                        report.errors.push(message);
                    } else {
                        report.warnings.push(message);
                    }
                },
                Err(e) => {
                    report.warnings.push(format!("Could not evaluate business rule '{}': {}", rule.name, e));
                }
            }
        }

        Ok(())
    }

    fn validate_requirements_comprehensive(&self, framework: &NormativeFramework, report: &mut ValidationReport) -> AionResult<()> {
        let mut requirement_titles = std::collections::HashSet::new();

        for requirement in &framework.requirements {
            let req_report = self.validate_requirement_detailed(requirement)?;
            report.merge_requirement_report(req_report);

            if !requirement_titles.insert(&requirement.title) {
                report.errors.push(format!("Duplicate requirement title: {}", requirement.title));
            }

            if requirement.mandatory && requirement.evidence_required.is_empty() {
                report.warnings.push(format!("Mandatory requirement '{}' has no specified evidence requirements", requirement.title));
            }

            for validation_rule in &requirement.validation_rules {
                if let Err(e) = self.validate_validation_rule_syntax(&validation_rule.expression) {
                    report.errors.push(format!("Invalid validation rule syntax in '{}': {}", requirement.title, e));
                }
            }
        }

        Ok(())
    }

    fn validate_requirement_detailed(&self, requirement: &Requirement) -> AionResult<RequirementValidationReport> {
        let mut report = RequirementValidationReport::new(requirement.id);

        if requirement.title.trim().is_empty() {
            report.errors.push("Requirement title cannot be empty".to_string());
        }

        if requirement.description.trim().is_empty() {
            report.errors.push("Requirement description cannot be empty".to_string());
        }

        if requirement.category.trim().is_empty() {
            report.errors.push("Requirement category cannot be empty".to_string());
        }

        if requirement.priority == 0 {
            report.warnings.push("Requirement priority not set".to_string());
        } else if requirement.priority > 10 {
            report.warnings.push("Requirement priority unusually high".to_string());
        }

        for condition in &requirement.conditions {
            if condition.expression.trim().is_empty() {
                report.errors.push("Condition expression cannot be empty".to_string());
            }
        }

        for exception in &requirement.exceptions {
            if exception.description.trim().is_empty() {
                report.errors.push("Exception description cannot be empty".to_string());
            }

            if let Some(valid_until) = exception.valid_until {
                if valid_until <= Utc::now() {
                    report.warnings.push("Exception has expired".to_string());
                }
            }
        }

        for validation_rule in &requirement.validation_rules {
            if validation_rule.expression.trim().is_empty() {
                report.errors.push("Validation rule expression cannot be empty".to_string());
            }

            if validation_rule.error_message.trim().is_empty() {
                report.errors.push("Validation rule error message cannot be empty".to_string());
            }
        }

        report.valid = report.errors.is_empty();
        Ok(report)
    }

    fn validate_framework_consistency(&self, framework: &NormativeFramework, report: &mut ValidationReport) -> AionResult<()> {
        for dep_id in &framework.dependencies {
            if dep_id == &framework.id {
                report.errors.push("Framework cannot depend on itself".to_string());
            }
        }

        for superseded_id in &framework.supersedes {
            if superseded_id == &framework.id {
                report.errors.push("Framework cannot supersede itself".to_string());
            }
        }

        let mut category_requirements: HashMap<String, Vec<&Requirement>> = HashMap::new();
        for requirement in &framework.requirements {
            category_requirements
                .entry(requirement.category.clone())
                .or_insert_with(Vec::new)
                .push(requirement);
        }

        for (category, requirements) in category_requirements {
            if requirements.len() > 1 {
                let mandatory_count = requirements.iter().filter(|r| r.mandatory).count();
                let optional_count = requirements.len() - mandatory_count;

                if mandatory_count > 5 {
                    report.warnings.push(format!("Category '{}' has many mandatory requirements ({})", category, mandatory_count));
                }

                if optional_count > 10 {
                    report.warnings.push(format!("Category '{}' has many optional requirements ({})", category, optional_count));
                }
            }
        }

        Ok(())
    }

    fn validate_framework_completeness(&self, framework: &NormativeFramework, report: &mut ValidationReport) -> AionResult<()> {
        let required_metadata_fields = vec!["sector", "region", "compliance_level", "risk_category"];

        for field in required_metadata_fields {
            if !framework.metadata.contains_key(field) {
                report.warnings.push(format!("Missing recommended metadata field: {}", field));
            }
        }

        if framework.tags.is_empty() {
            report.warnings.push("Framework has no tags for categorization".to_string());
        }

        let has_mandatory_requirements = framework.requirements.iter().any(|r| r.mandatory);
        if !has_mandatory_requirements {
            report.warnings.push("Framework has no mandatory requirements".to_string());
        }

        let has_validation_rules = framework.requirements.iter().any(|r| !r.validation_rules.is_empty());
        if !has_validation_rules {
            report.warnings.push("Framework has no validation rules".to_string());
        }

        Ok(())
    }

    fn validate_validation_rule_syntax(&self, expression: &str) -> AionResult<()> {
        if expression.trim().is_empty() {
            return Err(AionError::ValidationError {
                field: "expression".to_string(),
                message: "Expression cannot be empty".to_string(),
            });
        }

        let balanced_parens = self.check_balanced_parentheses(expression);
        if !balanced_parens {
            return Err(AionError::ValidationError {
                field: "expression".to_string(),
                message: "Unbalanced parentheses".to_string(),
            });
        }

        let valid_operators = vec!["AND", "OR", "NOT", "=", "!=", ">", "<", ">=", "<=", "LIKE", "IN", "BETWEEN"];
        let contains_valid_operator = valid_operators.iter().any(|op| expression.contains(op));

        if !contains_valid_operator && !expression.contains("MATCHES") {
            return Err(AionError::ValidationError {
                field: "expression".to_string(),
                message: "Expression must contain at least one valid operator".to_string(),
            });
        }

        Ok(())
    }

    fn check_balanced_parentheses(&self, expression: &str) -> bool {
        let mut depth = 0;
        for char in expression.chars() {
            match char {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    fn build_framework_context(&self, framework: &NormativeFramework) -> HashMap<String, String> {
        let mut context = HashMap::new();

        context.insert("title".to_string(), framework.title.clone());
        context.insert("description".to_string(), framework.description.clone());
        context.insert("authority".to_string(), framework.authority.clone());
        context.insert("version".to_string(), framework.version.clone());
        context.insert("status".to_string(), framework.status.clone());
        context.insert("effective_date".to_string(), framework.effective_date.to_rfc3339());
        context.insert("requirements_count".to_string(), framework.requirements.len().to_string());
        context.insert("tags_count".to_string(), framework.tags.len().to_string());
        context.insert("jurisdiction".to_string(), format!("{:?}", framework.jurisdiction));
        context.insert("normative_type".to_string(), format!("{:?}", framework.normative_type));

        if let Some(expiration) = framework.expiration_date {
            context.insert("expiration_date".to_string(), expiration.to_rfc3339());
        }

        for (key, value) in &framework.metadata {
            context.insert(format!("metadata_{}", key), value.clone());
        }

        context
    }

    fn calculate_validation_confidence(&self, report: &ValidationReport) -> f64 {
        let total_checks = report.errors.len() + report.warnings.len() + report.validations_passed;

        if total_checks == 0 {
            return 0.5;
        }

        let error_weight = 1.0;
        let warning_weight = 0.3;
        let success_weight = 0.1;

        let error_score = report.errors.len() as f64 * error_weight;
        let warning_score = report.warnings.len() as f64 * warning_weight;
        let success_score = report.validations_passed as f64 * success_weight;

        let negative_score = error_score + warning_score;
        let total_weighted = negative_score + success_score;

        if total_weighted == 0.0 {
            1.0
        } else {
            (success_score / total_weighted).max(0.0).min(1.0)
        }
    }
}

impl Default for ComprehensiveValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationEngine for ComprehensiveValidator {
    fn validate_framework(&self, framework: &NormativeFramework) -> AionResult<Vec<String>> {
        let report = self.validate_framework_comprehensive(framework)?;
        Ok(report.errors)
    }

    fn validate_requirement(&self, requirement: &Requirement) -> AionResult<Vec<String>> {
        let report = self.validate_requirement_detailed(requirement)?;
        Ok(report.errors)
    }

    fn validate_evidence(&self, evidence: &Evidence) -> AionResult<bool> {
        if evidence.description.trim().is_empty() {
            return Ok(false);
        }

        if evidence.source.trim().is_empty() {
            return Ok(false);
        }

        if evidence.verification_status.trim().is_empty() {
            return Ok(false);
        }

        if evidence.collected_date > Utc::now() {
            return Ok(false);
        }

        Ok(true)
    }
}

impl BusinessRuleEngine for ComprehensiveValidator {
    fn evaluate_rule(&self, rule_id: &uuid::Uuid, context: &HashMap<String, String>) -> AionResult<bool> {
        let rule = self.business_rules.iter()
            .find(|r| &r.id == rule_id)
            .ok_or_else(|| AionError::BusinessRuleViolation {
                rule_name: "unknown".to_string(),
                message: format!("Rule not found: {}", rule_id),
            })?;

        self.evaluate_business_rule(rule, context)
    }

    fn get_applicable_rules(&self, context: &HashMap<String, String>) -> AionResult<Vec<uuid::Uuid>> {
        let context_type = context.get("context").unwrap_or(&"unknown".to_string()).clone();

        Ok(self.business_rules.iter()
            .filter(|rule| rule.active && rule.context == context_type)
            .map(|rule| rule.id)
            .collect())
    }

    fn validate_business_logic(&self, entity_id: &str, context: &HashMap<String, String>) -> AionResult<Vec<String>> {
        let mut violations = Vec::new();
        let applicable_rules = self.get_applicable_rules(context)?;

        for rule_id in applicable_rules {
            match self.evaluate_rule(&rule_id, context) {
                Ok(true) => {},
                Ok(false) => {
                    if let Some(rule) = self.business_rules.iter().find(|r| r.id == rule_id) {
                        violations.push(format!("Entity {}: {}", entity_id, rule.description));
                    }
                },
                Err(e) => {
                    violations.push(format!("Entity {}: Rule evaluation error: {}", entity_id, e));
                }
            }
        }

        Ok(violations)
    }
}

impl ComprehensiveValidator {
    fn evaluate_business_rule(&self, rule: &BusinessRule, context: &HashMap<String, String>) -> AionResult<bool> {
        let expression = &rule.rule_expression;

        if expression.contains("title.length") {
            if let Some(title) = context.get("title") {
                return Ok(self.evaluate_length_condition(expression, title.len()));
            }
        }

        if expression.contains("description.length") {
            if let Some(description) = context.get("description") {
                return Ok(self.evaluate_length_condition(expression, description.len()));
            }
        }

        if expression.contains("requirements.count") {
            if let Some(count_str) = context.get("requirements_count") {
                if let Ok(count) = count_str.parse::<usize>() {
                    return Ok(self.evaluate_count_condition(expression, count));
                }
            }
        }

        if expression.contains("version MATCHES") {
            if let Some(version) = context.get("version") {
                return Ok(aion_core::validate_version(version));
            }
        }

        if expression.contains("authority.length") {
            if let Some(authority) = context.get("authority") {
                return Ok(self.evaluate_string_condition(expression, authority));
            }
        }

        if expression.contains("effective_date") {
            return Ok(self.evaluate_date_condition(expression, context));
        }

        Ok(true)
    }

    fn evaluate_length_condition(&self, expression: &str, length: usize) -> bool {
        if expression.contains(">= 10 AND") && expression.contains("<= 200") {
            return length >= 10 && length <= 200;
        }
        if expression.contains(">= 50") {
            return length >= 50;
        }
        if expression.contains("> 0") {
            return length > 0;
        }
        true
    }

    fn evaluate_count_condition(&self, expression: &str, count: usize) -> bool {
        if expression.contains(">= 1") {
            return count >= 1;
        }
        if expression.contains("> 0") {
            return count > 0;
        }
        true
    }

    fn evaluate_string_condition(&self, expression: &str, value: &str) -> bool {
        if expression.contains("NOT") && expression.contains("contains('unknown')") {
            return !value.contains("unknown");
        }
        if expression.contains("length > 0") {
            return !value.trim().is_empty();
        }
        true
    }

    fn evaluate_date_condition(&self, expression: &str, context: &HashMap<String, String>) -> bool {
        if expression.contains("NOW() + 5_YEARS") {
            if let Some(effective_date_str) = context.get("effective_date") {
                if let Ok(effective_date) = chrono::DateTime::parse_from_rfc3339(effective_date_str) {
                    let max_future = Utc::now() + chrono::Duration::days(5 * 365);
                    return effective_date.with_timezone(&Utc) <= max_future;
                }
            }
        }

        if expression.contains("expiration_date > effective_date") {
            if let (Some(exp_str), Some(eff_str)) = (context.get("expiration_date"), context.get("effective_date")) {
                if let (Ok(exp_date), Ok(eff_date)) = (
                    chrono::DateTime::parse_from_rfc3339(exp_str),
                    chrono::DateTime::parse_from_rfc3339(eff_str)
                ) {
                    return exp_date > eff_date;
                }
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub overall_valid: bool,
    pub confidence_score: f64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub validations_passed: usize,
    pub requirement_reports: Vec<RequirementValidationReport>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            overall_valid: true,
            confidence_score: 1.0,
            errors: Vec::new(),
            warnings: Vec::new(),
            validations_passed: 0,
            requirement_reports: Vec::new(),
        }
    }

    pub fn merge_requirement_report(&mut self, req_report: RequirementValidationReport) {
        self.errors.extend(req_report.errors.clone());
        self.warnings.extend(req_report.warnings.clone());
        self.requirement_reports.push(req_report);
    }
}

#[derive(Debug, Clone)]
pub struct RequirementValidationReport {
    pub requirement_id: uuid::Uuid,
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl RequirementValidationReport {
    pub fn new(requirement_id: uuid::Uuid) -> Self {
        Self {
            requirement_id,
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}