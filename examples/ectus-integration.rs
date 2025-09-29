// Example: AION-CR Integration with Ectus-R
// This demonstrates how to validate generated code for compliance

use aion_compliance::AdvancedComplianceEngine;
use aion_core::{GovernanceContext, NormativeFramework, BusinessRuleEngine};
use std::collections::HashMap;
use std::sync::Arc;

// Mock business rule engine for demo
struct MockBusinessRuleEngine;

impl BusinessRuleEngine for MockBusinessRuleEngine {
    fn validate_business_logic(&self, _entity_id: &str, _context: &HashMap<String, String>) -> Result<Vec<String>, aion_core::AionError> {
        Ok(vec![])
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AION-CR + Ectus-R Integration Example");
    println!("=========================================");

    // Initialize compliance engine
    let business_rule_engine = Arc::new(MockBusinessRuleEngine);
    let mut compliance_engine = AdvancedComplianceEngine::new(business_rule_engine);

    // Create governance context for a tech organization
    let context = GovernanceContext {
        organization: "Yatrogenesis".to_string(),
        sector: "technology".to_string(),
        region: "global".to_string(),
        risk_profile: "medium".to_string(),
        business_context: HashMap::from([
            ("organization_size".to_string(), "medium".to_string()),
            ("data_sensitivity".to_string(), "high".to_string()),
            ("regulatory_scope".to_string(), "international".to_string()),
        ]),
    };

    // Mock frameworks (in real usage, these would be loaded from database)
    let frameworks = vec![
        // Mock GDPR framework
        NormativeFramework {
            id: aion_core::NormativeId(uuid::Uuid::new_v4()),
            title: "GDPR - General Data Protection Regulation".to_string(),
            authority: "European Union".to_string(),
            jurisdiction: aion_core::Jurisdiction::Regional,
            effective_date: chrono::Utc::now() - chrono::Duration::days(365),
            version: "1.0".to_string(),
            requirements: vec![
                aion_core::Requirement {
                    id: uuid::Uuid::new_v4(),
                    title: "Data Protection by Design".to_string(),
                    description: "Data protection measures must be implemented by design and by default".to_string(),
                    mandatory: true,
                    category: "privacy".to_string(),
                    priority: 1,
                    conditions: vec![],
                    exceptions: vec![],
                    evidence_required: vec!["technical_documentation".to_string(), "privacy_assessment".to_string()],
                    validation_rules: vec![
                        aion_core::ValidationRule {
                            rule_type: "privacy_check".to_string(),
                            expression: "data_encryption == true AND access_controls == implemented".to_string(),
                            error_message: "Data must be encrypted and access controls implemented".to_string(),
                        }
                    ],
                }
            ],
            metadata: HashMap::new(),
        }
    ];

    println!("📋 Loaded {} compliance framework(s)", frameworks.len());

    // Simulate compliance assessment for generated code
    let entity_id = "ectus-generated-api-001";

    println!("🔍 Assessing compliance for entity: {}", entity_id);

    match compliance_engine.assess_compliance_comprehensive(entity_id, &frameworks, &context) {
        Ok(assessments) => {
            println!("✅ Compliance assessment completed!");

            for assessment in &assessments {
                println!("\n📊 Framework: {}", assessment.normative_framework.0);
                println!("   Status: {:?}", assessment.overall_status);
                println!("   Requirements assessed: {}", assessment.requirement_assessments.len());
                println!("   Findings: {}", assessment.findings.len());
                println!("   Recommendations: {}", assessment.recommendations.len());

                if !assessment.findings.is_empty() {
                    println!("   🚨 Critical findings:");
                    for finding in &assessment.findings {
                        if finding.severity == "critical" || finding.severity == "high" {
                            println!("     - {}: {}", finding.severity.to_uppercase(), finding.title);
                        }
                    }
                }

                if !assessment.recommendations.is_empty() {
                    println!("   💡 Top recommendations:");
                    for (i, rec) in assessment.recommendations.iter().take(3).enumerate() {
                        println!("     {}. [{}] {}", i+1, rec.priority.to_uppercase(), rec.title);
                    }
                }
            }

            // Generate compliance report
            if let Some(first_assessment) = assessments.first() {
                println!("\n📄 Generating compliance report...");
                match compliance_engine.generate_compliance_report(first_assessment) {
                    Ok(report) => {
                        println!("✅ Report generated successfully!");

                        // Save report to file
                        use std::fs;
                        fs::write("compliance_report.txt", &report)?;
                        println!("📁 Report saved to: compliance_report.txt");

                        // Print summary
                        let lines: Vec<&str> = report.lines().collect();
                        if lines.len() > 20 {
                            println!("\n📝 Report Summary (first 20 lines):");
                            for line in lines.iter().take(20) {
                                println!("{}", line);
                            }
                            println!("   ... (see full report in compliance_report.txt)");
                        } else {
                            println!("\n📝 Full Report:");
                            println!("{}", report);
                        }
                    }
                    Err(e) => println!("❌ Failed to generate report: {}", e),
                }
            }
        }
        Err(e) => {
            println!("❌ Compliance assessment failed: {}", e);
        }
    }

    println!("\n🎉 Integration example completed!");
    println!("\nNext steps:");
    println!("1. Integrate this with Ectus-R code generation");
    println!("2. Add automated fix suggestions");
    println!("3. Deploy as microservice for real-time compliance checking");

    Ok(())
}