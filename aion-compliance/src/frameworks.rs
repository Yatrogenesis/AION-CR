use aion_core::{
    AionResult, NormativeFramework, NormativeType, Jurisdiction, Requirement,
    Condition, ValidationRule, GovernanceContext
};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

pub struct ComplianceFrameworkLibrary;

impl ComplianceFrameworkLibrary {
    pub fn create_gdpr_framework() -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            "General Data Protection Regulation (GDPR)".to_string(),
            "European Union regulation on data protection and privacy for all individuals within the EU and the European Economic Area".to_string(),
            NormativeType::Regulation,
            Jurisdiction::International,
            "European Union".to_string(),
        );

        framework.tags = vec![
            "data-protection".to_string(),
            "privacy".to_string(),
            "eu".to_string(),
            "personal-data".to_string(),
        ];

        framework.metadata.insert("sector".to_string(), "all".to_string());
        framework.metadata.insert("region".to_string(), "europe".to_string());
        framework.metadata.insert("compliance_level".to_string(), "mandatory".to_string());
        framework.metadata.insert("effective_date".to_string(), "2018-05-25".to_string());

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Lawful Basis for Processing".to_string(),
            description: "Processing of personal data must have a lawful basis as defined in Article 6".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Consent obtained".to_string(),
                    expression: "consent = TRUE OR legal_obligation = TRUE OR vital_interests = TRUE".to_string(),
                    context_variables: vec!["consent".to_string(), "legal_obligation".to_string(), "vital_interests".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Consent records".to_string(),
                "Legal basis documentation".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "lawful_basis_documented".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "lawful_basis IS NOT NULL".to_string(),
                    error_message: "Lawful basis must be documented for all data processing".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "data-processing".to_string(),
        });

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Data Subject Rights".to_string(),
            description: "Individuals have rights regarding their personal data including access, rectification, erasure, and portability".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Response mechanism exists".to_string(),
                    expression: "response_mechanism = TRUE AND response_time <= 30_DAYS".to_string(),
                    context_variables: vec!["response_mechanism".to_string(), "response_time".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Data subject request procedures".to_string(),
                "Response time logs".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "response_procedure_exists".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "data_subject_response_procedure IS NOT NULL".to_string(),
                    error_message: "Data subject response procedure must be documented".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "data-subject-rights".to_string(),
        });

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Data Breach Notification".to_string(),
            description: "Personal data breaches must be notified to supervisory authority within 72 hours".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Notification within 72 hours".to_string(),
                    expression: "notification_time <= 72_HOURS".to_string(),
                    context_variables: vec!["notification_time".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Breach notification procedures".to_string(),
                "Incident response plan".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "breach_procedure_exists".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "breach_notification_procedure IS NOT NULL".to_string(),
                    error_message: "Data breach notification procedure must be documented".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "security".to_string(),
        });

        Ok(framework)
    }

    pub fn create_sox_framework() -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            "Sarbanes-Oxley Act (SOX)".to_string(),
            "United States federal law that set new or expanded requirements for all U.S. public companies and accounting firms".to_string(),
            NormativeType::Regulation,
            Jurisdiction::Federal,
            "Securities and Exchange Commission".to_string(),
        );

        framework.tags = vec![
            "financial-reporting".to_string(),
            "internal-controls".to_string(),
            "public-companies".to_string(),
            "auditing".to_string(),
        ];

        framework.metadata.insert("sector".to_string(), "financial".to_string());
        framework.metadata.insert("region".to_string(), "united-states".to_string());
        framework.metadata.insert("compliance_level".to_string(), "mandatory".to_string());
        framework.metadata.insert("effective_date".to_string(), "2002-07-30".to_string());

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Management Assessment of Internal Controls".to_string(),
            description: "Management must assess the effectiveness of internal control over financial reporting".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Annual assessment required".to_string(),
                    expression: "assessment_frequency = ANNUAL AND assessment_documented = TRUE".to_string(),
                    context_variables: vec!["assessment_frequency".to_string(), "assessment_documented".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Internal control assessment report".to_string(),
                "Management certifications".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "annual_assessment_completed".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "annual_icfr_assessment IS NOT NULL".to_string(),
                    error_message: "Annual internal control assessment must be completed".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "internal-controls".to_string(),
        });

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "CEO and CFO Certifications".to_string(),
            description: "Principal executive and financial officers must certify financial reports".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Quarterly certifications".to_string(),
                    expression: "ceo_certification = TRUE AND cfo_certification = TRUE".to_string(),
                    context_variables: vec!["ceo_certification".to_string(), "cfo_certification".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "CEO certification forms".to_string(),
                "CFO certification forms".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "executive_certifications_present".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "ceo_cert IS NOT NULL AND cfo_cert IS NOT NULL".to_string(),
                    error_message: "CEO and CFO certifications must be present".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "certifications".to_string(),
        });

        Ok(framework)
    }

    pub fn create_iso27001_framework() -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            "ISO/IEC 27001:2013 Information Security Management".to_string(),
            "International standard for information security management systems (ISMS)".to_string(),
            NormativeType::Standard,
            Jurisdiction::International,
            "International Organization for Standardization".to_string(),
        );

        framework.tags = vec![
            "information-security".to_string(),
            "isms".to_string(),
            "risk-management".to_string(),
            "security-controls".to_string(),
        ];

        framework.metadata.insert("sector".to_string(), "all".to_string());
        framework.metadata.insert("region".to_string(), "international".to_string());
        framework.metadata.insert("compliance_level".to_string(), "voluntary".to_string());
        framework.metadata.insert("effective_date".to_string(), "2013-10-01".to_string());

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Information Security Policy".to_string(),
            description: "Establish, implement, maintain and continually improve an information security management system".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Policy approved by management".to_string(),
                    expression: "policy_approved = TRUE AND policy_communicated = TRUE".to_string(),
                    context_variables: vec!["policy_approved".to_string(), "policy_communicated".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Information security policy document".to_string(),
                "Management approval records".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "security_policy_exists".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "information_security_policy IS NOT NULL".to_string(),
                    error_message: "Information security policy must be documented".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "governance".to_string(),
        });

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Risk Assessment and Treatment".to_string(),
            description: "Conduct regular risk assessments and implement risk treatment plans".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Annual risk assessment".to_string(),
                    expression: "risk_assessment_frequency <= 12_MONTHS".to_string(),
                    context_variables: vec!["risk_assessment_frequency".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Risk assessment reports".to_string(),
                "Risk treatment plans".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "risk_assessment_current".to_string(),
                    rule_type: "temporal".to_string(),
                    expression: "last_risk_assessment <= NOW() - 12_MONTHS".to_string(),
                    error_message: "Risk assessment must be conducted at least annually".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "risk-management".to_string(),
        });

        Ok(framework)
    }

    pub fn create_pci_dss_framework() -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            "Payment Card Industry Data Security Standard (PCI DSS)".to_string(),
            "Security standard for organizations that handle credit cards from major card schemes".to_string(),
            NormativeType::Standard,
            Jurisdiction::Sectoral,
            "Payment Card Industry Security Standards Council".to_string(),
        );

        framework.tags = vec![
            "payment-security".to_string(),
            "credit-cards".to_string(),
            "data-security".to_string(),
            "financial".to_string(),
        ];

        framework.metadata.insert("sector".to_string(), "financial,retail".to_string());
        framework.metadata.insert("region".to_string(), "global".to_string());
        framework.metadata.insert("compliance_level".to_string(), "mandatory".to_string());

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Build and Maintain Secure Networks".to_string(),
            description: "Install and maintain a firewall configuration to protect cardholder data".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Firewall properly configured".to_string(),
                    expression: "firewall_configured = TRUE AND default_passwords_changed = TRUE".to_string(),
                    context_variables: vec!["firewall_configured".to_string(), "default_passwords_changed".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Firewall configuration documentation".to_string(),
                "Network security assessments".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "firewall_configuration_documented".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "firewall_config_doc IS NOT NULL".to_string(),
                    error_message: "Firewall configuration must be documented".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "network-security".to_string(),
        });

        Ok(framework)
    }

    pub fn create_hipaa_framework() -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            "Health Insurance Portability and Accountability Act (HIPAA)".to_string(),
            "United States legislation that provides data privacy and security provisions for safeguarding medical information".to_string(),
            NormativeType::Regulation,
            Jurisdiction::Federal,
            "Department of Health and Human Services".to_string(),
        );

        framework.tags = vec![
            "healthcare".to_string(),
            "medical-privacy".to_string(),
            "phi".to_string(),
            "protected-health-information".to_string(),
        ];

        framework.metadata.insert("sector".to_string(), "healthcare".to_string());
        framework.metadata.insert("region".to_string(), "united-states".to_string());
        framework.metadata.insert("compliance_level".to_string(), "mandatory".to_string());

        framework.add_requirement(Requirement {
            id: Uuid::new_v4(),
            title: "Privacy Rule Compliance".to_string(),
            description: "Protect the privacy of individually identifiable health information".to_string(),
            mandatory: true,
            conditions: vec![
                Condition {
                    id: Uuid::new_v4(),
                    description: "Privacy policies implemented".to_string(),
                    expression: "privacy_policies = TRUE AND staff_training = TRUE".to_string(),
                    context_variables: vec!["privacy_policies".to_string(), "staff_training".to_string()],
                },
            ],
            exceptions: vec![],
            evidence_required: vec![
                "Privacy policies and procedures".to_string(),
                "Staff training records".to_string(),
            ],
            validation_rules: vec![
                ValidationRule {
                    id: Uuid::new_v4(),
                    name: "privacy_policies_exist".to_string(),
                    rule_type: "presence".to_string(),
                    expression: "hipaa_privacy_policies IS NOT NULL".to_string(),
                    error_message: "HIPAA privacy policies must be documented".to_string(),
                    severity: "error".to_string(),
                },
            ],
            priority: 1,
            category: "privacy".to_string(),
        });

        Ok(framework)
    }

    pub fn get_framework_by_type(framework_type: &str) -> AionResult<NormativeFramework> {
        match framework_type.to_lowercase().as_str() {
            "gdpr" => Self::create_gdpr_framework(),
            "sox" => Self::create_sox_framework(),
            "iso27001" => Self::create_iso27001_framework(),
            "pci-dss" | "pcidss" => Self::create_pci_dss_framework(),
            "hipaa" => Self::create_hipaa_framework(),
            _ => Err(aion_core::AionError::ValidationError {
                field: "framework_type".to_string(),
                message: format!("Unknown framework type: {}", framework_type),
            }),
        }
    }

    pub fn get_all_standard_frameworks() -> AionResult<Vec<NormativeFramework>> {
        Ok(vec![
            Self::create_gdpr_framework()?,
            Self::create_sox_framework()?,
            Self::create_iso27001_framework()?,
            Self::create_pci_dss_framework()?,
            Self::create_hipaa_framework()?,
        ])
    }

    pub fn get_applicable_frameworks(context: &GovernanceContext) -> AionResult<Vec<NormativeFramework>> {
        let all_frameworks = Self::get_all_standard_frameworks()?;
        let mut applicable = Vec::new();

        for framework in all_frameworks {
            if Self::is_framework_applicable(&framework, context) {
                applicable.push(framework);
            }
        }

        Ok(applicable)
    }

    fn is_framework_applicable(framework: &NormativeFramework, context: &GovernanceContext) -> bool {
        if let Some(applicable_sectors) = framework.metadata.get("sector") {
            if applicable_sectors != "all" && !applicable_sectors.split(',').any(|s| s.trim() == context.sector) {
                return false;
            }
        }

        if let Some(applicable_regions) = framework.metadata.get("region") {
            if applicable_regions != "global" && applicable_regions != "international" {
                if !applicable_regions.split(',').any(|r| r.trim() == context.region) {
                    return false;
                }
            }
        }

        match framework.jurisdiction {
            Jurisdiction::International => true,
            Jurisdiction::Federal => context.applicable_jurisdictions.contains(&Jurisdiction::Federal),
            Jurisdiction::State => context.applicable_jurisdictions.contains(&Jurisdiction::State),
            Jurisdiction::Regional => context.applicable_jurisdictions.contains(&Jurisdiction::Regional),
            Jurisdiction::Local => context.applicable_jurisdictions.contains(&Jurisdiction::Local),
            Jurisdiction::Sectoral => context.applicable_jurisdictions.contains(&Jurisdiction::Sectoral),
            Jurisdiction::Organizational => context.applicable_jurisdictions.contains(&Jurisdiction::Organizational),
            Jurisdiction::Departmental => context.applicable_jurisdictions.contains(&Jurisdiction::Departmental),
        }
    }

    pub fn create_custom_framework(
        title: String,
        description: String,
        authority: String,
        sector: String,
        requirements: Vec<(String, String, bool)>, // (title, description, mandatory)
    ) -> AionResult<NormativeFramework> {
        let mut framework = NormativeFramework::new(
            title,
            description,
            NormativeType::Framework,
            Jurisdiction::Organizational,
            authority,
        );

        framework.metadata.insert("sector".to_string(), sector);
        framework.metadata.insert("custom".to_string(), "true".to_string());

        for (idx, (req_title, req_description, mandatory)) in requirements.into_iter().enumerate() {
            framework.add_requirement(Requirement {
                id: Uuid::new_v4(),
                title: req_title,
                description: req_description,
                mandatory,
                conditions: vec![],
                exceptions: vec![],
                evidence_required: vec!["Documentation".to_string()],
                validation_rules: vec![
                    ValidationRule {
                        id: Uuid::new_v4(),
                        name: format!("requirement_{}_documented", idx),
                        rule_type: "presence".to_string(),
                        expression: "documentation IS NOT NULL".to_string(),
                        error_message: "Requirement must be documented".to_string(),
                        severity: "warning".to_string(),
                    },
                ],
                priority: if mandatory { 1 } else { 3 },
                category: "custom".to_string(),
            });
        }

        Ok(framework)
    }
}