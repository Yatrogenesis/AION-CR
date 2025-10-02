use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeruvianConstitution {
    pub constitution_name: String,
    pub promulgation_date: String,
    pub constitutional_principles: Vec<ConstitutionalPrinciple>,
    pub fundamental_rights: Vec<FundamentalRight>,
    pub state_organization: StateOrganization,
    pub constitutional_tribunal: ConstitutionalTribunal,
    pub decentralization_framework: DecentralizationFramework,
    pub constitutional_reform: ConstitutionalReform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalPrinciple {
    pub principle_name: String,
    pub article_number: i32,
    pub spanish_text: String,
    pub english_translation: String,
    pub jurisprudential_development: Vec<String>,
    pub practical_application: Vec<String>,
    pub comparative_analysis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRight {
    pub right_name: String,
    pub constitutional_basis: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub protection_mechanisms: Vec<String>,
    pub limitations: Vec<String>,
    pub international_standards: String,
    pub constitutional_tribunal_doctrine: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateOrganization {
    pub executive_power: ExecutivePower,
    pub legislative_power: LegislativePower,
    pub judicial_power: JudicialPower,
    pub autonomous_institutions: Vec<AutonomousInstitution>,
    pub electoral_system: ElectoralSystem,
    pub decentralized_governments: Vec<DecentralizedGovernment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutivePower {
    pub president: President,
    pub council_of_ministers: CouncilOfMinisters,
    pub public_administration: PublicAdministration,
    pub emergency_powers: EmergencyPowers,
    pub regulatory_powers: RegulatoryPowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct President {
    pub current_president: String,
    pub election_system: String,
    pub term_duration: String,
    pub constitutional_powers: Vec<PresidentialPower>,
    pub restrictions: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
    pub succession_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidentialPower {
    pub power_type: String,
    pub constitutional_article: String,
    pub spanish_description: String,
    pub english_description: String,
    pub limitations: Vec<String>,
    pub control_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilOfMinisters {
    pub prime_minister: String,
    pub ministers: Vec<Minister>,
    pub collective_responsibility: String,
    pub congressional_confidence: String,
    pub decision_making: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minister {
    pub ministry_name: String,
    pub minister_name: String,
    pub portfolio_areas: Vec<String>,
    pub regulatory_functions: Vec<String>,
    pub budget_responsibility: String,
    pub parliamentary_accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAdministration {
    pub organizational_structure: Vec<PublicEntity>,
    pub civil_service: CivilService,
    pub transparency_framework: TransparencyFramework,
    pub digitalization_initiatives: Vec<String>,
    pub efficiency_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicEntity {
    pub entity_name: String,
    pub entity_type: String,
    pub sector: String,
    pub functions: Vec<String>,
    pub autonomy_level: String,
    pub regulatory_capacity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilService {
    pub governing_law: String,
    pub merit_system: String,
    pub career_development: Vec<String>,
    pub ethics_framework: String,
    pub performance_evaluation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyFramework {
    pub transparency_law: String,
    pub access_to_information: String,
    pub accountability_mechanisms: Vec<String>,
    pub citizen_participation: Vec<String>,
    pub anti_corruption_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyPowers {
    pub emergency_types: Vec<String>,
    pub declaration_procedures: String,
    pub constitutional_limitations: Vec<String>,
    pub congressional_oversight: String,
    pub judicial_review: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryPowers {
    pub supreme_decrees: Vec<SupremeDecree>,
    pub ministerial_resolutions: Vec<MinisterialResolution>,
    pub regulatory_agencies: Vec<RegulatoryAgency>,
    pub regulatory_impact_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeDecree {
    pub decree_number: String,
    pub ministry: String,
    pub subject_matter: String,
    pub legal_basis: String,
    pub spanish_content: String,
    pub english_summary: String,
    pub implementation_date: String,
    pub affected_sectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinisterialResolution {
    pub resolution_number: String,
    pub issuing_ministry: String,
    pub administrative_subject: String,
    pub spanish_text: String,
    pub english_summary: String,
    pub implementation_guidelines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryAgency {
    pub agency_name: String,
    pub regulatory_sector: String,
    pub legal_framework: String,
    pub autonomy_level: String,
    pub regulatory_powers: Vec<String>,
    pub oversight_mechanisms: Vec<String>,
    pub accountability_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativePower {
    pub congress: Congress,
    pub legislative_process: LegislativeProcess,
    pub congressional_oversight: CongressionalOversight,
    pub budget_powers: BudgetPowers,
    pub impeachment_procedures: ImpeachmentProcedures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Congress {
    pub composition: String,
    pub election_system: String,
    pub term_duration: String,
    pub congressional_groups: Vec<CongressionalGroup>,
    pub leadership_structure: LeadershipStructure,
    pub committee_system: CommitteeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressionalGroup {
    pub group_name: String,
    pub members_count: u32,
    pub political_orientation: String,
    pub leadership: String,
    pub key_positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipStructure {
    pub congress_president: String,
    pub vice_presidents: Vec<String>,
    pub board_of_directors: Vec<String>,
    pub election_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeSystem {
    pub permanent_committees: Vec<PermanentCommittee>,
    pub special_committees: Vec<SpecialCommittee>,
    pub investigative_committees: Vec<InvestigativeCommittee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentCommittee {
    pub committee_name: String,
    pub jurisdiction: Vec<String>,
    pub membership: u32,
    pub chairman: String,
    pub key_legislation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialCommittee {
    pub committee_name: String,
    pub mandate: String,
    pub duration: String,
    pub specific_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigativeCommittee {
    pub investigation_subject: String,
    pub legal_basis: String,
    pub powers: Vec<String>,
    pub timeline: String,
    pub expected_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeProcess {
    pub bill_initiation: Vec<String>,
    pub committee_review: String,
    pub plenary_procedures: String,
    pub voting_requirements: HashMap<String, String>,
    pub presidential_promulgation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressionalOversight {
    pub oversight_mechanisms: Vec<String>,
    pub interpellation_procedures: String,
    pub censure_motions: String,
    pub investigative_powers: Vec<String>,
    pub accountability_sessions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetPowers {
    pub budget_approval: String,
    pub expenditure_oversight: String,
    pub public_debt_authorization: String,
    pub fiscal_control_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpeachmentProcedures {
    pub impeachable_officials: Vec<String>,
    pub grounds_for_impeachment: Vec<String>,
    pub procedural_requirements: String,
    pub trial_procedures: String,
    pub sanctions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialPower {
    pub supreme_court: SupremeCourt,
    pub superior_courts: Vec<SuperiorCourt>,
    pub specialized_courts: Vec<SpecializedCourt>,
    pub justices_of_peace: JusticesOfPeace,
    pub public_ministry: PublicMinistry,
    pub judicial_council: JudicialCouncil,
    pub judicial_administration: JudicialAdministration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourt {
    pub composition: String,
    pub president: String,
    pub chambers: Vec<SupremeChamber>,
    pub jurisdictional_functions: Vec<String>,
    pub administrative_functions: Vec<String>,
    pub disciplinary_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeChamber {
    pub chamber_name: String,
    pub specialization: String,
    pub judges: Vec<String>,
    pub jurisdiction: Vec<String>,
    pub notable_cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperiorCourt {
    pub court_name: String,
    pub territorial_jurisdiction: Vec<String>,
    pub chambers: Vec<String>,
    pub caseload_statistics: String,
    pub performance_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedCourt {
    pub court_type: String,
    pub specialization_area: String,
    pub jurisdiction: Vec<String>,
    pub special_procedures: Vec<String>,
    pub expert_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JusticesOfPeace {
    pub organizational_framework: String,
    pub election_system: String,
    pub jurisdiction: Vec<String>,
    pub community_integration: String,
    pub alternative_justice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMinistry {
    pub attorney_general: String,
    pub organizational_structure: Vec<String>,
    pub prosecutorial_functions: Vec<String>,
    pub independence_guarantees: Vec<String>,
    pub oversight_mechanisms: Vec<String>,
    pub anti_corruption_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialCouncil {
    pub composition: String,
    pub selection_functions: Vec<String>,
    pub disciplinary_functions: Vec<String>,
    pub evaluation_system: String,
    pub transparency_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialAdministration {
    pub administrative_council: String,
    pub budget_management: String,
    pub infrastructure_development: String,
    pub technology_modernization: Vec<String>,
    pub performance_monitoring: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousInstitution {
    pub institution_name: String,
    pub constitutional_basis: String,
    pub autonomy_level: String,
    pub functions: Vec<String>,
    pub governance_structure: String,
    pub accountability_mechanisms: Vec<String>,
    pub budgetary_autonomy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystem {
    pub electoral_institutions: Vec<ElectoralInstitution>,
    pub election_types: Vec<ElectionType>,
    pub electoral_procedures: ElectoralProcedures,
    pub political_parties: Vec<PoliticalParty>,
    pub campaign_finance: CampaignFinance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralInstitution {
    pub institution_name: String,
    pub functions: Vec<String>,
    pub composition: String,
    pub independence_level: String,
    pub oversight_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionType {
    pub election_name: String,
    pub frequency: String,
    pub electoral_system: String,
    pub eligibility_requirements: Vec<String>,
    pub campaign_regulations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralProcedures {
    pub voter_registration: String,
    pub candidate_registration: String,
    pub campaign_period: String,
    pub voting_procedures: String,
    pub vote_counting: String,
    pub dispute_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub party_name: String,
    pub registration_status: String,
    pub ideological_orientation: String,
    pub leadership: String,
    pub electoral_performance: Vec<String>,
    pub internal_democracy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignFinance {
    pub funding_sources: Vec<String>,
    pub spending_limits: String,
    pub disclosure_requirements: Vec<String>,
    pub oversight_mechanisms: String,
    pub sanctions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedGovernment {
    pub government_type: String,
    pub legal_framework: String,
    pub autonomy_level: String,
    pub competences: Vec<String>,
    pub governance_structure: String,
    pub fiscal_arrangements: String,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalTribunal {
    pub composition: String,
    pub appointment_process: String,
    pub term_duration: String,
    pub jurisdictional_competences: Vec<String>,
    pub procedural_rules: String,
    pub constitutional_doctrine: Vec<ConstitutionalDoctrine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalDoctrine {
    pub doctrine_name: String,
    pub legal_foundation: String,
    pub precedential_cases: Vec<String>,
    pub application_scope: String,
    pub evolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizationFramework {
    pub regional_governments: Vec<RegionalGovernment>,
    pub local_governments: Vec<LocalGovernment>,
    pub competence_distribution: CompetenceDistribution,
    pub fiscal_decentralization: FiscalDecentralization,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalGovernment {
    pub region_name: String,
    pub capital_city: String,
    pub governor: String,
    pub regional_council: String,
    pub population: u64,
    pub area_km2: u64,
    pub economic_profile: String,
    pub development_priorities: Vec<String>,
    pub regional_competences: Vec<String>,
    pub coordination_council: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGovernment {
    pub government_type: String, // Provincial or District
    pub government_name: String,
    pub mayor: String,
    pub municipal_council: String,
    pub population: u64,
    pub municipal_competences: Vec<String>,
    pub service_delivery: Vec<String>,
    pub participatory_budget: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetenceDistribution {
    pub national_competences: Vec<String>,
    pub regional_competences: Vec<String>,
    pub local_competences: Vec<String>,
    pub shared_competences: Vec<String>,
    pub conflict_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalDecentralization {
    pub revenue_distribution: String,
    pub transfer_mechanisms: Vec<String>,
    pub tax_assignment: String,
    pub borrowing_rules: String,
    pub fiscal_responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalReform {
    pub amendment_procedures: Vec<String>,
    pub referendum_requirements: String,
    pub constituent_assembly: String,
    pub reform_initiatives: Vec<String>,
    pub constitutional_convention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeruvianLegalCode {
    pub code_name: String,
    pub legal_basis: String,
    pub promulgation_date: String,
    pub structure: CodeStructure,
    pub fundamental_principles: Vec<String>,
    pub international_influence: String,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStructure {
    pub books: Vec<CodeBook>,
    pub total_articles: u32,
    pub preliminary_title: String,
    pub final_provisions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBook {
    pub book_number: i32,
    pub book_title: String,
    pub sections: Vec<CodeSection>,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSection {
    pub section_number: i32,
    pub section_title: String,
    pub articles: Vec<CodeArticle>,
    pub key_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeArticle {
    pub article_number: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub jurisprudential_interpretation: Vec<String>,
    pub doctrinal_commentary: Vec<String>,
    pub practical_application: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReform {
    pub reform_law: String,
    pub reform_date: String,
    pub modified_articles: Vec<String>,
    pub reform_rationale: String,
    pub implementation_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeruvianLegalSystem {
    pub constitution: PeruvianConstitution,
    pub legal_codes: Vec<PeruvianLegalCode>,
    pub special_legislation: Vec<SpecialLaw>,
    pub international_law: InternationalLaw,
    pub indigenous_law: IndigenousLaw,
    pub regulatory_framework: RegulatoryFramework,
    pub legal_profession: LegalProfession,
    pub legal_education: LegalEducation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialLaw {
    pub law_number: String,
    pub law_title: String,
    pub promulgation_date: String,
    pub subject_matter: String,
    pub spanish_text: String,
    pub english_summary: String,
    pub implementing_regulations: Vec<String>,
    pub enforcement_agencies: Vec<String>,
    pub compliance_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalLaw {
    pub constitutional_hierarchy: String,
    pub treaty_ratification: String,
    pub international_agreements: Vec<InternationalAgreement>,
    pub supranational_integration: Vec<String>,
    pub international_jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalAgreement {
    pub agreement_name: String,
    pub agreement_type: String,
    pub ratification_date: String,
    pub constitutional_status: String,
    pub spanish_text: String,
    pub english_text: String,
    pub implementation_law: String,
    pub compliance_monitoring: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndigenousLaw {
    pub constitutional_recognition: String,
    pub indigenous_languages: Vec<String>,
    pub consultation_law: String,
    pub territorial_rights: String,
    pub cultural_protection: Vec<String>,
    pub traditional_justice: String,
    pub intercultural_dialogue: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub regulatory_hierarchy: Vec<String>,
    pub regulatory_agencies: Vec<RegulatoryAgency>,
    pub administrative_procedures: String,
    pub regulatory_impact_assessment: String,
    pub citizen_participation: Vec<String>,
    pub regulatory_reform: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProfession {
    pub bar_associations: Vec<String>,
    pub professional_requirements: String,
    pub disciplinary_system: String,
    pub legal_specializations: Vec<String>,
    pub professional_development: String,
    pub ethics_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEducation {
    pub law_schools: Vec<LawSchool>,
    pub curriculum_standards: String,
    pub accreditation_system: String,
    pub clinical_programs: Vec<String>,
    pub postgraduate_education: Vec<String>,
    pub continuing_education: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawSchool {
    pub institution_name: String,
    pub accreditation_status: String,
    pub program_duration: String,
    pub specialization_areas: Vec<String>,
    pub clinical_offerings: Vec<String>,
    pub graduate_employment: String,
}

impl PeruvianLegalSystem {
    pub fn new() -> Self {
        let constitution = PeruvianConstitution {
            constitution_name: "Political Constitution of Peru".to_string(),
            promulgation_date: "1993-12-29".to_string(),
            constitutional_principles: vec![
                ConstitutionalPrinciple {
                    principle_name: "Human Dignity".to_string(),
                    article_number: 1,
                    spanish_text: "La defensa de la persona humana y el respeto de su dignidad son el fin supremo de la sociedad y del Estado.".to_string(),
                    english_translation: "The defense of the human person and the respect for their dignity are the supreme end of society and the State.".to_string(),
                    jurisprudential_development: vec!["Constitutional Tribunal precedent on dignity as foundational principle".to_string()],
                    practical_application: vec!["Basis for all human rights protection".to_string(), "Foundation for social policies".to_string()],
                    comparative_analysis: vec!["Similar to German Basic Law Article 1".to_string()],
                },
                ConstitutionalPrinciple {
                    principle_name: "Democratic Republic".to_string(),
                    article_number: 43,
                    spanish_text: "La República del Perú es democrática, social, independiente y soberana. El Estado es uno e indivisible. Su gobierno es unitario, representativo y descentralizado, y se organiza según el principio de la separación de poderes.".to_string(),
                    english_translation: "The Republic of Peru is democratic, social, independent and sovereign. The State is one and indivisible. Its government is unitary, representative and decentralized, and is organized according to the principle of separation of powers.".to_string(),
                    jurisprudential_development: vec!["Constitutional Tribunal interpretations on democratic governance".to_string()],
                    practical_application: vec!["Framework for government organization".to_string(), "Basis for decentralization".to_string()],
                    comparative_analysis: vec!["Unitary state model with decentralization".to_string()],
                }
            ],
            fundamental_rights: vec![
                FundamentalRight {
                    right_name: "Right to Life".to_string(),
                    constitutional_basis: "Article 2(1)".to_string(),
                    spanish_text: "Toda persona tiene derecho: A la vida, a su identidad, a su integridad moral, psíquica y física y a su libre desarrollo y bienestar.".to_string(),
                    english_translation: "Every person has the right: To life, to their identity, to their moral, psychological and physical integrity and to their free development and well-being.".to_string(),
                    protection_mechanisms: vec!["Constitutional protection action".to_string(), "Criminal law protection".to_string()],
                    limitations: vec!["Legitimate defense".to_string(), "State of emergency restrictions".to_string()],
                    international_standards: "American Convention on Human Rights compliance".to_string(),
                    constitutional_tribunal_doctrine: vec!["Life as fundamental value".to_string(), "State duty to protect".to_string()],
                }
            ],
            state_organization: StateOrganization {
                executive_power: ExecutivePower {
                    president: President {
                        current_president: "Dina Ercilia Boluarte Zegarra".to_string(),
                        election_system: "Direct universal suffrage with second round".to_string(),
                        term_duration: "5 years, non-consecutive re-election".to_string(),
                        constitutional_powers: vec![
                            PresidentialPower {
                                power_type: "Legislative Initiative".to_string(),
                                constitutional_article: "Article 107".to_string(),
                                spanish_description: "El Presidente de la República y los Congresistas tienen derecho a iniciativa en la formación de las leyes.".to_string(),
                                english_description: "The President of the Republic and Congressmen have the right of initiative in the formation of laws.".to_string(),
                                limitations: vec!["Cannot initiate laws on taxation".to_string()],
                                control_mechanisms: vec!["Congressional review".to_string(), "Constitutional Tribunal review".to_string()],
                            }
                        ],
                        restrictions: vec!["Constitutional limitations".to_string(), "Congressional oversight".to_string()],
                        accountability_mechanisms: vec!["Impeachment".to_string(), "Constitutional responsibility".to_string()],
                        succession_system: "Vice President succession".to_string(),
                    },
                    council_of_ministers: CouncilOfMinisters {
                        prime_minister: "Alberto Otárola Peñaranda".to_string(),
                        ministers: vec![
                            Minister {
                                ministry_name: "Ministry of Justice and Human Rights".to_string(),
                                minister_name: "Daniel Maurate Romero".to_string(),
                                portfolio_areas: vec!["Justice administration".to_string(), "Human rights".to_string(), "Prison system".to_string()],
                                regulatory_functions: vec!["Justice sector regulations".to_string(), "Human rights policies".to_string()],
                                budget_responsibility: "5.2 billion soles (2024)".to_string(),
                                parliamentary_accountability: vec!["Congressional interpellation".to_string(), "Question time".to_string()],
                            }
                        ],
                        collective_responsibility: "Solidarity responsibility before Congress".to_string(),
                        congressional_confidence: "Required vote of confidence".to_string(),
                        decision_making: "Collective decision making in Council".to_string(),
                    },
                    public_administration: PublicAdministration {
                        organizational_structure: vec![
                            PublicEntity {
                                entity_name: "National Superintendency of Tax Administration (SUNAT)".to_string(),
                                entity_type: "Specialized Technical Body".to_string(),
                                sector: "Economy and Finance".to_string(),
                                functions: vec!["Tax collection".to_string(), "Customs administration".to_string()],
                                autonomy_level: "Functional autonomy".to_string(),
                                regulatory_capacity: vec!["Tax regulations".to_string(), "Customs procedures".to_string()],
                            }
                        ],
                        civil_service: CivilService {
                            governing_law: "Civil Service Law - Law No. 30057".to_string(),
                            merit_system: "Open competition and merit-based selection".to_string(),
                            career_development: vec!["Professional development programs".to_string(), "Performance evaluation".to_string()],
                            ethics_framework: "Code of Ethics for Public Service".to_string(),
                            performance_evaluation: "Annual performance evaluation system".to_string(),
                        },
                        transparency_framework: TransparencyFramework {
                            transparency_law: "Transparency and Access to Information Law - Law No. 27806".to_string(),
                            access_to_information: "Constitutional right to access public information".to_string(),
                            accountability_mechanisms: vec!["Public accountability reports".to_string(), "Citizen oversight".to_string()],
                            citizen_participation: vec!["Participatory budgeting".to_string(), "Public consultations".to_string()],
                            anti_corruption_measures: vec!["Integrity plans".to_string(), "Transparency portals".to_string()],
                        },
                        digitalization_initiatives: vec!["Digital government platform".to_string(), "Electronic procedures".to_string()],
                        efficiency_measures: vec!["Administrative simplification".to_string(), "Results-based management".to_string()],
                    },
                    emergency_powers: EmergencyPowers {
                        emergency_types: vec!["State of emergency".to_string(), "State of siege".to_string()],
                        declaration_procedures: "Presidential decree with Council of Ministers agreement".to_string(),
                        constitutional_limitations: vec!["Maximum 60 days".to_string(), "Specific rights non-derogable".to_string()],
                        congressional_oversight: "Congressional ratification required".to_string(),
                        judicial_review: "Constitutional Tribunal review of constitutionality".to_string(),
                    },
                    regulatory_powers: RegulatoryPowers {
                        supreme_decrees: vec![
                            SupremeDecree {
                                decree_number: "DS-001-2024-JUS".to_string(),
                                ministry: "Ministry of Justice".to_string(),
                                subject_matter: "Judicial modernization".to_string(),
                                legal_basis: "Judicial Organization Law".to_string(),
                                spanish_content: "Decreto Supremo que aprueba el Plan Nacional de Modernización del Sistema de Justicia".to_string(),
                                english_summary: "Supreme Decree approving the National Plan for Justice System Modernization".to_string(),
                                implementation_date: "2024-01-15".to_string(),
                                affected_sectors: vec!["Judicial Power".to_string(), "Public Ministry".to_string()],
                            }
                        ],
                        ministerial_resolutions: vec![
                            MinisterialResolution {
                                resolution_number: "RM-045-2024-JUS".to_string(),
                                issuing_ministry: "Ministry of Justice".to_string(),
                                administrative_subject: "Court administration procedures".to_string(),
                                spanish_text: "Resolución Ministerial que establece procedimientos para la administración de expedientes judiciales".to_string(),
                                english_summary: "Ministerial Resolution establishing procedures for judicial case administration".to_string(),
                                implementation_guidelines: vec!["Digital case management".to_string(), "Timeline standards".to_string()],
                            }
                        ],
                        regulatory_agencies: vec![
                            RegulatoryAgency {
                                agency_name: "Supervisory Body for Investment in Energy and Mining (OSINERGMIN)".to_string(),
                                regulatory_sector: "Energy and Mining".to_string(),
                                legal_framework: "Law No. 26734".to_string(),
                                autonomy_level: "Technical and administrative autonomy".to_string(),
                                regulatory_powers: vec!["Tariff regulation".to_string(), "Quality standards".to_string(), "Safety oversight".to_string()],
                                oversight_mechanisms: vec!["Congressional oversight".to_string(), "Transparency reports".to_string()],
                                accountability_requirements: vec!["Annual reports".to_string(), "Public consultations".to_string()],
                            }
                        ],
                        regulatory_impact_assessment: "Mandatory for significant regulations".to_string(),
                    },
                },
                legislative_power: LegislativePower {
                    congress: Congress {
                        composition: "130 congresspeople, single chamber".to_string(),
                        election_system: "Proportional representation with preferential vote".to_string(),
                        term_duration: "5 years".to_string(),
                        congressional_groups: vec![
                            CongressionalGroup {
                                group_name: "Peru Libre".to_string(),
                                members_count: 37,
                                political_orientation: "Left-wing".to_string(),
                                leadership: "Vladimir Cerrón".to_string(),
                                key_positions: vec!["Committee chairs".to_string()],
                            }
                        ],
                        leadership_structure: LeadershipStructure {
                            congress_president: "Alejandro Soto Reyes".to_string(),
                            vice_presidents: vec!["María del Carmen Alva".to_string()],
                            board_of_directors: vec!["Members from different parliamentary groups".to_string()],
                            election_procedures: "Internal election by congressional groups".to_string(),
                        },
                        committee_system: CommitteeSystem {
                            permanent_committees: vec![
                                PermanentCommittee {
                                    committee_name: "Constitution and Regulations Committee".to_string(),
                                    jurisdiction: vec!["Constitutional matters".to_string(), "Congressional regulations".to_string()],
                                    membership: 15,
                                    chairman: "Hernando Guerra García".to_string(),
                                    key_legislation: vec!["Constitutional reform proposals".to_string()],
                                }
                            ],
                            special_committees: vec![
                                SpecialCommittee {
                                    committee_name: "Political Crisis Investigation".to_string(),
                                    mandate: "Investigate institutional crisis".to_string(),
                                    duration: "6 months".to_string(),
                                    specific_functions: vec!["Fact-finding".to_string(), "Recommendations".to_string()],
                                }
                            ],
                            investigative_committees: vec![
                                InvestigativeCommittee {
                                    investigation_subject: "Corruption in public works".to_string(),
                                    legal_basis: "Congressional powers of investigation".to_string(),
                                    powers: vec!["Summon witnesses".to_string(), "Request documents".to_string()],
                                    timeline: "12 months".to_string(),
                                    expected_outcomes: vec!["Investigation report".to_string(), "Recommendations to Public Ministry".to_string()],
                                }
                            ],
                        },
                    },
                    legislative_process: LegislativeProcess {
                        bill_initiation: vec!["Presidential initiative".to_string(), "Congressional initiative".to_string(), "Regional governments initiative".to_string()],
                        committee_review: "Mandatory committee review for all bills".to_string(),
                        plenary_procedures: "Three debate stages for regular bills".to_string(),
                        voting_requirements: {
                            let mut map = HashMap::new();
                            map.insert("Regular laws".to_string(), "Simple majority".to_string());
                            map.insert("Constitutional laws".to_string(), "Absolute majority".to_string());
                            map.insert("Constitutional amendments".to_string(), "Two-thirds majority".to_string());
                            map
                        },
                        presidential_promulgation: "Presidential promulgation within 15 days".to_string(),
                    },
                    congressional_oversight: CongressionalOversight {
                        oversight_mechanisms: vec!["Interpellations".to_string(), "Questions".to_string(), "Investigative committees".to_string()],
                        interpellation_procedures: "Formal questioning of ministers".to_string(),
                        censure_motions: "Censure vote requiring absolute majority".to_string(),
                        investigative_powers: vec!["Document requests".to_string(), "Witness testimony".to_string()],
                        accountability_sessions: "Regular accountability sessions with ministers".to_string(),
                    },
                    budget_powers: BudgetPowers {
                        budget_approval: "Annual budget law approval".to_string(),
                        expenditure_oversight: "Quarterly budget execution reports".to_string(),
                        public_debt_authorization: "Congressional authorization for debt".to_string(),
                        fiscal_control_mechanisms: vec!["Budget committee oversight".to_string(), "Comptroller General reports".to_string()],
                    },
                    impeachment_procedures: ImpeachmentProcedures {
                        impeachable_officials: vec!["President".to_string(), "Vice Presidents".to_string(), "Ministers".to_string(), "Supreme Court Justices".to_string()],
                        grounds_for_impeachment: vec!["Violation of Constitution".to_string(), "Treason".to_string(), "Preventing elections".to_string()],
                        procedural_requirements: "Two-thirds majority for impeachment".to_string(),
                        trial_procedures: "Congressional trial procedures".to_string(),
                        sanctions: vec!["Removal from office".to_string(), "Disqualification from public office".to_string()],
                    },
                },
                judicial_power: JudicialPower {
                    supreme_court: SupremeCourt {
                        composition: "18 Supreme Justices".to_string(),
                        president: "Javier Arévalo Vela".to_string(),
                        chambers: vec![
                            SupremeChamber {
                                chamber_name: "Civil Chamber".to_string(),
                                specialization: "Civil and commercial matters".to_string(),
                                judges: vec!["Specialized civil judges".to_string()],
                                jurisdiction: vec!["Cassation appeals".to_string(), "Jurisdictional conflicts".to_string()],
                                notable_cases: vec!["Major civil precedents".to_string()],
                            }
                        ],
                        jurisdictional_functions: vec!["Cassation court".to_string(), "Constitutional jurisdiction".to_string()],
                        administrative_functions: vec!["Judicial administration".to_string(), "Budget management".to_string()],
                        disciplinary_powers: vec!["Judicial discipline".to_string(), "Ethics enforcement".to_string()],
                    },
                    superior_courts: vec![
                        SuperiorCourt {
                            court_name: "Superior Court of Lima".to_string(),
                            territorial_jurisdiction: vec!["Lima Province".to_string(), "Constitutional Province of Callao".to_string()],
                            chambers: vec!["Civil".to_string(), "Criminal".to_string(), "Constitutional".to_string()],
                            caseload_statistics: "Highest caseload in the country".to_string(),
                            performance_indicators: vec!["Case resolution time".to_string(), "Backlog reduction".to_string()],
                        }
                    ],
                    specialized_courts: vec![
                        SpecializedCourt {
                            court_type: "Anti-Corruption Courts".to_string(),
                            specialization_area: "Corruption and organized crime".to_string(),
                            jurisdiction: vec!["High-level corruption cases".to_string(), "Money laundering".to_string()],
                            special_procedures: vec!["Protected witnesses".to_string(), "Asset forfeiture".to_string()],
                            expert_requirements: vec!["Specialized training".to_string(), "Security clearance".to_string()],
                        }
                    ],
                    justices_of_peace: JusticesOfPeace {
                        organizational_framework: "Community-based justice system".to_string(),
                        election_system: "Community election for 4-year terms".to_string(),
                        jurisdiction: vec!["Minor civil matters".to_string(), "Conciliation".to_string()],
                        community_integration: "Traditional and modern justice integration".to_string(),
                        alternative_justice: vec!["Mediation".to_string(), "Community service".to_string()],
                    },
                    public_ministry: PublicMinistry {
                        attorney_general: "Patricia Benavides Vargas".to_string(),
                        organizational_structure: vec!["National level".to_string(), "District level".to_string(), "Provincial level".to_string()],
                        prosecutorial_functions: vec!["Criminal prosecution".to_string(), "Constitutional defense".to_string(), "Family protection".to_string()],
                        independence_guarantees: vec!["Constitutional autonomy".to_string(), "Budgetary independence".to_string()],
                        oversight_mechanisms: vec!["Board of Supreme Prosecutors".to_string(), "Internal control".to_string()],
                        anti_corruption_role: "Specialized anti-corruption prosecutors".to_string(),
                    },
                    judicial_council: JudicialCouncil {
                        composition: "National Magistrates Council (CNM)".to_string(),
                        selection_functions: vec!["Judge selection".to_string(), "Prosecutor appointment".to_string()],
                        disciplinary_functions: vec!["Judicial discipline".to_string(), "Prosecutorial discipline".to_string()],
                        evaluation_system: "Merit-based evaluation and ratification".to_string(),
                        transparency_measures: vec!["Public selection processes".to_string(), "Transparency requirements".to_string()],
                    },
                    judicial_administration: JudicialAdministration {
                        administrative_council: "Executive Council of the Judicial Power".to_string(),
                        budget_management: "Autonomous budget management".to_string(),
                        infrastructure_development: "Court modernization programs".to_string(),
                        technology_modernization: vec!["Electronic filing systems".to_string(), "Virtual hearings".to_string()],
                        performance_monitoring: "Performance indicators and dashboards".to_string(),
                    },
                },
                autonomous_institutions: vec![
                    AutonomousInstitution {
                        institution_name: "Central Reserve Bank of Peru".to_string(),
                        constitutional_basis: "Article 84 Constitution".to_string(),
                        autonomy_level: "Full autonomy in monetary policy".to_string(),
                        functions: vec!["Monetary policy".to_string(), "Currency issuance".to_string(), "Financial system oversight".to_string()],
                        governance_structure: "Board of Directors with seven members".to_string(),
                        accountability_mechanisms: vec!["Congressional reporting".to_string(), "Transparency requirements".to_string()],
                        budgetary_autonomy: "Own budget and resources".to_string(),
                    }
                ],
                electoral_system: ElectoralSystem {
                    electoral_institutions: vec![
                        ElectoralInstitution {
                            institution_name: "National Jury of Elections (JNE)".to_string(),
                            functions: vec!["Election administration".to_string(), "Electoral dispute resolution".to_string()],
                            composition: "Five members appointed by different institutions".to_string(),
                            independence_level: "Constitutional independence".to_string(),
                            oversight_role: "Electoral process oversight".to_string(),
                        }
                    ],
                    election_types: vec![
                        ElectionType {
                            election_name: "Presidential Elections".to_string(),
                            frequency: "Every 5 years".to_string(),
                            electoral_system: "Two-round majority system".to_string(),
                            eligibility_requirements: vec!["Peruvian by birth".to_string(), "Over 35 years old".to_string()],
                            campaign_regulations: vec!["Campaign finance limits".to_string(), "Media access rules".to_string()],
                        }
                    ],
                    electoral_procedures: ElectoralProcedures {
                        voter_registration: "Automatic registration based on civil registry".to_string(),
                        candidate_registration: "Registration with electoral authorities".to_string(),
                        campaign_period: "Regulated campaign periods".to_string(),
                        voting_procedures: "Secret ballot with biometric verification".to_string(),
                        vote_counting: "Electronic counting with paper backup".to_string(),
                        dispute_resolution: "Electoral tribunals and appeals".to_string(),
                    },
                    political_parties: vec![
                        PoliticalParty {
                            party_name: "Peru Libre".to_string(),
                            registration_status: "Registered".to_string(),
                            ideological_orientation: "Left-wing".to_string(),
                            leadership: "Vladimir Cerrón".to_string(),
                            electoral_performance: vec!["2021 presidential victory".to_string()],
                            internal_democracy: "Internal elections required".to_string(),
                        }
                    ],
                    campaign_finance: CampaignFinance {
                        funding_sources: vec!["Private donations".to_string(), "Public funding".to_string()],
                        spending_limits: "Legal limits on campaign expenditure".to_string(),
                        disclosure_requirements: vec!["Donor disclosure".to_string(), "Expense reporting".to_string()],
                        oversight_mechanisms: "Electoral authority oversight".to_string(),
                        sanctions: vec!["Fines".to_string(), "Candidate disqualification".to_string()],
                    },
                },
                decentralized_governments: vec![
                    DecentralizedGovernment {
                        government_type: "Regional Government".to_string(),
                        legal_framework: "Regional Governments Law - Law No. 27867".to_string(),
                        autonomy_level: "Political, economic and administrative autonomy".to_string(),
                        competences: vec!["Regional development".to_string(), "Natural resource management".to_string()],
                        governance_structure: "Regional Governor and Regional Council".to_string(),
                        fiscal_arrangements: "Canon and transfers from central government".to_string(),
                        coordination_mechanisms: vec!["Intergovernmental coordination".to_string(), "Regional coordination councils".to_string()],
                    }
                ],
            },
            constitutional_tribunal: ConstitutionalTribunal {
                composition: "Seven members appointed for 5-year terms".to_string(),
                appointment_process: "Congressional appointment with two-thirds majority".to_string(),
                term_duration: "5 years, non-renewable".to_string(),
                jurisdictional_competences: vec![
                    "Constitutionality control".to_string(),
                    "Competence conflicts".to_string(),
                    "Constitutional protection actions".to_string()
                ],
                procedural_rules: "Constitutional Procedural Code".to_string(),
                constitutional_doctrine: vec![
                    ConstitutionalDoctrine {
                        doctrine_name: "Principle of Human Dignity".to_string(),
                        legal_foundation: "Article 1 Constitution".to_string(),
                        precedential_cases: vec!["STC 1417-2005-AA".to_string(), "STC 2273-2005-PHC".to_string()],
                        application_scope: "All constitutional interpretation".to_string(),
                        evolution: vec!["From basic principle to interpretive criterion".to_string()],
                    }
                ],
            },
            decentralization_framework: DecentralizationFramework {
                regional_governments: vec![
                    RegionalGovernment {
                        region_name: "Lima".to_string(),
                        capital_city: "Huacho".to_string(),
                        governor: "Mahino Cueto Aservi".to_string(),
                        regional_council: "25 regional councilors".to_string(),
                        population: 848452,
                        area_km2: 32130,
                        economic_profile: "Agriculture, industry, mining, tourism".to_string(),
                        development_priorities: vec!["Infrastructure".to_string(), "Economic development".to_string()],
                        regional_competences: vec!["Regional planning".to_string(), "Natural resources".to_string()],
                        coordination_council: "Regional Coordination Council".to_string(),
                    }
                ],
                local_governments: vec![
                    LocalGovernment {
                        government_type: "Provincial Municipality".to_string(),
                        government_name: "Metropolitan Municipality of Lima".to_string(),
                        mayor: "Rafael López Aliaga".to_string(),
                        municipal_council: "39 municipal councilors".to_string(),
                        population: 9674755,
                        municipal_competences: vec!["Urban planning".to_string(), "Public services".to_string()],
                        service_delivery: vec!["Water and sanitation".to_string(), "Public transportation".to_string()],
                        participatory_budget: "Annual participatory budget process".to_string(),
                    }
                ],
                competence_distribution: CompetenceDistribution {
                    national_competences: vec!["Foreign policy".to_string(), "Defense".to_string(), "Justice".to_string()],
                    regional_competences: vec!["Regional development".to_string(), "Environmental management".to_string()],
                    local_competences: vec!["Local services".to_string(), "Urban planning".to_string()],
                    shared_competences: vec!["Education".to_string(), "Health".to_string()],
                    conflict_resolution: "Constitutional Tribunal competence conflicts jurisdiction".to_string(),
                },
                fiscal_decentralization: FiscalDecentralization {
                    revenue_distribution: "Canon and transfer mechanisms".to_string(),
                    transfer_mechanisms: vec!["Regular transfers".to_string(), "Investment transfers".to_string()],
                    tax_assignment: "Limited local tax authority".to_string(),
                    borrowing_rules: "Fiscal responsibility and transparency law".to_string(),
                    fiscal_responsibility: "Debt limits and fiscal rules".to_string(),
                },
                coordination_mechanisms: vec!["Intergovernmental coordination".to_string(), "Sectoral councils".to_string()],
            },
            constitutional_reform: ConstitutionalReform {
                amendment_procedures: vec!["Congressional initiative".to_string(), "Presidential initiative".to_string()],
                referendum_requirements: "Referendum required for certain reforms".to_string(),
                constituent_assembly: "Constituent Assembly as alternative mechanism".to_string(),
                reform_initiatives: vec!["Political reform proposals".to_string(), "Judicial reform".to_string()],
                constitutional_convention: "Possible constitutional convention".to_string(),
            },
        };

        PeruvianLegalSystem {
            constitution,
            legal_codes: vec![
                PeruvianLegalCode {
                    code_name: "Civil Code".to_string(),
                    legal_basis: "Legislative Decree No. 295".to_string(),
                    promulgation_date: "1984-07-25".to_string(),
                    structure: CodeStructure {
                        books: vec![
                            CodeBook {
                                book_number: 1,
                                book_title: "Law of Persons".to_string(),
                                sections: vec![
                                    CodeSection {
                                        section_number: 1,
                                        section_title: "Natural Persons".to_string(),
                                        articles: vec![
                                            CodeArticle {
                                                article_number: "1".to_string(),
                                                spanish_text: "La persona humana es sujeto de derecho desde su nacimiento. La vida humana comienza con la concepción.".to_string(),
                                                english_translation: "The human person is a subject of law from birth. Human life begins with conception.".to_string(),
                                                jurisprudential_interpretation: vec!["Constitutional Tribunal precedent on beginning of life".to_string()],
                                                doctrinal_commentary: vec!["Debate on legal personality and conception".to_string()],
                                                practical_application: "Foundation for family law and inheritance".to_string(),
                                            }
                                        ],
                                        key_concepts: vec!["Legal personality".to_string(), "Beginning of life".to_string()],
                                    }
                                ],
                                scope: "Regulation of natural and juridical persons".to_string(),
                            }
                        ],
                        total_articles: 2122,
                        preliminary_title: "General principles of law".to_string(),
                        final_provisions: "Derogatory and transitional provisions".to_string(),
                    },
                    fundamental_principles: vec!["Legal personality".to_string(), "Contractual freedom".to_string()],
                    international_influence: "French Civil Code influence with modern adaptations".to_string(),
                    recent_reforms: vec![
                        CodeReform {
                            reform_law: "Law No. 30994".to_string(),
                            reform_date: "2019-09-03".to_string(),
                            modified_articles: vec!["Articles on domestic violence".to_string()],
                            reform_rationale: "Gender violence prevention".to_string(),
                            implementation_impact: "Enhanced protection for women and children".to_string(),
                        }
                    ],
                }
            ],
            special_legislation: vec![
                SpecialLaw {
                    law_number: "29785".to_string(),
                    law_title: "Law on the Right to Prior Consultation of Indigenous or Original Peoples".to_string(),
                    promulgation_date: "2011-09-07".to_string(),
                    subject_matter: "Indigenous consultation rights".to_string(),
                    spanish_text: "La presente Ley desarrolla el contenido, los principios y el procedimiento del derecho a la consulta previa a los pueblos indígenas u originarios reconocido en el Convenio 169 de la Organización Internacional del Trabajo (OIT).".to_string(),
                    english_summary: "This Law develops the content, principles and procedure of the right to prior consultation to indigenous or original peoples recognized in ILO Convention 169.".to_string(),
                    implementing_regulations: vec!["Supreme Decree No. 001-2012-MC".to_string()],
                    enforcement_agencies: vec!["Ministry of Culture".to_string(), "Vice Ministry of Interculturality".to_string()],
                    compliance_requirements: vec!["Prior consultation process".to_string(), "Good faith negotiation".to_string()],
                }
            ],
            international_law: InternationalLaw {
                constitutional_hierarchy: "Constitutional rank for human rights treaties".to_string(),
                treaty_ratification: "Congressional approval required".to_string(),
                international_agreements: vec![
                    InternationalAgreement {
                        agreement_name: "American Convention on Human Rights".to_string(),
                        agreement_type: "Human rights treaty".to_string(),
                        ratification_date: "1978-07-28".to_string(),
                        constitutional_status: "Constitutional hierarchy".to_string(),
                        spanish_text: "Los Estados Partes en esta Convención se comprometen a respetar los derechos y libertades reconocidos en ella y a garantizar su libre y pleno ejercicio.".to_string(),
                        english_text: "The States Parties to this Convention undertake to respect the rights and freedoms recognized herein and to ensure their free and full exercise.".to_string(),
                        implementation_law: "Direct application".to_string(),
                        compliance_monitoring: "Inter-American Commission and Court of Human Rights".to_string(),
                    }
                ],
                supranational_integration: vec!["Andean Community".to_string(), "Pacific Alliance".to_string()],
                international_jurisdiction: vec!["Inter-American Court of Human Rights".to_string(), "International Court of Justice".to_string()],
            },
            indigenous_law: IndigenousLaw {
                constitutional_recognition: "Article 2(19) - Cultural and ethnic identity".to_string(),
                indigenous_languages: vec!["Quechua".to_string(), "Aymara".to_string(), "47 Amazonian languages".to_string()],
                consultation_law: "Law No. 29785 - Prior Consultation Law".to_string(),
                territorial_rights: "Constitutional recognition of communal property".to_string(),
                cultural_protection: vec!["Cultural heritage protection".to_string(), "Traditional knowledge".to_string()],
                traditional_justice: "Recognition of indigenous justice systems".to_string(),
                intercultural_dialogue: vec!["Intercultural education".to_string(), "Cultural mediation".to_string()],
            },
            regulatory_framework: RegulatoryFramework {
                regulatory_hierarchy: vec!["Constitution".to_string(), "Laws".to_string(), "Supreme Decrees".to_string(), "Ministerial Resolutions".to_string()],
                regulatory_agencies: vec![],
                administrative_procedures: "General Administrative Procedure Law - Law No. 27444".to_string(),
                regulatory_impact_assessment: "Regulatory Quality System".to_string(),
                citizen_participation: vec!["Public consultations".to_string(), "Regulatory comments".to_string()],
                regulatory_reform: vec!["Administrative simplification".to_string(), "Regulatory improvement".to_string()],
            },
            legal_profession: LegalProfession {
                bar_associations: vec!["Lima Bar Association".to_string(), "Regional bar associations".to_string()],
                professional_requirements: "Law degree and bar examination".to_string(),
                disciplinary_system: "Bar association disciplinary procedures".to_string(),
                legal_specializations: vec!["Civil law".to_string(), "Criminal law".to_string(), "Constitutional law".to_string()],
                professional_development: "Continuing legal education requirements".to_string(),
                ethics_code: "Professional ethics code and deontology".to_string(),
            },
            legal_education: LegalEducation {
                law_schools: vec![
                    LawSchool {
                        institution_name: "Pontifical Catholic University of Peru".to_string(),
                        accreditation_status: "Fully accredited".to_string(),
                        program_duration: "6 years".to_string(),
                        specialization_areas: vec!["Constitutional law".to_string(), "International law".to_string()],
                        clinical_offerings: vec!["Legal clinic".to_string(), "Human rights clinic".to_string()],
                        graduate_employment: "High employment rate".to_string(),
                    }
                ],
                curriculum_standards: "National accreditation standards".to_string(),
                accreditation_system: "SUNEDU accreditation system".to_string(),
                clinical_programs: vec!["Legal aid clinics".to_string(), "Public interest law".to_string()],
                postgraduate_education: vec!["Master's programs".to_string(), "Doctoral programs".to_string()],
                continuing_education: "Mandatory continuing education for lawyers".to_string(),
            },
        }
    }

    pub fn get_regional_government(&self, name: &str) -> Option<&RegionalGovernment> {
        self.constitution.decentralization_framework.regional_governments
            .iter().find(|region| region.region_name == name)
    }

    pub fn search_special_laws(&self, query: &str) -> Vec<&SpecialLaw> {
        self.special_legislation.iter()
            .filter(|law| law.law_title.to_lowercase().contains(&query.to_lowercase()) ||
                          law.subject_matter.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn get_constitutional_principle(&self, principle_name: &str) -> Option<&ConstitutionalPrinciple> {
        self.constitution.constitutional_principles.iter()
            .find(|principle| principle.principle_name == principle_name)
    }

    pub fn analyze_decentralization(&self) -> String {
        format!(
            "Peru operates a unitary state with administrative decentralization. \
            The system includes {} regional governments and extensive local government autonomy. \
            Decentralization balances national unity with regional autonomy through \
            constitutionally defined competence distribution.",
            self.constitution.decentralization_framework.regional_governments.len()
        )
    }

    pub fn get_international_agreements(&self) -> &Vec<InternationalAgreement> {
        &self.international_law.international_agreements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peruvian_system_creation() {
        let system = PeruvianLegalSystem::new();
        assert_eq!(system.constitution.constitution_name, "Political Constitution of Peru");
        assert!(!system.legal_codes.is_empty());
    }

    #[test]
    fn test_regional_government_lookup() {
        let system = PeruvianLegalSystem::new();
        let lima = system.get_regional_government("Lima");
        assert!(lima.is_some());
        assert_eq!(lima.unwrap().capital_city, "Huacho");
    }

    #[test]
    fn test_constitutional_principle_search() {
        let system = PeruvianLegalSystem::new();
        let dignity = system.get_constitutional_principle("Human Dignity");
        assert!(dignity.is_some());
        assert_eq!(dignity.unwrap().article_number, 1);
    }

    #[test]
    fn test_decentralization_analysis() {
        let system = PeruvianLegalSystem::new();
        let analysis = system.analyze_decentralization();
        assert!(analysis.contains("unitary state"));
        assert!(analysis.contains("regional governments"));
    }
}