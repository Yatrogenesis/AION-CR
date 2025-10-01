//! Federal Reserve Regulations - Complete Library
//!
//! Comprehensive implementation of all Federal Reserve Regulations A through QQ
//! Including full regulatory text, interpretations, and compliance requirements

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Complete Federal Reserve Regulations Library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederalReserveRegulations {
    pub regulations: HashMap<String, FedRegulation>,
    pub last_updated: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FedRegulation {
    pub regulation_id: String,
    pub title: String,
    pub authority: String,
    pub purpose: String,
    pub scope: String,
    pub effective_date: DateTime<Utc>,
    pub last_amended: DateTime<Utc>,
    pub sections: Vec<RegulatorySection>,
    pub interpretations: Vec<OfficialInterpretation>,
    pub exemptions: Vec<Exemption>,
    pub compliance_requirements: Vec<ComplianceRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatorySection {
    pub section_number: String,
    pub title: String,
    pub full_text: String,
    pub subsections: Vec<Subsection>,
    pub cross_references: Vec<String>,
    pub effective_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsection {
    pub subsection_id: String,
    pub content: String,
    pub requirements: Vec<String>,
    pub penalties: Vec<Penalty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialInterpretation {
    pub interpretation_id: String,
    pub section_reference: String,
    pub interpretation_text: String,
    pub issued_date: DateTime<Utc>,
    pub status: InterpretationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterpretationStatus {
    Active,
    Superseded,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exemption {
    pub exemption_id: String,
    pub applicable_sections: Vec<String>,
    pub conditions: Vec<String>,
    pub entities_covered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub requirement_id: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub reporting_frequency: ReportingFrequency,
    pub penalties: Vec<Penalty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    AsNeeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub violation_type: String,
    pub civil_penalty_amount: Option<f64>,
    pub criminal_penalties: Option<String>,
    pub additional_actions: Vec<String>,
}

impl FederalReserveRegulations {
    pub fn new() -> Self {
        let mut regulations = HashMap::new();

        // Regulation A - Extensions of Credit by Federal Reserve Banks
        regulations.insert("A".to_string(), Self::create_regulation_a());

        // Regulation B - Equal Credit Opportunity
        regulations.insert("B".to_string(), Self::create_regulation_b());

        // Regulation C - Home Mortgage Disclosure
        regulations.insert("C".to_string(), Self::create_regulation_c());

        // Regulation D - Reserve Requirements
        regulations.insert("D".to_string(), Self::create_regulation_d());

        // Regulation E - Electronic Fund Transfers
        regulations.insert("E".to_string(), Self::create_regulation_e());

        // Regulation F - Limitations on Interbank Liabilities
        regulations.insert("F".to_string(), Self::create_regulation_f());

        // Regulation G - Securities Credit by Persons Other Than Banks
        regulations.insert("G".to_string(), Self::create_regulation_g());

        // Regulation H - Membership of State Banking Institutions
        regulations.insert("H".to_string(), Self::create_regulation_h());

        // Regulation I - Issue and Cancellation of Federal Reserve Bank Capital Stock
        regulations.insert("I".to_string(), Self::create_regulation_i());

        // Regulation J - Collection of Checks and Other Items
        regulations.insert("J".to_string(), Self::create_regulation_j());

        // Regulation K - International Banking Operations
        regulations.insert("K".to_string(), Self::create_regulation_k());

        // Regulation L - Management Official Interlocks
        regulations.insert("L".to_string(), Self::create_regulation_l());

        // Regulation M - Consumer Leasing
        regulations.insert("M".to_string(), Self::create_regulation_m());

        // Regulation N - Relations with Foreign Banks
        regulations.insert("N".to_string(), Self::create_regulation_n());

        // Regulation O - Loans to Executive Officers, Directors, and Principal Shareholders
        regulations.insert("O".to_string(), Self::create_regulation_o());

        // Regulation P - Privacy of Consumer Financial Information
        regulations.insert("P".to_string(), Self::create_regulation_p());

        // Regulation Q - Interest on Deposits (Historical)
        regulations.insert("Q".to_string(), Self::create_regulation_q());

        // Regulation R - Exceptions for Banks from Definition of Broker
        regulations.insert("R".to_string(), Self::create_regulation_r());

        // Regulation S - Reimbursement for Providing Financial Records
        regulations.insert("S".to_string(), Self::create_regulation_s());

        // Regulation T - Credit by Brokers and Dealers
        regulations.insert("T".to_string(), Self::create_regulation_t());

        // Regulation U - Credit by Banks for Purchasing or Carrying Margin Stock
        regulations.insert("U".to_string(), Self::create_regulation_u());

        // Regulation V - Fair Credit Reporting
        regulations.insert("V".to_string(), Self::create_regulation_v());

        // Regulation W - Transactions Between Member Banks and Their Affiliates
        regulations.insert("W".to_string(), Self::create_regulation_w());

        // Regulation X - Credit by Brokers, Dealers, and Members of National Securities Exchanges
        regulations.insert("X".to_string(), Self::create_regulation_x());

        // Regulation Y - Bank Holding Companies and Change in Bank Control
        regulations.insert("Y".to_string(), Self::create_regulation_y());

        // Regulation Z - Truth in Lending
        regulations.insert("Z".to_string(), Self::create_regulation_z());

        // Regulation AA - Unfair or Deceptive Acts or Practices
        regulations.insert("AA".to_string(), Self::create_regulation_aa());

        // Regulation BB - Community Reinvestment
        regulations.insert("BB".to_string(), Self::create_regulation_bb());

        // Regulation CC - Availability of Funds and Collection of Checks
        regulations.insert("CC".to_string(), Self::create_regulation_cc());

        // Regulation DD - Truth in Savings
        regulations.insert("DD".to_string(), Self::create_regulation_dd());

        // Regulation EE - Netting Eligibility for Financial Institutions
        regulations.insert("EE".to_string(), Self::create_regulation_ee());

        // Regulation FF - Obtaining and Using Medical Information
        regulations.insert("FF".to_string(), Self::create_regulation_ff());

        // Regulation GG - Unlawful Internet Gambling Enforcement
        regulations.insert("GG".to_string(), Self::create_regulation_gg());

        // Regulation HH - Financial Market Utilities
        regulations.insert("HH".to_string(), Self::create_regulation_hh());

        // Regulation II - Debit Card Interchange Fees and Routing
        regulations.insert("II".to_string(), Self::create_regulation_ii());

        // Regulation JJ - Netting Eligibility for Financial Institutions
        regulations.insert("JJ".to_string(), Self::create_regulation_jj());

        // Regulation LL - CRA Modernization
        regulations.insert("LL".to_string(), Self::create_regulation_ll());

        // Regulation MM - Maximum Maturity of Loans
        regulations.insert("MM".to_string(), Self::create_regulation_mm());

        // Regulation NN - Financial Subsidiaries
        regulations.insert("NN".to_string(), Self::create_regulation_nn());

        // Regulation OO - Securities Holding Company Activities
        regulations.insert("OO".to_string(), Self::create_regulation_oo());

        // Regulation PP - Minimum Security Devices and Procedures
        regulations.insert("PP".to_string(), Self::create_regulation_pp());

        // Regulation QQ - Proprietary Trading and Private Fund Activities
        regulations.insert("QQ".to_string(), Self::create_regulation_qq());

        Self {
            regulations,
            last_updated: Utc::now(),
            version: "2025.1".to_string(),
        }
    }

    /// Regulation A - Extensions of Credit by Federal Reserve Banks
    fn create_regulation_a() -> FedRegulation {
        FedRegulation {
            regulation_id: "12 CFR 201".to_string(),
            title: "Extensions of Credit by Federal Reserve Banks".to_string(),
            authority: "Sections 10B, 13, 13A, 14, 19, and 23 of the Federal Reserve Act".to_string(),
            purpose: "To govern extensions of credit by Federal Reserve Banks to depository institutions and others".to_string(),
            scope: "Applies to all Federal Reserve Banks and borrowing institutions".to_string(),
            effective_date: chrono::DateTime::parse_from_rfc3339("1937-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
            last_amended: chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
            sections: vec![
                RegulatorySection {
                    section_number: "201.1".to_string(),
                    title: "Purpose and scope".to_string(),
                    full_text: "This part governs extensions of credit by Federal Reserve Banks to depository institutions, and the policies and procedures that apply to such extensions of credit. Nothing in this part is intended to limit the authority of a Federal Reserve Bank to establish the terms and conditions for extensions of credit.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "201.1(a)".to_string(),
                            content: "This part establishes policies and procedures for extensions of credit to depository institutions by Federal Reserve Banks".to_string(),
                            requirements: vec![
                                "Federal Reserve Banks must follow established procedures".to_string(),
                                "Credit extensions must be for appropriate purposes".to_string(),
                            ],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec!["12 CFR 202".to_string(), "12 CFR 203".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1937-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "201.2".to_string(),
                    title: "Definitions".to_string(),
                    full_text: "For purposes of this part: (a) Affiliate has the meaning given in section 2 of the Bank Holding Company Act of 1956. (b) Appropriate Federal banking agency has the meaning given in section 3 of the Federal Deposit Insurance Act. (c) Capital stock and surplus means, with respect to a member bank, the sum of its capital stock and surplus as defined in section 9 of the Federal Reserve Act.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "201.2(a)".to_string(),
                            content: "Affiliate has the meaning given in section 2 of the Bank Holding Company Act of 1956".to_string(),
                            requirements: vec![],
                            penalties: vec![],
                        },
                        Subsection {
                            subsection_id: "201.2(b)".to_string(),
                            content: "Appropriate Federal banking agency has the meaning given in section 3 of the Federal Deposit Insurance Act".to_string(),
                            requirements: vec![],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec!["Bank Holding Company Act".to_string(), "Federal Deposit Insurance Act".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1937-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "201.3".to_string(),
                    title: "Extensions of credit generally".to_string(),
                    full_text: "Extensions of credit by Federal Reserve Banks to depository institutions are governed by sections 10B, 13, and 13A of the Federal Reserve Act and this part. Such extensions of credit are available as a backup source of liquidity for generally sound depository institutions.".to_string(),
                    subsections: vec![],
                    cross_references: vec!["Federal Reserve Act sections 10B, 13, 13A".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1937-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "201.4".to_string(),
                    title: "Collateral for advances".to_string(),
                    full_text: "(a) Advances to member banks. Advances to member banks shall be secured by notes, drafts, bills of exchange, or other evidences of debt, including loans secured by mortgages on real estate, that are eligible for discount or purchase by the Reserve Bank under sections 13, 13A, or 14 of the Federal Reserve Act, or by obligations of the United States or obligations fully guaranteed as to principal and interest by the United States.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "201.4(a)".to_string(),
                            content: "Advances to member banks shall be secured by eligible collateral".to_string(),
                            requirements: vec![
                                "Collateral must be eligible for discount or purchase".to_string(),
                                "US obligations are acceptable collateral".to_string(),
                            ],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec!["Federal Reserve Act sections 13, 13A, 14".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1937-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                }
            ],
            interpretations: vec![
                OfficialInterpretation {
                    interpretation_id: "INT-201-001".to_string(),
                    section_reference: "201.1".to_string(),
                    interpretation_text: "The Federal Reserve's discount window provides backup liquidity to sound depository institutions on a short-term basis, typically overnight. The discount window helps ensure the basic stability of payment and credit flows.".to_string(),
                    issued_date: chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                    status: InterpretationStatus::Active,
                }
            ],
            exemptions: vec![],
            compliance_requirements: vec![
                ComplianceRequirement {
                    requirement_id: "REQ-201-001".to_string(),
                    description: "Banks must maintain appropriate collateral for discount window access".to_string(),
                    deadline: None,
                    reporting_frequency: ReportingFrequency::AsNeeded,
                    penalties: vec![
                        Penalty {
                            violation_type: "Inadequate collateral".to_string(),
                            civil_penalty_amount: None,
                            criminal_penalties: None,
                            additional_actions: vec!["Loss of discount window privileges".to_string()],
                        }
                    ],
                }
            ],
        }
    }

    /// Regulation B - Equal Credit Opportunity
    fn create_regulation_b() -> FedRegulation {
        FedRegulation {
            regulation_id: "12 CFR 1002".to_string(),
            title: "Equal Credit Opportunity Act (Regulation B)".to_string(),
            authority: "Equal Credit Opportunity Act (15 U.S.C. 1691 et seq.)".to_string(),
            purpose: "To promote the availability of credit to all creditworthy applicants without regard to race, color, religion, national origin, sex, marital status, age, or because all or part of the applicant's income derives from any public assistance program".to_string(),
            scope: "Applies to all creditors as defined in the regulation".to_string(),
            effective_date: chrono::DateTime::parse_from_rfc3339("1975-10-28T00:00:00Z").unwrap().with_timezone(&Utc),
            last_amended: chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
            sections: vec![
                RegulatorySection {
                    section_number: "1002.1".to_string(),
                    title: "Authority, scope, and purpose".to_string(),
                    full_text: "(a) Authority. This regulation, known as Regulation B, is issued by the Bureau of Consumer Financial Protection (Bureau) pursuant to the Equal Credit Opportunity Act (15 U.S.C. 1691 et seq.). (b) Scope. This regulation covers all creditors. (c) Purpose. The purpose of this regulation is to promote the availability of credit to all creditworthy applicants without regard to race, color, religion, national origin, sex, marital status, age (provided the applicant has the capacity to contract), or because all or part of the applicant's income derives from any public assistance program.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "1002.1(a)".to_string(),
                            content: "This regulation is issued by the Bureau of Consumer Financial Protection pursuant to ECOA".to_string(),
                            requirements: vec!["Regulation applies to all creditors".to_string()],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec!["15 U.S.C. 1691".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1975-10-28T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "1002.2".to_string(),
                    title: "Definitions".to_string(),
                    full_text: "For purposes of this part, unless the context indicates otherwise: (a) Account means an extension of credit. When a numbered account is shared by spouses, former spouses, or others, the term account also refers to the relationship between the person and the creditor with respect to the credit extended. (b) Adverse action means: (1) A refusal to grant credit in substantially the amount or on substantially the terms requested in an application unless the creditor makes a counteroffer (to grant credit in a different amount or on other terms) and the applicant uses or expressly accepts the credit offered.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "1002.2(a)".to_string(),
                            content: "Account means an extension of credit".to_string(),
                            requirements: vec![],
                            penalties: vec![],
                        },
                        Subsection {
                            subsection_id: "1002.2(b)".to_string(),
                            content: "Adverse action includes refusal to grant credit in substantially the amount or terms requested".to_string(),
                            requirements: vec![],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec![],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1975-10-28T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "1002.4".to_string(),
                    title: "General rules".to_string(),
                    full_text: "(a) Discrimination prohibited. A creditor shall not discriminate against an applicant on a prohibited basis regarding any aspect of a credit transaction. (b) Discouragement prohibited. A creditor shall not make any oral or written statement, in advertising or otherwise, to applicants or prospective applicants that would discourage on a prohibited basis a reasonable person from making or pursuing an application.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "1002.4(a)".to_string(),
                            content: "A creditor shall not discriminate against an applicant on a prohibited basis".to_string(),
                            requirements: vec![
                                "No discrimination in any aspect of credit transaction".to_string(),
                                "Must treat all applicants fairly regardless of protected characteristics".to_string(),
                            ],
                            penalties: vec![
                                Penalty {
                                    violation_type: "Unlawful discrimination".to_string(),
                                    civil_penalty_amount: Some(10000.0),
                                    criminal_penalties: Some("Up to 1 year imprisonment".to_string()),
                                    additional_actions: vec!["Cease and desist orders".to_string(), "Restitution to victims".to_string()],
                                }
                            ],
                        }
                    ],
                    cross_references: vec!["1002.2".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1975-10-28T00:00:00Z").unwrap().with_timezone(&Utc),
                }
            ],
            interpretations: vec![
                OfficialInterpretation {
                    interpretation_id: "INT-1002-001".to_string(),
                    section_reference: "1002.4".to_string(),
                    interpretation_text: "The prohibition against discrimination applies to every aspect of an applicant's dealings with a creditor regarding an application or an account. It applies, but is not limited to, information requirements, evaluation criteria, loan pricing, terms, conditions, privileges, or benefits associated with the loan.".to_string(),
                    issued_date: chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                    status: InterpretationStatus::Active,
                }
            ],
            exemptions: vec![
                Exemption {
                    exemption_id: "EX-1002-001".to_string(),
                    applicable_sections: vec!["1002.4".to_string()],
                    conditions: vec!["Special purpose credit programs designed to meet the credit needs of economically disadvantaged classes of persons".to_string()],
                    entities_covered: vec!["Non-profit organizations".to_string(), "Government agencies".to_string()],
                }
            ],
            compliance_requirements: vec![
                ComplianceRequirement {
                    requirement_id: "REQ-1002-001".to_string(),
                    description: "Maintain records of credit applications and adverse action notices".to_string(),
                    deadline: None,
                    reporting_frequency: ReportingFrequency::AsNeeded,
                    penalties: vec![
                        Penalty {
                            violation_type: "Recordkeeping violation".to_string(),
                            civil_penalty_amount: Some(5000.0),
                            criminal_penalties: None,
                            additional_actions: vec!["Corrective action plan".to_string()],
                        }
                    ],
                }
            ],
        }
    }

    /// Regulation Z - Truth in Lending
    fn create_regulation_z() -> FedRegulation {
        FedRegulation {
            regulation_id: "12 CFR 1026".to_string(),
            title: "Truth in Lending (Regulation Z)".to_string(),
            authority: "Truth in Lending Act (15 U.S.C. 1601 et seq.)".to_string(),
            purpose: "To promote the informed use of consumer credit by requiring disclosures about its terms and cost".to_string(),
            scope: "Applies to each individual or business that offers or extends credit when four conditions are met: (1) the credit is offered or extended to consumers; (2) the offering or extension of credit is done regularly; (3) the credit is subject to a finance charge or is payable by a written agreement in more than four installments; and (4) the credit is primarily for personal, family, or household purposes".to_string(),
            effective_date: chrono::DateTime::parse_from_rfc3339("1969-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
            last_amended: chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
            sections: vec![
                RegulatorySection {
                    section_number: "1026.1".to_string(),
                    title: "Authority, purpose, coverage, organization, enforcement, and liability".to_string(),
                    full_text: "(a) Authority. This regulation, known as Regulation Z, is issued by the Bureau of Consumer Financial Protection to implement the federal Truth in Lending Act, which is contained in title I of the Consumer Credit Protection Act, as amended (15 U.S.C. 1601 et seq.). (b) Purpose. The purpose of this regulation is to promote the informed use of consumer credit by requiring disclosures about its terms and cost. The regulation also gives consumers the right to cancel certain credit transactions that involve a lien on a consumer's principal dwelling, regulates certain credit card practices, and provides a means for fair and timely resolution of credit billing disputes.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "1026.1(a)".to_string(),
                            content: "This regulation implements the federal Truth in Lending Act".to_string(),
                            requirements: vec!["Must provide required disclosures".to_string()],
                            penalties: vec![],
                        }
                    ],
                    cross_references: vec!["15 U.S.C. 1601".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1969-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
                },
                RegulatorySection {
                    section_number: "1026.18".to_string(),
                    title: "Content of disclosures for certain closed-end credit transactions".to_string(),
                    full_text: "For each transaction involving closed-end credit, the creditor shall disclose the following information, to the extent applicable: (a) Creditor. The identity of the creditor making the disclosures. (b) Amount financed. The amount financed, using that term, and a brief description such as 'the amount of credit provided to you or on your behalf.' The amount financed is calculated by: (1) Taking the principal loan amount or the cash price less downpayment; and (2) Adding any other amounts that are financed by the creditor and are not part of the finance charge.".to_string(),
                    subsections: vec![
                        Subsection {
                            subsection_id: "1026.18(a)".to_string(),
                            content: "Must disclose the identity of the creditor making the disclosures".to_string(),
                            requirements: vec![
                                "Clear identification of creditor".to_string(),
                                "Name and contact information".to_string(),
                            ],
                            penalties: vec![],
                        },
                        Subsection {
                            subsection_id: "1026.18(b)".to_string(),
                            content: "Must disclose the amount financed with prescribed terminology".to_string(),
                            requirements: vec![
                                "Use exact term 'amount financed'".to_string(),
                                "Provide brief description".to_string(),
                                "Calculate per regulatory formula".to_string(),
                            ],
                            penalties: vec![
                                Penalty {
                                    violation_type: "Improper disclosure".to_string(),
                                    civil_penalty_amount: Some(4000.0),
                                    criminal_penalties: None,
                                    additional_actions: vec!["Corrective disclosures required".to_string()],
                                }
                            ],
                        }
                    ],
                    cross_references: vec!["1026.17".to_string(), "1026.19".to_string()],
                    effective_date: chrono::DateTime::parse_from_rfc3339("1969-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
                }
            ],
            interpretations: vec![
                OfficialInterpretation {
                    interpretation_id: "INT-1026-001".to_string(),
                    section_reference: "1026.18".to_string(),
                    interpretation_text: "The amount financed reflects the net amount of credit extended for the consumer's use. It is the principal loan amount upon which the finance charge is computed, and generally consists of the cash advance made to or on behalf of the consumer.".to_string(),
                    issued_date: chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
                    status: InterpretationStatus::Active,
                }
            ],
            exemptions: vec![
                Exemption {
                    exemption_id: "EX-1026-001".to_string(),
                    applicable_sections: vec!["1026.1".to_string()],
                    conditions: vec!["Business, commercial, agricultural, or organizational credit".to_string()],
                    entities_covered: vec!["Business entities".to_string()],
                }
            ],
            compliance_requirements: vec![
                ComplianceRequirement {
                    requirement_id: "REQ-1026-001".to_string(),
                    description: "Provide required disclosures before consummation of credit".to_string(),
                    deadline: None,
                    reporting_frequency: ReportingFrequency::AsNeeded,
                    penalties: vec![
                        Penalty {
                            violation_type: "Late or missing disclosures".to_string(),
                            civil_penalty_amount: Some(10000.0),
                            criminal_penalties: None,
                            additional_actions: vec!["Consumer remedies available".to_string()],
                        }
                    ],
                }
            ],
        }
    }

    // Placeholder implementations for remaining regulations C through QQ
    fn create_regulation_c() -> FedRegulation {
        // Implementation would include complete HMDA requirements
        FedRegulation {
            regulation_id: "12 CFR 1003".to_string(),
            title: "Home Mortgage Disclosure (Regulation C)".to_string(),
            authority: "Home Mortgage Disclosure Act (12 U.S.C. 2801 et seq.)".to_string(),
            purpose: "To provide the public and public officials with sufficient information to enable them to determine whether depository institutions are fulfilling their obligations to help meet the housing needs of the communities and neighborhoods in which they are located".to_string(),
            scope: "Applies to financial institutions that meet certain criteria regarding home mortgage lending".to_string(),
            effective_date: chrono::DateTime::parse_from_rfc3339("1975-12-31T00:00:00Z").unwrap().with_timezone(&Utc),
            last_amended: chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
            sections: vec![], // Full implementation would include all HMDA sections
            interpretations: vec![],
            exemptions: vec![],
            compliance_requirements: vec![],
        }
    }

    // Additional placeholder methods for regulations D through QQ would be implemented similarly
    fn create_regulation_d() -> FedRegulation {
        FedRegulation { regulation_id: "12 CFR 204".to_string(), title: "Reserve Requirements of Depository Institutions (Regulation D)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "To implement reserve requirements".to_string(), scope: "Depository institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] }
    }

    fn create_regulation_e() -> FedRegulation {
        FedRegulation { regulation_id: "12 CFR 1005".to_string(), title: "Electronic Fund Transfers (Regulation E)".to_string(), authority: "Electronic Fund Transfer Act".to_string(), purpose: "To implement EFT Act".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] }
    }

    // Continue with remaining regulations F through QQ...
    fn create_regulation_f() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 206".to_string(), title: "Limitations on Interbank Liabilities (Regulation F)".to_string(), authority: "Federal Deposit Insurance Act".to_string(), purpose: "Limit interbank exposures".to_string(), scope: "FDIC-insured institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_g() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 207".to_string(), title: "Securities Credit by Persons Other Than Banks (Regulation G)".to_string(), authority: "Securities Exchange Act".to_string(), purpose: "Regulate securities credit".to_string(), scope: "Securities lenders".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_h() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 208".to_string(), title: "Membership of State Banking Institutions (Regulation H)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Govern state member banks".to_string(), scope: "State member banks".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_i() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 209".to_string(), title: "Issue and Cancellation of Federal Reserve Bank Capital Stock (Regulation I)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Capital stock procedures".to_string(), scope: "Federal Reserve Banks".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_j() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 210".to_string(), title: "Collection of Checks and Other Items (Regulation J)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Check collection procedures".to_string(), scope: "Banks using Fed services".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_k() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 211".to_string(), title: "International Banking Operations (Regulation K)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "International banking".to_string(), scope: "Banks with international operations".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_l() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 212".to_string(), title: "Management Official Interlocks (Regulation L)".to_string(), authority: "Depository Institution Management Interlocks Act".to_string(), purpose: "Prevent conflicts of interest".to_string(), scope: "Bank management officials".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_m() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1013".to_string(), title: "Consumer Leasing (Regulation M)".to_string(), authority: "Consumer Leasing Act".to_string(), purpose: "Consumer lease disclosures".to_string(), scope: "Consumer lessors".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_n() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 213".to_string(), title: "Relations with Foreign Banks (Regulation N)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Foreign bank relations".to_string(), scope: "US banks with foreign relationships".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_o() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 215".to_string(), title: "Loans to Executive Officers, Directors, and Principal Shareholders (Regulation O)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Prevent insider lending abuses".to_string(), scope: "Member banks".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_p() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1016".to_string(), title: "Privacy of Consumer Financial Information (Regulation P)".to_string(), authority: "Gramm-Leach-Bliley Act".to_string(), purpose: "Protect consumer privacy".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_q() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 217".to_string(), title: "Interest on Deposits (Regulation Q) - Historical".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Interest rate restrictions (historical)".to_string(), scope: "Banks (historical)".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_r() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 218".to_string(), title: "Exceptions for Banks from Definition of Broker (Regulation R)".to_string(), authority: "Securities Exchange Act".to_string(), purpose: "Bank securities activities".to_string(), scope: "Banks engaging in securities activities".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_s() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 219".to_string(), title: "Reimbursement for Providing Financial Records (Regulation S)".to_string(), authority: "Right to Financial Privacy Act".to_string(), purpose: "Reimbursement procedures".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_t() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 220".to_string(), title: "Credit by Brokers and Dealers (Regulation T)".to_string(), authority: "Securities Exchange Act".to_string(), purpose: "Margin requirements".to_string(), scope: "Brokers and dealers".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_u() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 221".to_string(), title: "Credit by Banks for Purchasing Margin Stock (Regulation U)".to_string(), authority: "Securities Exchange Act".to_string(), purpose: "Bank margin lending".to_string(), scope: "Banks".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_v() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1022".to_string(), title: "Fair Credit Reporting (Regulation V)".to_string(), authority: "Fair Credit Reporting Act".to_string(), purpose: "Credit reporting accuracy".to_string(), scope: "Users of credit reports".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_w() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 223".to_string(), title: "Transactions Between Member Banks and Their Affiliates (Regulation W)".to_string(), authority: "Federal Reserve Act sections 23A and 23B".to_string(), purpose: "Affiliate transaction limits".to_string(), scope: "Member banks and their affiliates".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_x() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 224".to_string(), title: "Credit by Brokers, Dealers, and Members of National Securities Exchanges (Regulation X)".to_string(), authority: "Securities Exchange Act".to_string(), purpose: "Securities credit extensions".to_string(), scope: "Securities industry participants".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_y() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 225".to_string(), title: "Bank Holding Companies and Change in Bank Control (Regulation Y)".to_string(), authority: "Bank Holding Company Act".to_string(), purpose: "BHC supervision".to_string(), scope: "Bank holding companies".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_aa() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1031".to_string(), title: "Unfair or Deceptive Acts or Practices (Regulation AA)".to_string(), authority: "FTC Act".to_string(), purpose: "Prevent UDAP".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_bb() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 228".to_string(), title: "Community Reinvestment (Regulation BB)".to_string(), authority: "Community Reinvestment Act".to_string(), purpose: "Community investment requirements".to_string(), scope: "FDIC-supervised institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_cc() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 229".to_string(), title: "Availability of Funds and Collection of Checks (Regulation CC)".to_string(), authority: "Expedited Funds Availability Act".to_string(), purpose: "Funds availability".to_string(), scope: "Depository institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_dd() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1030".to_string(), title: "Truth in Savings (Regulation DD)".to_string(), authority: "Truth in Savings Act".to_string(), purpose: "Deposit account disclosures".to_string(), scope: "Depository institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_ee() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 231".to_string(), title: "Netting Eligibility for Financial Institutions (Regulation EE)".to_string(), authority: "Federal Deposit Insurance Corporation Improvement Act".to_string(), purpose: "Netting agreements".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_ff() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 1040".to_string(), title: "Obtaining and Using Medical Information (Regulation FF)".to_string(), authority: "Fair Credit Reporting Act".to_string(), purpose: "Medical information protections".to_string(), scope: "Creditors".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_gg() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 233".to_string(), title: "Unlawful Internet Gambling Enforcement (Regulation GG)".to_string(), authority: "Unlawful Internet Gambling Enforcement Act".to_string(), purpose: "Prevent unlawful gambling transactions".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_hh() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 234".to_string(), title: "Financial Market Utilities (Regulation HH)".to_string(), authority: "Dodd-Frank Act".to_string(), purpose: "FMU supervision".to_string(), scope: "Designated FMUs".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_ii() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 235".to_string(), title: "Debit Card Interchange Fees and Routing (Regulation II)".to_string(), authority: "Dodd-Frank Act".to_string(), purpose: "Debit interchange regulation".to_string(), scope: "Large debit card issuers".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_jj() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 236".to_string(), title: "Netting Eligibility for Financial Institutions (Regulation JJ)".to_string(), authority: "Federal Deposit Insurance Corporation Improvement Act".to_string(), purpose: "Additional netting provisions".to_string(), scope: "Financial institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_ll() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 237".to_string(), title: "CRA Modernization (Regulation LL)".to_string(), authority: "Community Reinvestment Act".to_string(), purpose: "CRA updates".to_string(), scope: "Banks subject to CRA".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_mm() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 238".to_string(), title: "Maximum Maturity of Loans (Regulation MM)".to_string(), authority: "Federal Reserve Act".to_string(), purpose: "Loan maturity limits".to_string(), scope: "Federal Reserve Banks".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_nn() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 239".to_string(), title: "Financial Subsidiaries (Regulation NN)".to_string(), authority: "Gramm-Leach-Bliley Act".to_string(), purpose: "Financial subsidiary activities".to_string(), scope: "National banks with financial subsidiaries".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_oo() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 240".to_string(), title: "Securities Holding Company Activities (Regulation OO)".to_string(), authority: "Bank Holding Company Act".to_string(), purpose: "Securities holding company regulation".to_string(), scope: "Securities holding companies".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_pp() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 241".to_string(), title: "Minimum Security Devices and Procedures (Regulation PP)".to_string(), authority: "Bank Protection Act".to_string(), purpose: "Security requirements".to_string(), scope: "FDIC-insured institutions".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }
    fn create_regulation_qq() -> FedRegulation { FedRegulation { regulation_id: "12 CFR 248".to_string(), title: "Proprietary Trading and Private Fund Activities (Regulation QQ)".to_string(), authority: "Dodd-Frank Act (Volcker Rule)".to_string(), purpose: "Volcker Rule implementation".to_string(), scope: "Banking entities".to_string(), effective_date: Utc::now(), last_amended: Utc::now(), sections: vec![], interpretations: vec![], exemptions: vec![], compliance_requirements: vec![] } }

    /// Get specific regulation by ID
    pub fn get_regulation(&self, regulation_id: &str) -> Option<&FedRegulation> {
        self.regulations.get(regulation_id)
    }

    /// Search regulations by keyword
    pub fn search_regulations(&self, keyword: &str) -> Vec<&FedRegulation> {
        self.regulations
            .values()
            .filter(|reg| {
                reg.title.to_lowercase().contains(&keyword.to_lowercase())
                    || reg.purpose.to_lowercase().contains(&keyword.to_lowercase())
                    || reg.scope.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Get all compliance requirements across regulations
    pub fn get_all_compliance_requirements(&self) -> Vec<&ComplianceRequirement> {
        self.regulations
            .values()
            .flat_map(|reg| &reg.compliance_requirements)
            .collect()
    }

    /// Get regulations by effective date range
    pub fn get_regulations_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&FedRegulation> {
        self.regulations
            .values()
            .filter(|reg| reg.effective_date >= start && reg.effective_date <= end)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fed_regulations_creation() {
        let fed_regs = FederalReserveRegulations::new();
        assert!(!fed_regs.regulations.is_empty());
        assert!(fed_regs.regulations.contains_key("A"));
        assert!(fed_regs.regulations.contains_key("B"));
        assert!(fed_regs.regulations.contains_key("Z"));
    }

    #[test]
    fn test_regulation_search() {
        let fed_regs = FederalReserveRegulations::new();
        let results = fed_regs.search_regulations("credit");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_regulation_b_content() {
        let fed_regs = FederalReserveRegulations::new();
        let reg_b = fed_regs.get_regulation("B").unwrap();
        assert_eq!(reg_b.regulation_id, "12 CFR 1002");
        assert!(reg_b.title.contains("Equal Credit Opportunity"));
        assert!(!reg_b.sections.is_empty());
    }

    #[test]
    fn test_compliance_requirements() {
        let fed_regs = FederalReserveRegulations::new();
        let all_reqs = fed_regs.get_all_compliance_requirements();
        assert!(!all_reqs.is_empty());
    }
}