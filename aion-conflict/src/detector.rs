use aion_core::{
    AionError, AionResult, NormativeFramework, NormativeConflict, NormativeId,
    ConflictDetector, ConflictType, ConflictSeverity, ResolutionStrategy
};
use std::collections::{HashMap, HashSet};
use chrono::Utc;
use uuid::Uuid;
use rayon::prelude::*;

pub struct AdvancedConflictDetector {
    conflict_cache: HashMap<(NormativeId, NormativeId), Option<NormativeConflict>>,
    similarity_threshold: f64,
    semantic_analyzer: SemanticAnalyzer,
}

impl AdvancedConflictDetector {
    pub fn new() -> Self {
        Self {
            conflict_cache: HashMap::new(),
            similarity_threshold: 0.75,
            semantic_analyzer: SemanticAnalyzer::new(),
        }
    }

    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold;
        self
    }

    pub fn clear_cache(&mut self) {
        self.conflict_cache.clear();
    }

    pub fn detect_all_conflicts(&mut self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        let framework_pairs: Vec<_> = frameworks
            .iter()
            .enumerate()
            .flat_map(|(i, f1)| {
                frameworks[i + 1..]
                    .iter()
                    .map(move |f2| (f1, f2))
            })
            .collect();

        let detected_conflicts: Vec<_> = framework_pairs
            .par_iter()
            .filter_map(|(f1, f2)| {
                self.detect_framework_conflict(f1, f2).ok().flatten()
            })
            .collect();

        conflicts.extend(detected_conflicts);

        let hierarchical_conflicts = self.detect_hierarchical_conflicts(frameworks)?;
        conflicts.extend(hierarchical_conflicts);

        let temporal_conflicts = self.detect_temporal_conflicts(frameworks)?;
        conflicts.extend(temporal_conflicts);

        let jurisdictional_conflicts = self.detect_jurisdictional_conflicts(frameworks)?;
        conflicts.extend(jurisdictional_conflicts);

        Ok(conflicts)
    }

    fn detect_framework_conflict(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> AionResult<Option<NormativeConflict>> {
        if f1.id == f2.id {
            return Ok(None);
        }

        let cache_key = if f1.id.0 < f2.id.0 {
            (f1.id.clone(), f2.id.clone())
        } else {
            (f2.id.clone(), f1.id.clone())
        };

        if let Some(cached_result) = self.conflict_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        let conflict = self.analyze_framework_pair(f1, f2)?;
        Ok(conflict)
    }

    fn analyze_framework_pair(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> AionResult<Option<NormativeConflict>> {
        if !self.frameworks_potentially_conflict(f1, f2) {
            return Ok(None);
        }

        let requirement_conflicts = self.detect_requirement_conflicts(f1, f2)?;
        let authority_conflicts = self.detect_authority_conflicts(f1, f2)?;
        let scope_conflicts = self.detect_scope_conflicts(f1, f2)?;

        let all_conflicts = [requirement_conflicts, authority_conflicts, scope_conflicts].concat();

        if !all_conflicts.is_empty() {
            let most_severe = all_conflicts
                .iter()
                .max_by_key(|c| self.severity_priority(&c.severity))
                .unwrap();

            Ok(Some(most_severe.clone()))
        } else {
            Ok(None)
        }
    }

    fn frameworks_potentially_conflict(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> bool {
        f1.jurisdiction == f2.jurisdiction ||
        f1.jurisdiction == aion_core::Jurisdiction::International ||
        f2.jurisdiction == aion_core::Jurisdiction::International ||
        self.has_overlapping_scope(f1, f2)
    }

    fn has_overlapping_scope(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> bool {
        f1.tags.iter().any(|tag| f2.tags.contains(tag)) ||
        aion_core::calculate_similarity(&f1.description, &f2.description) > self.similarity_threshold
    }

    fn detect_requirement_conflicts(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        for req1 in &f1.requirements {
            for req2 in &f2.requirements {
                if let Some(conflict) = self.analyze_requirement_pair(f1, f2, req1, req2)? {
                    conflicts.push(conflict);
                }
            }
        }

        Ok(conflicts)
    }

    fn analyze_requirement_pair(
        &self,
        f1: &NormativeFramework,
        f2: &NormativeFramework,
        req1: &aion_core::Requirement,
        req2: &aion_core::Requirement,
    ) -> AionResult<Option<NormativeConflict>> {
        if req1.category != req2.category {
            return Ok(None);
        }

        let semantic_similarity = self.semantic_analyzer.calculate_similarity(&req1.description, &req2.description);

        if semantic_similarity > self.similarity_threshold {
            if req1.mandatory != req2.mandatory {
                return Ok(Some(NormativeConflict {
                    id: Uuid::new_v4(),
                    conflict_type: ConflictType::DirectContradiction,
                    severity: ConflictSeverity::High,
                    normative_a: f1.id.clone(),
                    normative_b: f2.id.clone(),
                    involved_frameworks: vec![f1.id.clone(), f2.id.clone()],
                    description: format!(
                        "Conflicting mandatory requirements: '{}' vs '{}'",
                        req1.title, req2.title
                    ),
                    affected_requirements: vec![req1.id, req2.id],
                    context: self.build_conflict_context(f1, f2, req1, req2),
                    discovered_at: Utc::now(),
                    resolution_strategy: Some(ResolutionStrategy::LexSuperior),
                    resolution_notes: None,
                    resolved_at: None,
                    resolved_by: None,
                }));
            }

            for cond1 in &req1.conditions {
                for cond2 in &req2.conditions {
                    if self.conditions_contradict(cond1, cond2)? {
                        return Ok(Some(NormativeConflict {
                            id: Uuid::new_v4(),
                            conflict_type: ConflictType::ImplicitConflict,
                            severity: ConflictSeverity::Medium,
                            normative_a: f1.id.clone(),
                            normative_b: f2.id.clone(),
                            involved_frameworks: vec![f1.id.clone(), f2.id.clone()],
                            description: format!(
                                "Contradictory conditions in requirements: '{}' vs '{}'",
                                req1.title, req2.title
                            ),
                            affected_requirements: vec![req1.id, req2.id],
                            context: self.build_conflict_context(f1, f2, req1, req2),
                            discovered_at: Utc::now(),
                            resolution_strategy: Some(ResolutionStrategy::Harmonization),
                            resolution_notes: None,
                            resolved_at: None,
                            resolved_by: None,
                        }));
                    }
                }
            }
        }

        Ok(None)
    }

    fn conditions_contradict(&self, cond1: &aion_core::Condition, cond2: &aion_core::Condition) -> AionResult<bool> {
        if cond1.expression.contains("NOT") && cond2.expression.contains(&cond1.expression.replace("NOT ", "")) {
            return Ok(true);
        }

        if cond2.expression.contains("NOT") && cond1.expression.contains(&cond2.expression.replace("NOT ", "")) {
            return Ok(true);
        }

        if cond1.expression.contains(">=") && cond2.expression.contains("<=") {
            if let (Some(val1), Some(val2)) = (self.extract_numeric_value(&cond1.expression), self.extract_numeric_value(&cond2.expression)) {
                return Ok(val1 > val2);
            }
        }

        Ok(false)
    }

    fn extract_numeric_value(&self, expression: &str) -> Option<f64> {
        use std::str::FromStr;

        for token in expression.split_whitespace() {
            if let Ok(val) = f64::from_str(token) {
                return Some(val);
            }
        }
        None
    }

    fn detect_authority_conflicts(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        if f1.authority == f2.authority && f1.jurisdiction == f2.jurisdiction {
            if self.has_overlapping_scope(f1, f2) {
                conflicts.push(NormativeConflict {
                    id: Uuid::new_v4(),
                    conflict_type: ConflictType::AuthorityConflict,
                    severity: ConflictSeverity::Medium,
                    normative_a: f1.id.clone(),
                    normative_b: f2.id.clone(),
                    involved_frameworks: vec![f1.id.clone(), f2.id.clone()],
                    description: format!(
                        "Same authority '{}' issued conflicting frameworks in same jurisdiction",
                        f1.authority
                    ),
                    affected_requirements: vec![],
                    context: HashMap::new(),
                    discovered_at: Utc::now(),
                    resolution_strategy: Some(ResolutionStrategy::LexPosterior),
                    resolution_notes: None,
                    resolved_at: None,
                    resolved_by: None,
                });
            }
        }

        Ok(conflicts)
    }

    fn detect_scope_conflicts(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        if f1.jurisdiction != f2.jurisdiction {
            let scope_overlap = self.calculate_scope_overlap(f1, f2);
            if scope_overlap > 0.6 {
                conflicts.push(NormativeConflict {
                    id: Uuid::new_v4(),
                    conflict_type: ConflictType::ScopeAmbiguity,
                    severity: ConflictSeverity::Low,
                    normative_a: f1.id.clone(),
                    normative_b: f2.id.clone(),
                    involved_frameworks: vec![f1.id.clone(), f2.id.clone()],
                    description: format!(
                        "Scope ambiguity between frameworks from different jurisdictions: {:?} vs {:?}",
                        f1.jurisdiction, f2.jurisdiction
                    ),
                    affected_requirements: vec![],
                    context: HashMap::from([
                        ("scope_overlap".to_string(), scope_overlap.to_string()),
                        ("jurisdiction_a".to_string(), format!("{:?}", f1.jurisdiction)),
                        ("jurisdiction_b".to_string(), format!("{:?}", f2.jurisdiction)),
                    ]),
                    discovered_at: Utc::now(),
                    resolution_strategy: Some(ResolutionStrategy::Contextualization),
                    resolution_notes: None,
                    resolved_at: None,
                    resolved_by: None,
                });
            }
        }

        Ok(conflicts)
    }

    fn calculate_scope_overlap(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> f64 {
        let tags1: HashSet<_> = f1.tags.iter().collect();
        let tags2: HashSet<_> = f2.tags.iter().collect();

        let intersection = tags1.intersection(&tags2).count();
        let union = tags1.union(&tags2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    fn detect_hierarchical_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        for framework in frameworks {
            for dep_id in &framework.dependencies {
                if let Some(dependency) = frameworks.iter().find(|f| &f.id == dep_id) {
                    if framework.effective_date < dependency.effective_date {
                        conflicts.push(NormativeConflict {
                            id: Uuid::new_v4(),
                            conflict_type: ConflictType::TemporalInconsistency,
                            severity: ConflictSeverity::Medium,
                            normative_a: framework.id.clone(),
                            normative_b: dependency.id.clone(),
                            involved_frameworks: vec![framework.id.clone(), dependency.id.clone()],
                            description: "Framework is effective before its dependency".to_string(),
                            affected_requirements: vec![],
                            context: HashMap::from([
                                ("framework_effective".to_string(), framework.effective_date.to_rfc3339()),
                                ("dependency_effective".to_string(), dependency.effective_date.to_rfc3339()),
                            ]),
                            discovered_at: Utc::now(),
                            resolution_strategy: Some(ResolutionStrategy::LexPosterior),
                            resolution_notes: None,
                            resolved_at: None,
                            resolved_by: None,
                        });
                    }
                }
            }
        }

        Ok(conflicts)
    }

    fn detect_temporal_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        for f1 in frameworks {
            for f2 in frameworks {
                if f1.id != f2.id && self.has_temporal_overlap(f1, f2) && self.has_overlapping_scope(f1, f2) {
                    let newer_supersedes_older = f2.supersedes.contains(&f1.id) || f1.supersedes.contains(&f2.id);

                    if !newer_supersedes_older {
                        conflicts.push(NormativeConflict {
                            id: Uuid::new_v4(),
                            conflict_type: ConflictType::TemporalInconsistency,
                            severity: ConflictSeverity::Low,
                            normative_a: f1.id.clone(),
                            normative_b: f2.id.clone(),
                    involved_frameworks: vec![f1.id.clone(), f2.id.clone()],
                            description: "Overlapping temporal validity with similar scope".to_string(),
                            affected_requirements: vec![],
                            context: HashMap::new(),
                            discovered_at: Utc::now(),
                            resolution_strategy: Some(ResolutionStrategy::LexPosterior),
                            resolution_notes: None,
                            resolved_at: None,
                            resolved_by: None,
                        });
                    }
                }
            }
        }

        Ok(conflicts)
    }

    fn has_temporal_overlap(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> bool {
        let f1_start = f1.effective_date;
        let f1_end = f1.expiration_date.unwrap_or_else(|| chrono::DateTime::from_timestamp(i64::MAX, 0).unwrap());
        let f2_start = f2.effective_date;
        let f2_end = f2.expiration_date.unwrap_or_else(|| chrono::DateTime::from_timestamp(i64::MAX, 0).unwrap());

        f1_start < f2_end && f2_start < f1_end
    }

    fn detect_jurisdictional_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>> {
        let mut conflicts = Vec::new();

        let international_frameworks: Vec<_> = frameworks
            .iter()
            .filter(|f| f.jurisdiction == aion_core::Jurisdiction::International)
            .collect();

        for int_framework in &international_frameworks {
            for framework in frameworks {
                if framework.jurisdiction != aion_core::Jurisdiction::International &&
                   self.has_overlapping_scope(int_framework, framework) {

                    let conflict_level = self.assess_jurisdictional_conflict_severity(int_framework, framework);

                    if conflict_level > ConflictSeverity::Informational {
                        conflicts.push(NormativeConflict {
                            id: Uuid::new_v4(),
                            conflict_type: ConflictType::JurisdictionalOverlap,
                            severity: conflict_level,
                            normative_a: int_framework.id.clone(),
                            normative_b: framework.id.clone(),
                            involved_frameworks: vec![int_framework.id.clone(), framework.id.clone()],
                            description: format!(
                                "Jurisdictional overlap: International vs {:?}",
                                framework.jurisdiction
                            ),
                            affected_requirements: vec![],
                            context: HashMap::new(),
                            discovered_at: Utc::now(),
                            resolution_strategy: Some(ResolutionStrategy::LexSuperior),
                            resolution_notes: None,
                            resolved_at: None,
                            resolved_by: None,
                        });
                    }
                }
            }
        }

        Ok(conflicts)
    }

    fn assess_jurisdictional_conflict_severity(&self, int_framework: &NormativeFramework, local_framework: &NormativeFramework) -> ConflictSeverity {
        let scope_overlap = self.calculate_scope_overlap(int_framework, local_framework);
        let mandatory_conflicts = self.count_mandatory_requirement_conflicts(int_framework, local_framework);

        if mandatory_conflicts > 0 {
            ConflictSeverity::High
        } else if scope_overlap > 0.8 {
            ConflictSeverity::Medium
        } else if scope_overlap > 0.5 {
            ConflictSeverity::Low
        } else {
            ConflictSeverity::Informational
        }
    }

    fn count_mandatory_requirement_conflicts(&self, f1: &NormativeFramework, f2: &NormativeFramework) -> usize {
        let mut conflicts = 0;

        for req1 in &f1.requirements {
            if req1.mandatory {
                for req2 in &f2.requirements {
                    if req2.mandatory && req1.category == req2.category {
                        let similarity = self.semantic_analyzer.calculate_similarity(&req1.description, &req2.description);
                        if similarity > self.similarity_threshold {
                            for cond1 in &req1.conditions {
                                for cond2 in &req2.conditions {
                                    if self.conditions_contradict(cond1, cond2).unwrap_or(false) {
                                        conflicts += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        conflicts
    }

    fn build_conflict_context(
        &self,
        f1: &NormativeFramework,
        f2: &NormativeFramework,
        req1: &aion_core::Requirement,
        req2: &aion_core::Requirement,
    ) -> HashMap<String, String> {
        HashMap::from([
            ("framework_a_title".to_string(), f1.title.clone()),
            ("framework_b_title".to_string(), f2.title.clone()),
            ("requirement_a_title".to_string(), req1.title.clone()),
            ("requirement_b_title".to_string(), req2.title.clone()),
            ("requirement_a_mandatory".to_string(), req1.mandatory.to_string()),
            ("requirement_b_mandatory".to_string(), req2.mandatory.to_string()),
            ("similarity_score".to_string(),
             self.semantic_analyzer.calculate_similarity(&req1.description, &req2.description).to_string()),
        ])
    }

    fn severity_priority(&self, severity: &ConflictSeverity) -> u8 {
        match severity {
            ConflictSeverity::Critical => 5,
            ConflictSeverity::High => 4,
            ConflictSeverity::Medium => 3,
            ConflictSeverity::Low => 2,
            ConflictSeverity::Informational => 1,
        }
    }
}

impl Default for AdvancedConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ConflictDetector for AdvancedConflictDetector {
    fn detect_conflicts(&self, frameworks: &[NormativeFramework]) -> AionResult<Vec<NormativeConflict>> {
        let mut detector = Self::new();
        detector.detect_all_conflicts(frameworks)
    }

    fn analyze_conflict_severity(&self, conflict: &NormativeConflict) -> AionResult<()> {
        Ok(())
    }

    fn get_conflicting_frameworks(&self, _id: &NormativeId) -> AionResult<Vec<NormativeId>> {
        Ok(Vec::new())
    }
}

struct SemanticAnalyzer;

impl SemanticAnalyzer {
    fn new() -> Self {
        Self
    }

    fn calculate_similarity(&self, text1: &str, text2: &str) -> f64 {
        aion_core::calculate_similarity(text1, text2)
    }
}