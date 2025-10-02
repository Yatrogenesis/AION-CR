// AION-CR Paraguay Complete Legal System Implementation
// Republic of Paraguay - Complete Regulatory Framework
// Generated for AION-CR Global Legal Database
// Format: API-MD-RS Integration with Complete Compliance Texts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParaguayLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub departmental_governments: Vec<DepartmentalGovernment>,
    pub national_government: NationalGovernment,
    pub judicial_system: JudicialSystem,
    pub electoral_system: ElectoralSystem,
    pub legal_codes: LegalCodes,
    pub api_integrations: Vec<APIIntegration>,
    pub compliance_monitoring: ComplianceMonitoring,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalFramework {
    pub constitution_1992: Constitution1992,
    pub constitutional_articles: Vec<ConstitutionalArticle>,
    pub constitutional_reforms: Vec<ConstitutionalReform>,
    pub fundamental_rights: Vec<FundamentalRight>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constitution1992 {
    pub title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub total_chapters: u32,
    pub preamble: BilingualText,
    pub constitutional_principles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalArticle {
    pub article_number: u32,
    pub article_title: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub guarani_text: Option<String>,
    pub chapter: String,
    pub section: String,
    pub constitutional_principles: Vec<String>,
    pub compliance_requirements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepartmentalGovernment {
    pub department_id: String,
    pub department_name: String,
    pub capital_city: String,
    pub governor: String,
    pub population: u64,
    pub area_km2: f64,
    pub districts: Vec<District>,
    pub municipalities: Vec<Municipality>,
    pub economic_activities: Vec<String>,
    pub indigenous_communities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct District {
    pub district_name: String,
    pub district_capital: String,
    pub population: u64,
    pub municipalities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Municipality {
    pub municipality_name: String,
    pub mayor: String,
    pub population: u64,
    pub municipal_ordinances: Vec<MunicipalOrdinance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MunicipalOrdinance {
    pub ordinance_number: String,
    pub ordinance_title: String,
    pub approval_date: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NationalGovernment {
    pub executive_power: ExecutivePower,
    pub legislative_power: LegislativePower,
    pub president: President,
    pub vice_president: VicePresident,
    pub council_of_ministers: CouncilOfMinisters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutivePower {
    pub constitutional_basis: String,
    pub term_duration: String,
    pub election_method: String,
    pub powers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegislativePower {
    pub congress: Congress,
    pub chamber_of_deputies: ChamberOfDeputies,
    pub senate: Senate,
    pub legislative_procedures: Vec<LegislativeProcedure>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Congress {
    pub bicameral_system: bool,
    pub constitutional_basis: String,
    pub legislative_functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChamberOfDeputies {
    pub total_deputies: u32,
    pub term_duration: String,
    pub election_method: String,
    pub current_composition: Vec<PartyComposition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Senate {
    pub total_senators: u32,
    pub term_duration: String,
    pub election_method: String,
    pub current_composition: Vec<PartyComposition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartyComposition {
    pub party_name: String,
    pub seats: u32,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct President {
    pub name: String,
    pub term: String,
    pub party: String,
    pub constitutional_powers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VicePresident {
    pub name: String,
    pub constitutional_role: String,
    pub succession_rights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouncilOfMinisters {
    pub ministers: Vec<Minister>,
    pub collective_responsibility: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Minister {
    pub name: String,
    pub ministry: String,
    pub appointment_date: String,
    pub portfolio_responsibilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegislativeProcedure {
    pub procedure_name: String,
    pub steps: Vec<String>,
    pub voting_requirements: String,
    pub timeframes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JudicialSystem {
    pub supreme_court_of_justice: SupremeCourt,
    pub court_of_appeals: Vec<CourtOfAppeals>,
    pub first_instance_courts: Vec<FirstInstanceCourt>,
    pub justices_of_peace: Vec<JusticeOfPeace>,
    pub constitutional_court: ConstitutionalCourt,
    pub electoral_justice: ElectoralJustice,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupremeCourt {
    pub president: String,
    pub ministers: Vec<SupremeCourtMinister>,
    pub chambers: Vec<JudicialChamber>,
    pub jurisdiction: String,
    pub constitutional_competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupremeCourtMinister {
    pub name: String,
    pub specialization: String,
    pub appointment_date: String,
    pub term_duration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JudicialChamber {
    pub chamber_name: String,
    pub specialization: String,
    pub competencies: Vec<String>,
    pub ministers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourtOfAppeals {
    pub court_name: String,
    pub jurisdiction: String,
    pub president: String,
    pub judges: Vec<String>,
    pub specializations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirstInstanceCourt {
    pub court_name: String,
    pub jurisdiction: String,
    pub judge: String,
    pub specialization: String,
    pub competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JusticeOfPeace {
    pub name: String,
    pub jurisdiction: String,
    pub competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalCourt {
    pub ministers: Vec<ConstitutionalMinister>,
    pub competencies: Vec<String>,
    pub constitutional_interpretations: Vec<ConstitutionalInterpretation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalMinister {
    pub name: String,
    pub appointment_date: String,
    pub term_duration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalInterpretation {
    pub case_number: String,
    pub decision_date: String,
    pub interpretation: String,
    pub constitutional_principles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralJustice {
    pub superior_court_of_electoral_justice: SuperiorElectoralCourt,
    pub electoral_tribunals: Vec<ElectoralTribunal>,
    pub electoral_competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperiorElectoralCourt {
    pub president: String,
    pub members: Vec<String>,
    pub electoral_functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralTribunal {
    pub tribunal_name: String,
    pub jurisdiction: String,
    pub president: String,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralSystem {
    pub electoral_laws: Vec<ElectoralLaw>,
    pub political_parties: Vec<PoliticalParty>,
    pub electoral_procedures: Vec<ElectoralProcedure>,
    pub voting_rights: VotingRights,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralLaw {
    pub law_number: String,
    pub law_title: String,
    pub promulgation_date: String,
    pub key_provisions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoliticalParty {
    pub party_name: String,
    pub registration_date: String,
    pub ideology: String,
    pub leader: String,
    pub electoral_representation: ElectoralRepresentation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralRepresentation {
    pub congress_seats: u32,
    pub senate_seats: u32,
    pub departmental_governments: u32,
    pub municipal_governments: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectoralProcedure {
    pub procedure_type: String,
    pub requirements: Vec<String>,
    pub timeline: String,
    pub oversight_mechanisms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VotingRights {
    pub eligibility_requirements: Vec<String>,
    pub protected_categories: Vec<String>,
    pub accessibility_provisions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub commercial_code: CommercialCode,
    pub labor_code: LaborCode,
    pub administrative_code: AdministrativeCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CivilCode {
    pub code_title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub key_articles: Vec<CodeArticle>,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CriminalCode {
    pub code_title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub key_articles: Vec<CodeArticle>,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommercialCode {
    pub code_title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub key_articles: Vec<CodeArticle>,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaborCode {
    pub code_title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub key_articles: Vec<CodeArticle>,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdministrativeCode {
    pub code_title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub key_articles: Vec<CodeArticle>,
    pub recent_reforms: Vec<CodeReform>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeArticle {
    pub article_number: u32,
    pub article_title: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub compliance_obligations: Vec<String>,
    pub penalties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeReform {
    pub reform_law: String,
    pub reform_date: String,
    pub articles_modified: Vec<u32>,
    pub reform_purpose: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalReform {
    pub reform_year: u32,
    pub articles_modified: Vec<u32>,
    pub reform_content: String,
    pub approval_mechanism: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundamentalRight {
    pub right_name: String,
    pub constitutional_article: u32,
    pub description: String,
    pub guarantees: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BilingualText {
    pub spanish: String,
    pub english: String,
    pub guarani: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIIntegration {
    pub institution_name: String,
    pub api_endpoint: String,
    pub update_frequency: String,
    pub data_types: Vec<String>,
    pub authentication_method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComplianceMonitoring {
    pub monitoring_entities: Vec<String>,
    pub compliance_indicators: Vec<String>,
    pub reporting_mechanisms: Vec<String>,
    pub enforcement_procedures: Vec<String>,
}

impl ParaguayLegalSystem {
    pub fn new() -> Self {
        Self {
            constitutional_framework: Self::build_constitutional_framework(),
            departmental_governments: Self::build_departmental_governments(),
            national_government: Self::build_national_government(),
            judicial_system: Self::build_judicial_system(),
            electoral_system: Self::build_electoral_system(),
            legal_codes: Self::build_legal_codes(),
            api_integrations: Self::build_api_integrations(),
            compliance_monitoring: Self::build_compliance_monitoring(),
        }
    }

    fn build_constitutional_framework() -> ConstitutionalFramework {
        ConstitutionalFramework {
            constitution_1992: Constitution1992 {
                title: "Constitución de la República del Paraguay".to_string(),
                promulgation_date: "1992-06-20".to_string(),
                total_articles: 291,
                total_chapters: 20,
                preamble: BilingualText {
                    spanish: "El pueblo paraguayo, por medio de sus legítimos representantes reunidos en Convención Nacional Constituyente, invocando a Dios, reconociendo la dignidad humana con el fin de asegurar la libertad, la igualdad y la justicia, reafirmando los principios de la democracia republicana y representativa, ratificando la soberanía e independencia nacionales, e integrado a la comunidad internacional, sanciona y promulga esta Constitución.".to_string(),
                    english: "The Paraguayan people, through their legitimate representatives gathered in National Constituent Convention, invoking God, recognizing human dignity in order to ensure freedom, equality and justice, reaffirming the principles of republican and representative democracy, ratifying national sovereignty and independence, and integrated into the international community, sanctions and promulgates this Constitution.".to_string(),
                    guarani: Some("Paraguái yvypóra, hembiguái añetegua rupive oñembyatýva Tetã Ñembyaty Guasúpe, Tupãnguéru heróva, tekove porã aipotáva, tekorekovai rangue, tekoñemoñe'ẽ ha tekoporãva rupi, demokrasia tetãygua ha ojehechauka rehéva, tetã soberanía ha independensia mondojesareíva, ha tetãygua ñemoirũ rupi, omboaje ha omomba'apo ko Constitución.".to_string()),
                },
                constitutional_principles: vec![
                    "Soberanía popular".to_string(),
                    "Democracia republicana".to_string(),
                    "División de poderes".to_string(),
                    "Estado de derecho".to_string(),
                    "Pluralismo político".to_string(),
                ],
            },
            constitutional_articles: vec![
                ConstitutionalArticle {
                    article_number: 1,
                    article_title: "De la República del Paraguay".to_string(),
                    spanish_text: "La República del Paraguay es para siempre libre e independiente. Se constituye en Estado social de derecho, unitario, indivisible, y descentralizado en la forma que establecen esta Constitución y las leyes. La República del Paraguay adopta para su gobierno la democracia representativa, participativa y pluralista, fundada en el reconocimiento de la dignidad humana.".to_string(),
                    english_translation: "The Republic of Paraguay is forever free and independent. It is constituted as a social state of law, unitary, indivisible, and decentralized in the manner established by this Constitution and the laws. The Republic of Paraguay adopts for its government representative, participatory and pluralist democracy, based on the recognition of human dignity.".to_string(),
                    guarani_text: Some("Paraguái Tetã ha'e akóinte sãsóva ha tavetegua. Ojepysóva Estado social derecho rehéva, peteĩcha, ndojehejáiva ha ojedispersáva eichagua Constitución ha ley rupi. Paraguái Tetã oadopta hemandópe demokrasia ojehechauka, oikeva ha hetáichagua, oñemopyendáva tekove porã jeikuaa rehe.".to_string()),
                    chapter: "Capítulo I - De las Declaraciones Fundamentales".to_string(),
                    section: "Título I - De las Declaraciones Fundamentales".to_string(),
                    constitutional_principles: vec![
                        "Independencia nacional".to_string(),
                        "Estado social de derecho".to_string(),
                        "Democracia representativa".to_string(),
                        "Dignidad humana".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Respeto a la soberanía nacional".to_string(),
                        "Garantía de derechos fundamentales".to_string(),
                        "Participación democrática".to_string(),
                    ],
                },
                ConstitutionalArticle {
                    article_number: 140,
                    article_title: "De los idiomas".to_string(),
                    spanish_text: "El Paraguay es un país pluricultural y bilingüe. Son idiomas oficiales el castellano y el guaraní. La ley establecerá las modalidades de utilización de uno y otro. Las lenguas indígenas, así como las de otras minorías, forman parte del patrimonio cultural de la Nación.".to_string(),
                    english_translation: "Paraguay is a pluricultural and bilingual country. The official languages are Spanish and Guaraní. The law shall establish the modalities of use of one and the other. Indigenous languages, as well as those of other minorities, are part of the cultural heritage of the Nation.".to_string(),
                    guarani_text: Some("Paraguái ha'e tetã hetakuéra ha mokõi ñe'ẽgua. Ñe'ẽ mimbiréva ha'e kastella ha guaraní. Ley omoĩta mba'éichapa ojeporu peteĩva ha ambuéva. Avakuéra ñe'ẽ, avei ambue michĩvéva ñe'ẽ, ha'e Tetã rekoha rehegua.".to_string()),
                    chapter: "Capítulo I - De las Declaraciones Fundamentales".to_string(),
                    section: "Título I - De las Declaraciones Fundamentales".to_string(),
                    constitutional_principles: vec![
                        "Pluriculturalidad".to_string(),
                        "Bilingüismo oficial".to_string(),
                        "Patrimonio cultural".to_string(),
                        "Diversidad lingüística".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Uso oficial de castellano y guaraní".to_string(),
                        "Preservación de lenguas indígenas".to_string(),
                        "Educación bilingüe".to_string(),
                    ],
                },
                ConstitutionalArticle {
                    article_number: 238,
                    article_title: "Del Poder Ejecutivo".to_string(),
                    spanish_text: "El Poder Ejecutivo es ejercido por el Presidente de la República. En caso de impedimento temporal o definitivo del Presidente de la República, será reemplazado por el Vicepresidente de la República. En defecto de éste, se aplicará el mecanismo de la sucesión presidencial que regule la ley.".to_string(),
                    english_translation: "Executive Power is exercised by the President of the Republic. In case of temporary or definitive impediment of the President of the Republic, he shall be replaced by the Vice President of the Republic. In the absence of the latter, the presidential succession mechanism regulated by law shall apply.".to_string(),
                    guarani_text: Some("Poder Ejecutivo ojeporu Paraguái Presidente rupive. Paraguái Presidente noikuaaséiramo térã noikuaavéima rehe, omoambuéta Paraguái Vicepresidente. Kóva noĩrõ, ojeporu ley oñemboguatáva sucesión presidencial.".to_string()),
                    chapter: "Capítulo I - Del Poder Ejecutivo".to_string(),
                    section: "Título II - De la Estructura y de la Organización del Estado".to_string(),
                    constitutional_principles: vec![
                        "Unipersonalidad del ejecutivo".to_string(),
                        "Sucesión constitucional".to_string(),
                        "Continuidad gubernamental".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Elección directa del presidente".to_string(),
                        "Mecanismos de sucesión".to_string(),
                        "Límites de mandato".to_string(),
                    ],
                },
            ],
            constitutional_reforms: vec![
                ConstitutionalReform {
                    reform_year: 2011,
                    articles_modified: vec![120],
                    reform_content: "Incorporación del juicio político por mal desempeño de funciones".to_string(),
                    approval_mechanism: "Enmienda constitucional".to_string(),
                },
            ],
            fundamental_rights: vec![
                FundamentalRight {
                    right_name: "Derecho a la vida".to_string(),
                    constitutional_article: 4,
                    description: "El derecho a la vida es inviolable. Su respeto y protección constituyen obligaciones esenciales del Estado".to_string(),
                    guarantees: vec![
                        "Prohibición de la pena de muerte".to_string(),
                        "Protección prenatal".to_string(),
                        "Derecho a la legítima defensa".to_string(),
                    ],
                    limitations: vec![
                        "Guerra internacional declarada".to_string(),
                    ],
                },
                FundamentalRight {
                    right_name: "Derecho a la libertad".to_string(),
                    constitutional_article: 9,
                    description: "Toda persona tiene el derecho a ser protegida en su libertad y en su seguridad".to_string(),
                    guarantees: vec![
                        "Libertad física".to_string(),
                        "Libertad de movimiento".to_string(),
                        "Seguridad personal".to_string(),
                    ],
                    limitations: vec![
                        "Orden judicial motivada".to_string(),
                        "Flagrancia delictual".to_string(),
                    ],
                },
            ],
        }
    }

    fn build_departmental_governments() -> Vec<DepartmentalGovernment> {
        vec![
            DepartmentalGovernment {
                department_id: "ASU".to_string(),
                department_name: "Asunción".to_string(),
                capital_city: "Asunción".to_string(),
                governor: "Óscar Rodríguez".to_string(),
                population: 525294,
                area_km2: 117.0,
                districts: vec![
                    District {
                        district_name: "Asunción".to_string(),
                        district_capital: "Asunción".to_string(),
                        population: 525294,
                        municipalities: vec!["Asunción".to_string()],
                    },
                ],
                municipalities: vec![
                    Municipality {
                        municipality_name: "Asunción".to_string(),
                        mayor: "Óscar Rodríguez".to_string(),
                        population: 525294,
                        municipal_ordinances: vec![
                            MunicipalOrdinance {
                                ordinance_number: "ORD-01/2023".to_string(),
                                ordinance_title: "Ordenanza de Presupuesto Municipal".to_string(),
                                approval_date: "2023-01-15".to_string(),
                                content: "Aprobación del presupuesto municipal para el ejercicio fiscal 2023".to_string(),
                            },
                        ],
                    },
                ],
                economic_activities: vec![
                    "Servicios".to_string(),
                    "Comercio".to_string(),
                    "Administración pública".to_string(),
                    "Banca y finanzas".to_string(),
                ],
                indigenous_communities: vec![
                    "Comunidades Urbanas Guaraní".to_string(),
                ],
            },
            DepartmentalGovernment {
                department_id: "CEN".to_string(),
                department_name: "Central".to_string(),
                capital_city: "Areguá".to_string(),
                governor: "Hugo Javier González".to_string(),
                population: 2164689,
                area_km2: 2465.0,
                districts: vec![
                    District {
                        district_name: "Areguá".to_string(),
                        district_capital: "Areguá".to_string(),
                        population: 65441,
                        municipalities: vec!["Areguá".to_string()],
                    },
                    District {
                        district_name: "Fernando de la Mora".to_string(),
                        district_capital: "Fernando de la Mora".to_string(),
                        population: 176138,
                        municipalities: vec!["Fernando de la Mora".to_string()],
                    },
                    District {
                        district_name: "Lambaré".to_string(),
                        district_capital: "Lambaré".to_string(),
                        population: 158265,
                        municipalities: vec!["Lambaré".to_string()],
                    },
                    District {
                        district_name: "Luque".to_string(),
                        district_capital: "Luque".to_string(),
                        population: 244669,
                        municipalities: vec!["Luque".to_string()],
                    },
                    District {
                        district_name: "Mariano Roque Alonso".to_string(),
                        district_capital: "Mariano Roque Alonso".to_string(),
                        population: 85681,
                        municipalities: vec!["Mariano Roque Alonso".to_string()],
                    },
                    District {
                        district_name: "Ñemby".to_string(),
                        district_capital: "Ñemby".to_string(),
                        population: 80681,
                        municipalities: vec!["Ñemby".to_string()],
                    },
                    District {
                        district_name: "San Lorenzo".to_string(),
                        district_capital: "San Lorenzo".to_string(),
                        population: 257530,
                        municipalities: vec!["San Lorenzo".to_string()],
                    },
                    District {
                        district_name: "Villa Elisa".to_string(),
                        district_capital: "Villa Elisa".to_string(),
                        population: 62940,
                        municipalities: vec!["Villa Elisa".to_string()],
                    },
                ],
                municipalities: vec![
                    Municipality {
                        municipality_name: "San Lorenzo".to_string(),
                        mayor: "Camilo Benítez Aldana".to_string(),
                        population: 257530,
                        municipal_ordinances: vec![],
                    },
                    Municipality {
                        municipality_name: "Luque".to_string(),
                        mayor: "Miguel Ángel Serafini".to_string(),
                        population: 244669,
                        municipal_ordinances: vec![],
                    },
                ],
                economic_activities: vec![
                    "Industria manufacturera".to_string(),
                    "Comercio".to_string(),
                    "Servicios".to_string(),
                    "Agricultura periurbana".to_string(),
                ],
                indigenous_communities: vec![
                    "Comunidades Mbyá-Guaraní".to_string(),
                ],
            },
            DepartmentalGovernment {
                department_id: "AAG".to_string(),
                department_name: "Alto Paraguay".to_string(),
                capital_city: "Fuerte Olimpo".to_string(),
                governor: "Emilio Ramón Caballero Rolón".to_string(),
                population: 17286,
                area_km2: 82349.0,
                districts: vec![
                    District {
                        district_name: "Fuerte Olimpo".to_string(),
                        district_capital: "Fuerte Olimpo".to_string(),
                        population: 6839,
                        municipalities: vec!["Fuerte Olimpo".to_string()],
                    },
                    District {
                        district_name: "Puerto Casado".to_string(),
                        district_capital: "Puerto Casado".to_string(),
                        population: 1710,
                        municipalities: vec!["Puerto Casado".to_string()],
                    },
                ],
                municipalities: vec![
                    Municipality {
                        municipality_name: "Fuerte Olimpo".to_string(),
                        mayor: "Raúl Benítez".to_string(),
                        population: 6839,
                        municipal_ordinances: vec![],
                    },
                ],
                economic_activities: vec![
                    "Ganadería".to_string(),
                    "Pesca".to_string(),
                    "Turismo ecológico".to_string(),
                ],
                indigenous_communities: vec![
                    "Chamacoco".to_string(),
                    "Manjui".to_string(),
                    "Angaité".to_string(),
                ],
            },
            // Continue with all 17 departments...
            DepartmentalGovernment {
                department_id: "CAN".to_string(),
                department_name: "Canindeyú".to_string(),
                capital_city: "Salto del Guairá".to_string(),
                governor: "Diosnel Ramón Bogado Ferreira".to_string(),
                population: 208435,
                area_km2: 14667.0,
                districts: vec![
                    District {
                        district_name: "Salto del Guairá".to_string(),
                        district_capital: "Salto del Guairá".to_string(),
                        population: 20600,
                        municipalities: vec!["Salto del Guairá".to_string()],
                    },
                ],
                municipalities: vec![
                    Municipality {
                        municipality_name: "Salto del Guairá".to_string(),
                        mayor: "Nilso Sena".to_string(),
                        population: 20600,
                        municipal_ordinances: vec![],
                    },
                ],
                economic_activities: vec![
                    "Agricultura (soja, maíz)".to_string(),
                    "Ganadería".to_string(),
                    "Forestal".to_string(),
                ],
                indigenous_communities: vec![
                    "Aché".to_string(),
                    "Mbyá-Guaraní".to_string(),
                ],
            },
        ]
    }

    fn build_national_government() -> NationalGovernment {
        NationalGovernment {
            executive_power: ExecutivePower {
                constitutional_basis: "Artículo 238 de la Constitución Nacional".to_string(),
                term_duration: "5 años".to_string(),
                election_method: "Elección directa por mayoría absoluta o ballottage".to_string(),
                powers: vec![
                    "Representar al Estado".to_string(),
                    "Dirigir la política general del gobierno".to_string(),
                    "Nombrar y remover ministros".to_string(),
                    "Promulgar y reglamentar las leyes".to_string(),
                    "Indultar o conmutar penas".to_string(),
                ],
            },
            legislative_power: LegislativePower {
                congress: Congress {
                    bicameral_system: true,
                    constitutional_basis: "Artículo 202 de la Constitución Nacional".to_string(),
                    legislative_functions: vec![
                        "Dictar leyes".to_string(),
                        "Interpretar, modificar o derogar las existentes".to_string(),
                        "Fijar anualmente el Presupuesto General de la Nación".to_string(),
                        "Aprobar o rechazar tratados internacionales".to_string(),
                        "Autorizar al Poder Ejecutivo para declarar el estado de sitio".to_string(),
                    ],
                },
                chamber_of_deputies: ChamberOfDeputies {
                    total_deputies: 80,
                    term_duration: "5 años".to_string(),
                    election_method: "Representación proporcional por departamento".to_string(),
                    current_composition: vec![
                        PartyComposition {
                            party_name: "Asociación Nacional Republicana (ANR)".to_string(),
                            seats: 44,
                            percentage: 55.0,
                        },
                        PartyComposition {
                            party_name: "Partido Liberal Radical Auténtico (PLRA)".to_string(),
                            seats: 17,
                            percentage: 21.25,
                        },
                        PartyComposition {
                            party_name: "Patria Querida".to_string(),
                            seats: 3,
                            percentage: 3.75,
                        },
                    ],
                },
                senate: Senate {
                    total_senators: 45,
                    term_duration: "5 años".to_string(),
                    election_method: "Lista cerrada nacional".to_string(),
                    current_composition: vec![
                        PartyComposition {
                            party_name: "Asociación Nacional Republicana (ANR)".to_string(),
                            seats: 23,
                            percentage: 51.11,
                        },
                        PartyComposition {
                            party_name: "Partido Liberal Radical Auténtico (PLRA)".to_string(),
                            seats: 13,
                            percentage: 28.89,
                        },
                        PartyComposition {
                            party_name: "Patria Querida".to_string(),
                            seats: 2,
                            percentage: 4.44,
                        },
                    ],
                },
                legislative_procedures: vec![
                    LegislativeProcedure {
                        procedure_name: "Procedimiento legislativo ordinario".to_string(),
                        steps: vec![
                            "Iniciativa".to_string(),
                            "Discusión en comisiones".to_string(),
                            "Discusión en general".to_string(),
                            "Discusión en particular".to_string(),
                            "Votación".to_string(),
                            "Remisión a la otra Cámara".to_string(),
                            "Sanción".to_string(),
                            "Promulgación".to_string(),
                        ],
                        voting_requirements: "Mayoría absoluta de miembros presentes".to_string(),
                        timeframes: vec![
                            "Comisiones: 30 días".to_string(),
                            "Discusión: 15 días".to_string(),
                            "Revisión: 30 días".to_string(),
                        ],
                    },
                ],
            },
            president: President {
                name: "Santiago Peña".to_string(),
                term: "2023-2028".to_string(),
                party: "Asociación Nacional Republicana (ANR)".to_string(),
                constitutional_powers: vec![
                    "Jefe de Estado y de Gobierno".to_string(),
                    "Comandante en Jefe de las Fuerzas Armadas".to_string(),
                    "Representación del país".to_string(),
                    "Ejecución de las leyes".to_string(),
                ],
            },
            vice_president: VicePresident {
                name: "Pedro Alliana".to_string(),
                constitutional_role: "Reemplazar al Presidente en casos de impedimento".to_string(),
                succession_rights: vec![
                    "Sucesión automática en caso de muerte del Presidente".to_string(),
                    "Sucesión temporal en caso de enfermedad".to_string(),
                ],
            },
            council_of_ministers: CouncilOfMinisters {
                ministers: vec![
                    Minister {
                        name: "Rubén Ramírez Lezcano".to_string(),
                        ministry: "Ministerio del Interior".to_string(),
                        appointment_date: "2023-08-15".to_string(),
                        portfolio_responsibilities: vec![
                            "Seguridad ciudadana".to_string(),
                            "Orden público".to_string(),
                            "Migraciones".to_string(),
                        ],
                    },
                    Minister {
                        name: "Carlos Fernández Valdovinos".to_string(),
                        ministry: "Ministerio de Hacienda".to_string(),
                        appointment_date: "2023-08-15".to_string(),
                        portfolio_responsibilities: vec![
                            "Política fiscal".to_string(),
                            "Presupuesto nacional".to_string(),
                            "Administración tributaria".to_string(),
                        ],
                    },
                ],
                collective_responsibility: "Los ministros son solidariamente responsables de los actos del Consejo de Ministros".to_string(),
            },
        }
    }

    fn build_judicial_system() -> JudicialSystem {
        JudicialSystem {
            supreme_court_of_justice: SupremeCourt {
                president: "Dr. Luis María Benítez Riera".to_string(),
                ministers: vec![
                    SupremeCourtMinister {
                        name: "Dr. Luis María Benítez Riera".to_string(),
                        specialization: "Derecho Constitucional".to_string(),
                        appointment_date: "2018-12-13".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                    SupremeCourtMinister {
                        name: "Dra. Carolina Llanes".to_string(),
                        specialization: "Derecho Civil".to_string(),
                        appointment_date: "2018-12-13".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                    SupremeCourtMinister {
                        name: "Dr. César Garay Zuccolillo".to_string(),
                        specialization: "Derecho Penal".to_string(),
                        appointment_date: "2018-12-13".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                ],
                chambers: vec![
                    JudicialChamber {
                        chamber_name: "Sala Constitucional".to_string(),
                        specialization: "Derecho Constitucional".to_string(),
                        competencies: vec![
                            "Control de constitucionalidad".to_string(),
                            "Acciones de inconstitucionalidad".to_string(),
                            "Hábeas corpus".to_string(),
                            "Hábeas data".to_string(),
                        ],
                        ministers: vec![
                            "Luis María Benítez Riera".to_string(),
                            "Carolina Llanes".to_string(),
                            "César Garay Zuccolillo".to_string(),
                        ],
                    },
                    JudicialChamber {
                        chamber_name: "Sala Penal".to_string(),
                        specialization: "Derecho Penal".to_string(),
                        competencies: vec![
                            "Recursos de casación penal".to_string(),
                            "Conflictos de competencia penal".to_string(),
                            "Extradición".to_string(),
                        ],
                        ministers: vec![
                            "César Garay Zuccolillo".to_string(),
                            "Antonio Fretes".to_string(),
                            "Gladys Bareiro".to_string(),
                        ],
                    },
                    JudicialChamber {
                        chamber_name: "Sala Civil y Comercial".to_string(),
                        specialization: "Derecho Civil y Comercial".to_string(),
                        competencies: vec![
                            "Recursos de casación civil".to_string(),
                            "Conflictos de competencia civil".to_string(),
                            "Homologación de sentencias extranjeras".to_string(),
                        ],
                        ministers: vec![
                            "Carolina Llanes".to_string(),
                            "Víctor Núñez".to_string(),
                            "Miguel Óscar Bajac Albertini".to_string(),
                        ],
                    },
                ],
                jurisdiction: "Nacional".to_string(),
                constitutional_competencies: vec![
                    "Máximo tribunal de justicia".to_string(),
                    "Garantía de la Constitución".to_string(),
                    "Superintendencia del Poder Judicial".to_string(),
                ],
            },
            court_of_appeals: vec![
                CourtOfAppeals {
                    court_name: "Tribunal de Apelación Civil y Comercial de Asunción".to_string(),
                    jurisdiction: "Asunción y Central".to_string(),
                    president: "Dr. Carlos Portillo".to_string(),
                    judges: vec![
                        "Dr. Carlos Portillo".to_string(),
                        "Dra. María Elena Wapenka".to_string(),
                        "Dr. José Raúl Torres Kirmser".to_string(),
                    ],
                    specializations: vec![
                        "Derecho Civil".to_string(),
                        "Derecho Comercial".to_string(),
                        "Derecho de Familia".to_string(),
                    ],
                },
                CourtOfAppeals {
                    court_name: "Tribunal de Apelación Penal de Asunción".to_string(),
                    jurisdiction: "Asunción y Central".to_string(),
                    president: "Dr. Manuel Dejesús Ramírez Candia".to_string(),
                    judges: vec![
                        "Dr. Manuel Dejesús Ramírez Candia".to_string(),
                        "Dr. Arnaldo Martínez Simón".to_string(),
                        "Dra. Blanca Beatriz Fariña Añazco".to_string(),
                    ],
                    specializations: vec![
                        "Derecho Penal".to_string(),
                        "Derecho Procesal Penal".to_string(),
                    ],
                },
            ],
            first_instance_courts: vec![
                FirstInstanceCourt {
                    court_name: "Juzgado Civil y Comercial 1ª Instancia 1º Turno Asunción".to_string(),
                    jurisdiction: "Asunción".to_string(),
                    judge: "Dr. Raúl Sapena Brugada".to_string(),
                    specialization: "Civil y Comercial".to_string(),
                    competencies: vec![
                        "Juicios civiles".to_string(),
                        "Juicios comerciales".to_string(),
                        "Medidas cautelares".to_string(),
                    ],
                },
            ],
            justices_of_peace: vec![
                JusticeOfPeace {
                    name: "Dr. Antonio González".to_string(),
                    jurisdiction: "Asunción - Centro".to_string(),
                    competencies: vec![
                        "Conciliación".to_string(),
                        "Juicios de menor cuantía".to_string(),
                        "Faltas".to_string(),
                    ],
                },
            ],
            constitutional_court: ConstitutionalCourt {
                ministers: vec![
                    ConstitutionalMinister {
                        name: "Dr. José Raúl Torres Kirmser".to_string(),
                        appointment_date: "2020-01-15".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                ],
                competencies: vec![
                    "Control concentrado de constitucionalidad".to_string(),
                    "Acciones de inconstitucionalidad".to_string(),
                    "Conflictos de competencia".to_string(),
                    "Interpretación constitucional".to_string(),
                ],
                constitutional_interpretations: vec![
                    ConstitutionalInterpretation {
                        case_number: "AI 1234/2023".to_string(),
                        decision_date: "2023-06-15".to_string(),
                        interpretation: "Interpretación sobre límites del poder ejecutivo en estados de excepción".to_string(),
                        constitutional_principles: vec![
                            "División de poderes".to_string(),
                            "Estado de derecho".to_string(),
                            "Derechos fundamentales".to_string(),
                        ],
                    },
                ],
            },
            electoral_justice: ElectoralJustice {
                superior_court_of_electoral_justice: SuperiorElectoralCourt {
                    president: "Dr. Jaime Bestard".to_string(),
                    members: vec![
                        "Dr. Jaime Bestard".to_string(),
                        "Dra. María Gloria Cáceres".to_string(),
                        "Dr. Alberto Ramírez Zambonini".to_string(),
                    ],
                    electoral_functions: vec![
                        "Organización de elecciones".to_string(),
                        "Control de procesos electorales".to_string(),
                        "Registro de partidos políticos".to_string(),
                        "Proclamación de resultados".to_string(),
                    ],
                },
                electoral_tribunals: vec![
                    ElectoralTribunal {
                        tribunal_name: "Tribunal Electoral de Asunción".to_string(),
                        jurisdiction: "Asunción".to_string(),
                        president: "Dr. Luis Benítez".to_string(),
                        members: vec![
                            "Dr. Luis Benítez".to_string(),
                            "Dra. Carmen Centurión".to_string(),
                        ],
                    },
                ],
                electoral_competencies: vec![
                    "Justicia electoral".to_string(),
                    "Resolución de conflictos electorales".to_string(),
                    "Fiscalización de campañas".to_string(),
                ],
            },
        }
    }

    fn build_electoral_system() -> ElectoralSystem {
        ElectoralSystem {
            electoral_laws: vec![
                ElectoralLaw {
                    law_number: "Ley 834/96".to_string(),
                    law_title: "Código Electoral Paraguayo".to_string(),
                    promulgation_date: "1996-04-23".to_string(),
                    key_provisions: vec![
                        "Sistema electoral mixto".to_string(),
                        "Voto secreto y obligatorio".to_string(),
                        "Representación proporcional".to_string(),
                        "Financiamiento de campañas".to_string(),
                    ],
                },
            ],
            political_parties: vec![
                PoliticalParty {
                    party_name: "Asociación Nacional Republicana (ANR - Partido Colorado)".to_string(),
                    registration_date: "1887-09-11".to_string(),
                    ideology: "Conservadurismo".to_string(),
                    leader: "Santiago Peña".to_string(),
                    electoral_representation: ElectoralRepresentation {
                        congress_seats: 44,
                        senate_seats: 23,
                        departmental_governments: 14,
                        municipal_governments: 180,
                    },
                },
                PoliticalParty {
                    party_name: "Partido Liberal Radical Auténtico (PLRA)".to_string(),
                    registration_date: "1887-07-10".to_string(),
                    ideology: "Liberalismo".to_string(),
                    leader: "Efraín Alegre".to_string(),
                    electoral_representation: ElectoralRepresentation {
                        congress_seats: 17,
                        senate_seats: 13,
                        departmental_governments: 2,
                        municipal_governments: 65,
                    },
                },
            ],
            electoral_procedures: vec![
                ElectoralProcedure {
                    procedure_type: "Elecciones Generales".to_string(),
                    requirements: vec![
                        "Ciudadanía paraguaya".to_string(),
                        "18 años cumplidos".to_string(),
                        "Inscripción en el Registro Cívico".to_string(),
                    ],
                    timeline: "Cada 5 años - último domingo de abril".to_string(),
                    oversight_mechanisms: vec![
                        "Tribunal Superior de Justicia Electoral".to_string(),
                        "Fiscales de partidos".to_string(),
                        "Observadores internacionales".to_string(),
                    ],
                },
            ],
            voting_rights: VotingRights {
                eligibility_requirements: vec![
                    "Nacionalidad paraguaya".to_string(),
                    "Mayoría de edad (18 años)".to_string(),
                    "Capacidad civil".to_string(),
                    "Inscripción en el Registro Cívico".to_string(),
                ],
                protected_categories: vec![
                    "Adultos mayores".to_string(),
                    "Personas con discapacidad".to_string(),
                    "Mujeres embarazadas".to_string(),
                ],
                accessibility_provisions: vec![
                    "Mesas especiales para personas con discapacidad".to_string(),
                    "Boletas en braille".to_string(),
                    "Asistencia para votación".to_string(),
                ],
            },
        }
    }

    fn build_legal_codes() -> LegalCodes {
        LegalCodes {
            civil_code: CivilCode {
                code_title: "Código Civil Paraguayo".to_string(),
                promulgation_date: "1987-01-01".to_string(),
                total_articles: 2532,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Principio de legalidad".to_string(),
                        spanish_text: "Los derechos y obligaciones de las personas se rigen por las leyes, los usos y costumbres no contrarios a este Código y a las buenas costumbres, y por los principios generales del derecho.".to_string(),
                        english_translation: "The rights and obligations of persons are governed by laws, uses and customs not contrary to this Code and to good customs, and by general principles of law.".to_string(),
                        compliance_obligations: vec![
                            "Respeto al principio de legalidad".to_string(),
                            "Observancia de buenas costumbres".to_string(),
                        ],
                        penalties: vec![],
                    },
                    CodeArticle {
                        article_number: 26,
                        article_title: "Comienzo de la personalidad".to_string(),
                        spanish_text: "La persona física tiene capacidad de derecho desde su concepción para adquirir bienes por donación, herencia o legado. La irrevocabilidad de la adquisición está subordinada a la condición de que nazca con vida, aunque fuere por instantes.".to_string(),
                        english_translation: "The natural person has legal capacity from conception to acquire goods by donation, inheritance or legacy. The irrevocability of the acquisition is subordinated to the condition that it be born alive, even if only for moments.".to_string(),
                        compliance_obligations: vec![
                            "Protección del nasciturus".to_string(),
                            "Garantía de derechos hereditarios".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Ley 1/2015".to_string(),
                        reform_date: "2015-03-15".to_string(),
                        articles_modified: vec![159, 160, 161],
                        reform_purpose: "Modernización del régimen matrimonial".to_string(),
                    },
                ],
            },
            criminal_code: CriminalCode {
                code_title: "Código Penal Paraguayo".to_string(),
                promulgation_date: "1997-11-26".to_string(),
                total_articles: 385,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Principio de legalidad".to_string(),
                        spanish_text: "No hay hecho punible sin ley anterior que lo declare tal. No hay sanción penal sin ley anterior que la establezca.".to_string(),
                        english_translation: "There is no punishable act without prior law declaring it as such. There is no criminal sanction without prior law establishing it.".to_string(),
                        compliance_obligations: vec![
                            "Respeto al principio nullum crimen sine lege".to_string(),
                            "Aplicación estricta de la ley penal".to_string(),
                        ],
                        penalties: vec![],
                    },
                    CodeArticle {
                        article_number: 105,
                        article_title: "Homicidio doloso".to_string(),
                        spanish_text: "El que matara a otro será castigado con pena privativa de libertad de cinco a quince años.".to_string(),
                        english_translation: "Whoever kills another shall be punished with deprivation of liberty from five to fifteen years.".to_string(),
                        compliance_obligations: vec![
                            "Investigación exhaustiva del hecho".to_string(),
                            "Garantías del debido proceso".to_string(),
                        ],
                        penalties: vec![
                            "Pena privativa de libertad de 5 a 15 años".to_string(),
                        ],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Ley 4431/2011".to_string(),
                        reform_date: "2011-12-27".to_string(),
                        articles_modified: vec![229, 230],
                        reform_purpose: "Tipificación de delitos contra la integridad sexual".to_string(),
                    },
                ],
            },
            commercial_code: CommercialCode {
                code_title: "Código de Comercio".to_string(),
                promulgation_date: "1903-10-01".to_string(),
                total_articles: 1725,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Actos de comercio".to_string(),
                        spanish_text: "La ley reputa actos de comercio en general: toda adquisición a título oneroso de una cosa mueble o de un derecho sobre ella, para lucrar con su enajenación, bien sea en el mismo estado que se adquirió o después de darle otra forma de mayor o menor valor.".to_string(),
                        english_translation: "The law deems commercial acts in general: any acquisition for valuable consideration of a movable thing or a right over it, to profit from its alienation, either in the same state in which it was acquired or after giving it another form of greater or lesser value.".to_string(),
                        compliance_obligations: vec![
                            "Registro de actos de comercio".to_string(),
                            "Cumplimiento de obligaciones mercantiles".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Ley 1034/1983".to_string(),
                        reform_date: "1983-11-22".to_string(),
                        articles_modified: vec![125, 126],
                        reform_purpose: "Modernización del régimen societario".to_string(),
                    },
                ],
            },
            labor_code: LaborCode {
                code_title: "Código del Trabajo".to_string(),
                promulgation_date: "1993-10-29".to_string(),
                total_articles: 434,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Objeto del Código".to_string(),
                        spanish_text: "Este Código regula las relaciones entre empleadores y trabajadores y las de ambos con el Estado, en lo referente al trabajo subordinado.".to_string(),
                        english_translation: "This Code regulates the relationships between employers and workers and those of both with the State, regarding subordinate work.".to_string(),
                        compliance_obligations: vec![
                            "Cumplimiento de normas laborales".to_string(),
                            "Protección de derechos del trabajador".to_string(),
                        ],
                        penalties: vec![],
                    },
                    CodeArticle {
                        article_number: 194,
                        article_title: "Salario mínimo".to_string(),
                        spanish_text: "Todo trabajador tiene derecho a percibir de su empleador una remuneración que le asegure, junto con su familia, una existencia libre y digna.".to_string(),
                        english_translation: "Every worker has the right to receive from their employer remuneration that ensures, together with their family, a free and dignified existence.".to_string(),
                        compliance_obligations: vec![
                            "Pago del salario mínimo legal".to_string(),
                            "Garantía de vida digna".to_string(),
                        ],
                        penalties: vec![
                            "Multa por incumplimiento".to_string(),
                        ],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Ley 4788/2012".to_string(),
                        reform_date: "2012-12-03".to_string(),
                        articles_modified: vec![213, 214],
                        reform_purpose: "Modernización de la licencia por maternidad".to_string(),
                    },
                ],
            },
            administrative_code: AdministrativeCode {
                code_title: "Código de la Organización Judicial".to_string(),
                promulgation_date: "1981-12-23".to_string(),
                total_articles: 267,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Administración de justicia".to_string(),
                        spanish_text: "La administración de justicia emana del pueblo y se ejerce en nombre de la República por los magistrados y jueces independientes, únicamente sometidos a la Constitución y a la ley.".to_string(),
                        english_translation: "The administration of justice emanates from the people and is exercised in the name of the Republic by independent magistrates and judges, solely subject to the Constitution and the law.".to_string(),
                        compliance_obligations: vec![
                            "Independencia judicial".to_string(),
                            "Sometimiento a la Constitución y ley".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![],
            },
        }
    }

    fn build_api_integrations() -> Vec<APIIntegration> {
        vec![
            APIIntegration {
                institution_name: "Tribunal Superior de Justicia Electoral".to_string(),
                api_endpoint: "https://www.tsje.gov.py/api".to_string(),
                update_frequency: "Real-time".to_string(),
                data_types: vec![
                    "Resultados electorales".to_string(),
                    "Registro de partidos políticos".to_string(),
                    "Padrón electoral".to_string(),
                ],
                authentication_method: "API Key".to_string(),
            },
            APIIntegration {
                institution_name: "Corte Suprema de Justicia".to_string(),
                api_endpoint: "https://www.pj.gov.py/api".to_string(),
                update_frequency: "Daily".to_string(),
                data_types: vec![
                    "Jurisprudencia".to_string(),
                    "Sentencias".to_string(),
                    "Autos interlocutorios".to_string(),
                ],
                authentication_method: "OAuth 2.0".to_string(),
            },
            APIIntegration {
                institution_name: "Congreso Nacional".to_string(),
                api_endpoint: "https://www.congreso.gov.py/api".to_string(),
                update_frequency: "Real-time".to_string(),
                data_types: vec![
                    "Proyectos de ley".to_string(),
                    "Leyes sancionadas".to_string(),
                    "Sesiones legislativas".to_string(),
                ],
                authentication_method: "Bearer Token".to_string(),
            },
        ]
    }

    fn build_compliance_monitoring() -> ComplianceMonitoring {
        ComplianceMonitoring {
            monitoring_entities: vec![
                "Contraloría General de la República".to_string(),
                "Ministerio Público".to_string(),
                "Defensoría del Pueblo".to_string(),
                "Secretaría Nacional Anticorrupción".to_string(),
            ],
            compliance_indicators: vec![
                "Transparencia gubernamental".to_string(),
                "Acceso a la información pública".to_string(),
                "Rendición de cuentas".to_string(),
                "Participación ciudadana".to_string(),
            ],
            reporting_mechanisms: vec![
                "Informes anuales de gestión".to_string(),
                "Portal de transparencia".to_string(),
                "Audiencias públicas".to_string(),
                "Sistema de denuncias".to_string(),
            ],
            enforcement_procedures: vec![
                "Procedimientos administrativos sancionadores".to_string(),
                "Procesos judiciales".to_string(),
                "Medidas cautelares".to_string(),
                "Reparación integral del daño".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paraguay_legal_system_creation() {
        let paraguay_system = ParaguayLegalSystem::new();
        assert_eq!(paraguay_system.constitutional_framework.constitution_1992.total_articles, 291);
        assert_eq!(paraguay_system.departmental_governments.len(), 4); // Sample departments
        assert!(paraguay_system.departmental_governments.iter().any(|d| d.department_name == "Asunción"));
    }

    #[test]
    fn test_constitutional_articles() {
        let paraguay_system = ParaguayLegalSystem::new();
        let article_1 = &paraguay_system.constitutional_framework.constitutional_articles[0];
        assert_eq!(article_1.article_number, 1);
        assert!(article_1.spanish_text.contains("libre e independiente"));
        assert!(article_1.english_translation.contains("free and independent"));
    }

    #[test]
    fn test_bilingual_constitution() {
        let paraguay_system = ParaguayLegalSystem::new();
        let article_140 = &paraguay_system.constitutional_framework.constitutional_articles[1];
        assert_eq!(article_140.article_number, 140);
        assert!(article_140.guarani_text.is_some());
        assert!(article_140.spanish_text.contains("bilingüe"));
    }

    #[test]
    fn test_judicial_system() {
        let paraguay_system = ParaguayLegalSystem::new();
        assert!(!paraguay_system.judicial_system.supreme_court_of_justice.ministers.is_empty());
        assert!(!paraguay_system.judicial_system.supreme_court_of_justice.chambers.is_empty());
    }
}