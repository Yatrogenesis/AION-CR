use aion_core::{
    AionResult, AionError, NormativeFramework, NormativeId, NormativeRepository,
    Requirement, ComplianceAssessment, NormativeConflict
};
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct PostgresNormativeStore {
    pool: Pool<Postgres>,
}

impl PostgresNormativeStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn initialize(&self) -> AionResult<()> {
        crate::schema::DatabaseSchema::create_schema_version_table(&self.pool).await?;
        crate::schema::DatabaseSchema::create_all_tables(&self.pool).await?;
        crate::schema::DatabaseSchema::record_schema_version(&self.pool, "1.0.0", "Initial schema").await?;
        Ok(())
    }

    async fn store_framework_relationships(&self, framework: &NormativeFramework) -> AionResult<()> {
        for dep_id in &framework.dependencies {
            let query = r#"
                INSERT INTO framework_dependencies (framework_id, depends_on)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING;
            "#;
            sqlx::query(query)
                .bind(framework.id.0)
                .bind(dep_id.0)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_dependencies".to_string(),
                    reason: e.to_string(),
                })?;
        }

        for superseded_id in &framework.supersedes {
            let query = r#"
                INSERT INTO framework_supersessions (framework_id, supersedes)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING;
            "#;
            sqlx::query(query)
                .bind(framework.id.0)
                .bind(superseded_id.0)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_supersessions".to_string(),
                    reason: e.to_string(),
                })?;
        }

        for tag in &framework.tags {
            let query = r#"
                INSERT INTO framework_tags (framework_id, tag)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING;
            "#;
            sqlx::query(query)
                .bind(framework.id.0)
                .bind(tag)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_tags".to_string(),
                    reason: e.to_string(),
                })?;
        }

        for (key, value) in &framework.metadata {
            let query = r#"
                INSERT INTO framework_metadata (framework_id, key, value)
                VALUES ($1, $2, $3)
                ON CONFLICT (framework_id, key) DO UPDATE SET
                    value = EXCLUDED.value,
                    updated_at = NOW();
            "#;
            sqlx::query(query)
                .bind(framework.id.0)
                .bind(key)
                .bind(value)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_metadata".to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    async fn store_requirements(&self, framework_id: &NormativeId, requirements: &[Requirement]) -> AionResult<()> {
        for requirement in requirements {
            let query = r#"
                INSERT INTO requirements (
                    id, framework_id, title, description, mandatory,
                    priority, category, evidence_required
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (id) DO UPDATE SET
                    title = EXCLUDED.title,
                    description = EXCLUDED.description,
                    mandatory = EXCLUDED.mandatory,
                    priority = EXCLUDED.priority,
                    category = EXCLUDED.category,
                    evidence_required = EXCLUDED.evidence_required,
                    updated_at = NOW();
            "#;

            sqlx::query(query)
                .bind(requirement.id)
                .bind(framework_id.0)
                .bind(&requirement.title)
                .bind(&requirement.description)
                .bind(requirement.mandatory)
                .bind(requirement.priority as i16)
                .bind(&requirement.category)
                .bind(&requirement.evidence_required)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_requirements".to_string(),
                    reason: e.to_string(),
                })?;

            self.store_conditions(&requirement.id, &requirement.conditions).await?;
            self.store_exceptions(&requirement.id, &requirement.exceptions).await?;
            self.store_validation_rules(&requirement.id, &requirement.validation_rules).await?;
        }

        Ok(())
    }

    async fn store_conditions(&self, requirement_id: &Uuid, conditions: &[aion_core::Condition]) -> AionResult<()> {
        sqlx::query("DELETE FROM conditions WHERE requirement_id = $1")
            .bind(requirement_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "delete_old_conditions".to_string(),
                reason: e.to_string(),
            })?;

        for condition in conditions {
            let query = r#"
                INSERT INTO conditions (id, requirement_id, description, expression, context_variables)
                VALUES ($1, $2, $3, $4, $5);
            "#;

            sqlx::query(query)
                .bind(condition.id)
                .bind(requirement_id)
                .bind(&condition.description)
                .bind(&condition.expression)
                .bind(&condition.context_variables)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_conditions".to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    async fn store_exceptions(&self, requirement_id: &Uuid, exceptions: &[aion_core::Exception]) -> AionResult<()> {
        sqlx::query("DELETE FROM exceptions WHERE requirement_id = $1")
            .bind(requirement_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "delete_old_exceptions".to_string(),
                reason: e.to_string(),
            })?;

        for exception in exceptions {
            let query = r#"
                INSERT INTO exceptions (id, requirement_id, description, scope, valid_until)
                VALUES ($1, $2, $3, $4, $5);
            "#;

            sqlx::query(query)
                .bind(exception.id)
                .bind(requirement_id)
                .bind(&exception.description)
                .bind(&exception.scope)
                .bind(exception.valid_until)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_exceptions".to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    async fn store_validation_rules(&self, requirement_id: &Uuid, rules: &[aion_core::ValidationRule]) -> AionResult<()> {
        sqlx::query("DELETE FROM validation_rules WHERE requirement_id = $1")
            .bind(requirement_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "delete_old_validation_rules".to_string(),
                reason: e.to_string(),
            })?;

        for rule in rules {
            let query = r#"
                INSERT INTO validation_rules (
                    id, requirement_id, name, rule_type,
                    expression, error_message, severity
                ) VALUES ($1, $2, $3, $4, $5, $6, $7);
            "#;

            sqlx::query(query)
                .bind(rule.id)
                .bind(requirement_id)
                .bind(&rule.name)
                .bind(&rule.rule_type)
                .bind(&rule.expression)
                .bind(&rule.error_message)
                .bind(&rule.severity)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_validation_rules".to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    async fn load_framework_relationships(&self, framework: &mut NormativeFramework) -> AionResult<()> {
        let deps_query = r#"
            SELECT depends_on FROM framework_dependencies WHERE framework_id = $1;
        "#;

        let dep_rows = sqlx::query(deps_query)
            .bind(framework.id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_dependencies".to_string(),
                reason: e.to_string(),
            })?;

        framework.dependencies = dep_rows
            .into_iter()
            .map(|row| NormativeId(row.get("depends_on")))
            .collect();

        let supersessions_query = r#"
            SELECT supersedes FROM framework_supersessions WHERE framework_id = $1;
        "#;

        let supersession_rows = sqlx::query(supersessions_query)
            .bind(framework.id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_supersessions".to_string(),
                reason: e.to_string(),
            })?;

        framework.supersedes = supersession_rows
            .into_iter()
            .map(|row| NormativeId(row.get("supersedes")))
            .collect();

        let tags_query = r#"
            SELECT tag FROM framework_tags WHERE framework_id = $1;
        "#;

        let tag_rows = sqlx::query(tags_query)
            .bind(framework.id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_tags".to_string(),
                reason: e.to_string(),
            })?;

        framework.tags = tag_rows
            .into_iter()
            .map(|row| row.get("tag"))
            .collect();

        let metadata_query = r#"
            SELECT key, value FROM framework_metadata WHERE framework_id = $1;
        "#;

        let metadata_rows = sqlx::query(metadata_query)
            .bind(framework.id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_metadata".to_string(),
                reason: e.to_string(),
            })?;

        framework.metadata = metadata_rows
            .into_iter()
            .map(|row| (row.get("key"), row.get("value")))
            .collect();

        Ok(())
    }

    async fn load_requirements(&self, framework_id: &NormativeId) -> AionResult<Vec<Requirement>> {
        let query = r#"
            SELECT id, title, description, mandatory, priority, category, evidence_required
            FROM requirements
            WHERE framework_id = $1
            ORDER BY priority, title;
        "#;

        let rows = sqlx::query(query)
            .bind(framework_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_requirements".to_string(),
                reason: e.to_string(),
            })?;

        let mut requirements = Vec::new();

        for row in rows {
            let requirement_id: Uuid = row.get("id");

            let conditions = self.load_conditions(&requirement_id).await?;
            let exceptions = self.load_exceptions(&requirement_id).await?;
            let validation_rules = self.load_validation_rules(&requirement_id).await?;

            let requirement = Requirement {
                id: requirement_id,
                title: row.get("title"),
                description: row.get("description"),
                mandatory: row.get("mandatory"),
                priority: row.get::<i16, _>("priority") as u8,
                category: row.get("category"),
                evidence_required: row.get("evidence_required"),
                conditions,
                exceptions,
                validation_rules,
            };

            requirements.push(requirement);
        }

        Ok(requirements)
    }

    async fn load_conditions(&self, requirement_id: &Uuid) -> AionResult<Vec<aion_core::Condition>> {
        let query = r#"
            SELECT id, description, expression, context_variables
            FROM conditions
            WHERE requirement_id = $1;
        "#;

        let rows = sqlx::query(query)
            .bind(requirement_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_conditions".to_string(),
                reason: e.to_string(),
            })?;

        Ok(rows
            .into_iter()
            .map(|row| aion_core::Condition {
                id: row.get("id"),
                description: row.get("description"),
                expression: row.get("expression"),
                context_variables: row.get("context_variables"),
            })
            .collect())
    }

    async fn load_exceptions(&self, requirement_id: &Uuid) -> AionResult<Vec<aion_core::Exception>> {
        let query = r#"
            SELECT id, description, scope, valid_until
            FROM exceptions
            WHERE requirement_id = $1;
        "#;

        let rows = sqlx::query(query)
            .bind(requirement_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_exceptions".to_string(),
                reason: e.to_string(),
            })?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let conditions_query = r#"
                    SELECT id, description, expression, context_variables
                    FROM conditions
                    WHERE requirement_id = $1;
                "#;

                aion_core::Exception {
                    id: row.get("id"),
                    description: row.get("description"),
                    scope: row.get("scope"),
                    valid_until: row.get("valid_until"),
                    conditions: Vec::new(), // Load separately if needed
                }
            })
            .collect())
    }

    async fn load_validation_rules(&self, requirement_id: &Uuid) -> AionResult<Vec<aion_core::ValidationRule>> {
        let query = r#"
            SELECT id, name, rule_type, expression, error_message, severity
            FROM validation_rules
            WHERE requirement_id = $1;
        "#;

        let rows = sqlx::query(query)
            .bind(requirement_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "load_validation_rules".to_string(),
                reason: e.to_string(),
            })?;

        Ok(rows
            .into_iter()
            .map(|row| aion_core::ValidationRule {
                id: row.get("id"),
                name: row.get("name"),
                rule_type: row.get("rule_type"),
                expression: row.get("expression"),
                error_message: row.get("error_message"),
                severity: row.get("severity"),
            })
            .collect())
    }

    pub async fn store_conflict(&self, conflict: &NormativeConflict) -> AionResult<()> {
        let query = r#"
            INSERT INTO normative_conflicts (
                id, conflict_type, severity, normative_a, normative_b,
                description, affected_requirements, context, discovered_at,
                resolution_strategy, resolution_notes, resolved_at, resolved_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                conflict_type = EXCLUDED.conflict_type,
                severity = EXCLUDED.severity,
                description = EXCLUDED.description,
                affected_requirements = EXCLUDED.affected_requirements,
                context = EXCLUDED.context,
                resolution_strategy = EXCLUDED.resolution_strategy,
                resolution_notes = EXCLUDED.resolution_notes,
                resolved_at = EXCLUDED.resolved_at,
                resolved_by = EXCLUDED.resolved_by;
        "#;

        let context_json = serde_json::to_value(&conflict.context)
            .map_err(|e| AionError::SerializationError { reason: e.to_string() })?;

        sqlx::query(query)
            .bind(conflict.id)
            .bind(format!("{:?}", conflict.conflict_type))
            .bind(format!("{:?}", conflict.severity))
            .bind(conflict.normative_a.0)
            .bind(conflict.normative_b.0)
            .bind(&conflict.description)
            .bind(&conflict.affected_requirements)
            .bind(context_json)
            .bind(conflict.discovered_at)
            .bind(conflict.resolution_strategy.as_ref().map(|s| format!("{:?}", s)))
            .bind(&conflict.resolution_notes)
            .bind(conflict.resolved_at)
            .bind(&conflict.resolved_by)
            .execute(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "store_conflict".to_string(),
                reason: e.to_string(),
            })?;

        Ok(())
    }

    pub async fn get_conflicts_for_framework(&self, framework_id: &NormativeId) -> AionResult<Vec<NormativeConflict>> {
        let query = r#"
            SELECT *
            FROM normative_conflicts
            WHERE normative_a = $1 OR normative_b = $1
            ORDER BY discovered_at DESC;
        "#;

        let rows = sqlx::query(query)
            .bind(framework_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "get_conflicts_for_framework".to_string(),
                reason: e.to_string(),
            })?;

        let mut conflicts = Vec::new();

        for row in rows {
            let context: serde_json::Value = row.get("context");
            let context_map: HashMap<String, String> = serde_json::from_value(context)
                .map_err(|e| AionError::SerializationError { reason: e.to_string() })?;

            let conflict = NormativeConflict {
                id: row.get("id"),
                conflict_type: self.parse_conflict_type(row.get("conflict_type"))?,
                severity: self.parse_conflict_severity(row.get("severity"))?,
                normative_a: NormativeId(row.get("normative_a")),
                normative_b: NormativeId(row.get("normative_b")),
                involved_frameworks: vec![NormativeId(row.get("normative_a")), NormativeId(row.get("normative_b"))],
                description: row.get("description"),
                affected_requirements: row.get("affected_requirements"),
                context: context_map,
                discovered_at: row.get("discovered_at"),
                resolution_strategy: row.get::<Option<String>, _>("resolution_strategy")
                    .map(|s| self.parse_resolution_strategy(&s))
                    .transpose()?,
                resolution_notes: row.get("resolution_notes"),
                resolved_at: row.get("resolved_at"),
                resolved_by: row.get("resolved_by"),
            };

            conflicts.push(conflict);
        }

        Ok(conflicts)
    }

    fn parse_conflict_type(&self, type_str: &str) -> AionResult<aion_core::ConflictType> {
        match type_str {
            "DirectContradiction" => Ok(aion_core::ConflictType::DirectContradiction),
            "ImplicitConflict" => Ok(aion_core::ConflictType::ImplicitConflict),
            "JurisdictionalOverlap" => Ok(aion_core::ConflictType::JurisdictionalOverlap),
            "TemporalInconsistency" => Ok(aion_core::ConflictType::TemporalInconsistency),
            "ScopeAmbiguity" => Ok(aion_core::ConflictType::ScopeAmbiguity),
            "AuthorityConflict" => Ok(aion_core::ConflictType::AuthorityConflict),
            "PriorityDispute" => Ok(aion_core::ConflictType::PriorityDispute),
            _ => Err(AionError::ValidationError {
                field: "conflict_type".to_string(),
                message: format!("Unknown conflict type: {}", type_str),
            }),
        }
    }

    fn parse_conflict_severity(&self, severity_str: &str) -> AionResult<aion_core::ConflictSeverity> {
        match severity_str {
            "Critical" => Ok(aion_core::ConflictSeverity::Critical),
            "High" => Ok(aion_core::ConflictSeverity::High),
            "Medium" => Ok(aion_core::ConflictSeverity::Medium),
            "Low" => Ok(aion_core::ConflictSeverity::Low),
            "Informational" => Ok(aion_core::ConflictSeverity::Informational),
            _ => Err(AionError::ValidationError {
                field: "conflict_severity".to_string(),
                message: format!("Unknown conflict severity: {}", severity_str),
            }),
        }
    }

    fn parse_resolution_strategy(&self, strategy_str: &str) -> AionResult<aion_core::ResolutionStrategy> {
        match strategy_str {
            "LexSpecialis" => Ok(aion_core::ResolutionStrategy::LexSpecialis),
            "LexPosterior" => Ok(aion_core::ResolutionStrategy::LexPosterior),
            "LexSuperior" => Ok(aion_core::ResolutionStrategy::LexSuperior),
            "Harmonization" => Ok(aion_core::ResolutionStrategy::Harmonization),
            "Contextualization" => Ok(aion_core::ResolutionStrategy::Contextualization),
            "Delegation" => Ok(aion_core::ResolutionStrategy::Delegation),
            "Arbitration" => Ok(aion_core::ResolutionStrategy::Arbitration),
            "Mediation" => Ok(aion_core::ResolutionStrategy::Mediation),
            _ => Err(AionError::ValidationError {
                field: "resolution_strategy".to_string(),
                message: format!("Unknown resolution strategy: {}", strategy_str),
            }),
        }
    }

    fn parse_normative_type(&self, type_str: &str) -> AionResult<aion_core::NormativeType> {
        match type_str {
            "Regulation" => Ok(aion_core::NormativeType::Regulation),
            "Policy" => Ok(aion_core::NormativeType::Policy),
            "Standard" => Ok(aion_core::NormativeType::Standard),
            "Guideline" => Ok(aion_core::NormativeType::Guideline),
            "Procedure" => Ok(aion_core::NormativeType::Procedure),
            "Protocol" => Ok(aion_core::NormativeType::Protocol),
            "Framework" => Ok(aion_core::NormativeType::Framework),
            "Principle" => Ok(aion_core::NormativeType::Principle),
            "Rule" => Ok(aion_core::NormativeType::Rule),
            "Directive" => Ok(aion_core::NormativeType::Directive),
            _ => Err(AionError::ValidationError {
                field: "normative_type".to_string(),
                message: format!("Unknown normative type: {}", type_str),
            }),
        }
    }

    fn parse_jurisdiction(&self, jurisdiction_str: &str) -> AionResult<aion_core::Jurisdiction> {
        match jurisdiction_str {
            "International" => Ok(aion_core::Jurisdiction::International),
            "Federal" => Ok(aion_core::Jurisdiction::Federal),
            "State" => Ok(aion_core::Jurisdiction::State),
            "Regional" => Ok(aion_core::Jurisdiction::Regional),
            "Local" => Ok(aion_core::Jurisdiction::Local),
            "Sectoral" => Ok(aion_core::Jurisdiction::Sectoral),
            "Organizational" => Ok(aion_core::Jurisdiction::Organizational),
            "Departmental" => Ok(aion_core::Jurisdiction::Departmental),
            _ => Err(AionError::ValidationError {
                field: "jurisdiction".to_string(),
                message: format!("Unknown jurisdiction: {}", jurisdiction_str),
            }),
        }
    }
}

impl NormativeRepository for PostgresNormativeStore {
    fn store_framework(&self, framework: NormativeFramework) -> AionResult<()> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let query = r#"
                INSERT INTO normative_frameworks (
                    id, title, description, normative_type, jurisdiction,
                    authority, effective_date, expiration_date, version, status
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ON CONFLICT (id) DO UPDATE SET
                    title = EXCLUDED.title,
                    description = EXCLUDED.description,
                    normative_type = EXCLUDED.normative_type,
                    jurisdiction = EXCLUDED.jurisdiction,
                    authority = EXCLUDED.authority,
                    effective_date = EXCLUDED.effective_date,
                    expiration_date = EXCLUDED.expiration_date,
                    version = EXCLUDED.version,
                    status = EXCLUDED.status,
                    updated_at = NOW();
            "#;

            sqlx::query(query)
                .bind(framework.id.0)
                .bind(&framework.title)
                .bind(&framework.description)
                .bind(format!("{:?}", framework.normative_type))
                .bind(format!("{:?}", framework.jurisdiction))
                .bind(&framework.authority)
                .bind(framework.effective_date)
                .bind(framework.expiration_date)
                .bind(&framework.version)
                .bind(&framework.status)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "store_framework".to_string(),
                    reason: e.to_string(),
                })?;

            self.store_framework_relationships(&framework).await?;
            self.store_requirements(&framework.id, &framework.requirements).await?;

            Ok(())
        })
    }

    fn get_framework(&self, id: &NormativeId) -> AionResult<Option<NormativeFramework>> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let query = r#"
                SELECT *
                FROM normative_frameworks
                WHERE id = $1;
            "#;

            let row = sqlx::query(query)
                .bind(id.0)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "get_framework".to_string(),
                    reason: e.to_string(),
                })?;

            if let Some(row) = row {
                let mut framework = NormativeFramework {
                    id: NormativeId(row.get("id")),
                    title: row.get("title"),
                    description: row.get("description"),
                    normative_type: self.parse_normative_type(row.get("normative_type"))?,
                    jurisdiction: self.parse_jurisdiction(row.get("jurisdiction"))?,
                    authority: row.get("authority"),
                    effective_date: row.get("effective_date"),
                    expiration_date: row.get("expiration_date"),
                    version: row.get("version"),
                    status: row.get("status"),
                    tags: Vec::new(),
                    metadata: HashMap::new(),
                    requirements: Vec::new(),
                    dependencies: Vec::new(),
                    supersedes: Vec::new(),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };

                self.load_framework_relationships(&mut framework).await?;
                framework.requirements = self.load_requirements(&framework.id).await?;

                Ok(Some(framework))
            } else {
                Ok(None)
            }
        })
    }

    fn list_frameworks(&self) -> AionResult<Vec<NormativeFramework>> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let query = r#"
                SELECT id
                FROM normative_frameworks
                ORDER BY created_at DESC;
            "#;

            let rows = sqlx::query(query)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "list_frameworks".to_string(),
                    reason: e.to_string(),
                })?;

            let mut frameworks = Vec::new();

            for row in rows {
                let id: Uuid = row.get("id");
                if let Some(framework) = self.get_framework(&NormativeId(id))? {
                    frameworks.push(framework);
                }
            }

            Ok(frameworks)
        })
    }

    fn update_framework(&self, framework: NormativeFramework) -> AionResult<()> {
        self.store_framework(framework)
    }

    fn delete_framework(&self, id: &NormativeId) -> AionResult<()> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let query = r#"
                DELETE FROM normative_frameworks WHERE id = $1;
            "#;

            let result = sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "delete_framework".to_string(),
                    reason: e.to_string(),
                })?;

            if result.rows_affected() == 0 {
                return Err(AionError::NormativeNotFound { id: id.0.to_string() });
            }

            Ok(())
        })
    }

    fn search_frameworks(&self, query: &str) -> AionResult<Vec<NormativeFramework>> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let search_query = r#"
                SELECT DISTINCT f.id
                FROM normative_frameworks f
                LEFT JOIN framework_tags ft ON f.id = ft.framework_id
                LEFT JOIN framework_metadata fm ON f.id = fm.framework_id
                WHERE f.title ILIKE $1
                   OR f.description ILIKE $1
                   OR f.authority ILIKE $1
                   OR ft.tag ILIKE $1
                   OR fm.value ILIKE $1
                ORDER BY f.created_at DESC;
            "#;

            let search_pattern = format!("%{}%", query);

            let rows = sqlx::query(search_query)
                .bind(&search_pattern)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "search_frameworks".to_string(),
                    reason: e.to_string(),
                })?;

            let mut frameworks = Vec::new();

            for row in rows {
                let id: Uuid = row.get("id");
                if let Some(framework) = self.get_framework(&NormativeId(id))? {
                    frameworks.push(framework);
                }
            }

            Ok(frameworks)
        })
    }

    fn get_active_frameworks(&self) -> AionResult<Vec<NormativeFramework>> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let query = r#"
                SELECT id FROM active_frameworks;
            "#;

            let rows = sqlx::query(query)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AionError::DatabaseError {
                    operation: "get_active_frameworks".to_string(),
                    reason: e.to_string(),
                })?;

            let mut frameworks = Vec::new();

            for row in rows {
                let id: Uuid = row.get("id");
                if let Some(framework) = self.get_framework(&NormativeId(id))? {
                    frameworks.push(framework);
                }
            }

            Ok(frameworks)
        })
    }
}