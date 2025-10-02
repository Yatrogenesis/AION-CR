use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UruguayanConstitution {
    pub constitution_name: String,
    pub promulgation_date: String,
    pub last_amendment: String,
    pub constitutional_articles: Vec<ConstitutionalArticle>,
    pub institutional_framework: InstitutionalFramework,
    pub rights_guarantees: RightsAndGuarantees,
    pub territorial_organization: TerritorialOrganization,
    pub constitutional_reform: ConstitutionalReform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalArticle {
    pub article_number: i32,
    pub article_title: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub constitutional_significance: String,
    pub jurisprudential_development: Vec<String>,
    pub practical_implementation: Vec<String>,
    pub international_compliance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalFramework {
    pub executive_power: ExecutivePower,
    pub legislative_power: LegislativePower,
    pub judicial_power: JudicialPower,
    pub electoral_court: ElectoralCourt,
    pub court_accounts: CourtOfAccounts,
    pub administrative_litigation_tribunal: AdministrativeLitigationTribunal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutivePower {
    pub president: President,
    pub council_ministers: CouncilOfMinisters,
    pub ministries: Vec<Ministry>,
    pub decentralized_services: Vec<DecentralizedService>,
    pub autonomous_entities: Vec<AutonomousEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct President {
    pub current_president: String,
    pub election_system: String,
    pub term_duration: String,
    pub constitutional_powers: Vec<PresidentialPower>,
    pub restrictions: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidentialPower {
    pub power_type: String,
    pub constitutional_article: String,
    pub spanish_description: String,
    pub english_description: String,
    pub limitations: Vec<String>,
    pub oversight_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilOfMinisters {
    pub composition: String,
    pub decision_making: String,
    pub collective_responsibility: String,
    pub parliamentary_relations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ministry {
    pub ministry_name: String,
    pub minister_name: String,
    pub competence_areas: Vec<String>,
    pub organizational_structure: Vec<String>,
    pub budget_allocation: String,
    pub regulatory_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedService {
    pub service_name: String,
    pub service_type: String,
    pub functions: Vec<String>,
    pub autonomy_level: String,
    pub oversight_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousEntity {
    pub entity_name: String,
    pub constitutional_basis: String,
    pub functions: Vec<String>,
    pub independence_level: String,
    pub accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativePower {
    pub general_assembly: GeneralAssembly,
    pub chamber_representatives: ChamberOfRepresentatives,
    pub chamber_senators: ChamberOfSenators,
    pub legislative_process: LegislativeProcess,
    pub oversight_functions: OversightFunctions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralAssembly {
    pub composition: String,
    pub joint_sessions: Vec<String>,
    pub special_powers: Vec<String>,
    pub constitutional_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChamberOfRepresentatives {
    pub composition: String,
    pub election_system: String,
    pub term_duration: String,
    pub exclusive_powers: Vec<String>,
    pub current_composition: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChamberOfSenators {
    pub composition: String,
    pub election_system: String,
    pub term_duration: String,
    pub exclusive_powers: Vec<String>,
    pub vice_president_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub party_name: String,
    pub seats_representatives: u32,
    pub seats_senators: u32,
    pub political_orientation: String,
    pub leadership: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeProcess {
    pub bill_initiation: Vec<String>,
    pub committee_system: String,
    pub voting_procedures: HashMap<String, String>,
    pub bicameral_coordination: String,
    pub presidential_promulgation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversightFunctions {
    pub interpellation_power: String,
    pub investigative_committees: String,
    pub censure_procedures: String,
    pub budget_oversight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialPower {
    pub supreme_court_justice: SupremeCourtOfJustice,
    pub appellate_tribunals: Vec<AppellateTribunal>,
    pub first_instance_courts: Vec<FirstInstanceCourt>,
    pub specialized_jurisdictions: Vec<SpecializedJurisdiction>,
    pub judicial_organization: JudicialOrganization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourtOfJustice {
    pub composition: String,
    pub appointment_process: String,
    pub jurisdictional_functions: Vec<String>,
    pub administrative_functions: Vec<String>,
    pub disciplinary_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppellateTribunal {
    pub tribunal_name: String,
    pub territorial_jurisdiction: Vec<String>,
    pub specialized_chambers: Vec<String>,
    pub case_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstInstanceCourt {
    pub court_type: String,
    pub jurisdiction_area: String,
    pub competences: Vec<String>,
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedJurisdiction {
    pub jurisdiction_name: String,
    pub specialized_area: String,
    pub legal_framework: String,
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialOrganization {
    pub judicial_administration: String,
    pub disciplinary_system: String,
    pub judicial_career: String,
    pub budget_management: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralCourt {
    pub composition: String,
    pub appointment_process: String,
    pub electoral_functions: Vec<String>,
    pub dispute_resolution: String,
    pub oversight_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtOfAccounts {
    pub composition: String,
    pub constitutional_mandate: String,
    pub audit_functions: Vec<String>,
    pub reporting_obligations: Vec<String>,
    pub independence_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrativeLitigationTribunal {
    pub composition: String,
    pub jurisdiction: Vec<String>,
    pub procedures: Vec<String>,
    pub appeal_mechanisms: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsAndGuarantees {
    pub fundamental_rights: Vec<FundamentalRight>,
    pub social_rights: Vec<SocialRight>,
    pub economic_rights: Vec<EconomicRight>,
    pub political_rights: Vec<PoliticalRight>,
    pub guarantees: Vec<ConstitutionalGuarantee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRight {
    pub right_name: String,
    pub constitutional_article: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub protection_mechanisms: Vec<String>,
    pub limitations: Vec<String>,
    pub international_standards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRight {
    pub right_name: String,
    pub constitutional_basis: String,
    pub implementation_laws: Vec<String>,
    pub state_obligations: Vec<String>,
    pub progressivity_principle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicRight {
    pub right_name: String,
    pub constitutional_framework: String,
    pub regulatory_development: Vec<String>,
    pub market_regulations: Vec<String>,
    pub social_function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalRight {
    pub right_name: String,
    pub electoral_framework: String,
    pub participation_mechanisms: Vec<String>,
    pub democratic_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalGuarantee {
    pub guarantee_type: String,
    pub procedural_framework: String,
    pub judicial_protection: String,
    pub effectiveness_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritorialOrganization {
    pub departments: Vec<Department>,
    pub municipalities: Vec<Municipality>,
    pub decentralization_framework: DecentralizationFramework,
    pub regional_development: RegionalDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub department_name: String,
    pub capital_city: String,
    pub population: u64,
    pub area_km2: u64,
    pub economic_profile: String,
    pub departmental_government: DepartmentalGovernment,
    pub municipalities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentalGovernment {
    pub intendant: String,
    pub departmental_board: String,
    pub election_system: String,
    pub competences: Vec<String>,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Municipality {
    pub municipality_name: String,
    pub department: String,
    pub population: u64,
    pub municipal_government: MunicipalGovernment,
    pub local_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunicipalGovernment {
    pub mayor: String,
    pub municipal_council: String,
    pub election_system: String,
    pub local_competences: Vec<String>,
    pub citizen_participation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizationFramework {
    pub legal_basis: String,
    pub competence_distribution: String,
    pub fiscal_arrangements: String,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalDevelopment {
    pub development_policies: Vec<String>,
    pub territorial_planning: String,
    pub investment_coordination: String,
    pub inter_departmental_cooperation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalReform {
    pub amendment_procedures: Vec<String>,
    pub constitutional_convention: String,
    pub referendum_requirements: String,
    pub reform_initiatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UruguayanLegalCode {
    pub code_name: String,
    pub promulgation_date: String,
    pub structure: CodeStructure,
    pub fundamental_principles: Vec<String>,
    pub recent_reforms: Vec<CodeReform>,
    pub international_influence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStructure {
    pub books: Vec<CodeBook>,
    pub total_articles: u32,
    pub preliminary_dispositions: String,
    pub final_dispositions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBook {
    pub book_number: i32,
    pub book_title: String,
    pub titles: Vec<CodeTitle>,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTitle {
    pub title_number: i32,
    pub title_name: String,
    pub chapters: Vec<CodeChapter>,
    pub key_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChapter {
    pub chapter_number: i32,
    pub chapter_title: String,
    pub articles: Vec<CodeArticle>,
    pub regulatory_scope: String,
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
    pub modified_provisions: Vec<String>,
    pub reform_objectives: Vec<String>,
    pub implementation_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UruguayanLegalSystem {
    pub constitution: UruguayanConstitution,
    pub legal_codes: Vec<UruguayanLegalCode>,
    pub special_legislation: Vec<SpecialLaw>,
    pub international_law: InternationalLaw,
    pub regulatory_framework: RegulatoryFramework,
    pub democratic_institutions: DemocraticInstitutions,
    pub social_protection: SocialProtectionSystem,
    pub economic_framework: EconomicFramework,
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
    pub compliance_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalLaw {
    pub constitutional_integration: String,
    pub human_rights_treaties: Vec<HumanRightsTreaty>,
    pub commercial_agreements: Vec<CommercialAgreement>,
    pub regional_integration: Vec<String>,
    pub international_jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanRightsTreaty {
    pub treaty_name: String,
    pub ratification_date: String,
    pub constitutional_rank: String,
    pub spanish_text: String,
    pub english_text: String,
    pub implementation_mechanisms: Vec<String>,
    pub monitoring_bodies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialAgreement {
    pub agreement_name: String,
    pub agreement_type: String,
    pub parties: Vec<String>,
    pub economic_impact: String,
    pub implementation_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub administrative_law: String,
    pub regulatory_agencies: Vec<RegulatoryAgency>,
    pub public_procurement: String,
    pub environmental_regulation: String,
    pub financial_regulation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryAgency {
    pub agency_name: String,
    pub regulatory_sector: String,
    pub legal_framework: String,
    pub independence_level: String,
    pub regulatory_powers: Vec<String>,
    pub oversight_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticInstitutions {
    pub electoral_system: ElectoralSystem,
    pub political_parties: Vec<PoliticalPartyStructure>,
    pub civil_society: CivilSociety,
    pub transparency_mechanisms: TransparencyMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystem {
    pub electoral_laws: Vec<String>,
    pub election_types: Vec<String>,
    pub electoral_procedures: String,
    pub campaign_regulations: String,
    pub electoral_justice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalPartyStructure {
    pub party_name: String,
    pub legal_status: String,
    pub internal_democracy: String,
    pub financing_rules: String,
    pub accountability_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilSociety {
    pub association_freedom: String,
    pub ngo_framework: String,
    pub participation_mechanisms: Vec<String>,
    pub advocacy_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyMechanisms {
    pub access_information: String,
    pub government_transparency: Vec<String>,
    pub accountability_institutions: Vec<String>,
    pub citizen_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProtectionSystem {
    pub social_security: SocialSecurity,
    pub healthcare_system: HealthcareSystem,
    pub education_system: EducationSystem,
    pub labor_rights: LaborRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSecurity {
    pub pension_system: String,
    pub unemployment_insurance: String,
    pub family_allowances: String,
    pub disability_benefits: String,
    pub legal_framework: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareSystem {
    pub public_health_system: String,
    pub universal_coverage: String,
    pub healthcare_rights: Vec<String>,
    pub regulatory_framework: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationSystem {
    pub educational_framework: String,
    pub compulsory_education: String,
    pub higher_education: String,
    pub educational_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborRights {
    pub labor_legislation: Vec<String>,
    pub collective_bargaining: String,
    pub union_rights: Vec<String>,
    pub workplace_protections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicFramework {
    pub economic_constitution: String,
    pub monetary_system: String,
    pub fiscal_framework: String,
    pub trade_regulation: String,
    pub competition_law: String,
}

impl UruguayanLegalSystem {
    pub fn new() -> Self {
        let constitution = UruguayanConstitution {
            constitution_name: "Constitution of the Oriental Republic of Uruguay".to_string(),
            promulgation_date: "1967-02-15".to_string(),
            last_amendment: "2004-11-07".to_string(),
            constitutional_articles: vec![
                ConstitutionalArticle {
                    article_number: 1,
                    article_title: "National Sovereignty".to_string(),
                    spanish_text: "La República Oriental del Uruguay es la asociación política de todos los habitantes comprendidos dentro de su territorio.".to_string(),
                    english_translation: "The Oriental Republic of Uruguay is the political association of all inhabitants within its territory.".to_string(),
                    constitutional_significance: "Defines the foundational nature of the Uruguayan state".to_string(),
                    jurisprudential_development: vec!["Supreme Court precedents on territorial sovereignty".to_string()],
                    practical_implementation: vec!["Basis for citizenship and territorial jurisdiction".to_string()],
                    international_compliance: "Consistent with international law principles".to_string(),
                },
                ConstitutionalArticle {
                    article_number: 7,
                    article_title: "Fundamental Rights".to_string(),
                    spanish_text: "Los habitantes de la República tienen derecho a ser protegidos en el goce de su vida, honor, libertad, seguridad, trabajo y propiedad. Nadie puede ser privado de estos derechos sino conforme a las leyes que se establecen por razones de interés general.".to_string(),
                    english_translation: "The inhabitants of the Republic have the right to be protected in the enjoyment of their life, honor, liberty, security, work and property. No one may be deprived of these rights except in accordance with laws established for reasons of general interest.".to_string(),
                    constitutional_significance: "Establishes fundamental rights protection".to_string(),
                    jurisprudential_development: vec!["Constitutional interpretation of fundamental rights scope".to_string()],
                    practical_implementation: vec!["Legal framework for rights protection".to_string(), "Judicial guarantees".to_string()],
                    international_compliance: "Aligned with international human rights standards".to_string(),
                },
                ConstitutionalArticle {
                    article_number: 82,
                    article_title: "Popular Sovereignty".to_string(),
                    spanish_text: "La Nación adopta para su gobierno la forma democrática republicana. Su soberanía será ejercida directamente por el Cuerpo Electoral en los casos de elección, iniciativa y referéndum, e indirectamente por los Poderes representativos, conforme a las reglas establecidas en esta Constitución; todo conforme al principio de que todo el poder público emana del pueblo.".to_string(),
                    english_translation: "The Nation adopts the democratic republican form for its government. Its sovereignty shall be exercised directly by the Electoral Body in cases of election, initiative and referendum, and indirectly by the representative Powers, according to the rules established in this Constitution; all in accordance with the principle that all public power emanates from the people.".to_string(),
                    constitutional_significance: "Establishes democratic governance and popular sovereignty".to_string(),
                    jurisprudential_development: vec!["Electoral Court interpretations on direct democracy".to_string()],
                    practical_implementation: vec!["Electoral system".to_string(), "Referendum procedures".to_string(), "Legislative representation".to_string()],
                    international_compliance: "Democratic governance standards".to_string(),
                }
            ],
            institutional_framework: InstitutionalFramework {
                executive_power: ExecutivePower {
                    president: President {
                        current_president: "Luis Alberto Lacalle Pou".to_string(),
                        election_system: "Direct election with absolute majority or runoff".to_string(),
                        term_duration: "5 years, immediate re-election prohibited".to_string(),
                        constitutional_powers: vec![
                            PresidentialPower {
                                power_type: "Legislative Initiative".to_string(),
                                constitutional_article: "Article 168(7)".to_string(),
                                spanish_description: "Participar en la formación de las leyes con arreglo a la presente Constitución, promulgarlas y hacerlas ejecutar".to_string(),
                                english_description: "Participate in the formation of laws according to this Constitution, promulgate them and have them executed".to_string(),
                                limitations: vec!["Cannot initiate laws that increase public expenditure without funding source".to_string()],
                                oversight_mechanisms: vec!["Legislative review".to_string(), "Constitutional control".to_string()],
                            }
                        ],
                        restrictions: vec!["Cannot dissolve Parliament".to_string(), "Cannot extend term".to_string()],
                        accountability_mechanisms: vec!["Parliamentary oversight".to_string(), "Electoral accountability".to_string(), "Impeachment procedures".to_string()],
                    },
                    council_ministers: CouncilOfMinisters {
                        composition: "President and Ministers".to_string(),
                        decision_making: "Collective decision making".to_string(),
                        collective_responsibility: "Solidarity responsibility".to_string(),
                        parliamentary_relations: "Ministers accountable to Parliament".to_string(),
                    },
                    ministries: vec![
                        Ministry {
                            ministry_name: "Ministry of Interior".to_string(),
                            minister_name: "Luis Alberto Heber".to_string(),
                            competence_areas: vec!["Public security".to_string(), "Police".to_string(), "Immigration".to_string()],
                            organizational_structure: vec!["National Police".to_string(), "Fire Department".to_string()],
                            budget_allocation: "12.5 billion pesos (2024)".to_string(),
                            regulatory_powers: vec!["Security regulations".to_string(), "Immigration rules".to_string()],
                        }
                    ],
                    decentralized_services: vec![
                        DecentralizedService {
                            service_name: "National Administration of Public Education (ANEP)".to_string(),
                            service_type: "Autonomous Service".to_string(),
                            functions: vec!["Primary education".to_string(), "Secondary education".to_string(), "Technical education".to_string()],
                            autonomy_level: "Administrative and technical autonomy".to_string(),
                            oversight_mechanisms: vec!["Legislative oversight".to_string(), "Court of Accounts audit".to_string()],
                        }
                    ],
                    autonomous_entities: vec![
                        AutonomousEntity {
                            entity_name: "Central Bank of Uruguay".to_string(),
                            constitutional_basis: "Article 196".to_string(),
                            functions: vec!["Monetary policy".to_string(), "Financial system regulation".to_string()],
                            independence_level: "Operational independence".to_string(),
                            accountability: vec!["Parliamentary reporting".to_string(), "Transparency requirements".to_string()],
                        }
                    ],
                },
                legislative_power: LegislativePower {
                    general_assembly: GeneralAssembly {
                        composition: "Chamber of Representatives and Chamber of Senators".to_string(),
                        joint_sessions: vec!["Presidential election".to_string(), "Constitutional amendment".to_string()],
                        special_powers: vec!["Constitutional amendment".to_string(), "Impeachment procedures".to_string()],
                        constitutional_functions: vec!["Legislation".to_string(), "Budget approval".to_string(), "Oversight".to_string()],
                    },
                    chamber_representatives: ChamberOfRepresentatives {
                        composition: "99 representatives".to_string(),
                        election_system: "Proportional representation".to_string(),
                        term_duration: "5 years".to_string(),
                        exclusive_powers: vec!["Budget initiation".to_string(), "Impeachment accusation".to_string()],
                        current_composition: vec![
                            PoliticalParty {
                                party_name: "National Party".to_string(),
                                seats_representatives: 41,
                                seats_senators: 11,
                                political_orientation: "Center-right".to_string(),
                                leadership: "Luis Alberto Lacalle Pou".to_string(),
                            },
                            PoliticalParty {
                                party_name: "Broad Front".to_string(),
                                seats_representatives: 42,
                                seats_senators: 13,
                                political_orientation: "Center-left coalition".to_string(),
                                leadership: "Fernando Pereira".to_string(),
                            }
                        ],
                    },
                    chamber_senators: ChamberOfSenators {
                        composition: "30 senators plus Vice President".to_string(),
                        election_system: "National constituency proportional representation".to_string(),
                        term_duration: "5 years".to_string(),
                        exclusive_powers: vec!["Impeachment trial".to_string(), "International treaty approval".to_string()],
                        vice_president_role: "President of the Senate".to_string(),
                    },
                    legislative_process: LegislativeProcess {
                        bill_initiation: vec!["Government initiative".to_string(), "Parliamentary initiative".to_string(), "Popular initiative".to_string()],
                        committee_system: "Specialized permanent committees".to_string(),
                        voting_procedures: {
                            let mut map = HashMap::new();
                            map.insert("Regular laws".to_string(), "Simple majority".to_string());
                            map.insert("Constitutional laws".to_string(), "Absolute majority".to_string());
                            map.insert("Constitutional amendments".to_string(), "Two-thirds majority".to_string());
                            map
                        },
                        bicameral_coordination: "Both chambers must approve identical text".to_string(),
                        presidential_promulgation: "Presidential promulgation or veto".to_string(),
                    },
                    oversight_functions: OversightFunctions {
                        interpellation_power: "Ministers may be interpellated".to_string(),
                        investigative_committees: "Special investigative committees".to_string(),
                        censure_procedures: "Vote of no confidence procedures".to_string(),
                        budget_oversight: "Budget execution oversight".to_string(),
                    },
                },
                judicial_power: JudicialPower {
                    supreme_court_justice: SupremeCourtOfJustice {
                        composition: "5 justices".to_string(),
                        appointment_process: "General Assembly appointment with two-thirds majority".to_string(),
                        jurisdictional_functions: vec!["Constitutional review".to_string(), "Cassation appeals".to_string()],
                        administrative_functions: vec!["Judicial administration".to_string(), "Disciplinary oversight".to_string()],
                        disciplinary_powers: vec!["Judge discipline".to_string(), "Professional ethics enforcement".to_string()],
                    },
                    appellate_tribunals: vec![
                        AppellateTribunal {
                            tribunal_name: "Appellate Tribunal of Montevideo".to_string(),
                            territorial_jurisdiction: vec!["Montevideo".to_string(), "Metropolitan area".to_string()],
                            specialized_chambers: vec!["Civil".to_string(), "Criminal".to_string(), "Labor".to_string()],
                            case_types: vec!["Civil appeals".to_string(), "Criminal appeals".to_string()],
                        }
                    ],
                    first_instance_courts: vec![
                        FirstInstanceCourt {
                            court_type: "Civil Court".to_string(),
                            jurisdiction_area: "Montevideo".to_string(),
                            competences: vec!["Civil disputes".to_string(), "Commercial matters".to_string()],
                            procedures: vec!["Ordinary procedure".to_string(), "Summary procedure".to_string()],
                        }
                    ],
                    specialized_jurisdictions: vec![
                        SpecializedJurisdiction {
                            jurisdiction_name: "Labor Jurisdiction".to_string(),
                            specialized_area: "Labor and social security disputes".to_string(),
                            legal_framework: "Labor Procedure Code".to_string(),
                            procedures: vec!["Conciliation".to_string(), "Oral trial".to_string()],
                        }
                    ],
                    judicial_organization: JudicialOrganization {
                        judicial_administration: "Supreme Court administrative direction".to_string(),
                        disciplinary_system: "Disciplinary Tribunal".to_string(),
                        judicial_career: "Merit-based judicial career".to_string(),
                        budget_management: "Autonomous budget administration".to_string(),
                    },
                },
                electoral_court: ElectoralCourt {
                    composition: "5 members appointed by different institutions".to_string(),
                    appointment_process: "Mixed appointment system".to_string(),
                    electoral_functions: vec!["Electoral administration".to_string(), "Voter registration".to_string(), "Electoral justice".to_string()],
                    dispute_resolution: "Electoral dispute resolution".to_string(),
                    oversight_role: "Electoral process oversight".to_string(),
                },
                court_accounts: CourtOfAccounts {
                    composition: "7 members appointed by General Assembly".to_string(),
                    constitutional_mandate: "External control of public administration".to_string(),
                    audit_functions: vec!["Financial audit".to_string(), "Performance audit".to_string(), "Compliance audit".to_string()],
                    reporting_obligations: vec!["Annual report to Parliament".to_string(), "Public reporting".to_string()],
                    independence_guarantees: vec!["Constitutional autonomy".to_string(), "Budgetary independence".to_string()],
                },
                administrative_litigation_tribunal: AdministrativeLitigationTribunal {
                    composition: "5 members with legal expertise".to_string(),
                    jurisdiction: vec!["Administrative law disputes".to_string(), "Public contract disputes".to_string()],
                    procedures: vec!["Written procedure".to_string(), "Oral hearings".to_string()],
                    appeal_mechanisms: "Appeal to Supreme Court on points of law".to_string(),
                },
            },
            rights_guarantees: RightsAndGuarantees {
                fundamental_rights: vec![
                    FundamentalRight {
                        right_name: "Right to Life".to_string(),
                        constitutional_article: "Article 7".to_string(),
                        spanish_text: "Los habitantes de la República tienen derecho a ser protegidos en el goce de su vida".to_string(),
                        english_translation: "The inhabitants of the Republic have the right to be protected in the enjoyment of their life".to_string(),
                        protection_mechanisms: vec!["Constitutional protection".to_string(), "Criminal law protection".to_string()],
                        limitations: vec!["Legitimate defense".to_string(), "Law enforcement".to_string()],
                        international_standards: "American Convention on Human Rights".to_string(),
                    }
                ],
                social_rights: vec![
                    SocialRight {
                        right_name: "Right to Health".to_string(),
                        constitutional_basis: "Article 44".to_string(),
                        implementation_laws: vec!["National Health System Law".to_string()],
                        state_obligations: vec!["Universal health coverage".to_string(), "Health promotion".to_string()],
                        progressivity_principle: "Progressive realization of health rights".to_string(),
                    }
                ],
                economic_rights: vec![
                    EconomicRight {
                        right_name: "Right to Work".to_string(),
                        constitutional_framework: "Articles 53-57".to_string(),
                        regulatory_development: vec!["Labor Code".to_string(), "Social Security Law".to_string()],
                        market_regulations: vec!["Minimum wage".to_string(), "Working hours".to_string()],
                        social_function: "Work as social right and duty".to_string(),
                    }
                ],
                political_rights: vec![
                    PoliticalRight {
                        right_name: "Right to Vote".to_string(),
                        electoral_framework: "Electoral Law".to_string(),
                        participation_mechanisms: vec!["Elections".to_string(), "Referendums".to_string(), "Popular initiative".to_string()],
                        democratic_guarantees: vec!["Secret ballot".to_string(), "Electoral justice".to_string()],
                    }
                ],
                guarantees: vec![
                    ConstitutionalGuarantee {
                        guarantee_type: "Habeas Corpus".to_string(),
                        procedural_framework: "Constitutional and legal procedures".to_string(),
                        judicial_protection: "Immediate judicial review".to_string(),
                        effectiveness_mechanisms: vec!["Emergency procedures".to_string(), "Direct access to courts".to_string()],
                    }
                ],
            },
            territorial_organization: TerritorialOrganization {
                departments: vec![
                    Department {
                        department_name: "Montevideo".to_string(),
                        capital_city: "Montevideo".to_string(),
                        population: 1319108,
                        area_km2: 530,
                        economic_profile: "Services, government, industry, port".to_string(),
                        departmental_government: DepartmentalGovernment {
                            intendant: "Carolina Cosse".to_string(),
                            departmental_board: "31 members".to_string(),
                            election_system: "Direct election".to_string(),
                            competences: vec!["Urban planning".to_string(), "Local services".to_string(), "Municipal police".to_string()],
                            coordination_mechanisms: vec!["National-departmental coordination".to_string()],
                        },
                        municipalities: vec!["Montevideo has no municipalities - special regime".to_string()],
                    },
                    Department {
                        department_name: "Canelones".to_string(),
                        capital_city: "Canelones".to_string(),
                        population: 590600,
                        area_km2: 4536,
                        economic_profile: "Agriculture, industry, tourism, viticulture".to_string(),
                        departmental_government: DepartmentalGovernment {
                            intendant: "Yamandú Orsi".to_string(),
                            departmental_board: "31 members".to_string(),
                            election_system: "Direct election".to_string(),
                            competences: vec!["Territorial planning".to_string(), "Infrastructure".to_string(), "Environmental management".to_string()],
                            coordination_mechanisms: vec!["Inter-departmental agreements".to_string()],
                        },
                        municipalities: vec!["Las Piedras".to_string(), "Pando".to_string(), "Santa Lucía".to_string()],
                    },
                    Department {
                        department_name: "Maldonado".to_string(),
                        capital_city: "Maldonado".to_string(),
                        population: 164300,
                        area_km2: 4793,
                        economic_profile: "Tourism, agriculture, real estate".to_string(),
                        departmental_government: DepartmentalGovernment {
                            intendant: "Enrique Antía".to_string(),
                            departmental_board: "31 members".to_string(),
                            election_system: "Direct election".to_string(),
                            competences: vec!["Tourism development".to_string(), "Coastal management".to_string()],
                            coordination_mechanisms: vec!["Tourism coordination councils".to_string()],
                        },
                        municipalities: vec!["Punta del Este".to_string(), "San Carlos".to_string()],
                    }
                ],
                municipalities: vec![
                    Municipality {
                        municipality_name: "Las Piedras".to_string(),
                        department: "Canelones".to_string(),
                        population: 71258,
                        municipal_government: MunicipalGovernment {
                            mayor: "Gustavo Borsari".to_string(),
                            municipal_council: "5 members".to_string(),
                            election_system: "Direct election".to_string(),
                            local_competences: vec!["Local services".to_string(), "Municipal transit".to_string()],
                            citizen_participation: "Neighborhood councils".to_string(),
                        },
                        local_services: vec!["Waste collection".to_string(), "Street maintenance".to_string(), "Local policing".to_string()],
                    }
                ],
                decentralization_framework: DecentralizationFramework {
                    legal_basis: "Constitution and Decentralization Law".to_string(),
                    competence_distribution: "National, departmental, and municipal levels".to_string(),
                    fiscal_arrangements: "National transfers and local taxes".to_string(),
                    coordination_mechanisms: vec!["Congress of Intendants".to_string(), "Sectoral coordination".to_string()],
                },
                regional_development: RegionalDevelopment {
                    development_policies: vec!["Regional development plans".to_string(), "Productive transformation".to_string()],
                    territorial_planning: "National territorial planning framework".to_string(),
                    investment_coordination: "Public-private investment coordination".to_string(),
                    inter_departmental_cooperation: vec!["Regional agreements".to_string(), "Joint projects".to_string()],
                },
            },
            constitutional_reform: ConstitutionalReform {
                amendment_procedures: vec!["Parliamentary initiative".to_string(), "Popular initiative".to_string(), "Constitutional convention".to_string()],
                constitutional_convention: "Two-thirds parliamentary majority or popular initiative".to_string(),
                referendum_requirements: "Mandatory referendum for constitutional amendments".to_string(),
                reform_initiatives: vec!["Recent social security reform".to_string(), "Water as human right".to_string()],
            },
        };

        let legal_codes = vec![
            UruguayanLegalCode {
                code_name: "Civil Code".to_string(),
                promulgation_date: "1868-01-01".to_string(),
                structure: CodeStructure {
                    books: vec![
                        CodeBook {
                            book_number: 1,
                            book_title: "Of Persons".to_string(),
                            titles: vec![
                                CodeTitle {
                                    title_number: 1,
                                    title_name: "Of the enjoyment and exercise of civil rights".to_string(),
                                    chapters: vec![
                                        CodeChapter {
                                            chapter_number: 1,
                                            chapter_title: "Of natural persons".to_string(),
                                            articles: vec![
                                                CodeArticle {
                                                    article_number: "21".to_string(),
                                                    spanish_text: "Son personas todos los individuos de la especie humana, cualquiera que sea su edad, sexo, estirpe o condición".to_string(),
                                                    english_translation: "All individuals of the human species are persons, whatever their age, sex, lineage or condition".to_string(),
                                                    jurisprudential_interpretation: vec!["Supreme Court precedents on legal personality".to_string()],
                                                    doctrinal_commentary: vec!["Universal recognition of human personality".to_string()],
                                                    practical_application: "Foundation for all civil rights".to_string(),
                                                }
                                            ],
                                            regulatory_scope: "Legal personality and capacity".to_string(),
                                        }
                                    ],
                                    key_principles: vec!["Legal personality".to_string(), "Equality before law".to_string()],
                                }
                            ],
                            scope: "Regulation of persons and their legal capacity".to_string(),
                        }
                    ],
                    total_articles: 2393,
                    preliminary_dispositions: "General principles of law application".to_string(),
                    final_dispositions: "Transitional and derogatory provisions".to_string(),
                },
                fundamental_principles: vec!["Legal personality".to_string(), "Contractual freedom".to_string(), "Property rights".to_string()],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Law 19.580".to_string(),
                        reform_date: "2017-12-22".to_string(),
                        modified_provisions: vec!["Domestic violence provisions".to_string()],
                        reform_objectives: vec!["Gender violence prevention".to_string(), "Victim protection".to_string()],
                        implementation_impact: "Enhanced protection mechanisms for domestic violence victims".to_string(),
                    }
                ],
                international_influence: "French Civil Code influence with modern adaptations".to_string(),
            }
        ];

        UruguayanLegalSystem {
            constitution,
            legal_codes,
            special_legislation: vec![
                SpecialLaw {
                    law_number: "18.331".to_string(),
                    law_title: "Personal Data Protection Law".to_string(),
                    promulgation_date: "2008-08-11".to_string(),
                    subject_matter: "Personal data protection and privacy".to_string(),
                    spanish_text: "La presente ley tiene por objeto la protección de los datos personales registrados en cualquier soporte que los haga susceptibles de tratamiento, y toda operación o conjunto de operaciones sobre dichos datos".to_string(),
                    english_summary: "This law aims to protect personal data recorded in any medium that makes them susceptible to processing, and any operation or set of operations on such data".to_string(),
                    implementing_regulations: vec!["Regulatory Decree 414/009".to_string()],
                    compliance_requirements: vec!["Consent requirements".to_string(), "Data protection measures".to_string(), "Registry compliance".to_string()],
                }
            ],
            international_law: InternationalLaw {
                constitutional_integration: "Constitutional hierarchy for human rights treaties".to_string(),
                human_rights_treaties: vec![
                    HumanRightsTreaty {
                        treaty_name: "American Convention on Human Rights".to_string(),
                        ratification_date: "1985-04-19".to_string(),
                        constitutional_rank: "Constitutional hierarchy".to_string(),
                        spanish_text: "Los Estados Partes en esta Convención se comprometen a respetar los derechos y libertades reconocidos en ella".to_string(),
                        english_text: "The States Parties to this Convention undertake to respect the rights and freedoms recognized herein".to_string(),
                        implementation_mechanisms: vec!["Direct application".to_string(), "Constitutional review".to_string()],
                        monitoring_bodies: vec!["Inter-American Commission on Human Rights".to_string(), "Inter-American Court of Human Rights".to_string()],
                    }
                ],
                commercial_agreements: vec![
                    CommercialAgreement {
                        agreement_name: "MERCOSUR Treaty".to_string(),
                        agreement_type: "Regional integration".to_string(),
                        parties: vec!["Argentina".to_string(), "Brazil".to_string(), "Paraguay".to_string(), "Uruguay".to_string()],
                        economic_impact: "Free trade and customs union".to_string(),
                        implementation_law: "Law 16.196".to_string(),
                    }
                ],
                regional_integration: vec!["MERCOSUR".to_string(), "UNASUR".to_string(), "ALADI".to_string()],
                international_jurisdiction: vec!["International Court of Justice".to_string(), "Inter-American Court of Human Rights".to_string()],
            },
            regulatory_framework: RegulatoryFramework {
                administrative_law: "General framework for administrative procedures".to_string(),
                regulatory_agencies: vec![
                    RegulatoryAgency {
                        agency_name: "Communications Services Regulatory Unit (URSEC)".to_string(),
                        regulatory_sector: "Telecommunications".to_string(),
                        legal_framework: "Law 17.296".to_string(),
                        independence_level: "Technical and administrative autonomy".to_string(),
                        regulatory_powers: vec!["Tariff regulation".to_string(), "Quality standards".to_string(), "Competition oversight".to_string()],
                        oversight_mechanisms: vec!["Parliamentary oversight".to_string(), "Court of Accounts audit".to_string()],
                    }
                ],
                public_procurement: "Comprehensive public procurement legal framework".to_string(),
                environmental_regulation: "Environmental Protection Law and implementing regulations".to_string(),
                financial_regulation: "Central Bank and financial system regulation".to_string(),
            },
            democratic_institutions: DemocraticInstitutions {
                electoral_system: ElectoralSystem {
                    electoral_laws: vec!["Electoral Law".to_string(), "Political Parties Law".to_string()],
                    election_types: vec!["Presidential".to_string(), "Parliamentary".to_string(), "Departmental".to_string()],
                    electoral_procedures: "Proportional representation with preference voting".to_string(),
                    campaign_regulations: "Campaign finance and media access regulations".to_string(),
                    electoral_justice: "Electoral Court jurisdiction".to_string(),
                },
                political_parties: vec![
                    PoliticalPartyStructure {
                        party_name: "National Party".to_string(),
                        legal_status: "Legally recognized political party".to_string(),
                        internal_democracy: "Internal elections required".to_string(),
                        financing_rules: "Public and private financing regulations".to_string(),
                        accountability_requirements: vec!["Financial reporting".to_string(), "Electoral transparency".to_string()],
                    }
                ],
                civil_society: CivilSociety {
                    association_freedom: "Constitutional freedom of association".to_string(),
                    ngo_framework: "Civil associations legal framework".to_string(),
                    participation_mechanisms: vec!["Public consultations".to_string(), "Citizen initiatives".to_string()],
                    advocacy_rights: vec!["Petition rights".to_string(), "Public participation".to_string()],
                },
                transparency_mechanisms: TransparencyMechanisms {
                    access_information: "Public Information Access Law".to_string(),
                    government_transparency: vec!["Open government initiatives".to_string(), "Public data portals".to_string()],
                    accountability_institutions: vec!["Court of Accounts".to_string(), "Ombudsman".to_string()],
                    citizen_oversight: vec!["Civil society monitoring".to_string(), "Media oversight".to_string()],
                },
            },
            social_protection: SocialProtectionSystem {
                social_security: SocialSecurity {
                    pension_system: "Mixed public-private pension system".to_string(),
                    unemployment_insurance: "Unemployment insurance system".to_string(),
                    family_allowances: "Family allowance system".to_string(),
                    disability_benefits: "Disability and invalidity benefits".to_string(),
                    legal_framework: vec!["Social Security Law".to_string(), "Pension System Law".to_string()],
                },
                healthcare_system: HealthcareSystem {
                    public_health_system: "National Integrated Health System".to_string(),
                    universal_coverage: "Universal health coverage guarantee".to_string(),
                    healthcare_rights: vec!["Right to health".to_string(), "Access to medicines".to_string()],
                    regulatory_framework: vec!["Health System Law".to_string(), "Public Health Code".to_string()],
                },
                education_system: EducationSystem {
                    educational_framework: "National Education System".to_string(),
                    compulsory_education: "14 years of compulsory education".to_string(),
                    higher_education: "University autonomy and public higher education".to_string(),
                    educational_rights: vec!["Right to education".to_string(), "Educational equality".to_string()],
                },
                labor_rights: LaborRights {
                    labor_legislation: vec!["Labor Relations Law".to_string(), "Collective Bargaining Law".to_string()],
                    collective_bargaining: "Tripartite collective bargaining system".to_string(),
                    union_rights: vec!["Freedom of association".to_string(), "Right to strike".to_string()],
                    workplace_protections: vec!["Occupational safety".to_string(), "Non-discrimination".to_string()],
                },
            },
            economic_framework: EconomicFramework {
                economic_constitution: "Constitutional economic framework".to_string(),
                monetary_system: "Central Bank monetary system".to_string(),
                fiscal_framework: "Fiscal responsibility and transparency framework".to_string(),
                trade_regulation: "Foreign trade and investment regulation".to_string(),
                competition_law: "Competition and consumer protection law".to_string(),
            },
        }
    }

    pub fn get_department(&self, name: &str) -> Option<&Department> {
        self.constitution.territorial_organization.departments
            .iter().find(|dept| dept.department_name == name)
    }

    pub fn search_constitutional_articles(&self, query: &str) -> Vec<&ConstitutionalArticle> {
        self.constitution.constitutional_articles.iter()
            .filter(|article| article.article_title.to_lowercase().contains(&query.to_lowercase()) ||
                            article.spanish_text.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn get_political_party_composition(&self) -> &Vec<PoliticalParty> {
        &self.constitution.institutional_framework.legislative_power.chamber_representatives.current_composition
    }

    pub fn analyze_democratic_system(&self) -> String {
        format!(
            "Uruguay operates as a democratic republic with strong institutional framework. \
            The system features separation of powers, proportional representation, \
            and robust checks and balances. The country has {} departments with \
            significant decentralization and participatory democracy mechanisms including \
            referendums and popular initiatives.",
            self.constitution.territorial_organization.departments.len()
        )
    }

    pub fn get_human_rights_framework(&self) -> &RightsAndGuarantees {
        &self.constitution.rights_guarantees
    }

    pub fn get_international_obligations(&self) -> &Vec<HumanRightsTreaty> {
        &self.international_law.human_rights_treaties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uruguayan_system_creation() {
        let system = UruguayanLegalSystem::new();
        assert_eq!(system.constitution.constitution_name, "Constitution of the Oriental Republic of Uruguay");
        assert!(!system.legal_codes.is_empty());
    }

    #[test]
    fn test_department_lookup() {
        let system = UruguayanLegalSystem::new();
        let montevideo = system.get_department("Montevideo");
        assert!(montevideo.is_some());
        assert_eq!(montevideo.unwrap().capital_city, "Montevideo");
    }

    #[test]
    fn test_constitutional_article_search() {
        let system = UruguayanLegalSystem::new();
        let articles = system.search_constitutional_articles("sovereignty");
        assert!(!articles.is_empty());
    }

    #[test]
    fn test_democratic_analysis() {
        let system = UruguayanLegalSystem::new();
        let analysis = system.analyze_democratic_system();
        assert!(analysis.contains("democratic republic"));
        assert!(analysis.contains("proportional representation"));
    }
}