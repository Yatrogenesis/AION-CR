use aion_core::{AionResult, AionError};
use sqlx::{Pool, Postgres, Row};

pub struct DatabaseSchema;

impl DatabaseSchema {
    pub async fn create_all_tables(pool: &Pool<Postgres>) -> AionResult<()> {
        Self::create_normative_frameworks_table(pool).await?;
        Self::create_requirements_table(pool).await?;
        Self::create_conditions_table(pool).await?;
        Self::create_exceptions_table(pool).await?;
        Self::create_validation_rules_table(pool).await?;
        Self::create_conflicts_table(pool).await?;
        Self::create_compliance_assessments_table(pool).await?;
        Self::create_requirement_assessments_table(pool).await?;
        Self::create_evidence_table(pool).await?;
        Self::create_findings_table(pool).await?;
        Self::create_recommendations_table(pool).await?;
        Self::create_audit_trail_table(pool).await?;
        Self::create_business_rules_table(pool).await?;
        Self::create_framework_dependencies_table(pool).await?;
        Self::create_framework_supersessions_table(pool).await?;
        Self::create_framework_tags_table(pool).await?;
        Self::create_framework_metadata_table(pool).await?;
        Self::create_conflict_resolutions_table(pool).await?;
        Self::create_indexes(pool).await?;
        Self::create_views(pool).await?;
        Ok(())
    }

    async fn create_normative_frameworks_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS normative_frameworks (
                id UUID PRIMARY KEY,
                title VARCHAR(200) NOT NULL,
                description TEXT NOT NULL,
                normative_type VARCHAR(50) NOT NULL,
                jurisdiction VARCHAR(50) NOT NULL,
                authority VARCHAR(200) NOT NULL,
                effective_date TIMESTAMPTZ NOT NULL,
                expiration_date TIMESTAMPTZ,
                version VARCHAR(20) NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'active',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_normative_type CHECK (normative_type IN (
                    'Regulation', 'Policy', 'Standard', 'Guideline', 'Procedure',
                    'Protocol', 'Framework', 'Principle', 'Rule', 'Directive'
                )),
                CONSTRAINT valid_jurisdiction CHECK (jurisdiction IN (
                    'International', 'Federal', 'State', 'Regional', 'Local',
                    'Sectoral', 'Organizational', 'Departmental'
                )),
                CONSTRAINT valid_status CHECK (status IN ('active', 'inactive', 'draft', 'archived')),
                CONSTRAINT valid_dates CHECK (expiration_date IS NULL OR expiration_date > effective_date)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_normative_frameworks_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_requirements_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS requirements (
                id UUID PRIMARY KEY,
                framework_id UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                title VARCHAR(200) NOT NULL,
                description TEXT NOT NULL,
                mandatory BOOLEAN NOT NULL DEFAULT FALSE,
                priority SMALLINT NOT NULL DEFAULT 5,
                category VARCHAR(100) NOT NULL,
                evidence_required TEXT[] NOT NULL DEFAULT '{}',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_priority CHECK (priority >= 1 AND priority <= 10)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_requirements_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_conditions_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS conditions (
                id UUID PRIMARY KEY,
                requirement_id UUID NOT NULL REFERENCES requirements(id) ON DELETE CASCADE,
                description TEXT NOT NULL,
                expression TEXT NOT NULL,
                context_variables TEXT[] NOT NULL DEFAULT '{}',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_conditions_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_exceptions_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS exceptions (
                id UUID PRIMARY KEY,
                requirement_id UUID NOT NULL REFERENCES requirements(id) ON DELETE CASCADE,
                description TEXT NOT NULL,
                scope TEXT NOT NULL,
                valid_until TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_exceptions_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_validation_rules_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS validation_rules (
                id UUID PRIMARY KEY,
                requirement_id UUID NOT NULL REFERENCES requirements(id) ON DELETE CASCADE,
                name VARCHAR(100) NOT NULL,
                rule_type VARCHAR(50) NOT NULL,
                expression TEXT NOT NULL,
                error_message TEXT NOT NULL,
                severity VARCHAR(20) NOT NULL DEFAULT 'error',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_severity CHECK (severity IN ('error', 'warning', 'info'))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_validation_rules_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_conflicts_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS normative_conflicts (
                id UUID PRIMARY KEY,
                conflict_type VARCHAR(50) NOT NULL,
                severity VARCHAR(20) NOT NULL,
                normative_a UUID NOT NULL REFERENCES normative_frameworks(id),
                normative_b UUID NOT NULL REFERENCES normative_frameworks(id),
                description TEXT NOT NULL,
                affected_requirements UUID[] NOT NULL DEFAULT '{}',
                context JSONB NOT NULL DEFAULT '{}',
                discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                resolution_strategy VARCHAR(50),
                resolution_notes TEXT,
                resolved_at TIMESTAMPTZ,
                resolved_by VARCHAR(100),
                CONSTRAINT valid_conflict_type CHECK (conflict_type IN (
                    'DirectContradiction', 'ImplicitConflict', 'JurisdictionalOverlap',
                    'TemporalInconsistency', 'ScopeAmbiguity', 'AuthorityConflict', 'PriorityDispute'
                )),
                CONSTRAINT valid_severity CHECK (severity IN ('Critical', 'High', 'Medium', 'Low', 'Informational')),
                CONSTRAINT different_frameworks CHECK (normative_a != normative_b)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_conflicts_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_compliance_assessments_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS compliance_assessments (
                id UUID PRIMARY KEY,
                entity_id VARCHAR(100) NOT NULL,
                normative_framework UUID NOT NULL REFERENCES normative_frameworks(id),
                assessment_date TIMESTAMPTZ NOT NULL,
                assessor VARCHAR(100) NOT NULL,
                overall_status VARCHAR(20) NOT NULL,
                next_review_date TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_overall_status CHECK (overall_status IN (
                    'Compliant', 'NonCompliant', 'PartiallyCompliant',
                    'NotApplicable', 'UnderReview', 'Pending', 'Exempt'
                ))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_compliance_assessments_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_requirement_assessments_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS requirement_assessments (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                compliance_assessment_id UUID NOT NULL REFERENCES compliance_assessments(id) ON DELETE CASCADE,
                requirement_id UUID NOT NULL REFERENCES requirements(id),
                status VARCHAR(20) NOT NULL,
                gaps TEXT[] NOT NULL DEFAULT '{}',
                notes TEXT NOT NULL DEFAULT '',
                risk_level VARCHAR(20) NOT NULL DEFAULT 'low',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_status CHECK (status IN (
                    'Compliant', 'NonCompliant', 'PartiallyCompliant',
                    'NotApplicable', 'UnderReview', 'Pending', 'Exempt'
                )),
                CONSTRAINT valid_risk_level CHECK (risk_level IN ('critical', 'high', 'medium', 'low'))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_requirement_assessments_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_evidence_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS evidence (
                id UUID PRIMARY KEY,
                requirement_assessment_id UUID NOT NULL REFERENCES requirement_assessments(id) ON DELETE CASCADE,
                evidence_type VARCHAR(100) NOT NULL,
                description TEXT NOT NULL,
                source VARCHAR(200) NOT NULL,
                collected_date TIMESTAMPTZ NOT NULL,
                verification_status VARCHAR(50) NOT NULL DEFAULT 'pending',
                metadata JSONB NOT NULL DEFAULT '{}',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_verification_status CHECK (verification_status IN (
                    'pending', 'verified', 'rejected', 'expired', 'under_review'
                ))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_evidence_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_findings_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS findings (
                id UUID PRIMARY KEY,
                compliance_assessment_id UUID NOT NULL REFERENCES compliance_assessments(id) ON DELETE CASCADE,
                finding_type VARCHAR(100) NOT NULL,
                severity VARCHAR(20) NOT NULL,
                title VARCHAR(200) NOT NULL,
                description TEXT NOT NULL,
                affected_requirements UUID[] NOT NULL DEFAULT '{}',
                root_cause TEXT,
                impact_assessment TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_severity CHECK (severity IN ('critical', 'high', 'medium', 'low', 'info'))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_findings_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_recommendations_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS recommendations (
                id UUID PRIMARY KEY,
                compliance_assessment_id UUID NOT NULL REFERENCES compliance_assessments(id) ON DELETE CASCADE,
                title VARCHAR(200) NOT NULL,
                description TEXT NOT NULL,
                priority VARCHAR(20) NOT NULL,
                effort_estimate VARCHAR(50),
                timeline VARCHAR(100),
                responsible_party VARCHAR(100),
                related_findings UUID[] NOT NULL DEFAULT '{}',
                status VARCHAR(20) NOT NULL DEFAULT 'open',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_priority CHECK (priority IN ('critical', 'high', 'medium', 'low')),
                CONSTRAINT valid_status CHECK (status IN ('open', 'in_progress', 'completed', 'cancelled'))
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_recommendations_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_audit_trail_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS audit_trail (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                entity_type VARCHAR(50) NOT NULL,
                entity_id VARCHAR(100) NOT NULL,
                action VARCHAR(100) NOT NULL,
                actor VARCHAR(100) NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                details JSONB NOT NULL DEFAULT '{}',
                previous_state JSONB,
                new_state JSONB
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_audit_trail_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_business_rules_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS business_rules (
                id UUID PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                description TEXT NOT NULL,
                rule_expression TEXT NOT NULL,
                context VARCHAR(100) NOT NULL,
                priority SMALLINT NOT NULL DEFAULT 5,
                active BOOLEAN NOT NULL DEFAULT TRUE,
                version VARCHAR(20) NOT NULL DEFAULT '1.0.0',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_priority CHECK (priority >= 1 AND priority <= 10)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_business_rules_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_framework_dependencies_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS framework_dependencies (
                framework_id UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                depends_on UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                dependency_type VARCHAR(50) NOT NULL DEFAULT 'requires',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (framework_id, depends_on),
                CONSTRAINT no_self_dependency CHECK (framework_id != depends_on)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_framework_dependencies_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_framework_supersessions_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS framework_supersessions (
                framework_id UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                supersedes UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                supersession_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                notes TEXT,
                PRIMARY KEY (framework_id, supersedes),
                CONSTRAINT no_self_supersession CHECK (framework_id != supersedes)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_framework_supersessions_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_framework_tags_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS framework_tags (
                framework_id UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                tag VARCHAR(100) NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (framework_id, tag)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_framework_tags_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_framework_metadata_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS framework_metadata (
                framework_id UUID NOT NULL REFERENCES normative_frameworks(id) ON DELETE CASCADE,
                key VARCHAR(100) NOT NULL,
                value TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (framework_id, key)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_framework_metadata_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_conflict_resolutions_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS conflict_resolutions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                conflict_id UUID NOT NULL REFERENCES normative_conflicts(id) ON DELETE CASCADE,
                strategy_used VARCHAR(50) NOT NULL,
                confidence_score REAL NOT NULL DEFAULT 0.0,
                resolution_notes TEXT NOT NULL,
                metadata JSONB NOT NULL DEFAULT '{}',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT valid_confidence CHECK (confidence_score >= 0.0 AND confidence_score <= 1.0)
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_conflict_resolutions_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    async fn create_indexes(pool: &Pool<Postgres>) -> AionResult<()> {
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_frameworks_status ON normative_frameworks(status);",
            "CREATE INDEX IF NOT EXISTS idx_frameworks_jurisdiction ON normative_frameworks(jurisdiction);",
            "CREATE INDEX IF NOT EXISTS idx_frameworks_effective_date ON normative_frameworks(effective_date);",
            "CREATE INDEX IF NOT EXISTS idx_frameworks_authority ON normative_frameworks(authority);",
            "CREATE INDEX IF NOT EXISTS idx_requirements_framework_id ON requirements(framework_id);",
            "CREATE INDEX IF NOT EXISTS idx_requirements_mandatory ON requirements(mandatory);",
            "CREATE INDEX IF NOT EXISTS idx_requirements_category ON requirements(category);",
            "CREATE INDEX IF NOT EXISTS idx_conditions_requirement_id ON conditions(requirement_id);",
            "CREATE INDEX IF NOT EXISTS idx_conflicts_severity ON normative_conflicts(severity);",
            "CREATE INDEX IF NOT EXISTS idx_conflicts_discovered_at ON normative_conflicts(discovered_at);",
            "CREATE INDEX IF NOT EXISTS idx_conflicts_normative_a ON normative_conflicts(normative_a);",
            "CREATE INDEX IF NOT EXISTS idx_conflicts_normative_b ON normative_conflicts(normative_b);",
            "CREATE INDEX IF NOT EXISTS idx_assessments_entity_id ON compliance_assessments(entity_id);",
            "CREATE INDEX IF NOT EXISTS idx_assessments_status ON compliance_assessments(overall_status);",
            "CREATE INDEX IF NOT EXISTS idx_assessments_date ON compliance_assessments(assessment_date);",
            "CREATE INDEX IF NOT EXISTS idx_audit_trail_entity ON audit_trail(entity_type, entity_id);",
            "CREATE INDEX IF NOT EXISTS idx_audit_trail_timestamp ON audit_trail(timestamp);",
            "CREATE INDEX IF NOT EXISTS idx_audit_trail_actor ON audit_trail(actor);",
            "CREATE INDEX IF NOT EXISTS idx_business_rules_context ON business_rules(context);",
            "CREATE INDEX IF NOT EXISTS idx_business_rules_active ON business_rules(active);",
            "CREATE INDEX IF NOT EXISTS idx_framework_tags_tag ON framework_tags(tag);",
            "CREATE INDEX IF NOT EXISTS idx_framework_metadata_key ON framework_metadata(key);",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql).execute(pool).await.map_err(|e| AionError::DatabaseError {
                operation: "create_indexes".to_string(),
                reason: e.to_string(),
            })?;
        }

        Ok(())
    }

    async fn create_views(pool: &Pool<Postgres>) -> AionResult<()> {
        let active_frameworks_view = r#"
            CREATE OR REPLACE VIEW active_frameworks AS
            SELECT * FROM normative_frameworks
            WHERE status = 'active'
            AND effective_date <= NOW()
            AND (expiration_date IS NULL OR expiration_date > NOW());
        "#;

        let framework_summary_view = r#"
            CREATE OR REPLACE VIEW framework_summary AS
            SELECT
                f.id,
                f.title,
                f.normative_type,
                f.jurisdiction,
                f.authority,
                f.status,
                COUNT(r.id) as requirement_count,
                COUNT(CASE WHEN r.mandatory THEN 1 END) as mandatory_requirement_count
            FROM normative_frameworks f
            LEFT JOIN requirements r ON f.id = r.framework_id
            GROUP BY f.id, f.title, f.normative_type, f.jurisdiction, f.authority, f.status;
        "#;

        let conflict_summary_view = r#"
            CREATE OR REPLACE VIEW conflict_summary AS
            SELECT
                severity,
                conflict_type,
                COUNT(*) as conflict_count,
                COUNT(CASE WHEN resolved_at IS NOT NULL THEN 1 END) as resolved_count
            FROM normative_conflicts
            GROUP BY severity, conflict_type;
        "#;

        let compliance_overview_view = r#"
            CREATE OR REPLACE VIEW compliance_overview AS
            SELECT
                entity_id,
                overall_status,
                COUNT(*) as assessment_count,
                MAX(assessment_date) as latest_assessment
            FROM compliance_assessments
            GROUP BY entity_id, overall_status;
        "#;

        let views = vec![
            active_frameworks_view,
            framework_summary_view,
            conflict_summary_view,
            compliance_overview_view,
        ];

        for view_sql in views {
            sqlx::query(view_sql).execute(pool).await.map_err(|e| AionError::DatabaseError {
                operation: "create_views".to_string(),
                reason: e.to_string(),
            })?;
        }

        Ok(())
    }

    pub async fn drop_all_tables(pool: &Pool<Postgres>) -> AionResult<()> {
        let tables = vec![
            "DROP VIEW IF EXISTS compliance_overview CASCADE;",
            "DROP VIEW IF EXISTS conflict_summary CASCADE;",
            "DROP VIEW IF EXISTS framework_summary CASCADE;",
            "DROP VIEW IF EXISTS active_frameworks CASCADE;",
            "DROP TABLE IF EXISTS conflict_resolutions CASCADE;",
            "DROP TABLE IF EXISTS framework_metadata CASCADE;",
            "DROP TABLE IF EXISTS framework_tags CASCADE;",
            "DROP TABLE IF EXISTS framework_supersessions CASCADE;",
            "DROP TABLE IF EXISTS framework_dependencies CASCADE;",
            "DROP TABLE IF EXISTS business_rules CASCADE;",
            "DROP TABLE IF EXISTS audit_trail CASCADE;",
            "DROP TABLE IF EXISTS recommendations CASCADE;",
            "DROP TABLE IF EXISTS findings CASCADE;",
            "DROP TABLE IF EXISTS evidence CASCADE;",
            "DROP TABLE IF EXISTS requirement_assessments CASCADE;",
            "DROP TABLE IF EXISTS compliance_assessments CASCADE;",
            "DROP TABLE IF EXISTS normative_conflicts CASCADE;",
            "DROP TABLE IF EXISTS validation_rules CASCADE;",
            "DROP TABLE IF EXISTS exceptions CASCADE;",
            "DROP TABLE IF EXISTS conditions CASCADE;",
            "DROP TABLE IF EXISTS requirements CASCADE;",
            "DROP TABLE IF EXISTS normative_frameworks CASCADE;",
        ];

        for table_sql in tables {
            sqlx::query(table_sql).execute(pool).await.map_err(|e| AionError::DatabaseError {
                operation: "drop_tables".to_string(),
                reason: e.to_string(),
            })?;
        }

        Ok(())
    }

    pub async fn get_schema_version(pool: &Pool<Postgres>) -> AionResult<String> {
        let version_query = r#"
            SELECT version FROM schema_version ORDER BY applied_at DESC LIMIT 1;
        "#;

        let row = sqlx::query(version_query)
            .fetch_optional(pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "get_schema_version".to_string(),
                reason: e.to_string(),
            })?;

        if let Some(row) = row {
            Ok(row.try_get("version").unwrap_or_else(|_| "unknown".to_string()))
        } else {
            Ok("not_initialized".to_string())
        }
    }

    pub async fn create_schema_version_table(pool: &Pool<Postgres>) -> AionResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS schema_version (
                version VARCHAR(20) PRIMARY KEY,
                description TEXT,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
        "#;

        sqlx::query(query).execute(pool).await.map_err(|e| AionError::DatabaseError {
            operation: "create_schema_version_table".to_string(),
            reason: e.to_string(),
        })?;

        Ok(())
    }

    pub async fn record_schema_version(pool: &Pool<Postgres>, version: &str, description: &str) -> AionResult<()> {
        let query = r#"
            INSERT INTO schema_version (version, description)
            VALUES ($1, $2)
            ON CONFLICT (version) DO NOTHING;
        "#;

        sqlx::query(query)
            .bind(version)
            .bind(description)
            .execute(pool)
            .await
            .map_err(|e| AionError::DatabaseError {
                operation: "record_schema_version".to_string(),
                reason: e.to_string(),
            })?;

        Ok(())
    }
}