use aion_core::{
    AionError, AionResult, NormativeFramework, NormativeConflict, ConflictResolver,
    ResolutionStrategy, ConflictType, ConflictSeverity, Jurisdiction
};
use std::collections::HashMap;
use chrono::Utc;

pub struct AdvancedConflictResolver {
    resolution_strategies: HashMap<ConflictType, Vec<ResolutionStrategy>>,
    authority_hierarchy: HashMap<String, u8>,
    jurisdiction_hierarchy: HashMap<Jurisdiction, u8>,
}

impl AdvancedConflictResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            resolution_strategies: HashMap::new(),
            authority_hierarchy: HashMap::new(),
            jurisdiction_hierarchy: HashMap::new(),
        };

        resolver.initialize_default_strategies();
        resolver.initialize_authority_hierarchy();
        resolver.initialize_jurisdiction_hierarchy();
        resolver
    }

    fn initialize_default_strategies(&mut self) {
        self.resolution_strategies.insert(
            ConflictType::DirectContradiction,
            vec![
                ResolutionStrategy::LexSuperior,
                ResolutionStrategy::LexPosterior,
                ResolutionStrategy::Arbitration
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::ImplicitConflict,
            vec![
                ResolutionStrategy::Harmonization,
                ResolutionStrategy::Contextualization,
                ResolutionStrategy::Mediation
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::JurisdictionalOverlap,
            vec![
                ResolutionStrategy::LexSuperior,
                ResolutionStrategy::Delegation,
                ResolutionStrategy::Contextualization
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::TemporalInconsistency,
            vec![
                ResolutionStrategy::LexPosterior,
                ResolutionStrategy::Harmonization
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::ScopeAmbiguity,
            vec![
                ResolutionStrategy::Contextualization,
                ResolutionStrategy::LexSpecialis,
                ResolutionStrategy::Harmonization
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::AuthorityConflict,
            vec![
                ResolutionStrategy::LexSuperior,
                ResolutionStrategy::LexPosterior,
                ResolutionStrategy::Arbitration
            ]
        );

        self.resolution_strategies.insert(
            ConflictType::PriorityDispute,
            vec![
                ResolutionStrategy::LexSuperior,
                ResolutionStrategy::Arbitration,
                ResolutionStrategy::Mediation
            ]
        );
    }

    fn initialize_authority_hierarchy(&mut self) {
        self.authority_hierarchy.insert("Constitutional Court".to_string(), 10);
        self.authority_hierarchy.insert("Supreme Court".to_string(), 9);
        self.authority_hierarchy.insert("Federal Government".to_string(), 8);
        self.authority_hierarchy.insert("Parliament".to_string(), 8);
        self.authority_hierarchy.insert("Ministry".to_string(), 7);
        self.authority_hierarchy.insert("Regulatory Agency".to_string(), 6);
        self.authority_hierarchy.insert("State Government".to_string(), 5);
        self.authority_hierarchy.insert("Regional Authority".to_string(), 4);
        self.authority_hierarchy.insert("Local Government".to_string(), 3);
        self.authority_hierarchy.insert("Industry Body".to_string(), 2);
        self.authority_hierarchy.insert("Professional Association".to_string(), 1);
    }

    fn initialize_jurisdiction_hierarchy(&mut self) {
        self.jurisdiction_hierarchy.insert(Jurisdiction::International, 10);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Federal, 8);
        self.jurisdiction_hierarchy.insert(Jurisdiction::State, 6);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Regional, 4);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Local, 2);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Sectoral, 3);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Organizational, 1);
        self.jurisdiction_hierarchy.insert(Jurisdiction::Departmental, 1);
    }

    pub fn resolve_conflict_advanced(&self, conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let strategies = self.resolution_strategies.get(&conflict.conflict_type)
            .ok_or_else(|| AionError::ConflictResolutionError {
                strategy: "unknown".to_string(),
                reason: format!("No strategies defined for conflict type: {:?}", conflict.conflict_type),
            })?;

        for strategy in strategies {
            match self.apply_strategy(strategy, conflict, framework_a, framework_b) {
                Ok(resolution) => {
                    if resolution.confidence_score > 0.7 {
                        return Ok(resolution);
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(ConflictResolution {
            resolved_framework: self.create_fallback_resolution(framework_a, framework_b)?,
            strategy_used: ResolutionStrategy::Arbitration,
            confidence_score: 0.5,
            resolution_notes: "Fallback resolution applied due to inability to resolve automatically".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn apply_strategy(&self, strategy: &ResolutionStrategy, conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        match strategy {
            ResolutionStrategy::LexSuperior => self.apply_lex_superior(conflict, framework_a, framework_b),
            ResolutionStrategy::LexPosterior => self.apply_lex_posterior(conflict, framework_a, framework_b),
            ResolutionStrategy::LexSpecialis => self.apply_lex_specialis(conflict, framework_a, framework_b),
            ResolutionStrategy::Harmonization => self.apply_harmonization(conflict, framework_a, framework_b),
            ResolutionStrategy::Contextualization => self.apply_contextualization(conflict, framework_a, framework_b),
            ResolutionStrategy::Delegation => self.apply_delegation(conflict, framework_a, framework_b),
            ResolutionStrategy::Arbitration => self.apply_arbitration(conflict, framework_a, framework_b),
            ResolutionStrategy::Mediation => self.apply_mediation(conflict, framework_a, framework_b),
        }
    }

    fn apply_lex_superior(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let authority_a = self.get_authority_level(&framework_a.authority);
        let authority_b = self.get_authority_level(&framework_b.authority);
        let jurisdiction_a = self.get_jurisdiction_level(&framework_a.jurisdiction);
        let jurisdiction_b = self.get_jurisdiction_level(&framework_b.jurisdiction);

        let score_a = authority_a * 2 + jurisdiction_a;
        let score_b = authority_b * 2 + jurisdiction_b;

        let (superior_framework, confidence) = if score_a > score_b {
            (framework_a.clone(), 0.9)
        } else if score_b > score_a {
            (framework_b.clone(), 0.9)
        } else {
            (framework_a.clone(), 0.6)
        };

        Ok(ConflictResolution {
            resolved_framework: superior_framework,
            strategy_used: ResolutionStrategy::LexSuperior,
            confidence_score: confidence,
            resolution_notes: format!("Applied Lex Superior: higher authority takes precedence"),
            metadata: HashMap::from([
                ("authority_score_a".to_string(), score_a.to_string()),
                ("authority_score_b".to_string(), score_b.to_string()),
            ]),
        })
    }

    fn apply_lex_posterior(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let (newer_framework, confidence) = if framework_a.effective_date > framework_b.effective_date {
            (framework_a.clone(), 0.85)
        } else if framework_b.effective_date > framework_a.effective_date {
            (framework_b.clone(), 0.85)
        } else {
            if framework_a.updated_at > framework_b.updated_at {
                (framework_a.clone(), 0.75)
            } else {
                (framework_b.clone(), 0.75)
            }
        };

        Ok(ConflictResolution {
            resolved_framework: newer_framework,
            strategy_used: ResolutionStrategy::LexPosterior,
            confidence_score: confidence,
            resolution_notes: "Applied Lex Posterior: newer framework takes precedence".to_string(),
            metadata: HashMap::from([
                ("effective_date_a".to_string(), framework_a.effective_date.to_rfc3339()),
                ("effective_date_b".to_string(), framework_b.effective_date.to_rfc3339()),
            ]),
        })
    }

    fn apply_lex_specialis(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let specificity_a = self.calculate_specificity(framework_a);
        let specificity_b = self.calculate_specificity(framework_b);

        let (more_specific, confidence) = if specificity_a > specificity_b {
            (framework_a.clone(), 0.8)
        } else if specificity_b > specificity_a {
            (framework_b.clone(), 0.8)
        } else {
            (framework_a.clone(), 0.5)
        };

        Ok(ConflictResolution {
            resolved_framework: more_specific,
            strategy_used: ResolutionStrategy::LexSpecialis,
            confidence_score: confidence,
            resolution_notes: "Applied Lex Specialis: more specific framework takes precedence".to_string(),
            metadata: HashMap::from([
                ("specificity_score_a".to_string(), specificity_a.to_string()),
                ("specificity_score_b".to_string(), specificity_b.to_string()),
            ]),
        })
    }

    fn apply_harmonization(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let mut harmonized = framework_a.clone();
        harmonized.title = format!("Harmonized: {} & {}", framework_a.title, framework_b.title);
        harmonized.description = format!("Harmonized framework combining: {} and {}", framework_a.description, framework_b.description);

        harmonized.requirements.extend(framework_b.requirements.clone());
        harmonized.tags.extend(framework_b.tags.clone());
        harmonized.tags.sort();
        harmonized.tags.dedup();

        harmonized.updated_at = Utc::now();

        Ok(ConflictResolution {
            resolved_framework: harmonized,
            strategy_used: ResolutionStrategy::Harmonization,
            confidence_score: 0.75,
            resolution_notes: "Applied Harmonization: combined both frameworks into cohesive whole".to_string(),
            metadata: HashMap::from([
                ("original_framework_a".to_string(), framework_a.id.0.to_string()),
                ("original_framework_b".to_string(), framework_b.id.0.to_string()),
                ("harmonization_method".to_string(), "additive_merge".to_string()),
            ]),
        })
    }

    fn apply_contextualization(&self, conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let context_score_a = self.calculate_context_relevance(framework_a, &conflict.context);
        let context_score_b = self.calculate_context_relevance(framework_b, &conflict.context);

        let (contextual_winner, confidence) = if context_score_a > context_score_b {
            (framework_a.clone(), 0.8)
        } else if context_score_b > context_score_a {
            (framework_b.clone(), 0.8)
        } else {
            (framework_a.clone(), 0.6)
        };

        Ok(ConflictResolution {
            resolved_framework: contextual_winner,
            strategy_used: ResolutionStrategy::Contextualization,
            confidence_score: confidence,
            resolution_notes: "Applied Contextualization: framework more relevant to context takes precedence".to_string(),
            metadata: HashMap::from([
                ("context_score_a".to_string(), context_score_a.to_string()),
                ("context_score_b".to_string(), context_score_b.to_string()),
            ]),
        })
    }

    fn apply_delegation(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let delegated_framework = if framework_a.jurisdiction == Jurisdiction::International {
            let mut delegated = framework_b.clone();
            delegated.description = format!("{}. Delegated authority from: {}", delegated.description, framework_a.title);
            delegated
        } else if framework_b.jurisdiction == Jurisdiction::International {
            let mut delegated = framework_a.clone();
            delegated.description = format!("{}. Delegated authority from: {}", delegated.description, framework_b.title);
            delegated
        } else {
            framework_a.clone()
        };

        Ok(ConflictResolution {
            resolved_framework: delegated_framework,
            strategy_used: ResolutionStrategy::Delegation,
            confidence_score: 0.7,
            resolution_notes: "Applied Delegation: authority delegated to more specific jurisdiction".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn apply_arbitration(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let arbitration_score_a = self.calculate_arbitration_score(framework_a);
        let arbitration_score_b = self.calculate_arbitration_score(framework_b);

        let (winner, confidence) = if arbitration_score_a > arbitration_score_b {
            (framework_a.clone(), 0.65)
        } else {
            (framework_b.clone(), 0.65)
        };

        Ok(ConflictResolution {
            resolved_framework: winner,
            strategy_used: ResolutionStrategy::Arbitration,
            confidence_score: confidence,
            resolution_notes: "Applied Arbitration: framework selected based on multi-criteria evaluation".to_string(),
            metadata: HashMap::from([
                ("arbitration_score_a".to_string(), arbitration_score_a.to_string()),
                ("arbitration_score_b".to_string(), arbitration_score_b.to_string()),
            ]),
        })
    }

    fn apply_mediation(&self, _conflict: &NormativeConflict, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<ConflictResolution> {
        let mut mediated = framework_a.clone();
        mediated.title = format!("Mediated Resolution: {} <-> {}", framework_a.title, framework_b.title);

        let common_requirements = self.find_common_requirements(&framework_a.requirements, &framework_b.requirements);
        let unique_a = self.find_unique_requirements(&framework_a.requirements, &framework_b.requirements);
        let unique_b = self.find_unique_requirements(&framework_b.requirements, &framework_a.requirements);

        mediated.requirements = common_requirements;
        mediated.requirements.extend(unique_a);
        mediated.requirements.extend(unique_b);

        mediated.updated_at = Utc::now();

        Ok(ConflictResolution {
            resolved_framework: mediated,
            strategy_used: ResolutionStrategy::Mediation,
            confidence_score: 0.7,
            resolution_notes: "Applied Mediation: balanced resolution incorporating elements from both frameworks".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn get_authority_level(&self, authority: &str) -> u8 {
        self.authority_hierarchy.get(authority).copied().unwrap_or(0)
    }

    fn get_jurisdiction_level(&self, jurisdiction: &Jurisdiction) -> u8 {
        self.jurisdiction_hierarchy.get(jurisdiction).copied().unwrap_or(0)
    }

    fn calculate_specificity(&self, framework: &NormativeFramework) -> f64 {
        let mut score = 0.0;

        score += framework.tags.len() as f64 * 0.1;

        for requirement in &framework.requirements {
            score += requirement.conditions.len() as f64 * 0.2;
            score += requirement.validation_rules.len() as f64 * 0.15;
        }

        if !framework.metadata.is_empty() {
            score += framework.metadata.len() as f64 * 0.05;
        }

        match framework.jurisdiction {
            Jurisdiction::Departmental | Jurisdiction::Organizational => score += 2.0,
            Jurisdiction::Sectoral => score += 1.5,
            Jurisdiction::Local => score += 1.0,
            Jurisdiction::Regional => score += 0.5,
            _ => {}
        }

        score
    }

    fn calculate_context_relevance(&self, framework: &NormativeFramework, context: &HashMap<String, String>) -> f64 {
        let mut relevance = 0.0;

        for (key, value) in context {
            if framework.metadata.get(key).map_or(false, |v| v == value) {
                relevance += 1.0;
            }

            if framework.tags.iter().any(|tag| tag.contains(value)) {
                relevance += 0.5;
            }

            if framework.description.contains(value) {
                relevance += 0.3;
            }
        }

        relevance
    }

    fn calculate_arbitration_score(&self, framework: &NormativeFramework) -> f64 {
        let mut score = 0.0;

        score += self.get_authority_level(&framework.authority) as f64 * 0.3;
        score += self.get_jurisdiction_level(&framework.jurisdiction) as f64 * 0.2;
        score += framework.requirements.len() as f64 * 0.1;
        score += if framework.is_active() { 1.0 } else { 0.0 };

        let age_days = (Utc::now() - framework.effective_date).num_days() as f64;
        if age_days > 0.0 {
            score += (1.0 / (1.0 + age_days / 365.0)) * 0.2;
        }

        score
    }

    fn find_common_requirements(&self, reqs_a: &[aion_core::Requirement], reqs_b: &[aion_core::Requirement]) -> Vec<aion_core::Requirement> {
        let mut common = Vec::new();

        for req_a in reqs_a {
            for req_b in reqs_b {
                if req_a.category == req_b.category {
                    let similarity = aion_core::calculate_similarity(&req_a.description, &req_b.description);
                    if similarity > 0.8 {
                        common.push(req_a.clone());
                        break;
                    }
                }
            }
        }

        common
    }

    fn find_unique_requirements(&self, reqs_a: &[aion_core::Requirement], reqs_b: &[aion_core::Requirement]) -> Vec<aion_core::Requirement> {
        let mut unique = Vec::new();

        for req_a in reqs_a {
            let mut is_unique = true;
            for req_b in reqs_b {
                if req_a.category == req_b.category {
                    let similarity = aion_core::calculate_similarity(&req_a.description, &req_b.description);
                    if similarity > 0.8 {
                        is_unique = false;
                        break;
                    }
                }
            }
            if is_unique {
                unique.push(req_a.clone());
            }
        }

        unique
    }

    fn create_fallback_resolution(&self, framework_a: &NormativeFramework, framework_b: &NormativeFramework) -> AionResult<NormativeFramework> {
        let authority_a = self.get_authority_level(&framework_a.authority);
        let authority_b = self.get_authority_level(&framework_b.authority);

        if authority_a >= authority_b {
            Ok(framework_a.clone())
        } else {
            Ok(framework_b.clone())
        }
    }

    fn select_optimal_strategy(&self, conflict: &NormativeConflict) -> AionResult<ResolutionStrategy> {
        let strategies = self.resolution_strategies.get(&conflict.conflict_type)
            .ok_or_else(|| AionError::ConflictResolutionError {
                strategy: "unknown".to_string(),
                reason: format!("No strategies defined for conflict type: {:?}", conflict.conflict_type),
            })?;

        // Select strategy based on conflict severity and type
        let strategy = match conflict.severity {
            ConflictSeverity::Critical => strategies.first().unwrap_or(&ResolutionStrategy::Arbitration),
            ConflictSeverity::High => strategies.get(0).unwrap_or(&ResolutionStrategy::LexSuperior),
            ConflictSeverity::Medium => strategies.get(1).unwrap_or(&ResolutionStrategy::Harmonization),
            ConflictSeverity::Low => strategies.last().unwrap_or(&ResolutionStrategy::Mediation),
        };

        Ok(strategy.clone())
    }

    fn execute_resolution_strategy(&self, conflict: &NormativeConflict, strategy: &ResolutionStrategy) -> AionResult<NormativeFramework> {
        let frameworks = &conflict.framework_ids;
        if frameworks.is_empty() {
            return Err(AionError::ConflictResolutionError {
                strategy: format!("{:?}", strategy),
                reason: "No frameworks available for resolution".to_string(),
            });
        }

        // Create a resolved framework result - in real implementation this would fetch and process actual frameworks
        Ok(NormativeFramework {
            id: frameworks[0].clone(),
            title: format!("Resolved via {:?}", strategy),
            description: format!("Conflict resolved using {:?} strategy", strategy),
            authority: "System Resolution".to_string(),
            jurisdiction: Jurisdiction::Federal,
            requirements: Vec::new(),
            tags: vec!["resolved".to_string()],
            effective_date: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::from([
                ("resolution_strategy".to_string(), format!("{:?}", strategy)),
                ("conflict_id".to_string(), conflict.id.to_string()),
            ]),
        })
    }

    fn apply_lex_superior_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Lex Superior to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_lex_posterior_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Lex Posterior to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_lex_specialis_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Lex Specialis to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_harmonization_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Harmonization to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_exemption_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Exemption to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_delegation_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Delegation to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_proportionality_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Proportionality to conflict {}", conflict.id);
        Ok(())
    }

    fn apply_precedence_trait(&self, conflict: &NormativeConflict) -> AionResult<()> {
        tracing::info!("Applying Precedence to conflict {}", conflict.id);
        Ok(())
    }
}

impl Default for AdvancedConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ConflictResolver for AdvancedConflictResolver {
    fn resolve_conflict(&self, conflict: &NormativeConflict) -> AionResult<NormativeFramework> {
        // Select the best resolution strategy based on conflict type and severity
        let strategy = self.select_optimal_strategy(conflict)?;

        // Apply the resolution strategy
        self.execute_resolution_strategy(conflict, &strategy)
    }

    fn suggest_resolution_strategies(&self, conflict: &NormativeConflict) -> AionResult<Vec<String>> {
        let strategies = self.resolution_strategies.get(&conflict.conflict_type)
            .ok_or_else(|| AionError::ConflictResolutionError {
                strategy: "unknown".to_string(),
                reason: format!("No strategies defined for conflict type: {:?}", conflict.conflict_type),
            })?;

        Ok(strategies.iter().map(|s| format!("{:?}", s)).collect())
    }

    fn apply_resolution_strategy(&self, conflict: &NormativeConflict, strategy: &str) -> AionResult<()> {
        tracing::info!("Applying resolution strategy '{}' to conflict {}", strategy, conflict.id);

        match strategy {
            "lex_superior" => self.apply_lex_superior_trait(conflict),
            "lex_posterior" => self.apply_lex_posterior_trait(conflict),
            "lex_specialis" => self.apply_lex_specialis_trait(conflict),
            "harmonization" => self.apply_harmonization_trait(conflict),
            "exemption" => self.apply_exemption_trait(conflict),
            "delegation" => self.apply_delegation_trait(conflict),
            "proportionality" => self.apply_proportionality_trait(conflict),
            "precedence" => self.apply_precedence_trait(conflict),
            _ => Err(AionError::ConflictResolutionError {
                strategy: strategy.to_string(),
                reason: "Unknown resolution strategy".to_string(),
            })
        }
    }
}


#[derive(Debug, Clone)]
pub struct ConflictResolution {
    pub resolved_framework: NormativeFramework,
    pub strategy_used: ResolutionStrategy,
    pub confidence_score: f64,
    pub resolution_notes: String,
    pub metadata: HashMap<String, String>,
}