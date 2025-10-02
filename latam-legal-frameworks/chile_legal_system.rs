use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChileanConstitution {
    pub constitution_name: String,
    pub promulgation_date: String,
    pub constitutional_history: Vec<ConstitutionalPeriod>,
    pub chapters: Vec<ConstitutionalChapter>,
    pub fundamental_rights: Vec<FundamentalRight>,
    pub state_organization: StateOrganization,
    pub constitutional_tribunal: ConstitutionalTribunal,
    pub amendment_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalPeriod {
    pub period_name: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub key_characteristics: Vec<String>,
    pub political_context: String,
    pub major_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalChapter {
    pub chapter_number: i32,
    pub chapter_title: String,
    pub articles: Vec<ConstitutionalArticle>,
    pub constitutional_principles: Vec<String>,
    pub jurisprudence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalArticle {
    pub article_number: i32,
    pub article_title: String,
    pub article_text: String,
    pub constitutional_interpretation: Vec<String>,
    pub recent_reforms: Vec<String>,
    pub comparative_law: Vec<String>,
    pub practical_application: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRight {
    pub right_name: String,
    pub constitutional_basis: String,
    pub protection_mechanisms: Vec<String>,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
    pub international_standards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateOrganization {
    pub executive_power: ExecutivePower,
    pub legislative_power: LegislativePower,
    pub judicial_power: JudicialPower,
    pub autonomous_institutions: Vec<AutonomousInstitution>,
    pub territorial_organization: TerritorialOrganization,
    pub checks_balances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutivePower {
    pub president: President,
    pub government: Government,
    pub public_administration: PublicAdministration,
    pub emergency_powers: EmergencyPowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct President {
    pub current_president: String,
    pub election_system: String,
    pub term_duration: String,
    pub powers: Vec<String>,
    pub restrictions: Vec<String>,
    pub accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Government {
    pub cabinet_structure: String,
    pub ministers: Vec<Minister>,
    pub appointment_process: String,
    pub collective_responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minister {
    pub ministry_name: String,
    pub minister_name: String,
    pub portfolio_areas: Vec<String>,
    pub regulatory_powers: Vec<String>,
    pub budget_allocation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAdministration {
    pub civil_service: CivilService,
    pub decentralized_services: Vec<String>,
    pub regulatory_agencies: Vec<String>,
    pub transparency_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilService {
    pub career_system: String,
    pub merit_principles: Vec<String>,
    pub professional_development: String,
    pub ethics_framework: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyPowers {
    pub emergency_types: Vec<String>,
    pub declaration_procedures: String,
    pub limitations: Vec<String>,
    pub congressional_oversight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativePower {
    pub congress: Congress,
    pub legislative_process: LegislativeProcess,
    pub oversight_functions: Vec<String>,
    pub budget_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Congress {
    pub chamber_of_deputies: ChamberDeputies,
    pub senate: Senate,
    pub joint_sessions: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChamberDeputies {
    pub composition: String,
    pub election_system: String,
    pub term_duration: String,
    pub exclusive_powers: Vec<String>,
    pub current_composition: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Senate {
    pub composition: String,
    pub election_system: String,
    pub term_duration: String,
    pub exclusive_powers: Vec<String>,
    pub regional_representation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub party_name: String,
    pub seats_deputies: u32,
    pub seats_senate: u32,
    pub political_orientation: String,
    pub party_leader: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryGroup {
    pub group_name: String,
    pub member_parties: Vec<String>,
    pub political_position: String,
    pub parliamentary_strength: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeProcess {
    pub bill_initiation: Vec<String>,
    pub committee_system: String,
    pub voting_procedures: String,
    pub bicameral_coordination: String,
    pub presidential_veto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialPower {
    pub supreme_court: SupremeCourt,
    pub courts_appeal: Vec<CourtAppeal>,
    pub specialized_courts: Vec<SpecializedCourt>,
    pub prosecution_service: ProsecutionService,
    pub judicial_council: JudicialCouncil,
    pub judicial_career: JudicialCareer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourt {
    pub composition: String,
    pub president_court: String,
    pub jurisdictions: Vec<String>,
    pub administrative_functions: Vec<String>,
    pub disciplinary_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtAppeal {
    pub jurisdiction_name: String,
    pub territorial_scope: Vec<String>,
    pub specialized_chambers: Vec<String>,
    pub caseload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedCourt {
    pub court_type: String,
    pub specialized_area: String,
    pub jurisdiction: Vec<String>,
    pub special_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsecutionService {
    pub national_prosecutor: String,
    pub organizational_structure: String,
    pub investigation_powers: Vec<String>,
    pub independence_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialCouncil {
    pub composition: String,
    pub functions: Vec<String>,
    pub appointment_role: String,
    pub disciplinary_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialCareer {
    pub entry_requirements: Vec<String>,
    pub promotion_system: String,
    pub training_system: String,
    pub retirement_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousInstitution {
    pub institution_name: String,
    pub constitutional_basis: String,
    pub functions: Vec<String>,
    pub independence_level: String,
    pub accountability_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritorialOrganization {
    pub regions: Vec<Region>,
    pub provinces: Vec<Province>,
    pub communes: Vec<Commune>,
    pub decentralization_level: String,
    pub regional_development: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub region_name: String,
    pub region_capital: String,
    pub provinces_included: Vec<String>,
    pub population: u64,
    pub area_km2: u64,
    pub economic_profile: String,
    pub regional_government: RegionalGovernment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalGovernment {
    pub regional_governor: String,
    pub regional_council: String,
    pub competences: Vec<String>,
    pub budget_management: String,
    pub development_planning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub province_name: String,
    pub provincial_capital: String,
    pub communes_included: Vec<String>,
    pub provincial_governor: String,
    pub administrative_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commune {
    pub commune_name: String,
    pub mayor: String,
    pub municipal_council: String,
    pub local_services: Vec<String>,
    pub budget_size: String,
    pub population: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalTribunal {
    pub composition: String,
    pub appointment_process: String,
    pub jurisdictions: Vec<String>,
    pub constitutional_review: String,
    pub precedent_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChileanLegalCode {
    pub code_name: String,
    pub promulgation_date: String,
    pub last_major_reform: String,
    pub structure: CodeStructure,
    pub key_principles: Vec<String>,
    pub international_influence: String,
    pub modernization_efforts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStructure {
    pub books: Vec<CodeBook>,
    pub total_articles: u32,
    pub special_procedures: Vec<String>,
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
    pub articles: Vec<CodeArticle>,
    pub key_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeArticle {
    pub article_number: String,
    pub article_text: String,
    pub jurisprudence: Vec<String>,
    pub doctrinal_interpretation: Vec<String>,
    pub comparative_law: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChileanLegalSystem {
    pub constitution: ChileanConstitution,
    pub legal_codes: Vec<ChileanLegalCode>,
    pub special_laws: Vec<SpecialLaw>,
    pub regulatory_framework: RegulatoryFramework,
    pub international_law: InternationalLaw,
    pub legal_education: LegalEducation,
    pub legal_profession: LegalProfession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialLaw {
    pub law_number: String,
    pub law_title: String,
    pub promulgation_date: String,
    pub subject_matter: String,
    pub key_provisions: Vec<String>,
    pub implementing_regulations: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub supreme_decrees: Vec<SupremeDecree>,
    pub ministerial_resolutions: Vec<MinisterialResolution>,
    pub regulatory_agencies: Vec<RegulatoryAgency>,
    pub technical_standards: Vec<TechnicalStandard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeDecree {
    pub decree_number: String,
    pub ministry: String,
    pub subject_matter: String,
    pub legal_basis: String,
    pub regulatory_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinisterialResolution {
    pub resolution_number: String,
    pub ministry: String,
    pub administrative_matter: String,
    pub implementation_details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryAgency {
    pub agency_name: String,
    pub regulatory_sector: String,
    pub powers: Vec<String>,
    pub independence_level: String,
    pub accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalStandard {
    pub standard_number: String,
    pub standard_title: String,
    pub technical_area: String,
    pub compliance_level: String,
    pub certification_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalLaw {
    pub treaty_incorporation: String,
    pub international_agreements: Vec<InternationalAgreement>,
    pub supranational_integration: Vec<String>,
    pub dispute_resolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalAgreement {
    pub agreement_name: String,
    pub agreement_type: String,
    pub ratification_date: String,
    pub constitutional_rank: String,
    pub implementation_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEducation {
    pub law_schools: Vec<LawSchool>,
    pub curriculum_standards: String,
    pub bar_examination: String,
    pub continuing_education: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawSchool {
    pub institution_name: String,
    pub accreditation_status: String,
    pub program_duration: String,
    pub specializations: Vec<String>,
    pub graduate_outcomes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProfession {
    pub bar_association: String,
    pub professional_ethics: String,
    pub disciplinary_system: String,
    pub specialization_areas: Vec<String>,
    pub professional_development: String,
}

impl ChileanLegalSystem {
    pub fn new() -> Self {
        let constitution = ChileanConstitution {
            constitution_name: "Political Constitution of the Republic of Chile".to_string(),
            promulgation_date: "1980-10-21".to_string(),
            constitutional_history: vec![
                ConstitutionalPeriod {
                    period_name: "Military Government Constitution".to_string(),
                    start_date: "1980-10-21".to_string(),
                    end_date: Some("1990-03-11".to_string()),
                    key_characteristics: vec!["Authoritarian framework".to_string(), "Strong executive".to_string()],
                    political_context: "Military dictatorship under Augusto Pinochet".to_string(),
                    major_reforms: vec!["Initial constitutional framework".to_string()],
                },
                ConstitutionalPeriod {
                    period_name: "Democratic Transition".to_string(),
                    start_date: "1990-03-11".to_string(),
                    end_date: Some("2005-08-17".to_string()),
                    key_characteristics: vec!["Gradual democratization".to_string(), "Constitutional constraints".to_string()],
                    political_context: "Return to democracy with constitutional limitations".to_string(),
                    major_reforms: vec!["1989 constitutional reforms".to_string(), "Removal of designated senators".to_string()],
                },
                ConstitutionalPeriod {
                    period_name: "Constitutional Modernization".to_string(),
                    start_date: "2005-08-17".to_string(),
                    end_date: None,
                    key_characteristics: vec!["Democratic consolidation".to_string(), "Human rights emphasis".to_string()],
                    political_context: "Full democratic governance".to_string(),
                    major_reforms: vec!["2005 major constitutional reform".to_string(), "Presidential term limits".to_string()],
                }
            ],
            chapters: vec![
                ConstitutionalChapter {
                    chapter_number: 1,
                    chapter_title: "Fundamental Bases of Institutionality".to_string(),
                    articles: vec![
                        ConstitutionalArticle {
                            article_number: 1,
                            article_title: "Human Dignity and Rights".to_string(),
                            article_text: "Human beings are born free and equal in dignity and rights. The family is the fundamental nucleus of society.".to_string(),
                            constitutional_interpretation: vec!["Constitutional Court emphasis on human dignity".to_string()],
                            recent_reforms: vec!["2005 strengthening of human rights".to_string()],
                            comparative_law: vec!["Similar to German Basic Law Article 1".to_string()],
                            practical_application: "Foundation for all human rights jurisprudence".to_string(),
                        },
                        ConstitutionalArticle {
                            article_number: 3,
                            article_title: "Unitary State".to_string(),
                            article_text: "The Chilean State is unitary. Its territory is divided into regions. Its administration shall be functional and territorially decentralized and deconcentrated as appropriate.".to_string(),
                            constitutional_interpretation: vec!["Balance between unity and decentralization".to_string()],
                            recent_reforms: vec!["Regional government strengthening".to_string()],
                            comparative_law: vec!["Different from federal systems".to_string()],
                            practical_application: "Framework for territorial organization".to_string(),
                        },
                        ConstitutionalArticle {
                            article_number: 4,
                            article_title: "Chilean Sovereignty".to_string(),
                            article_text: "Chile is a republic. The sovereignty resides essentially in the Nation. Its exercise is carried out by the people through plebiscites and periodic elections and, also, by the authorities that this Constitution establishes.".to_string(),
                            constitutional_interpretation: vec!["Popular sovereignty principle".to_string()],
                            recent_reforms: vec!["Clarification of sovereignty exercise".to_string()],
                            comparative_law: vec!["Similar to French constitutional tradition".to_string()],
                            practical_application: "Basis for democratic legitimacy".to_string(),
                        }
                    ],
                    constitutional_principles: vec![
                        "Human dignity".to_string(),
                        "Unitary state".to_string(),
                        "Popular sovereignty".to_string(),
                        "Republican government".to_string()
                    ],
                    jurisprudence: vec!["Constitutional Court leading cases".to_string()],
                }
            ],
            fundamental_rights: vec![
                FundamentalRight {
                    right_name: "Right to Life and Physical Integrity".to_string(),
                    constitutional_basis: "Article 19 No. 1".to_string(),
                    protection_mechanisms: vec!["Constitutional protection action".to_string(), "Criminal law protection".to_string()],
                    limitations: vec!["Legitimate defense".to_string(), "Capital punishment (abolished)".to_string()],
                    jurisprudence: vec!["Constitutional Court cases on life protection".to_string()],
                    international_standards: "American Convention on Human Rights compliance".to_string(),
                },
                FundamentalRight {
                    right_name: "Right to Private Property".to_string(),
                    constitutional_basis: "Article 19 No. 24".to_string(),
                    protection_mechanisms: vec!["Constitutional protection".to_string(), "Just compensation for expropriation".to_string()],
                    limitations: vec!["Social function of property".to_string(), "Expropriation for public utility".to_string()],
                    jurisprudence: vec!["Extensive Constitutional Court jurisprudence".to_string()],
                    international_standards: "Investment protection standards".to_string(),
                }
            ],
            state_organization: StateOrganization {
                executive_power: ExecutivePower {
                    president: President {
                        current_president: "Gabriel Boric Font".to_string(),
                        election_system: "Two-round direct election".to_string(),
                        term_duration: "4 years, non-consecutive re-election".to_string(),
                        powers: vec![
                            "Head of State and Government".to_string(),
                            "Supreme Commander of Armed Forces".to_string(),
                            "Legislative initiative".to_string(),
                            "Regulatory power".to_string()
                        ],
                        restrictions: vec!["Constitutional limitations".to_string(), "Congressional oversight".to_string()],
                        accountability: vec!["Political trial".to_string(), "Constitutional responsibility".to_string()],
                    },
                    government: Government {
                        cabinet_structure: "Council of Ministers with sectoral coordination".to_string(),
                        ministers: vec![
                            Minister {
                                ministry_name: "Ministry of Justice and Human Rights".to_string(),
                                minister_name: "Luis Cordero Vega".to_string(),
                                portfolio_areas: vec!["Justice administration".to_string(), "Human rights".to_string(), "Legislative coordination".to_string()],
                                regulatory_powers: vec!["Justice system regulations".to_string(), "Human rights policies".to_string()],
                                budget_allocation: "950 billion pesos (2024)".to_string(),
                            }
                        ],
                        appointment_process: "Presidential appointment with congressional ratification for some positions".to_string(),
                        collective_responsibility: "Solidarity responsibility before Congress".to_string(),
                    },
                    public_administration: PublicAdministration {
                        civil_service: CivilService {
                            career_system: "Merit-based public employment".to_string(),
                            merit_principles: vec!["Open competition".to_string(), "Professional qualifications".to_string()],
                            professional_development: "National Training and Civil Service Directorate".to_string(),
                            ethics_framework: "Public Ethics Law and transparency obligations".to_string(),
                        },
                        decentralized_services: vec!["Service agencies".to_string(), "Public enterprises".to_string()],
                        regulatory_agencies: vec!["Independent regulatory authorities".to_string()],
                        transparency_mechanisms: vec!["Transparency Law".to_string(), "Public information access".to_string()],
                    },
                    emergency_powers: EmergencyPowers {
                        emergency_types: vec!["State of siege".to_string(), "State of emergency".to_string(), "State of catastrophe".to_string()],
                        declaration_procedures: "Presidential declaration with constitutional requirements".to_string(),
                        limitations: vec!["Time limits".to_string(), "Territorial scope".to_string(), "Rights that cannot be suspended".to_string()],
                        congressional_oversight: "Congressional approval and oversight required".to_string(),
                    },
                },
                legislative_power: LegislativePower {
                    congress: Congress {
                        chamber_of_deputies: ChamberDeputies {
                            composition: "155 deputies".to_string(),
                            election_system: "Proportional representation in 28 districts".to_string(),
                            term_duration: "4 years".to_string(),
                            exclusive_powers: vec!["Budget approval".to_string(), "Political trials".to_string()],
                            current_composition: vec![
                                PoliticalParty {
                                    party_name: "Approve Dignity".to_string(),
                                    seats_deputies: 37,
                                    seats_senate: 6,
                                    political_orientation: "Left coalition".to_string(),
                                    party_leader: "Gabriel Boric".to_string(),
                                },
                                PoliticalParty {
                                    party_name: "Chile We Can Do More".to_string(),
                                    seats_deputies: 53,
                                    seats_senate: 24,
                                    political_orientation: "Center-right coalition".to_string(),
                                    party_leader: "Sebastián Sichel".to_string(),
                                }
                            ],
                        },
                        senate: Senate {
                            composition: "50 senators".to_string(),
                            election_system: "Direct election in 15 senatorial districts".to_string(),
                            term_duration: "8 years, renewable".to_string(),
                            exclusive_powers: vec!["High appointments approval".to_string(), "Political trials judgment".to_string()],
                            regional_representation: "Two senators per region".to_string(),
                        },
                        joint_sessions: vec!["Constitutional amendment".to_string(), "Presidential inauguration".to_string()],
                        parliamentary_groups: vec![
                            ParliamentaryGroup {
                                group_name: "Government Coalition".to_string(),
                                member_parties: vec!["Approve Dignity".to_string(), "Democratic Socialism".to_string()],
                                political_position: "Government support".to_string(),
                                parliamentary_strength: "Minority government".to_string(),
                            }
                        ],
                    },
                    legislative_process: LegislativeProcess {
                        bill_initiation: vec!["Presidential message".to_string(), "Parliamentary motion".to_string()],
                        committee_system: "Specialized permanent and special committees".to_string(),
                        voting_procedures: "Simple and qualified majorities depending on matter".to_string(),
                        bicameral_coordination: "Both chambers must approve identical text".to_string(),
                        presidential_veto: "Total or partial veto with congressional override".to_string(),
                    },
                    oversight_functions: vec!["Interpellations".to_string(), "Censure motions".to_string(), "Investigative commissions".to_string()],
                    budget_powers: vec!["Annual budget approval".to_string(), "Public expenditure oversight".to_string()],
                },
                judicial_power: JudicialPower {
                    supreme_court: SupremeCourt {
                        composition: "21 justices".to_string(),
                        president_court: "Justice Juan Eduardo Fuentes Belmar".to_string(),
                        jurisdictions: vec!["Cassation court".to_string(), "Administrative superintendence".to_string()],
                        administrative_functions: vec!["Judicial career management".to_string(), "Court administration".to_string()],
                        disciplinary_powers: vec!["Judicial discipline".to_string(), "Professional ethics enforcement".to_string()],
                    },
                    courts_appeal: vec![
                        CourtAppeal {
                            jurisdiction_name: "Santiago Court of Appeals".to_string(),
                            territorial_scope: vec!["Santiago Metropolitan Region".to_string()],
                            specialized_chambers: vec!["Civil".to_string(), "Criminal".to_string(), "Family".to_string()],
                            caseload: "Highest volume in the country".to_string(),
                        }
                    ],
                    specialized_courts: vec![
                        SpecializedCourt {
                            court_type: "Family Courts".to_string(),
                            specialized_area: "Family law and child protection".to_string(),
                            jurisdiction: vec!["Family relations".to_string(), "Child welfare".to_string()],
                            special_procedures: vec!["Mediation procedures".to_string(), "Expedited processes".to_string()],
                        }
                    ],
                    prosecution_service: ProsecutionService {
                        national_prosecutor: "Ángel Valencia Espinoza".to_string(),
                        organizational_structure: "National, regional, and local levels".to_string(),
                        investigation_powers: vec!["Criminal investigation".to_string(), "Public prosecution".to_string()],
                        independence_guarantees: vec!["Constitutional autonomy".to_string(), "Functional independence".to_string()],
                    },
                    judicial_council: JudicialCouncil {
                        composition: "Mixed composition with judicial and external members".to_string(),
                        functions: vec!["Judicial appointments".to_string(), "Judicial evaluation".to_string()],
                        appointment_role: "Selection of court presidents and judicial appointments".to_string(),
                        disciplinary_role: "Investigation of judicial misconduct".to_string(),
                    },
                    judicial_career: JudicialCareer {
                        entry_requirements: vec!["Law degree".to_string(), "Professional experience".to_string(), "Competitive examination".to_string()],
                        promotion_system: "Merit-based evaluation and selection".to_string(),
                        training_system: "Judicial Academy continuous training".to_string(),
                        retirement_system: "Age and service requirements".to_string(),
                    },
                },
                autonomous_institutions: vec![
                    AutonomousInstitution {
                        institution_name: "Central Bank of Chile".to_string(),
                        constitutional_basis: "Article 109 Constitution".to_string(),
                        functions: vec!["Monetary policy".to_string(), "Financial stability".to_string()],
                        independence_level: "Full operational independence".to_string(),
                        accountability_mechanisms: vec!["Congressional reporting".to_string(), "Transparency requirements".to_string()],
                    },
                    AutonomousInstitution {
                        institution_name: "National Television Council".to_string(),
                        constitutional_basis: "Article 19 No. 12 Constitution".to_string(),
                        functions: vec!["Television content regulation".to_string(), "Media pluralism".to_string()],
                        independence_level: "Administrative and technical independence".to_string(),
                        accountability_mechanisms: vec!["Annual reports".to_string(), "Public oversight".to_string()],
                    }
                ],
                territorial_organization: TerritorialOrganization {
                    regions: vec![
                        Region {
                            region_name: "Arica and Parinacota Region".to_string(),
                            region_capital: "Arica".to_string(),
                            provinces_included: vec!["Arica".to_string(), "Parinacota".to_string()],
                            population: 226068,
                            area_km2: 16873,
                            economic_profile: "Mining, agriculture, services, border trade".to_string(),
                            regional_government: RegionalGovernment {
                                regional_governor: "Jorge Díaz Ibarra".to_string(),
                                regional_council: "Regional Council with elected councilors".to_string(),
                                competences: vec!["Regional development".to_string(), "Investment promotion".to_string()],
                                budget_management: "Regional investment budget".to_string(),
                                development_planning: "Regional development strategy".to_string(),
                            },
                        },
                        Region {
                            region_name: "Santiago Metropolitan Region".to_string(),
                            region_capital: "Santiago".to_string(),
                            provinces_included: vec!["Santiago".to_string(), "Cordillera".to_string(), "Chacabuco".to_string(), "Maipo".to_string(), "Melipilla".to_string(), "Talagante".to_string()],
                            population: 7112808,
                            area_km2: 15403,
                            economic_profile: "Services, industry, government, finance".to_string(),
                            regional_government: RegionalGovernment {
                                regional_governor: "Claudio Orrego Larraín".to_string(),
                                regional_council: "Metropolitan Regional Council".to_string(),
                                competences: vec!["Urban planning".to_string(), "Transportation".to_string(), "Environmental management".to_string()],
                                budget_management: "Largest regional budget".to_string(),
                                development_planning: "Metropolitan development plan".to_string(),
                            },
                        }
                    ],
                    provinces: vec![
                        Province {
                            province_name: "Santiago Province".to_string(),
                            provincial_capital: "Santiago".to_string(),
                            communes_included: vec!["Santiago".to_string(), "Las Condes".to_string(), "Providencia".to_string()],
                            provincial_governor: "Gonzalo Durán Baronti".to_string(),
                            administrative_functions: vec!["Provincial coordination".to_string(), "Security coordination".to_string()],
                        }
                    ],
                    communes: vec![
                        Commune {
                            commune_name: "Santiago".to_string(),
                            mayor: "Irací Hassler Jacob".to_string(),
                            municipal_council: "Municipal Council with 10 councilors".to_string(),
                            local_services: vec!["Education".to_string(), "Health".to_string(), "Security".to_string(), "Urban planning".to_string()],
                            budget_size: "145 billion pesos".to_string(),
                            population: 404495,
                        }
                    ],
                    decentralization_level: "Moderate decentralization with regional governments".to_string(),
                    regional_development: "Regional development agencies and investment funds".to_string(),
                },
                checks_balances: vec![
                    "Constitutional Tribunal review".to_string(),
                    "Congressional oversight".to_string(),
                    "Judicial independence".to_string(),
                    "Autonomous institutions".to_string()
                ],
            },
            constitutional_tribunal: ConstitutionalTribunal {
                composition: "10 justices appointed by different authorities".to_string(),
                appointment_process: "Mixed appointment by President, Senate, and Supreme Court".to_string(),
                jurisdictions: vec!["Constitutionality review".to_string(), "Conflicts of competence".to_string(), "Constitutional protection".to_string()],
                constitutional_review: "Abstract and concrete constitutional review".to_string(),
                precedent_system: "Binding precedent in constitutional interpretation".to_string(),
            },
            amendment_procedures: "Different procedures for different constitutional matters, requiring qualified majorities".to_string(),
        };

        let legal_codes = vec![
            ChileanLegalCode {
                code_name: "Civil Code".to_string(),
                promulgation_date: "1857-01-01".to_string(),
                last_major_reform: "2018-05-31".to_string(),
                structure: CodeStructure {
                    books: vec![
                        CodeBook {
                            book_number: 1,
                            book_title: "Of Persons".to_string(),
                            titles: vec![
                                CodeTitle {
                                    title_number: 1,
                                    title_name: "Of Legal and Physical Persons".to_string(),
                                    articles: vec![
                                        CodeArticle {
                                            article_number: "55".to_string(),
                                            article_text: "Persons are natural or juridical. Of the juridical personality and the rules special to them, it is dealt with at the end of this Code.".to_string(),
                                            jurisprudence: vec!["Supreme Court jurisprudence on legal personality".to_string()],
                                            doctrinal_interpretation: vec!["Classical distinction between natural and juridical persons".to_string()],
                                            comparative_law: vec!["Similar to French Civil Code".to_string()],
                                        }
                                    ],
                                    key_concepts: vec!["Legal personality".to_string(), "Natural persons".to_string(), "Juridical persons".to_string()],
                                }
                            ],
                            scope: "Regulation of persons and their legal capacity".to_string(),
                        }
                    ],
                    total_articles: 2524,
                    special_procedures: vec!["Family law procedures".to_string(), "Property registration".to_string()],
                },
                key_principles: vec!["Legal personality".to_string(), "Contractual freedom".to_string(), "Property rights".to_string()],
                international_influence: "Based on French Civil Code with local adaptations".to_string(),
                modernization_efforts: vec!["Family law reform".to_string(), "Gender equality provisions".to_string()],
            }
        ];

        ChileanLegalSystem {
            constitution,
            legal_codes,
            special_laws: vec![
                SpecialLaw {
                    law_number: "20.393".to_string(),
                    law_title: "Criminal Liability of Legal Persons".to_string(),
                    promulgation_date: "2009-12-02".to_string(),
                    subject_matter: "Corporate criminal responsibility".to_string(),
                    key_provisions: vec!["Prevention models".to_string(), "Compliance programs".to_string()],
                    implementing_regulations: vec!["Prosecution guidelines".to_string()],
                    enforcement_mechanisms: vec!["Criminal penalties".to_string(), "Dissolution sanctions".to_string()],
                }
            ],
            regulatory_framework: RegulatoryFramework {
                supreme_decrees: vec![
                    SupremeDecree {
                        decree_number: "100".to_string(),
                        ministry: "Ministry of Justice".to_string(),
                        subject_matter: "Court organization".to_string(),
                        legal_basis: "Judicial Organization Code".to_string(),
                        regulatory_content: "Detailed court structure and procedures".to_string(),
                    }
                ],
                ministerial_resolutions: vec![
                    MinisterialResolution {
                        resolution_number: "1500".to_string(),
                        ministry: "Ministry of Justice".to_string(),
                        administrative_matter: "Judicial administration procedures".to_string(),
                        implementation_details: vec!["Case management systems".to_string()],
                    }
                ],
                regulatory_agencies: vec![
                    RegulatoryAgency {
                        agency_name: "National Energy Commission".to_string(),
                        regulatory_sector: "Energy sector".to_string(),
                        powers: vec!["Tariff regulation".to_string(), "Market oversight".to_string()],
                        independence_level: "Operational independence".to_string(),
                        accountability: vec!["Ministry oversight".to_string(), "Congressional reporting".to_string()],
                    }
                ],
                technical_standards: vec![
                    TechnicalStandard {
                        standard_number: "NCh 430".to_string(),
                        standard_title: "Construction Materials Standards".to_string(),
                        technical_area: "Construction and building".to_string(),
                        compliance_level: "Mandatory for public works".to_string(),
                        certification_body: "National Standards Institute".to_string(),
                    }
                ],
            },
            international_law: InternationalLaw {
                treaty_incorporation: "Constitutional rank for human rights treaties".to_string(),
                international_agreements: vec![
                    InternationalAgreement {
                        agreement_name: "American Convention on Human Rights".to_string(),
                        agreement_type: "Human rights treaty".to_string(),
                        ratification_date: "1990-08-21".to_string(),
                        constitutional_rank: "Constitutional hierarchy".to_string(),
                        implementation_status: "Fully implemented".to_string(),
                    }
                ],
                supranational_integration: vec!["Pacific Alliance".to_string(), "MERCOSUR associate".to_string()],
                dispute_resolution: vec!["International arbitration".to_string(), "Inter-American Court of Human Rights".to_string()],
            },
            legal_education: LegalEducation {
                law_schools: vec![
                    LawSchool {
                        institution_name: "University of Chile Law School".to_string(),
                        accreditation_status: "Fully accredited".to_string(),
                        program_duration: "6 years".to_string(),
                        specializations: vec!["Constitutional law".to_string(), "Commercial law".to_string()],
                        graduate_outcomes: "High bar passage rate".to_string(),
                    }
                ],
                curriculum_standards: "National accreditation requirements".to_string(),
                bar_examination: "Professional practice examination required".to_string(),
                continuing_education: "Mandatory continuing legal education".to_string(),
            },
            legal_profession: LegalProfession {
                bar_association: "Chilean Bar Association (regional colleges)".to_string(),
                professional_ethics: "Professional ethics code and disciplinary system".to_string(),
                disciplinary_system: "Regional bar disciplinary procedures".to_string(),
                specialization_areas: vec!["Civil law".to_string(), "Criminal law".to_string(), "Commercial law".to_string()],
                professional_development: "Continuing education requirements".to_string(),
            },
        }
    }

    pub fn get_region(&self, name: &str) -> Option<&Region> {
        self.constitution.state_organization.territorial_organization.regions
            .iter().find(|region| region.region_name == name)
    }

    pub fn get_constitutional_article(&self, article_number: i32) -> Option<&ConstitutionalArticle> {
        for chapter in &self.constitution.chapters {
            if let Some(article) = chapter.articles.iter().find(|art| art.article_number == article_number) {
                return Some(article);
            }
        }
        None
    }

    pub fn search_legal_codes(&self, query: &str) -> Vec<&ChileanLegalCode> {
        self.legal_codes.iter()
            .filter(|code| code.code_name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn analyze_constitutional_evolution(&self) -> String {
        format!(
            "Chilean constitutional evolution: {} periods from {} to present. \
            The 1980 Constitution has undergone significant democratic reforms, \
            particularly the 2005 reforms that eliminated authoritarian enclaves. \
            Current challenges include constitutional reform debates and democratic deepening.",
            self.constitution.constitutional_history.len(),
            self.constitution.promulgation_date
        )
    }

    pub fn get_territorial_organization(&self) -> &TerritorialOrganization {
        &self.constitution.state_organization.territorial_organization
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chilean_system_creation() {
        let system = ChileanLegalSystem::new();
        assert_eq!(system.constitution.constitution_name, "Political Constitution of the Republic of Chile");
        assert!(!system.legal_codes.is_empty());
    }

    #[test]
    fn test_region_lookup() {
        let system = ChileanLegalSystem::new();
        let santiago = system.get_region("Santiago Metropolitan Region");
        assert!(santiago.is_some());
        assert_eq!(santiago.unwrap().region_capital, "Santiago");
    }

    #[test]
    fn test_constitutional_article_search() {
        let system = ChileanLegalSystem::new();
        let article_1 = system.get_constitutional_article(1);
        assert!(article_1.is_some());
        assert!(article_1.unwrap().article_text.contains("Human beings are born free"));
    }

    #[test]
    fn test_constitutional_evolution() {
        let system = ChileanLegalSystem::new();
        let evolution = system.analyze_constitutional_evolution();
        assert!(evolution.contains("1980 Constitution"));
        assert!(evolution.contains("democratic reforms"));
    }
}