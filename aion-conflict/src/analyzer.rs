use aion_core::types::*;
use aion_core::{AionResult};

pub struct ConflictAnalyzer {
    analysis_cache: std::collections::HashMap<String, ConflictAnalysis>,
}

#[derive(Debug, Clone)]
pub struct ConflictAnalysis {
    pub severity_score: f64,
    pub impact_assessment: String,
    pub resolution_complexity: ResolutionComplexity,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ResolutionComplexity {
    Simple,
    Moderate,
    Complex,
    RequiresIntervention,
}

impl ConflictAnalyzer {
    pub fn new() -> Self {
        Self {
            analysis_cache: std::collections::HashMap::new(),
        }
    }

    pub fn analyze_conflict(&mut self, conflict: &NormativeConflict) -> AionResult<ConflictAnalysis> {
        let severity_score = self.calculate_severity(conflict);
        let impact_assessment = self.assess_impact(conflict);
        let complexity = self.determine_complexity(conflict);
        let recommendations = self.generate_recommendations(conflict);

        let analysis = ConflictAnalysis {
            severity_score,
            impact_assessment,
            resolution_complexity: complexity,
            recommendations,
        };

        self.analysis_cache.insert(conflict.id.to_string(), analysis.clone());
        Ok(analysis)
    }

    fn calculate_severity(&self, conflict: &NormativeConflict) -> f64 {
        match conflict.conflict_type {
            ConflictType::DirectContradiction => 9.0,
            ConflictType::ImplicitConflict => 6.0,
            ConflictType::JurisdictionalOverlap => 7.0,
            ConflictType::TemporalInconsistency => 5.0,
            ConflictType::ScopeAmbiguity => 4.0,
            ConflictType::AuthorityConflict => 8.0,
            ConflictType::RequirementConflict => 7.5,
            ConflictType::ImplementationConflict => 6.5,
        }
    }

    fn assess_impact(&self, conflict: &NormativeConflict) -> String {
        format!("Conflict between {} frameworks with {} severity",
                conflict.involved_frameworks.len(),
                match self.calculate_severity(conflict) {
                    x if x > 8.0 => "critical",
                    x if x > 6.0 => "high",
                    x if x > 4.0 => "medium",
                    _ => "low"
                })
    }

    fn determine_complexity(&self, conflict: &NormativeConflict) -> ResolutionComplexity {
        let severity = self.calculate_severity(conflict);
        let framework_count = conflict.involved_frameworks.len();

        match (severity, framework_count) {
            (s, c) if s > 8.0 || c > 3 => ResolutionComplexity::RequiresIntervention,
            (s, c) if s > 6.0 || c > 2 => ResolutionComplexity::Complex,
            (s, _) if s > 4.0 => ResolutionComplexity::Moderate,
            _ => ResolutionComplexity::Simple,
        }
    }

    fn generate_recommendations(&self, conflict: &NormativeConflict) -> Vec<String> {
        let mut recommendations = Vec::new();

        match conflict.conflict_type {
            ConflictType::DirectContradiction => {
                recommendations.push("Immediate review and resolution required".to_string());
                recommendations.push("Consider stakeholder consultation".to_string());
            }
            ConflictType::JurisdictionalOverlap => {
                recommendations.push("Clarify jurisdictional boundaries".to_string());
                recommendations.push("Establish precedence rules".to_string());
            }
            _ => {
                recommendations.push("Regular monitoring recommended".to_string());
            }
        }

        recommendations
    }
}

impl Default for ConflictAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}