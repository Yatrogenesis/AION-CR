use clap::{Args, Subcommand};
use aion_core::{AionResult, AionError, NormativeFramework, GovernanceContext, Jurisdiction};
use aion_normative::{NormativeEngine, InMemoryNormativeRepository, ComprehensiveValidator};
use aion_compliance::{ComplianceFrameworkLibrary, AdvancedComplianceEngine};
use aion_conflict::{AdvancedConflictDetector, AdvancedConflictResolver};
use std::sync::Arc;
use colored::*;

#[derive(Args)]
pub struct FrameworkCommand {
    #[command(subcommand)]
    pub action: FrameworkAction,
}

#[derive(Subcommand)]
pub enum FrameworkAction {
    List,
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        authority: String,
        #[arg(long)]
        framework_type: Option<String>,
    },
    Load {
        #[arg(long)]
        framework_type: String,
    },
    Search {
        query: String,
    },
    Validate {
        #[arg(long)]
        framework_id: Option<String>,
    },
}

#[derive(Args)]
pub struct ComplianceCommand {
    #[command(subcommand)]
    pub action: ComplianceAction,
}

#[derive(Subcommand)]
pub enum ComplianceAction {
    Assess {
        #[arg(long)]
        entity_id: String,
        #[arg(long)]
        frameworks: Vec<String>,
        #[arg(long)]
        organization: Option<String>,
        #[arg(long)]
        sector: Option<String>,
    },
    Report {
        #[arg(long)]
        entity_id: String,
        #[arg(long)]
        format: Option<String>,
    },
    LoadStandards,
}

#[derive(Args)]
pub struct ConflictCommand {
    #[command(subcommand)]
    pub action: ConflictAction,
}

#[derive(Subcommand)]
pub enum ConflictAction {
    Detect,
    Resolve {
        #[arg(long)]
        conflict_id: String,
    },
    List,
}

#[derive(Args)]
pub struct AuditCommand {
    #[command(subcommand)]
    pub action: AuditAction,
}

#[derive(Subcommand)]
pub enum AuditAction {
    Trail {
        #[arg(long)]
        entity_id: String,
    },
    Integrity,
}

#[derive(Args)]
pub struct DatabaseCommand {
    #[command(subcommand)]
    pub action: DatabaseAction,
}

#[derive(Subcommand)]
pub enum DatabaseAction {
    Init,
    Migrate,
    Reset,
    Status,
}

#[derive(Args)]
pub struct ServerCommand {
    #[arg(long, default_value = "8080")]
    pub port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}

#[derive(Args)]
pub struct InitCommand {
    #[arg(long)]
    pub organization: String,
    #[arg(long)]
    pub sector: String,
    #[arg(long)]
    pub region: String,
}

impl FrameworkCommand {
    pub async fn execute(self) -> AionResult<()> {
        let repository = Arc::new(InMemoryNormativeRepository::new());
        let validator = Arc::new(ComprehensiveValidator::new());
        let mut engine = NormativeEngine::new(repository, validator);

        match self.action {
            FrameworkAction::List => {
                println!("{}", "📋 Available Frameworks:".blue().bold());
                let frameworks = engine.get_active_frameworks()?;

                if frameworks.is_empty() {
                    println!("  {}", "No frameworks loaded.".yellow());
                    println!("  Use 'aion-cr framework load-standards' to load standard frameworks.");
                } else {
                    for framework in frameworks {
                        println!("  {} {} ({})",
                            "•".green(),
                            framework.title.bold(),
                            format!("{:?}", framework.normative_type).cyan()
                        );
                        println!("    Authority: {}", framework.authority);
                        println!("    Requirements: {}", framework.requirements.len());
                        println!();
                    }
                }
            },
            FrameworkAction::Create { title, description, authority, framework_type } => {
                let normative_type = match framework_type.as_deref() {
                    Some("regulation") => aion_core::NormativeType::Regulation,
                    Some("policy") => aion_core::NormativeType::Policy,
                    Some("standard") => aion_core::NormativeType::Standard,
                    Some("guideline") => aion_core::NormativeType::Guideline,
                    _ => aion_core::NormativeType::Framework,
                };

                let framework = NormativeFramework::new(
                    title.clone(),
                    description,
                    normative_type,
                    Jurisdiction::Organizational,
                    authority,
                );

                let id = engine.register_framework(framework)?;
                println!("{} Framework '{}' created with ID: {}",
                    "✅".green(),
                    title.bold(),
                    id.0.to_string().cyan()
                );
            },
            FrameworkAction::Load { framework_type } => {
                match ComplianceFrameworkLibrary::get_framework_by_type(&framework_type) {
                    Ok(framework) => {
                        let title = framework.title.clone();
                        let id = engine.register_framework(framework)?;
                        println!("{} Loaded framework '{}' with ID: {}",
                            "✅".green(),
                            title.bold(),
                            id.0.to_string().cyan()
                        );
                    },
                    Err(_) => {
                        println!("{} Unknown framework type: '{}'", "❌".red(), framework_type);
                        println!("Available types: gdpr, sox, iso27001, pci-dss, hipaa");
                    }
                }
            },
            FrameworkAction::Search { query } => {
                println!("{} Searching for: '{}'", "🔍".blue(), query.bold());
                let results = engine.search_frameworks(&query)?;

                if results.is_empty() {
                    println!("  {}", "No frameworks found.".yellow());
                } else {
                    for framework in results {
                        println!("  {} {} ({})",
                            "•".green(),
                            framework.title.bold(),
                            format!("{:?}", framework.normative_type).cyan()
                        );
                    }
                }
            },
            FrameworkAction::Validate { framework_id: _ } => {
                println!("{} Validating all frameworks...", "🔍".blue());
                let frameworks = engine.get_active_frameworks()?;
                let validator = ComprehensiveValidator::new();

                for framework in frameworks {
                    let report = validator.validate_framework_comprehensive(&framework)?;

                    if report.overall_valid {
                        println!("  {} {}", "✅".green(), framework.title.bold());
                    } else {
                        println!("  {} {} ({} errors)",
                            "❌".red(),
                            framework.title.bold(),
                            report.errors.len().to_string().red()
                        );
                        for error in &report.errors {
                            println!("    - {}", error);
                        }
                    }
                }
            },
        }

        Ok(())
    }
}

impl ComplianceCommand {
    pub async fn execute(self) -> AionResult<()> {
        match self.action {
            ComplianceAction::Assess { entity_id, frameworks, organization, sector } => {
                println!("{} Assessing compliance for entity: {}",
                    "📊".blue(),
                    entity_id.bold()
                );

                let repository = Arc::new(InMemoryNormativeRepository::new());
                let validator = Arc::new(ComprehensiveValidator::new());
                let mut engine = NormativeEngine::new(repository.clone(), validator.clone());

                let mut loaded_frameworks = Vec::new();
                for framework_type in frameworks {
                    match ComplianceFrameworkLibrary::get_framework_by_type(&framework_type) {
                        Ok(framework) => {
                            engine.register_framework(framework.clone())?;
                            loaded_frameworks.push(framework);
                        },
                        Err(_) => {
                            println!("  {} Unknown framework: {}", "⚠️".yellow(), framework_type);
                        }
                    }
                }

                if loaded_frameworks.is_empty() {
                    return Err(AionError::ValidationError {
                        field: "frameworks".to_string(),
                        message: "No valid frameworks specified".to_string(),
                    });
                }

                let context = GovernanceContext {
                    organization: organization.unwrap_or_else(|| "Unknown Organization".to_string()),
                    sector: sector.unwrap_or_else(|| "general".to_string()),
                    region: "global".to_string(),
                    applicable_jurisdictions: vec![Jurisdiction::International, Jurisdiction::Organizational],
                    business_context: std::collections::HashMap::new(),
                    risk_profile: "medium".to_string(),
                    maturity_level: "developing".to_string(),
                };

                let compliance_engine = AdvancedComplianceEngine::new(validator);
                let assessments = compliance_engine.assess_compliance_comprehensive(
                    &entity_id,
                    &loaded_frameworks,
                    &context
                )?;

                for assessment in assessments {
                    println!("\n{} Framework Assessment", "📋".blue());
                    println!("Framework: {}", assessment.normative_framework.0.to_string().cyan());
                    println!("Status: {}", format!("{:?}", assessment.overall_status).bold());

                    let compliant = assessment.requirement_assessments.iter()
                        .filter(|a| a.status == aion_core::ComplianceStatus::Compliant)
                        .count();
                    let total = assessment.requirement_assessments.len();

                    println!("Compliance Rate: {}/{} ({}%)",
                        compliant.to_string().green(),
                        total.to_string().cyan(),
                        ((compliant * 100) / total.max(1)).to_string().bold()
                    );

                    if !assessment.findings.is_empty() {
                        println!("\n{} Findings:", "🔍".red());
                        for finding in &assessment.findings {
                            println!("  {} [{}] {}",
                                "•".red(),
                                finding.severity.to_uppercase().red(),
                                finding.title
                            );
                        }
                    }

                    if !assessment.recommendations.is_empty() {
                        println!("\n{} Recommendations:", "💡".yellow());
                        for rec in &assessment.recommendations {
                            println!("  {} [{}] {}",
                                "•".yellow(),
                                rec.priority.to_uppercase().yellow(),
                                rec.title
                            );
                        }
                    }
                }
            },
            ComplianceAction::Report { entity_id, format: _ } => {
                println!("{} Generating compliance report for: {}",
                    "📄".blue(),
                    entity_id.bold()
                );
                println!("Report generation not yet implemented in CLI mode.");
                println!("Use the web interface or API for full reporting capabilities.");
            },
            ComplianceAction::LoadStandards => {
                println!("{} Loading standard compliance frameworks...", "📚".blue());

                let repository = Arc::new(InMemoryNormativeRepository::new());
                let validator = Arc::new(ComprehensiveValidator::new());
                let mut engine = NormativeEngine::new(repository, validator);

                let frameworks = ComplianceFrameworkLibrary::get_all_standard_frameworks()?;
                let mut loaded_count = 0;

                for framework in frameworks {
                    let title = framework.title.clone();
                    match engine.register_framework(framework) {
                        Ok(_) => {
                            println!("  {} Loaded: {}", "✅".green(), title.bold());
                            loaded_count += 1;
                        },
                        Err(e) => {
                            println!("  {} Failed to load {}: {}", "❌".red(), title, e);
                        }
                    }
                }

                println!("\n{} Successfully loaded {} standard frameworks",
                    "🎉".green(),
                    loaded_count.to_string().bold()
                );
            },
        }

        Ok(())
    }
}

impl ConflictCommand {
    pub async fn execute(self) -> AionResult<()> {
        match self.action {
            ConflictAction::Detect => {
                println!("{} Detecting normative conflicts...", "🔍".blue());

                let repository = Arc::new(InMemoryNormativeRepository::new());
                let validator = Arc::new(ComprehensiveValidator::new());
                let mut engine = NormativeEngine::new(repository, validator);

                let frameworks = ComplianceFrameworkLibrary::get_all_standard_frameworks()?;
                for framework in &frameworks {
                    engine.register_framework(framework.clone())?;
                }

                let mut detector = AdvancedConflictDetector::new();
                let conflicts = detector.detect_all_conflicts(&frameworks)?;

                if conflicts.is_empty() {
                    println!("  {} No conflicts detected!", "✅".green());
                } else {
                    println!("  {} Found {} conflicts:", "⚠️".yellow(), conflicts.len());

                    for conflict in conflicts {
                        println!("\n  {} [{:?}] {:?}",
                            "•".red(),
                            conflict.severity,
                            conflict.conflict_type
                        );
                        println!("    {}", conflict.description);
                        println!("    Frameworks: {} vs {}",
                            conflict.normative_a.0.to_string().cyan(),
                            conflict.normative_b.0.to_string().cyan()
                        );
                    }
                }
            },
            ConflictAction::Resolve { conflict_id } => {
                println!("{} Resolving conflict: {}", "🔧".blue(), conflict_id.bold());
                println!("Conflict resolution not yet implemented in CLI mode.");
                println!("Use the web interface or API for conflict resolution.");
            },
            ConflictAction::List => {
                println!("{} Listing all conflicts...", "📋".blue());
                println!("Conflict listing not yet implemented in CLI mode.");
            },
        }

        Ok(())
    }
}

impl AuditCommand {
    pub async fn execute(self) -> AionResult<()> {
        match self.action {
            AuditAction::Trail { entity_id } => {
                println!("{} Audit trail for entity: {}", "📝".blue(), entity_id.bold());
                println!("Audit trail functionality not yet implemented in CLI mode.");
            },
            AuditAction::Integrity => {
                println!("{} Checking system integrity...", "🔒".blue());
                println!("Integrity checking not yet implemented in CLI mode.");
            },
        }

        Ok(())
    }
}

impl DatabaseCommand {
    pub async fn execute(self) -> AionResult<()> {
        match self.action {
            DatabaseAction::Init => {
                println!("{} Initializing database...", "🗄️".blue());
                println!("Database initialization requires PostgreSQL connection.");
                println!("Use --database-url parameter or DATABASE_URL environment variable.");
            },
            DatabaseAction::Migrate => {
                println!("{} Running database migrations...", "🔄".blue());
                println!("Migration functionality requires database connection.");
            },
            DatabaseAction::Reset => {
                println!("{} Resetting database...", "⚠️".yellow());
                println!("Database reset functionality requires confirmation and database connection.");
            },
            DatabaseAction::Status => {
                println!("{} Database status:", "📊".blue());
                println!("  Status: Not connected (CLI mode)");
                println!("  Mode: In-memory repository");
                println!("  Use --database-url to connect to PostgreSQL");
            },
        }

        Ok(())
    }
}

impl ServerCommand {
    pub async fn execute(self) -> AionResult<()> {
        println!("{} Starting AION-CR server...", "🚀".blue());
        println!("Host: {}", self.host.cyan());
        println!("Port: {}", self.port.to_string().cyan());
        println!("\nServer functionality not yet implemented.");
        println!("The server will provide:");
        println!("  {} Web interface for compliance management", "•".green());
        println!("  {} REST API for integration", "•".green());
        println!("  {} Real-time conflict monitoring", "•".green());
        println!("  {} Compliance dashboards and reports", "•".green());

        Ok(())
    }
}

impl InitCommand {
    pub async fn execute(self) -> AionResult<()> {
        println!("{} Initializing AION-CR for organization...", "🏗️".blue());
        println!("Organization: {}", self.organization.bold());
        println!("Sector: {}", self.sector.cyan());
        println!("Region: {}", self.region.cyan());

        println!("\n{} Creating governance context...", "📋".blue());

        let context = GovernanceContext {
            organization: self.organization.clone(),
            sector: self.sector.clone(),
            region: self.region.clone(),
            applicable_jurisdictions: vec![
                Jurisdiction::International,
                Jurisdiction::Federal,
                Jurisdiction::Organizational,
            ],
            business_context: std::collections::HashMap::new(),
            risk_profile: "medium".to_string(),
            maturity_level: "developing".to_string(),
        };

        println!("{} Loading applicable frameworks...", "📚".blue());
        let applicable_frameworks = ComplianceFrameworkLibrary::get_applicable_frameworks(&context)?;

        println!("  {} Found {} applicable frameworks:", "✅".green(), applicable_frameworks.len());
        for framework in &applicable_frameworks {
            println!("    {} {}", "•".green(), framework.title.bold());
        }

        println!("\n{} AION-CR initialization complete!", "🎉".green());
        println!("Next steps:");
        println!("  {} Load frameworks: aion-cr compliance load-standards", "1.".bold());
        println!("  {} Assess compliance: aion-cr compliance assess --entity-id your-org --frameworks gdpr,sox", "2.".bold());
        println!("  {} Detect conflicts: aion-cr conflict detect", "3.".bold());

        Ok(())
    }
}