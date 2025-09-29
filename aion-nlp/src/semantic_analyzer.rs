use aion_core::{AionResult, AionError, NormativeFramework, Requirement};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalysis {
    pub document_id: String,
    pub semantic_concepts: Vec<SemanticConcept>,
    pub regulatory_entities: Vec<RegulatoryEntity>,
    pub obligations: Vec<Obligation>,
    pub conditions: Vec<ExtractedCondition>,
    pub temporal_constraints: Vec<TemporalConstraint>,
    pub semantic_similarity_score: f64,
    pub complexity_score: f64,
    pub ambiguity_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConcept {
    pub concept: String,
    pub category: ConceptCategory,
    pub confidence: f64,
    pub context: String,
    pub related_concepts: Vec<String>,
    pub regulatory_significance: RegulatorySignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptCategory {
    DataProtection,
    FinancialCompliance,
    SecurityRequirement,
    ProcessualObligation,
    TechnicalStandard,
    LegalFramework,
    ComplianceMetric,
    RiskManagement,
    AuditRequirement,
    GovernanceStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulatorySignificance {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryEntity {
    pub entity_name: String,
    pub entity_type: EntityType,
    pub jurisdiction: String,
    pub authority_level: u8,
    pub regulatory_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    RegulatoryBody,
    Government,
    StandardsOrganization,
    Industry,
    InternalAuthority,
    CoordinatingBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub obligation_text: String,
    pub obligation_type: ObligationType,
    pub mandatory: bool,
    pub subject: String,
    pub action: String,
    pub object: String,
    pub conditions: Vec<String>,
    pub temporal_scope: Option<String>,
    pub penalty_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObligationType {
    Shall,
    Must,
    Should,
    May,
    ShallNot,
    MustNot,
    Prohibited,
    Required,
    Optional,
    Recommended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedCondition {
    pub condition_text: String,
    pub condition_type: ConditionType,
    pub logical_operator: LogicalOperator,
    pub variables: Vec<String>,
    pub threshold_values: Vec<String>,
    pub boolean_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Temporal,
    Quantitative,
    Categorical,
    Boolean,
    Contextual,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
    If,
    Unless,
    When,
    Where,
    Provided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConstraint {
    pub constraint_text: String,
    pub temporal_type: TemporalType,
    pub duration: Option<String>,
    pub frequency: Option<String>,
    pub deadline: Option<String>,
    pub trigger_event: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalType {
    Deadline,
    Duration,
    Frequency,
    StartDate,
    EndDate,
    ReviewCycle,
    ReportingPeriod,
    RetentionPeriod,
}

pub struct AdvancedSemanticAnalyzer {
    regulatory_patterns: HashMap<String, Regex>,
    obligation_patterns: HashMap<ObligationType, Regex>,
    temporal_patterns: HashMap<TemporalType, Regex>,
    concept_embeddings: HashMap<String, Vec<f64>>,
    regulatory_ontology: RegulatoryOntology,
}

impl AdvancedSemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            regulatory_patterns: HashMap::new(),
            obligation_patterns: HashMap::new(),
            temporal_patterns: HashMap::new(),
            concept_embeddings: HashMap::new(),
            regulatory_ontology: RegulatoryOntology::new(),
        };

        analyzer.initialize_patterns();
        analyzer.initialize_concept_embeddings();
        analyzer
    }

    fn initialize_patterns(&mut self) {
        self.regulatory_patterns.insert(
            "data_protection".to_string(),
            Regex::new(r"(?i)(personal\s+data|data\s+protection|privacy|consent|data\s+subject|controller|processor)").unwrap(),
        );

        self.regulatory_patterns.insert(
            "financial_compliance".to_string(),
            Regex::new(r"(?i)(capital\s+requirement|liquidity|risk\s+management|financial\s+reporting|audit|internal\s+control)").unwrap(),
        );

        self.regulatory_patterns.insert(
            "cybersecurity".to_string(),
            Regex::new(r"(?i)(cybersecurity|information\s+security|incident|breach|vulnerability|threat|security\s+control)").unwrap(),
        );

        self.obligation_patterns.insert(
            ObligationType::Shall,
            Regex::new(r"(?i)\b(shall|must)\b").unwrap(),
        );

        self.obligation_patterns.insert(
            ObligationType::Should,
            Regex::new(r"(?i)\b(should|ought\s+to|recommended)\b").unwrap(),
        );

        self.obligation_patterns.insert(
            ObligationType::May,
            Regex::new(r"(?i)\b(may|can|permitted|allowed)\b").unwrap(),
        );

        self.obligation_patterns.insert(
            ObligationType::Prohibited,
            Regex::new(r"(?i)\b(shall\s+not|must\s+not|prohibited|forbidden|banned)\b").unwrap(),
        );

        self.temporal_patterns.insert(
            TemporalType::Deadline,
            Regex::new(r"(?i)(within\s+\d+\s+(days?|months?|years?)|by\s+\w+|\d+\s+(day|month|year)\s+deadline)").unwrap(),
        );

        self.temporal_patterns.insert(
            TemporalType::Frequency,
            Regex::new(r"(?i)(annually|quarterly|monthly|weekly|daily|every\s+\d+\s+(days?|months?|years?))").unwrap(),
        );

        self.temporal_patterns.insert(
            TemporalType::Duration,
            Regex::new(r"(?i)(for\s+\d+\s+(days?|months?|years?)|duration\s+of|\d+\s+(day|month|year)\s+period)").unwrap(),
        );
    }

    fn initialize_concept_embeddings(&mut self) {
        self.concept_embeddings.insert("data_protection".to_string(), vec![0.8, 0.2, 0.9, 0.1, 0.7]);
        self.concept_embeddings.insert("privacy".to_string(), vec![0.9, 0.1, 0.8, 0.2, 0.6]);
        self.concept_embeddings.insert("consent".to_string(), vec![0.7, 0.3, 0.9, 0.1, 0.8]);
        self.concept_embeddings.insert("financial_reporting".to_string(), vec![0.1, 0.9, 0.2, 0.8, 0.3]);
        self.concept_embeddings.insert("risk_management".to_string(), vec![0.3, 0.7, 0.4, 0.9, 0.2]);
        self.concept_embeddings.insert("cybersecurity".to_string(), vec![0.2, 0.1, 0.3, 0.2, 0.9]);
        self.concept_embeddings.insert("incident_response".to_string(), vec![0.1, 0.2, 0.2, 0.3, 0.8]);
    }

    pub fn analyze_normative_text(&self, text: &str, framework_id: &str) -> AionResult<SemanticAnalysis> {
        let semantic_concepts = self.extract_semantic_concepts(text)?;
        let regulatory_entities = self.extract_regulatory_entities(text)?;
        let obligations = self.extract_obligations(text)?;
        let conditions = self.extract_conditions(text)?;
        let temporal_constraints = self.extract_temporal_constraints(text)?;

        let complexity_score = self.calculate_complexity_score(text, &semantic_concepts);
        let ambiguity_indicators = self.detect_ambiguity(text);

        Ok(SemanticAnalysis {
            document_id: framework_id.to_string(),
            semantic_concepts,
            regulatory_entities,
            obligations,
            conditions,
            temporal_constraints,
            semantic_similarity_score: 0.0,
            complexity_score,
            ambiguity_indicators,
        })
    }

    fn extract_semantic_concepts(&self, text: &str) -> AionResult<Vec<SemanticConcept>> {
        let mut concepts = Vec::new();

        for (category, pattern) in &self.regulatory_patterns {
            for mat in pattern.find_iter(text) {
                let concept_text = mat.as_str();
                let context = self.extract_context(text, mat.start(), mat.end());

                let concept = SemanticConcept {
                    concept: concept_text.to_string(),
                    category: self.categorize_concept(category),
                    confidence: self.calculate_concept_confidence(concept_text, &context),
                    context,
                    related_concepts: self.find_related_concepts(concept_text),
                    regulatory_significance: self.assess_regulatory_significance(concept_text),
                };

                concepts.push(concept);
            }
        }

        Ok(concepts)
    }

    fn extract_regulatory_entities(&self, text: &str) -> AionResult<Vec<RegulatoryEntity>> {
        let mut entities = Vec::new();

        let entity_patterns = vec![
            (r"(?i)(European\s+Commission|EC\b)", EntityType::RegulatoryBody, "EU"),
            (r"(?i)(FDA|Food\s+and\s+Drug\s+Administration)", EntityType::RegulatoryBody, "US"),
            (r"(?i)(GDPR|General\s+Data\s+Protection\s+Regulation)", EntityType::RegulatoryBody, "EU"),
            (r"(?i)(Basel\s+Committee|BCBS)", EntityType::StandardsOrganization, "International"),
            (r"(?i)(NIST|National\s+Institute\s+of\s+Standards)", EntityType::StandardsOrganization, "US"),
            (r"(?i)(ISO|International\s+Organization\s+for\s+Standardization)", EntityType::StandardsOrganization, "International"),
        ];

        for (pattern_str, entity_type, jurisdiction) in entity_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            for mat in pattern.find_iter(text) {
                entities.push(RegulatoryEntity {
                    entity_name: mat.as_str().to_string(),
                    entity_type: entity_type.clone(),
                    jurisdiction: jurisdiction.to_string(),
                    authority_level: self.assess_authority_level(&entity_type),
                    regulatory_scope: self.determine_regulatory_scope(&entity_type),
                });
            }
        }

        Ok(entities)
    }

    fn extract_obligations(&self, text: &str) -> AionResult<Vec<Obligation>> {
        let mut obligations = Vec::new();

        for (obligation_type, pattern) in &self.obligation_patterns {
            for mat in pattern.find_iter(text) {
                let sentence = self.extract_sentence_containing(text, mat.start());
                let parsed_obligation = self.parse_obligation(&sentence, obligation_type)?;
                obligations.push(parsed_obligation);
            }
        }

        Ok(obligations)
    }

    fn parse_obligation(&self, sentence: &str, obligation_type: &ObligationType) -> AionResult<Obligation> {
        let subject_pattern = Regex::new(r"(?i)^([^,]+?)(?:\s+shall|\s+must|\s+should|\s+may)").unwrap();
        let action_pattern = Regex::new(r"(?i)(?:shall|must|should|may)\s+([^,]+?)(?:\s+(?:the|an|a)\s+|\s*$)").unwrap();

        let subject = subject_pattern
            .captures(sentence)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "entity".to_string());

        let action = action_pattern
            .captures(sentence)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "comply".to_string());

        let mandatory = matches!(obligation_type, ObligationType::Shall | ObligationType::Must | ObligationType::Required);

        let conditions = self.extract_obligation_conditions(sentence);
        let temporal_scope = self.extract_temporal_scope(sentence);
        let penalty_indicators = self.extract_penalty_indicators(sentence);

        Ok(Obligation {
            obligation_text: sentence.to_string(),
            obligation_type: obligation_type.clone(),
            mandatory,
            subject,
            action,
            object: "compliance requirement".to_string(),
            conditions,
            temporal_scope,
            penalty_indicators,
        })
    }

    fn extract_conditions(&self, text: &str) -> AionResult<Vec<ExtractedCondition>> {
        let mut conditions = Vec::new();

        let condition_patterns = vec![
            (r"(?i)if\s+(.+?)(?:\s+then|\s*,|\s*$)", LogicalOperator::If),
            (r"(?i)unless\s+(.+?)(?:\s+then|\s*,|\s*$)", LogicalOperator::Unless),
            (r"(?i)when\s+(.+?)(?:\s+then|\s*,|\s*$)", LogicalOperator::When),
            (r"(?i)where\s+(.+?)(?:\s+then|\s*,|\s*$)", LogicalOperator::Where),
            (r"(?i)provided\s+that\s+(.+?)(?:\s+then|\s*,|\s*$)", LogicalOperator::Provided),
        ];

        for (pattern_str, logical_op) in condition_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            for caps in pattern.captures_iter(text) {
                if let Some(condition_text) = caps.get(1) {
                    let condition = ExtractedCondition {
                        condition_text: condition_text.as_str().to_string(),
                        condition_type: self.classify_condition_type(condition_text.as_str()),
                        logical_operator: logical_op.clone(),
                        variables: self.extract_variables(condition_text.as_str()),
                        threshold_values: self.extract_threshold_values(condition_text.as_str()),
                        boolean_expression: self.generate_boolean_expression(condition_text.as_str()),
                    };
                    conditions.push(condition);
                }
            }
        }

        Ok(conditions)
    }

    fn extract_temporal_constraints(&self, text: &str) -> AionResult<Vec<TemporalConstraint>> {
        let mut constraints = Vec::new();

        for (temporal_type, pattern) in &self.temporal_patterns {
            for mat in pattern.find_iter(text) {
                let constraint_text = mat.as_str();
                let constraint = TemporalConstraint {
                    constraint_text: constraint_text.to_string(),
                    temporal_type: temporal_type.clone(),
                    duration: self.extract_duration(constraint_text),
                    frequency: self.extract_frequency(constraint_text),
                    deadline: self.extract_deadline(constraint_text),
                    trigger_event: self.extract_trigger_event(text, mat.start()),
                };
                constraints.push(constraint);
            }
        }

        Ok(constraints)
    }

    fn calculate_complexity_score(&self, text: &str, concepts: &[SemanticConcept]) -> f64 {
        let text_length = text.len() as f64;
        let concept_count = concepts.len() as f64;
        let sentence_count = text.split('.').count() as f64;
        let word_count = text.split_whitespace().count() as f64;

        let avg_sentence_length = word_count / sentence_count.max(1.0);
        let concept_density = concept_count / word_count.max(1.0);

        let complexity = (avg_sentence_length * 0.3) + (concept_density * 100.0 * 0.4) + (concept_count * 0.3);
        complexity.min(10.0).max(0.0)
    }

    fn detect_ambiguity(&self, text: &str) -> Vec<String> {
        let mut indicators = Vec::new();

        let ambiguity_patterns = vec![
            (r"(?i)\b(reasonable|appropriate|sufficient|adequate|necessary)\b", "Subjective qualifier"),
            (r"(?i)\b(may\s+include|such\s+as|including\s+but\s+not\s+limited\s+to)\b", "Non-exhaustive list"),
            (r"(?i)\b(as\s+soon\s+as\s+possible|when\s+appropriate|if\s+necessary)\b", "Temporal ambiguity"),
            (r"(?i)\b(or\s+similar|and\s+the\s+like|etc\.?)\b", "Vague terminology"),
        ];

        for (pattern_str, description) in ambiguity_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            if pattern.is_match(text) {
                indicators.push(description.to_string());
            }
        }

        indicators
    }

    fn categorize_concept(&self, category: &str) -> ConceptCategory {
        match category {
            "data_protection" => ConceptCategory::DataProtection,
            "financial_compliance" => ConceptCategory::FinancialCompliance,
            "cybersecurity" => ConceptCategory::SecurityRequirement,
            _ => ConceptCategory::LegalFramework,
        }
    }

    fn calculate_concept_confidence(&self, concept: &str, context: &str) -> f64 {
        let context_strength = if context.len() > 50 { 0.8 } else { 0.5 };
        let concept_clarity = if concept.len() > 3 { 0.7 } else { 0.4 };
        (context_strength + concept_clarity) / 2.0
    }

    fn find_related_concepts(&self, concept: &str) -> Vec<String> {
        if let Some(embedding) = self.concept_embeddings.get(concept) {
            let mut related = Vec::new();
            for (other_concept, other_embedding) in &self.concept_embeddings {
                if other_concept != concept {
                    let similarity = self.cosine_similarity(embedding, other_embedding);
                    if similarity > 0.7 {
                        related.push(other_concept.clone());
                    }
                }
            }
            related
        } else {
            Vec::new()
        }
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    fn assess_regulatory_significance(&self, concept: &str) -> RegulatorySignificance {
        let high_significance_terms = ["shall", "must", "required", "mandatory", "compliance"];
        let medium_significance_terms = ["should", "recommended", "guidelines"];

        if high_significance_terms.iter().any(|term| concept.to_lowercase().contains(term)) {
            RegulatorySignificance::High
        } else if medium_significance_terms.iter().any(|term| concept.to_lowercase().contains(term)) {
            RegulatorySignificance::Medium
        } else {
            RegulatorySignificance::Low
        }
    }

    fn extract_context(&self, text: &str, start: usize, end: usize) -> String {
        let context_window = 100;
        let context_start = start.saturating_sub(context_window);
        let context_end = (end + context_window).min(text.len());
        text[context_start..context_end].to_string()
    }

    fn assess_authority_level(&self, entity_type: &EntityType) -> u8 {
        match entity_type {
            EntityType::Government => 10,
            EntityType::RegulatoryBody => 9,
            EntityType::StandardsOrganization => 7,
            EntityType::CoordinatingBody => 6,
            EntityType::Industry => 4,
            EntityType::InternalAuthority => 2,
        }
    }

    fn determine_regulatory_scope(&self, entity_type: &EntityType) -> Vec<String> {
        match entity_type {
            EntityType::Government => vec!["all sectors".to_string()],
            EntityType::RegulatoryBody => vec!["regulated industries".to_string()],
            EntityType::StandardsOrganization => vec!["industry standards".to_string()],
            _ => vec!["specific domain".to_string()],
        }
    }

    fn extract_sentence_containing(&self, text: &str, position: usize) -> String {
        let sentences: Vec<&str> = text.split('.').collect();
        let mut char_count = 0;

        for sentence in sentences {
            char_count += sentence.len() + 1;
            if char_count > position {
                return sentence.trim().to_string();
            }
        }

        "".to_string()
    }

    fn extract_obligation_conditions(&self, sentence: &str) -> Vec<String> {
        let condition_indicators = ["if", "when", "where", "unless", "provided that"];
        let mut conditions = Vec::new();

        for indicator in &condition_indicators {
            if let Some(pos) = sentence.to_lowercase().find(indicator) {
                let condition = sentence[pos..].to_string();
                conditions.push(condition);
            }
        }

        conditions
    }

    fn extract_temporal_scope(&self, sentence: &str) -> Option<String> {
        for (_, pattern) in &self.temporal_patterns {
            if let Some(mat) = pattern.find(sentence) {
                return Some(mat.as_str().to_string());
            }
        }
        None
    }

    fn extract_penalty_indicators(&self, sentence: &str) -> Vec<String> {
        let penalty_pattern = Regex::new(r"(?i)(penalty|fine|sanction|violation|non-compliance|breach)").unwrap();
        penalty_pattern
            .find_iter(sentence)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn classify_condition_type(&self, condition_text: &str) -> ConditionType {
        if self.temporal_patterns.values().any(|p| p.is_match(condition_text)) {
            ConditionType::Temporal
        } else if condition_text.chars().any(|c| c.is_numeric()) {
            ConditionType::Quantitative
        } else if condition_text.to_lowercase().contains("true") || condition_text.to_lowercase().contains("false") {
            ConditionType::Boolean
        } else {
            ConditionType::Contextual
        }
    }

    fn extract_variables(&self, condition_text: &str) -> Vec<String> {
        let var_pattern = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
        var_pattern
            .find_iter(condition_text)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn extract_threshold_values(&self, condition_text: &str) -> Vec<String> {
        let threshold_pattern = Regex::new(r"\b\d+(?:\.\d+)?\b").unwrap();
        threshold_pattern
            .find_iter(condition_text)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn generate_boolean_expression(&self, condition_text: &str) -> String {
        condition_text
            .replace(" and ", " AND ")
            .replace(" or ", " OR ")
            .replace(" not ", " NOT ")
    }

    fn extract_duration(&self, constraint_text: &str) -> Option<String> {
        let duration_pattern = Regex::new(r"\b\d+\s+(days?|months?|years?)\b").unwrap();
        duration_pattern.find(constraint_text).map(|m| m.as_str().to_string())
    }

    fn extract_frequency(&self, constraint_text: &str) -> Option<String> {
        let frequency_words = ["annually", "quarterly", "monthly", "weekly", "daily"];
        for word in &frequency_words {
            if constraint_text.to_lowercase().contains(word) {
                return Some(word.to_string());
            }
        }
        None
    }

    fn extract_deadline(&self, constraint_text: &str) -> Option<String> {
        let deadline_pattern = Regex::new(r"(?i)by\s+\w+|within\s+\d+\s+\w+").unwrap();
        deadline_pattern.find(constraint_text).map(|m| m.as_str().to_string())
    }

    fn extract_trigger_event(&self, text: &str, position: usize) -> Option<String> {
        let trigger_patterns = ["upon", "after", "following", "in response to"];
        let context = self.extract_context(text, position, position + 1);

        for pattern in &trigger_patterns {
            if context.to_lowercase().contains(pattern) {
                return Some(pattern.to_string());
            }
        }
        None
    }

    pub fn calculate_semantic_similarity(&self, analysis1: &SemanticAnalysis, analysis2: &SemanticAnalysis) -> f64 {
        let concepts1: std::collections::HashSet<_> = analysis1.semantic_concepts.iter().map(|c| &c.concept).collect();
        let concepts2: std::collections::HashSet<_> = analysis2.semantic_concepts.iter().map(|c| &c.concept).collect();

        let intersection = concepts1.intersection(&concepts2).count();
        let union = concepts1.union(&concepts2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

impl Default for AdvancedSemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct RegulatoryOntology {
    concept_hierarchy: HashMap<String, Vec<String>>,
    relationship_types: Vec<String>,
}

impl RegulatoryOntology {
    fn new() -> Self {
        let mut concept_hierarchy = HashMap::new();

        concept_hierarchy.insert("data_protection".to_string(), vec![
            "personal_data".to_string(),
            "consent".to_string(),
            "data_subject_rights".to_string(),
            "controller".to_string(),
            "processor".to_string(),
        ]);

        concept_hierarchy.insert("financial_compliance".to_string(), vec![
            "capital_requirements".to_string(),
            "liquidity_management".to_string(),
            "risk_assessment".to_string(),
            "financial_reporting".to_string(),
        ]);

        Self {
            concept_hierarchy,
            relationship_types: vec![
                "requires".to_string(),
                "conflicts_with".to_string(),
                "complements".to_string(),
                "supersedes".to_string(),
            ],
        }
    }
}