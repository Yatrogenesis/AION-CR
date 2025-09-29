use aion_core::types::*;
use aion_core::{AionResult, AionError, Jurisdiction};
use std::collections::{HashMap, HashSet};

/// Hierarchical framework management for complex regulatory environments
pub struct HierarchyManager {
    hierarchy_cache: HashMap<String, FrameworkHierarchy>,
    jurisdiction_map: HashMap<Jurisdiction, Vec<NormativeId>>,
}

#[derive(Debug, Clone)]
pub struct FrameworkHierarchy {
    pub root_framework: NormativeId,
    pub parent_frameworks: Vec<NormativeId>,
    pub child_frameworks: Vec<NormativeId>,
    pub precedence_order: Vec<NormativeId>,
    pub jurisdiction_level: JurisdictionLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JurisdictionLevel {
    International,
    Regional,
    National,
    Subnational,
    Organizational,
}

#[derive(Debug, Clone)]
pub struct ConflictResolution {
    pub strategy: ResolutionStrategy,
    pub primary_framework: NormativeId,
    pub secondary_frameworks: Vec<NormativeId>,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub enum ResolutionStrategy {
    HigherJurisdictionPrecedence,
    MoreSpecificFramework,
    NewerVersionPrecedence,
    StricterRequirement,
    CustomResolution(String),
}

impl HierarchyManager {
    pub fn new() -> Self {
        Self {
            hierarchy_cache: HashMap::new(),
            jurisdiction_map: HashMap::new(),
        }
    }

    pub fn build_hierarchy(&mut self, frameworks: &[NormativeFramework]) -> AionResult<()> {
        // Clear existing mappings
        self.jurisdiction_map.clear();

        // Group frameworks by jurisdiction
        for framework in frameworks {
            self.jurisdiction_map
                .entry(framework.jurisdiction.clone())
                .or_insert_with(Vec::new)
                .push(framework.id.clone());
        }

        // Build hierarchical relationships
        for framework in frameworks {
            let hierarchy = self.analyze_framework_hierarchy(framework, frameworks)?;
            self.hierarchy_cache.insert(framework.id.0.to_string(), hierarchy);
        }

        Ok(())
    }

    fn analyze_framework_hierarchy(
        &self,
        framework: &NormativeFramework,
        all_frameworks: &[NormativeFramework],
    ) -> AionResult<FrameworkHierarchy> {
        let jurisdiction_level = self.determine_jurisdiction_level(&framework.jurisdiction);
        let mut parent_frameworks = Vec::new();
        let mut child_frameworks = Vec::new();

        // Find parent frameworks (higher jurisdiction level or related)
        for other in all_frameworks {
            if other.id == framework.id {
                continue;
            }

            let other_level = self.determine_jurisdiction_level(&other.jurisdiction);

            // Check for hierarchical relationships
            if self.is_parent_framework(framework, other, &jurisdiction_level, &other_level) {
                parent_frameworks.push(other.id.clone());
            } else if self.is_child_framework(framework, other, &jurisdiction_level, &other_level) {
                child_frameworks.push(other.id.clone());
            }
        }

        // Determine precedence order
        let precedence_order = self.calculate_precedence_order(framework, all_frameworks)?;

        Ok(FrameworkHierarchy {
            root_framework: framework.id.clone(),
            parent_frameworks,
            child_frameworks,
            precedence_order,
            jurisdiction_level,
        })
    }

    fn determine_jurisdiction_level(&self, jurisdiction: &Jurisdiction) -> JurisdictionLevel {
        match jurisdiction {
            Jurisdiction::International => JurisdictionLevel::International,
            Jurisdiction::Regional => JurisdictionLevel::Regional,
            Jurisdiction::Federal => JurisdictionLevel::National,
            Jurisdiction::State => JurisdictionLevel::Subnational,
            Jurisdiction::Local => JurisdictionLevel::Subnational,
            Jurisdiction::Organizational => JurisdictionLevel::Organizational,
            Jurisdiction::Sectoral => JurisdictionLevel::Organizational,
            Jurisdiction::Departmental => JurisdictionLevel::Organizational,
        }
    }

    fn is_parent_framework(
        &self,
        framework: &NormativeFramework,
        potential_parent: &NormativeFramework,
        framework_level: &JurisdictionLevel,
        parent_level: &JurisdictionLevel,
    ) -> bool {
        // Higher jurisdiction levels are typically parents
        match (parent_level, framework_level) {
            (JurisdictionLevel::International, JurisdictionLevel::Regional) => true,
            (JurisdictionLevel::International, JurisdictionLevel::National) => true,
            (JurisdictionLevel::Regional, JurisdictionLevel::National) => true,
            (JurisdictionLevel::National, JurisdictionLevel::Subnational) => true,
            _ => {
                // Check for sector-specific relationships
                self.has_sector_relationship(framework, potential_parent)
            }
        }
    }

    fn is_child_framework(
        &self,
        framework: &NormativeFramework,
        potential_child: &NormativeFramework,
        framework_level: &JurisdictionLevel,
        child_level: &JurisdictionLevel,
    ) -> bool {
        // Lower jurisdiction levels are typically children
        match (framework_level, child_level) {
            (JurisdictionLevel::International, JurisdictionLevel::Regional) => true,
            (JurisdictionLevel::International, JurisdictionLevel::National) => true,
            (JurisdictionLevel::Regional, JurisdictionLevel::National) => true,
            (JurisdictionLevel::National, JurisdictionLevel::Subnational) => true,
            _ => {
                // Check for specialization relationships
                self.has_specialization_relationship(framework, potential_child)
            }
        }
    }

    fn has_sector_relationship(&self, framework: &NormativeFramework, other: &NormativeFramework) -> bool {
        // Check if frameworks are related by sector or domain
        let framework_title_lower = framework.title.to_lowercase();
        let other_title_lower = other.title.to_lowercase();
        let framework_keywords: HashSet<_> = framework_title_lower
            .split_whitespace()
            .collect();
        let other_keywords: HashSet<_> = other_title_lower
            .split_whitespace()
            .collect();

        let intersection = framework_keywords.intersection(&other_keywords).count();
        intersection > 0 && framework.effective_date < other.effective_date
    }

    fn has_specialization_relationship(&self, framework: &NormativeFramework, other: &NormativeFramework) -> bool {
        // Check if other framework is a specialization of this one
        let framework_categories: HashSet<_> = framework.requirements.iter()
            .map(|r| &r.category)
            .collect();
        let other_categories: HashSet<_> = other.requirements.iter()
            .map(|r| &r.category)
            .collect();

        // If other framework covers a subset of categories, it might be a specialization
        other_categories.is_subset(&framework_categories) && other_categories.len() < framework_categories.len()
    }

    fn calculate_precedence_order(
        &self,
        framework: &NormativeFramework,
        all_frameworks: &[NormativeFramework],
    ) -> AionResult<Vec<NormativeId>> {
        let mut precedence = Vec::new();
        let mut frameworks_by_priority: Vec<_> = all_frameworks.iter()
            .filter(|f| f.id != framework.id)
            .collect();

        // Sort by jurisdiction level first, then by effective date
        frameworks_by_priority.sort_by(|a, b| {
            let a_level = self.determine_jurisdiction_level(&a.jurisdiction);
            let b_level = self.determine_jurisdiction_level(&b.jurisdiction);

            match self.compare_jurisdiction_levels(&a_level, &b_level) {
                std::cmp::Ordering::Equal => b.effective_date.cmp(&a.effective_date), // Newer first
                other => other,
            }
        });

        for framework in frameworks_by_priority {
            precedence.push(framework.id.clone());
        }

        Ok(precedence)
    }

    fn compare_jurisdiction_levels(&self, a: &JurisdictionLevel, b: &JurisdictionLevel) -> std::cmp::Ordering {
        let a_priority = match a {
            JurisdictionLevel::International => 0,
            JurisdictionLevel::Regional => 1,
            JurisdictionLevel::National => 2,
            JurisdictionLevel::Subnational => 3,
            JurisdictionLevel::Organizational => 4,
        };

        let b_priority = match b {
            JurisdictionLevel::International => 0,
            JurisdictionLevel::Regional => 1,
            JurisdictionLevel::National => 2,
            JurisdictionLevel::Subnational => 3,
            JurisdictionLevel::Organizational => 4,
        };

        a_priority.cmp(&b_priority)
    }

    pub fn resolve_conflicts(
        &self,
        conflicting_frameworks: &[NormativeId],
        all_frameworks: &[NormativeFramework],
    ) -> AionResult<ConflictResolution> {
        if conflicting_frameworks.is_empty() {
            return Err(AionError::ValidationError { field: "frameworks".to_string(), message: "No conflicting frameworks provided".to_string() });
        }

        // Find framework details
        let frameworks: Vec<_> = conflicting_frameworks.iter()
            .filter_map(|id| all_frameworks.iter().find(|f| &f.id == id))
            .collect();

        if frameworks.is_empty() {
            return Err(AionError::ValidationError { field: "frameworks".to_string(), message: "No valid frameworks found".to_string() });
        }

        // Apply resolution strategy
        let resolution = self.apply_resolution_strategy(&frameworks)?;
        Ok(resolution)
    }

    fn apply_resolution_strategy(&self, frameworks: &[&NormativeFramework]) -> AionResult<ConflictResolution> {
        // Strategy 1: Higher jurisdiction precedence
        let mut frameworks_by_jurisdiction = frameworks.to_vec();
        frameworks_by_jurisdiction.sort_by(|a, b| {
            let a_level = self.determine_jurisdiction_level(&a.jurisdiction);
            let b_level = self.determine_jurisdiction_level(&b.jurisdiction);
            self.compare_jurisdiction_levels(&a_level, &b_level)
        });

        let primary = frameworks_by_jurisdiction[0];
        let secondary: Vec<_> = frameworks_by_jurisdiction[1..].iter().map(|f| f.id.clone()).collect();

        Ok(ConflictResolution {
            strategy: ResolutionStrategy::HigherJurisdictionPrecedence,
            primary_framework: primary.id.clone(),
            secondary_frameworks: secondary,
            reasoning: format!(
                "Framework '{}' takes precedence due to higher jurisdiction level ({:?})",
                primary.title,
                self.determine_jurisdiction_level(&primary.jurisdiction)
            ),
        })
    }

    pub fn get_applicable_frameworks(
        &self,
        jurisdiction: &Jurisdiction,
        sector: Option<&str>,
    ) -> Vec<NormativeId> {
        let mut applicable = Vec::new();

        // Add frameworks for exact jurisdiction match
        if let Some(frameworks) = self.jurisdiction_map.get(jurisdiction) {
            applicable.extend(frameworks.clone());
        }

        // Add higher-level frameworks that might apply
        match jurisdiction {
            Jurisdiction::State | Jurisdiction::Local => {
                if let Some(federal) = self.jurisdiction_map.get(&Jurisdiction::Federal) {
                    applicable.extend(federal.clone());
                }
                if let Some(regional) = self.jurisdiction_map.get(&Jurisdiction::Regional) {
                    applicable.extend(regional.clone());
                }
                if let Some(international) = self.jurisdiction_map.get(&Jurisdiction::International) {
                    applicable.extend(international.clone());
                }
            }
            Jurisdiction::Federal => {
                if let Some(regional) = self.jurisdiction_map.get(&Jurisdiction::Regional) {
                    applicable.extend(regional.clone());
                }
                if let Some(international) = self.jurisdiction_map.get(&Jurisdiction::International) {
                    applicable.extend(international.clone());
                }
            }
            Jurisdiction::Regional => {
                if let Some(international) = self.jurisdiction_map.get(&Jurisdiction::International) {
                    applicable.extend(international.clone());
                }
            }
            _ => {}
        }

        applicable
    }

    pub fn get_hierarchy(&self, framework_id: &NormativeId) -> Option<&FrameworkHierarchy> {
        self.hierarchy_cache.get(&framework_id.0.to_string())
    }
}

impl Default for HierarchyManager {
    fn default() -> Self {
        Self::new()
    }
}