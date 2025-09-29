use aion_core::types::*;
use aion_core::{AionResult};
use std::collections::HashMap;

/// Normative framework analyzer for complex compliance assessment
pub struct NormativeAnalyzer {
    cache: HashMap<String, AnalysisResult>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub complexity_score: f64,
    pub conflict_potential: f64,
    pub implementation_risk: f64,
    pub recommendations: Vec<String>,
}

impl NormativeAnalyzer {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn analyze_framework(&mut self, framework: &NormativeFramework) -> AionResult<AnalysisResult> {
        // Check cache first
        if let Some(cached) = self.cache.get(&framework.id.0.to_string()) {
            return Ok(cached.clone());
        }

        let complexity = self.calculate_complexity(framework);
        let conflict_potential = self.assess_conflict_potential(framework);
        let implementation_risk = self.evaluate_implementation_risk(framework);
        let recommendations = self.generate_recommendations(framework);

        let result = AnalysisResult {
            complexity_score: complexity,
            conflict_potential,
            implementation_risk,
            recommendations,
        };

        // Cache result
        self.cache.insert(framework.id.0.to_string(), result.clone());
        Ok(result)
    }

    fn calculate_complexity(&self, framework: &NormativeFramework) -> f64 {
        let requirement_count = framework.requirements.len() as f64;
        let mandatory_ratio = framework.requirements.iter()
            .filter(|r| r.mandatory)
            .count() as f64 / requirement_count.max(1.0);

        let avg_evidence_count = framework.requirements.iter()
            .map(|r| r.evidence_required.len())
            .sum::<usize>() as f64 / requirement_count.max(1.0);

        // Complexity formula: weighted combination of factors
        (requirement_count * 0.3 + mandatory_ratio * 0.4 + avg_evidence_count * 0.3).min(10.0)
    }

    fn assess_conflict_potential(&self, framework: &NormativeFramework) -> f64 {
        let mut conflict_score = 0.0;

        for requirement in &framework.requirements {
            // High priority mandatory requirements have higher conflict potential
            if requirement.mandatory && requirement.priority == 1 {
                conflict_score += 2.0;
            } else if requirement.mandatory {
                conflict_score += 1.0;
            }

            // Requirements with complex validation rules increase conflict potential
            conflict_score += requirement.validation_rules.len() as f64 * 0.5;
        }

        (conflict_score / framework.requirements.len().max(1) as f64).min(10.0_f64)
    }

    fn evaluate_implementation_risk(&self, framework: &NormativeFramework) -> f64 {
        let mut risk_score = 0.0;

        // Age of framework affects implementation risk
        let age_years = chrono::Utc::now().signed_duration_since(framework.effective_date).num_days() as f64 / 365.25;
        if age_years > 5.0 {
            risk_score += 1.0; // Older frameworks may need updates
        }

        // Complex requirements increase implementation risk
        for requirement in &framework.requirements {
            if requirement.evidence_required.len() > 3 {
                risk_score += 0.5;
            }
            if requirement.validation_rules.len() > 2 {
                risk_score += 0.3;
            }
        }

        (risk_score / framework.requirements.len().max(1) as f64).min(10.0_f64)
    }

    fn generate_recommendations(&self, framework: &NormativeFramework) -> Vec<String> {
        let mut recommendations = Vec::new();

        let mandatory_count = framework.requirements.iter().filter(|r| r.mandatory).count();
        let total_count = framework.requirements.len();

        if mandatory_count as f64 / total_count as f64 > 0.8 {
            recommendations.push("Consider phased implementation due to high mandatory requirement ratio".to_string());
        }

        if framework.requirements.iter().any(|r| r.evidence_required.len() > 5) {
            recommendations.push("Implement automated evidence collection for complex requirements".to_string());
        }

        if framework.requirements.iter().any(|r| r.validation_rules.len() > 3) {
            recommendations.push("Develop specialized validation tooling for complex rules".to_string());
        }

        recommendations.push("Regular compliance monitoring and automated reporting recommended".to_string());

        recommendations
    }

    pub fn compare_frameworks(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> AionResult<FrameworkComparison> {
        Ok(FrameworkComparison {
            similarity_score: self.calculate_similarity(framework1, framework2),
            common_requirements: self.find_common_requirements(framework1, framework2),
            unique_to_first: self.find_unique_requirements(framework1, framework2),
            unique_to_second: self.find_unique_requirements(framework2, framework1),
        })
    }

    fn calculate_similarity(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> f64 {
        // Simple similarity based on requirement categories
        let categories1: std::collections::HashSet<_> = framework1.requirements.iter()
            .map(|r| &r.category)
            .collect();
        let categories2: std::collections::HashSet<_> = framework2.requirements.iter()
            .map(|r| &r.category)
            .collect();

        let intersection = categories1.intersection(&categories2).count();
        let union = categories1.union(&categories2).count();

        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }

    fn find_common_requirements(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> Vec<String> {
        let mut common = Vec::new();

        for req1 in &framework1.requirements {
            for req2 in &framework2.requirements {
                if req1.category == req2.category && self.requirements_similar(&req1.title, &req2.title) {
                    common.push(format!("{} (similar to {})", req1.title, req2.title));
                }
            }
        }

        common
    }

    fn find_unique_requirements(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> Vec<String> {
        let categories2: std::collections::HashSet<_> = framework2.requirements.iter()
            .map(|r| &r.category)
            .collect();

        framework1.requirements.iter()
            .filter(|req| !categories2.contains(&req.category))
            .map(|req| req.title.clone())
            .collect()
    }

    fn requirements_similar(&self, title1: &str, title2: &str) -> bool {
        // Simple similarity check - could be enhanced with NLP
        let title1_lower = title1.to_lowercase();
        let title2_lower = title2.to_lowercase();
        let words1: std::collections::HashSet<_> = title1_lower.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = title2_lower.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 { false } else { intersection as f64 / union as f64 > 0.3 }
    }
}

#[derive(Debug, Clone)]
pub struct FrameworkComparison {
    pub similarity_score: f64,
    pub common_requirements: Vec<String>,
    pub unique_to_first: Vec<String>,
    pub unique_to_second: Vec<String>,
}

impl Default for NormativeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}