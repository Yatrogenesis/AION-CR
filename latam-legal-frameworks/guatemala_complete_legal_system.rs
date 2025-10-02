// AION-CR Guatemala Complete Legal System Implementation
// Republic of Guatemala - Complete Regulatory Framework
// Generated for AION-CR Global Legal Database
// Format: API-MD-RS Integration with Complete Compliance Texts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuatemalaLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub departmental_governments: Vec<DepartmentalGovernment>,
    pub national_government: NationalGovernment,
    pub judicial_system: JudicialSystem,
    pub electoral_system: ElectoralSystem,
    pub legal_codes: LegalCodes,
    pub indigenous_jurisdiction: IndigenousJurisdiction,
    pub api_integrations: Vec<APIIntegration>,
    pub compliance_monitoring: ComplianceMonitoring,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalFramework {
    pub constitution_1985: Constitution1985,
    pub constitutional_articles: Vec<ConstitutionalArticle>,
    pub constitutional_reforms: Vec<ConstitutionalReform>,
    pub fundamental_rights: Vec<FundamentalRight>,
    pub constitutional_court: ConstitutionalCourt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constitution1985 {
    pub title: String,
    pub promulgation_date: String,
    pub total_articles: u32,
    pub total_titles: u32,
    pub preamble: BilingualText,
    pub constitutional_principles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalArticle {
    pub article_number: u32,
    pub article_title: String,
    pub spanish_text: String,
    pub english_translation: String,
    pub mayan_text: Option<String>,
    pub title: String,
    pub chapter: String,
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
    pub municipalities: Vec<Municipality>,
    pub linguistic_communities: Vec<LinguisticCommunity>,
    pub economic_activities: Vec<String>,
    pub indigenous_peoples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Municipality {
    pub municipality_name: String,
    pub mayor: String,
    pub population: u64,
    pub municipal_code: String,
    pub municipal_ordinances: Vec<MunicipalOrdinance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinguisticCommunity {
    pub community_name: String,
    pub language: String,
    pub population: u64,
    pub cultural_practices: Vec<String>,
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
    pub legislative_procedures: Vec<LegislativeProcedure>,
    pub current_legislation: Vec<NationalLaw>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Congress {
    pub unicameral_system: bool,
    pub total_deputies: u32,
    pub constitutional_basis: String,
    pub legislative_functions: Vec<String>,
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
pub struct NationalLaw {
    pub law_number: String,
    pub law_title: String,
    pub promulgation_date: String,
    pub articles: Vec<LawArticle>,
    pub regulatory_scope: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LawArticle {
    pub article_number: u32,
    pub content: String,
    pub compliance_obligations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JudicialSystem {
    pub supreme_court_of_justice: SupremeCourt,
    pub courts_of_appeals: Vec<CourtOfAppeals>,
    pub first_instance_courts: Vec<FirstInstanceCourt>,
    pub justices_of_peace: Vec<JusticeOfPeace>,
    pub specialized_courts: Vec<SpecializedCourt>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupremeCourt {
    pub president: String,
    pub magistrates: Vec<SupremeCourtMagistrate>,
    pub chambers: Vec<JudicialChamber>,
    pub jurisdiction: String,
    pub constitutional_competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupremeCourtMagistrate {
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
    pub magistrates: Vec<String>,
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
pub struct SpecializedCourt {
    pub court_name: String,
    pub specialization: String,
    pub jurisdiction: String,
    pub judges: Vec<String>,
    pub specific_competencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalCourt {
    pub magistrates: Vec<ConstitutionalMagistrate>,
    pub competencies: Vec<String>,
    pub constitutional_interpretations: Vec<ConstitutionalInterpretation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstitutionalMagistrate {
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
pub struct ElectoralSystem {
    pub supreme_electoral_tribunal: SupremeElectoralTribunal,
    pub departmental_electoral_boards: Vec<DepartmentalElectoralBoard>,
    pub electoral_laws: Vec<ElectoralLaw>,
    pub political_parties: Vec<PoliticalParty>,
    pub electoral_procedures: Vec<ElectoralProcedure>,
    pub voting_rights: VotingRights,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupremeElectoralTribunal {
    pub president: String,
    pub magistrates: Vec<String>,
    pub electoral_functions: Vec<String>,
    pub current_term: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepartmentalElectoralBoard {
    pub department: String,
    pub president: String,
    pub members: Vec<String>,
    pub electoral_jurisdiction: String,
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
    pub departmental_governments: u32,
    pub municipal_governments: u32,
    pub presidential_votes: Option<u64>,
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
pub struct IndigenousJurisdiction {
    pub constitutional_recognition: String,
    pub indigenous_peoples: Vec<IndigenousPeople>,
    pub customary_law_systems: Vec<CustomaryLawSystem>,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndigenousPeople {
    pub people_name: String,
    pub language: String,
    pub population: u64,
    pub territory: String,
    pub traditional_authorities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomaryLawSystem {
    pub law_system_name: String,
    pub people: String,
    pub principles: Vec<String>,
    pub dispute_resolution: Vec<String>,
    pub traditional_sanctions: Vec<String>,
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
    pub mayan: Option<String>,
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

impl GuatemalaLegalSystem {
    pub fn new() -> Self {
        Self {
            constitutional_framework: Self::build_constitutional_framework(),
            departmental_governments: Self::build_departmental_governments(),
            national_government: Self::build_national_government(),
            judicial_system: Self::build_judicial_system(),
            electoral_system: Self::build_electoral_system(),
            legal_codes: Self::build_legal_codes(),
            indigenous_jurisdiction: Self::build_indigenous_jurisdiction(),
            api_integrations: Self::build_api_integrations(),
            compliance_monitoring: Self::build_compliance_monitoring(),
        }
    }

    fn build_constitutional_framework() -> ConstitutionalFramework {
        ConstitutionalFramework {
            constitution_1985: Constitution1985 {
                title: "Constitución Política de la República de Guatemala".to_string(),
                promulgation_date: "1985-05-31".to_string(),
                total_articles: 281,
                total_titles: 7,
                preamble: BilingualText {
                    spanish: "Nosotros, los representantes del pueblo de Guatemala, reunidos en Asamblea Nacional Constituyente, invocando el nombre de Dios, inspirados en los ideales de nuestros antepasados y recogiendo nuestras tradiciones y herencia cultural; decididos a impulsar la plena vigencia de los Derechos Humanos dentro de un orden institucional estable, permanente y popular, donde gobernados y gobernantes procedan con absoluto apego a la ley; solemnes al sentar las bases jurídicas de la convivencia nacional y del orden político e institucional que debe regir al Estado para lograr la justicia, la seguridad, la paz y el desarrollo integral de la persona humana; promulgamos esta Constitución de la República de Guatemala, para que proteja y desarrolle la vida, los bienes, el honor, la dignidad y demás derechos de la persona humana, así como el derecho de los pueblos indígenas de Guatemala.".to_string(),
                    english: "We, the representatives of the people of Guatemala, gathered in National Constituent Assembly, invoking the name of God, inspired by the ideals of our ancestors and gathering our traditions and cultural heritage; determined to promote the full validity of Human Rights within a stable, permanent and popular institutional order, where governed and governors proceed with absolute adherence to the law; solemn in laying the legal foundations of national coexistence and the political and institutional order that should govern the State to achieve justice, security, peace and integral development of the human person; we promulgate this Constitution of the Republic of Guatemala, so that it protects and develops life, goods, honor, dignity and other rights of the human person, as well as the rights of the indigenous peoples of Guatemala.".to_string(),
                    mayan: Some("Oj, ri kitzalkatesalab' rech ri tinimit Guatemala, kaq'apisan pa Nimalaj Moloy Tzij, kowokisaj ri ub'i' ri Ajaw, kojol ri naoj rech ri qatat qamam chuqa' kek'am ri qawinakem chuqa' ri qawinaqil rachb'al; nojchak ri kab'an ronojel ruwach chaq'ij ri Derechos Humanos chupam jun nojchak, man k'o ta rujalchanik chuqa' rech ri tinimit moltzij.".to_string()),
                },
                constitutional_principles: vec![
                    "Protección de la persona humana".to_string(),
                    "Deberes del Estado".to_string(),
                    "Derecho a la vida".to_string(),
                    "Libertad e igualdad".to_string(),
                    "Libertad de acción".to_string(),
                ],
            },
            constitutional_articles: vec![
                ConstitutionalArticle {
                    article_number: 1,
                    article_title: "Protección de la Persona".to_string(),
                    spanish_text: "El Estado de Guatemala se organiza para proteger a la persona y a la familia; su fin supremo es la realización del bien común.".to_string(),
                    english_translation: "The State of Guatemala is organized to protect the person and the family; its supreme end is the realization of the common good.".to_string(),
                    mayan_text: Some("Ri moloj Guatemala nuk'un chi utz kuto' ri winaq chuqa' ri rachoch; ruwach'ulew re nimalaj utzilal pa ronojel.".to_string()),
                    title: "Título I - La Persona Humana, Fines y Deberes del Estado".to_string(),
                    chapter: "Capítulo Único".to_string(),
                    constitutional_principles: vec![
                        "Protección de la persona".to_string(),
                        "Protección de la familia".to_string(),
                        "Bien común".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Políticas de protección familiar".to_string(),
                        "Promoción del bien común".to_string(),
                        "Garantías individuales".to_string(),
                    ],
                },
                ConstitutionalArticle {
                    article_number: 2,
                    article_title: "Deberes del Estado".to_string(),
                    spanish_text: "Es deber del Estado garantizarle a los habitantes de la República la vida, la libertad, la justicia, la seguridad, la paz y el desarrollo integral de la persona.".to_string(),
                    english_translation: "It is the duty of the State to guarantee to the inhabitants of the Republic life, liberty, justice, security, peace and integral development of the person.".to_string(),
                    mayan_text: Some("Raj chaq rech ri moloj Guatemala ri kuya' chike ri e k'o pa re tinamit re ri k'aslemal, ri atzilinel, ri kutan, ri chajinel, ri utzil chuqa' ri ronojel ruk'iyirisaxik ri winaq.".to_string()),
                    title: "Título I - La Persona Humana, Fines y Deberes del Estado".to_string(),
                    chapter: "Capítulo Único".to_string(),
                    constitutional_principles: vec![
                        "Garantía de vida".to_string(),
                        "Garantía de libertad".to_string(),
                        "Garantía de justicia".to_string(),
                        "Garantía de seguridad".to_string(),
                        "Garantía de paz".to_string(),
                        "Desarrollo integral".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Protección efectiva de derechos".to_string(),
                        "Sistemas de seguridad".to_string(),
                        "Acceso a la justicia".to_string(),
                        "Políticas de desarrollo".to_string(),
                    ],
                },
                ConstitutionalArticle {
                    article_number: 66,
                    article_title: "Protección a grupos étnicos".to_string(),
                    spanish_text: "Guatemala está formada por diversos grupos étnicos entre los que figuran los grupos indígenas de ascendencia maya. El Estado reconoce, respeta y promueve sus formas de vida, costumbres, tradiciones, formas de organización social, el uso del traje indígena en hombres y mujeres, idiomas y dialectos.".to_string(),
                    english_translation: "Guatemala is formed by diverse ethnic groups among which are the indigenous groups of Mayan descent. The State recognizes, respects and promotes their ways of life, customs, traditions, forms of social organization, the use of indigenous clothing in men and women, languages and dialects.".to_string(),
                    mayan_text: Some("Guatemala nuk'un ruma k'iya' ruwinaqil amaq', chikixo'l ri mayab' amaq'. Ri moloj nukuj, nuchak'oj chuqa' nukanoj ri kiwinaqil, kitaqanem, kisamajinem, kichomanem, ri kemom kipo' achijab' chuqa' ixoqib', kitzij chuqa' kichomanem tzij.".to_string()),
                    title: "Título II - Derechos Humanos".to_string(),
                    chapter: "Capítulo II - Derechos Sociales".to_string(),
                    constitutional_principles: vec![
                        "Diversidad étnica".to_string(),
                        "Reconocimiento indígena".to_string(),
                        "Respeto cultural".to_string(),
                        "Promoción de tradiciones".to_string(),
                    ],
                    compliance_requirements: vec![
                        "Políticas de reconocimiento cultural".to_string(),
                        "Protección de idiomas indígenas".to_string(),
                        "Respeto a formas de organización".to_string(),
                        "Promoción de vestimenta tradicional".to_string(),
                    ],
                },
            ],
            constitutional_reforms: vec![
                ConstitutionalReform {
                    reform_year: 1993,
                    articles_modified: vec![173, 174, 175],
                    reform_content: "Consulta Popular para reformas constitucionales".to_string(),
                    approval_mechanism: "Referéndum".to_string(),
                },
            ],
            fundamental_rights: vec![
                FundamentalRight {
                    right_name: "Derecho a la vida".to_string(),
                    constitutional_article: 3,
                    description: "El Estado garantiza y protege la vida humana desde su concepción, así como la integridad y la seguridad de la persona".to_string(),
                    guarantees: vec![
                        "Prohibición de la pena de muerte para delitos políticos".to_string(),
                        "Protección prenatal".to_string(),
                        "Seguridad personal".to_string(),
                    ],
                    limitations: vec![
                        "Legítima defensa".to_string(),
                        "Guerra legalmente declarada".to_string(),
                    ],
                },
                FundamentalRight {
                    right_name: "Derecho a la igualdad".to_string(),
                    constitutional_article: 4,
                    description: "En Guatemala todos los seres humanos son libres e iguales en dignidad y derechos".to_string(),
                    guarantees: vec![
                        "No discriminación".to_string(),
                        "Igualdad ante la ley".to_string(),
                        "Igual protección".to_string(),
                    ],
                    limitations: vec![],
                },
            ],
            constitutional_court: ConstitutionalCourt {
                magistrates: vec![
                    ConstitutionalMagistrate {
                        name: "Dra. Gloria Patricia Porras Escobar".to_string(),
                        appointment_date: "2019-04-14".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                    ConstitutionalMagistrate {
                        name: "Dr. José Francisco de Mata Vela".to_string(),
                        appointment_date: "2019-04-14".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                ],
                competencies: vec![
                    "Conocer en única instancia las impugnaciones contra leyes, reglamentos o disposiciones de carácter general".to_string(),
                    "Conocer en apelación de todos los amparos interpuestos ante cualquiera de los tribunales de justicia".to_string(),
                    "Conocer en casación los recursos interpuestos contra las sentencias de amparo y en los casos determinados por la ley".to_string(),
                    "Conocer en única instancia en calidad de Tribunal Extraordinario de Amparo".to_string(),
                ],
                constitutional_interpretations: vec![
                    ConstitutionalInterpretation {
                        case_number: "EI-2023-01".to_string(),
                        decision_date: "2023-03-15".to_string(),
                        interpretation: "Interpretación sobre derechos de los pueblos indígenas y consulta previa".to_string(),
                        constitutional_principles: vec![
                            "Diversidad cultural".to_string(),
                            "Derechos indígenas".to_string(),
                            "Consulta previa".to_string(),
                        ],
                    },
                ],
            },
        }
    }

    fn build_departmental_governments() -> Vec<DepartmentalGovernment> {
        vec![
            DepartmentalGovernment {
                department_id: "GT".to_string(),
                department_name: "Guatemala".to_string(),
                capital_city: "Guatemala".to_string(),
                governor: "Manuel Benedicto Lucas García".to_string(),
                population: 3156284,
                area_km2: 2126.0,
                municipalities: vec![
                    Municipality {
                        municipality_name: "Guatemala".to_string(),
                        mayor: "Ricardo Quiñónez".to_string(),
                        population: 994938,
                        municipal_code: "0101".to_string(),
                        municipal_ordinances: vec![
                            MunicipalOrdinance {
                                ordinance_number: "ORD-01-2023".to_string(),
                                ordinance_title: "Ordenanza de Tránsito Municipal".to_string(),
                                approval_date: "2023-01-15".to_string(),
                                content: "Regulación del tránsito vehicular y peatonal en la ciudad de Guatemala".to_string(),
                            },
                        ],
                    },
                    Municipality {
                        municipality_name: "Mixco".to_string(),
                        mayor: "Óscar Humberto Coyoy Echeverría".to_string(),
                        population: 473080,
                        municipal_code: "0108".to_string(),
                        municipal_ordinances: vec![],
                    },
                    Municipality {
                        municipality_name: "Villa Nueva".to_string(),
                        mayor: "Javier Eduardo Gramajo López".to_string(),
                        population: 618397,
                        municipal_code: "0109".to_string(),
                        municipal_ordinances: vec![],
                    },
                ],
                linguistic_communities: vec![
                    LinguisticCommunity {
                        community_name: "Comunidad K'iche'".to_string(),
                        language: "K'iche'".to_string(),
                        population: 245000,
                        cultural_practices: vec![
                            "Ceremonias mayas".to_string(),
                            "Textiles tradicionales".to_string(),
                            "Agricultura ancestral".to_string(),
                        ],
                    },
                    LinguisticCommunity {
                        community_name: "Comunidad Kaqchikel".to_string(),
                        language: "Kaqchikel".to_string(),
                        population: 189000,
                        cultural_practices: vec![
                            "Medicina tradicional".to_string(),
                            "Artesanías".to_string(),
                            "Calendario maya".to_string(),
                        ],
                    },
                ],
                economic_activities: vec![
                    "Servicios".to_string(),
                    "Industria manufacturera".to_string(),
                    "Comercio".to_string(),
                    "Construcción".to_string(),
                ],
                indigenous_peoples: vec![
                    "K'iche'".to_string(),
                    "Kaqchikel".to_string(),
                    "Poqomam".to_string(),
                ],
            },
            DepartmentalGovernment {
                department_id: "HP".to_string(),
                department_name: "Huehuetenango".to_string(),
                capital_city: "Huehuetenango".to_string(),
                governor: "César Augusto Castañón".to_string(),
                population: 1143887,
                area_km2: 7400.0,
                municipalities: vec![
                    Municipality {
                        municipality_name: "Huehuetenango".to_string(),
                        mayor: "Mynor Ambrocio Cordón Reyes".to_string(),
                        population: 79426,
                        municipal_code: "1301".to_string(),
                        municipal_ordinances: vec![],
                    },
                ],
                linguistic_communities: vec![
                    LinguisticCommunity {
                        community_name: "Comunidad Mam".to_string(),
                        language: "Mam".to_string(),
                        population: 478000,
                        cultural_practices: vec![
                            "Cofradías tradicionales".to_string(),
                            "Tejidos ancestrales".to_string(),
                            "Ceremonias agrícolas".to_string(),
                        ],
                    },
                    LinguisticCommunity {
                        community_name: "Comunidad Q'anjob'al".to_string(),
                        language: "Q'anjob'al".to_string(),
                        population: 77700,
                        cultural_practices: vec![
                            "Autoridades ancestrales".to_string(),
                            "Rituales de lluvia".to_string(),
                            "Música tradicional".to_string(),
                        ],
                    },
                ],
                economic_activities: vec![
                    "Agricultura".to_string(),
                    "Ganadería".to_string(),
                    "Textiles".to_string(),
                    "Café".to_string(),
                ],
                indigenous_peoples: vec![
                    "Mam".to_string(),
                    "Q'anjob'al".to_string(),
                    "Chuj".to_string(),
                    "Akateko".to_string(),
                ],
            },
            DepartmentalGovernment {
                department_id: "QZ".to_string(),
                department_name: "Quetzaltenango".to_string(),
                capital_city: "Quetzaltenango".to_string(),
                governor: "Carlos Enrique Rodas Mejía".to_string(),
                population: 789358,
                area_km2: 1951.0,
                municipalities: vec![
                    Municipality {
                        municipality_name: "Quetzaltenango".to_string(),
                        mayor: "Juan Fernando López Fuentes".to_string(),
                        population: 180706,
                        municipal_code: "0901".to_string(),
                        municipal_ordinances: vec![],
                    },
                ],
                linguistic_communities: vec![
                    LinguisticCommunity {
                        community_name: "Comunidad K'iche'".to_string(),
                        language: "K'iche'".to_string(),
                        population: 394000,
                        cultural_practices: vec![
                            "Danza folklórica".to_string(),
                            "Cerámica tradicional".to_string(),
                            "Mercados ancestrales".to_string(),
                        ],
                    },
                    LinguisticCommunity {
                        community_name: "Comunidad Mam".to_string(),
                        language: "Mam".to_string(),
                        population: 158000,
                        cultural_practices: vec![
                            "Temazcales".to_string(),
                            "Bordados tradicionales".to_string(),
                            "Fiestas patronales".to_string(),
                        ],
                    },
                ],
                economic_activities: vec![
                    "Industria textil".to_string(),
                    "Agricultura".to_string(),
                    "Comercio".to_string(),
                    "Turismo".to_string(),
                ],
                indigenous_peoples: vec![
                    "K'iche'".to_string(),
                    "Mam".to_string(),
                    "Sipakapense".to_string(),
                ],
            },
            // Continue with remaining 19 departments...
        ]
    }

    fn build_national_government() -> NationalGovernment {
        NationalGovernment {
            executive_power: ExecutivePower {
                constitutional_basis: "Artículo 182 de la Constitución Política".to_string(),
                term_duration: "4 años".to_string(),
                election_method: "Elección directa, mayoría absoluta o segunda vuelta".to_string(),
                powers: vec![
                    "Ejecutar las leyes".to_string(),
                    "Dirigir la política exterior".to_string(),
                    "Nombrar y remover ministros".to_string(),
                    "Comandar las Fuerzas Armadas".to_string(),
                    "Convocar al Congreso a sesiones extraordinarias".to_string(),
                ],
            },
            legislative_power: LegislativePower {
                congress: Congress {
                    unicameral_system: true,
                    total_deputies: 160,
                    constitutional_basis: "Artículo 157 de la Constitución Política".to_string(),
                    legislative_functions: vec![
                        "Decretar, reformar y derogar las leyes".to_string(),
                        "Aprobar, modificar o improbar el Presupuesto General de Ingresos y Egresos del Estado".to_string(),
                        "Decretar impuestos ordinarios y extraordinarios, arbitrios y contribuciones especiales".to_string(),
                        "Aprobar antes de su ratificación los tratados, convenios o cualquier arreglo internacional".to_string(),
                        "Declarar la guerra y aprobar o improbar la paz".to_string(),
                    ],
                    current_composition: vec![
                        PartyComposition {
                            party_name: "Unidad Nacional de la Esperanza (UNE)".to_string(),
                            seats: 54,
                            percentage: 33.75,
                        },
                        PartyComposition {
                            party_name: "Vamos".to_string(),
                            seats: 44,
                            percentage: 27.5,
                        },
                        PartyComposition {
                            party_name: "Valor".to_string(),
                            seats: 8,
                            percentage: 5.0,
                        },
                    ],
                },
                legislative_procedures: vec![
                    LegislativeProcedure {
                        procedure_name: "Procedimiento legislativo ordinario".to_string(),
                        steps: vec![
                            "Iniciativa".to_string(),
                            "Admisión".to_string(),
                            "Estudio en comisión".to_string(),
                            "Dictamen".to_string(),
                            "Discusión en el pleno".to_string(),
                            "Votación".to_string(),
                            "Sanción".to_string(),
                            "Promulgación".to_string(),
                        ],
                        voting_requirements: "Mayoría absoluta del total de diputados".to_string(),
                        timeframes: vec![
                            "Comisión: 30 días".to_string(),
                            "Discusión: 15 días".to_string(),
                            "Sanción: 15 días".to_string(),
                        ],
                    },
                ],
                current_legislation: vec![
                    NationalLaw {
                        law_number: "Decreto 2-70".to_string(),
                        law_title: "Código Civil".to_string(),
                        promulgation_date: "1970-07-14".to_string(),
                        articles: vec![
                            LawArticle {
                                article_number: 1,
                                content: "La personalidad civil comienza con el nacimiento y termina con la muerte; sin embargo, al que está por nacer se le considera nacido para todo lo que le favorece, siempre que nazca en condiciones de viabilidad".to_string(),
                                compliance_obligations: vec![
                                    "Registro civil obligatorio".to_string(),
                                    "Protección del nasciturus".to_string(),
                                ],
                            },
                        ],
                        regulatory_scope: "Nacional".to_string(),
                    },
                ],
            },
            president: President {
                name: "Alejandro Eduardo Giammattei Falla".to_string(),
                term: "2020-2024".to_string(),
                party: "Vamos".to_string(),
                constitutional_powers: vec![
                    "Jefe de Estado y de Gobierno".to_string(),
                    "Comandante General del Ejército".to_string(),
                    "Representación del Estado".to_string(),
                    "Ejecución de políticas públicas".to_string(),
                ],
            },
            vice_president: VicePresident {
                name: "César Guillermo Castillo Reyes".to_string(),
                constitutional_role: "Coadyuvar con el Presidente en el cumplimiento de sus funciones".to_string(),
                succession_rights: vec![
                    "Sucesión automática en caso de muerte del Presidente".to_string(),
                    "Encargado del Ejecutivo en ausencias temporales".to_string(),
                ],
            },
            council_of_ministers: CouncilOfMinisters {
                ministers: vec![
                    Minister {
                        name: "Francisco José Jiménez Irungaray".to_string(),
                        ministry: "Ministerio de Gobernación".to_string(),
                        appointment_date: "2020-01-14".to_string(),
                        portfolio_responsibilities: vec![
                            "Seguridad ciudadana".to_string(),
                            "Orden público".to_string(),
                            "Migración".to_string(),
                        ],
                    },
                    Minister {
                        name: "Álvaro González-Ricci Diez".to_string(),
                        ministry: "Ministerio de Finanzas Públicas".to_string(),
                        appointment_date: "2020-01-14".to_string(),
                        portfolio_responsibilities: vec![
                            "Política fiscal".to_string(),
                            "Presupuesto público".to_string(),
                            "Deuda pública".to_string(),
                        ],
                    },
                ],
                collective_responsibility: "Los ministros son responsables de sus actos y solidariamente del Consejo de Ministros".to_string(),
            },
        }
    }

    fn build_judicial_system() -> JudicialSystem {
        JudicialSystem {
            supreme_court_of_justice: SupremeCourt {
                president: "Dra. Silvia Patricia Valdés Quezada".to_string(),
                magistrates: vec![
                    SupremeCourtMagistrate {
                        name: "Dra. Silvia Patricia Valdés Quezada".to_string(),
                        specialization: "Derecho Constitucional".to_string(),
                        appointment_date: "2019-10-13".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                    SupremeCourtMagistrate {
                        name: "Dr. Neftaly Aldana Herrera".to_string(),
                        specialization: "Derecho Penal".to_string(),
                        appointment_date: "2019-10-13".to_string(),
                        term_duration: "5 años".to_string(),
                    },
                ],
                chambers: vec![
                    JudicialChamber {
                        chamber_name: "Cámara Penal".to_string(),
                        specialization: "Derecho Penal".to_string(),
                        competencies: vec![
                            "Recursos de casación penal".to_string(),
                            "Conflictos de competencia".to_string(),
                            "Extradición".to_string(),
                            "Antejuicio".to_string(),
                        ],
                        magistrates: vec![
                            "Neftaly Aldana Herrera".to_string(),
                            "Ranulfo Rafael Rojas Cetina".to_string(),
                            "María Consuelo Porras Argueta".to_string(),
                        ],
                    },
                    JudicialChamber {
                        chamber_name: "Cámara Civil".to_string(),
                        specialization: "Derecho Civil".to_string(),
                        competencies: vec![
                            "Recursos de casación civil".to_string(),
                            "Conflictos de competencia civil".to_string(),
                            "Recursos de amparo".to_string(),
                        ],
                        magistrates: vec![
                            "Silvia Patricia Valdés Quezada".to_string(),
                            "José Arturo Sierra González".to_string(),
                            "Nester Mauricio Vásquez Pimentel".to_string(),
                        ],
                    },
                ],
                jurisdiction: "Nacional".to_string(),
                constitutional_competencies: vec![
                    "Función jurisdiccional suprema".to_string(),
                    "Superintendencia del Organismo Judicial".to_string(),
                    "Garantía constitucional".to_string(),
                ],
            },
            courts_of_appeals: vec![
                CourtOfAppeals {
                    court_name: "Sala de la Corte de Apelaciones del Ramo Civil de Guatemala".to_string(),
                    jurisdiction: "Guatemala".to_string(),
                    president: "Dr. César Augusto Conde Rada".to_string(),
                    judges: vec![
                        "Dr. César Augusto Conde Rada".to_string(),
                        "Dra. Claudia Regina Palomo de Lemus".to_string(),
                        "Dr. Mario René Molina Barrientos".to_string(),
                    ],
                    specializations: vec![
                        "Derecho Civil".to_string(),
                        "Derecho Mercantil".to_string(),
                        "Derecho de Familia".to_string(),
                    ],
                },
                CourtOfAppeals {
                    court_name: "Sala de la Corte de Apelaciones del Ramo Penal de Guatemala".to_string(),
                    jurisdiction: "Guatemala".to_string(),
                    president: "Dr. José Manuel Aldana Salguero".to_string(),
                    judges: vec![
                        "Dr. José Manuel Aldana Salguero".to_string(),
                        "Dra. Gladys Chacón Estrada".to_string(),
                        "Dr. Erick Danilo Santiago de León".to_string(),
                    ],
                    specializations: vec![
                        "Derecho Penal".to_string(),
                        "Derecho Procesal Penal".to_string(),
                    ],
                },
            ],
            first_instance_courts: vec![
                FirstInstanceCourt {
                    court_name: "Juzgado Primero de Primera Instancia Civil".to_string(),
                    jurisdiction: "Guatemala".to_string(),
                    judge: "Dr. Hugo Érick Granai Herrera".to_string(),
                    specialization: "Civil".to_string(),
                    competencies: vec![
                        "Juicios ordinarios civiles".to_string(),
                        "Juicios sumarios".to_string(),
                        "Procesos ejecutivos".to_string(),
                    ],
                },
            ],
            justices_of_peace: vec![
                JusticeOfPeace {
                    name: "Lic. María Elena Castañeda".to_string(),
                    jurisdiction: "Zona 1, Guatemala".to_string(),
                    competencies: vec![
                        "Faltas".to_string(),
                        "Conciliación".to_string(),
                        "Asuntos de menor cuantía".to_string(),
                    ],
                },
            ],
            specialized_courts: vec![
                SpecializedCourt {
                    court_name: "Tribunal de Femicidio y otras Formas de Violencia contra la Mujer".to_string(),
                    specialization: "Violencia de Género".to_string(),
                    jurisdiction: "Nacional".to_string(),
                    judges: vec![
                        "Dra. Yassmín Yolanda Barrios Aguilar".to_string(),
                        "Dra. Iris Yolanda Rosales Álvarez".to_string(),
                    ],
                    specific_competencies: vec![
                        "Delitos de femicidio".to_string(),
                        "Violencia contra la mujer".to_string(),
                        "Violencia sexual".to_string(),
                    ],
                },
                SpecializedCourt {
                    court_name: "Tribunal de Mayor Riesgo".to_string(),
                    specialization: "Crimen Organizado".to_string(),
                    jurisdiction: "Nacional".to_string(),
                    judges: vec![
                        "Dr. Miguel Ángel Gálvez Salazar".to_string(),
                        "Dra. Claudette Domínguez Aguilar".to_string(),
                    ],
                    specific_competencies: vec![
                        "Crimen organizado".to_string(),
                        "Narcotráfico".to_string(),
                        "Lavado de dinero".to_string(),
                        "Corrupción".to_string(),
                    ],
                },
            ],
        }
    }

    fn build_electoral_system() -> ElectoralSystem {
        ElectoralSystem {
            supreme_electoral_tribunal: SupremeElectoralTribunal {
                president: "Dr. Irma Elizabeth Palencia Orellana".to_string(),
                magistrates: vec![
                    "Dr. Irma Elizabeth Palencia Orellana".to_string(),
                    "Dr. José Rodolfo Cofiño Morales".to_string(),
                    "Dr. Gabriel Aguilera Peralta".to_string(),
                    "Dr. Mynor Franco Flores".to_string(),
                    "Dr. Julio Ramón Echeverría Vallejo".to_string(),
                ],
                electoral_functions: vec![
                    "Organización de procesos electorales".to_string(),
                    "Fiscalización de partidos políticos".to_string(),
                    "Escrutinio de votos".to_string(),
                    "Declaración oficial de resultados".to_string(),
                    "Registro de ciudadanos".to_string(),
                ],
                current_term: "2019-2025".to_string(),
            },
            departmental_electoral_boards: vec![
                DepartmentalElectoralBoard {
                    department: "Guatemala".to_string(),
                    president: "Lic. Ana Lucia Reyes de Méndez".to_string(),
                    members: vec![
                        "Lic. Ana Lucia Reyes de Méndez".to_string(),
                        "Lic. Jorge Luis Donado Vielman".to_string(),
                        "Lic. Manuel Ruben Lemus Morales".to_string(),
                    ],
                    electoral_jurisdiction: "22 municipios".to_string(),
                },
            ],
            electoral_laws: vec![
                ElectoralLaw {
                    law_number: "Decreto 1-85".to_string(),
                    law_title: "Ley Electoral y de Partidos Políticos".to_string(),
                    promulgation_date: "1985-12-18".to_string(),
                    key_provisions: vec![
                        "Sistema electoral proporcional".to_string(),
                        "Voto universal y secreto".to_string(),
                        "Financiamiento público de campañas".to_string(),
                        "Registro de partidos políticos".to_string(),
                        "Fiscalización electoral".to_string(),
                    ],
                },
            ],
            political_parties: vec![
                PoliticalParty {
                    party_name: "Unidad Nacional de la Esperanza (UNE)".to_string(),
                    registration_date: "2002-04-12".to_string(),
                    ideology: "Socialdemócrata".to_string(),
                    leader: "Sandra Torres".to_string(),
                    electoral_representation: ElectoralRepresentation {
                        congress_seats: 54,
                        departmental_governments: 8,
                        municipal_governments: 89,
                        presidential_votes: Some(1651165),
                    },
                },
                PoliticalParty {
                    party_name: "Vamos".to_string(),
                    registration_date: "2017-01-20".to_string(),
                    ideology: "Centro-derecha".to_string(),
                    leader: "Alejandro Giammattei".to_string(),
                    electoral_representation: ElectoralRepresentation {
                        congress_seats: 44,
                        departmental_governments: 5,
                        municipal_governments: 67,
                        presidential_votes: Some(1854718),
                    },
                },
            ],
            electoral_procedures: vec![
                ElectoralProcedure {
                    procedure_type: "Elecciones Generales".to_string(),
                    requirements: vec![
                        "Ciudadanía guatemalteca".to_string(),
                        "18 años cumplidos".to_string(),
                        "Estar inscrito en el Registro de Ciudadanos".to_string(),
                        "Estar en goce de sus derechos civiles y políticos".to_string(),
                    ],
                    timeline: "Cada 4 años - primer domingo de septiembre".to_string(),
                    oversight_mechanisms: vec![
                        "Tribunal Supremo Electoral".to_string(),
                        "Juntas Electorales Departamentales".to_string(),
                        "Juntas Electorales Municipales".to_string(),
                        "Fiscales de partidos políticos".to_string(),
                        "Observadores nacionales e internacionales".to_string(),
                    ],
                },
            ],
            voting_rights: VotingRights {
                eligibility_requirements: vec![
                    "Nacionalidad guatemalteca".to_string(),
                    "Mayoría de edad (18 años)".to_string(),
                    "Capacidad civil".to_string(),
                    "Inscripción en el padrón electoral".to_string(),
                ],
                protected_categories: vec![
                    "Personas con discapacidad".to_string(),
                    "Adultos mayores".to_string(),
                    "Mujeres embarazadas".to_string(),
                    "Personas privadas de libertad (en ciertos casos)".to_string(),
                ],
                accessibility_provisions: vec![
                    "Centros de votación accesibles".to_string(),
                    "Papeletas en braille".to_string(),
                    "Asistencia para el voto".to_string(),
                    "Intérpretes de lenguas mayas".to_string(),
                ],
            },
        }
    }

    fn build_legal_codes() -> LegalCodes {
        LegalCodes {
            civil_code: CivilCode {
                code_title: "Código Civil".to_string(),
                promulgation_date: "1973-07-14".to_string(),
                total_articles: 2180,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Personalidad civil".to_string(),
                        spanish_text: "La personalidad civil comienza con el nacimiento y termina con la muerte; sin embargo, al que está por nacer se le considera nacido para todo lo que le favorece, siempre que nazca en condiciones de viabilidad.".to_string(),
                        english_translation: "Civil personality begins with birth and ends with death; however, the unborn is considered born for everything that favors him, provided he is born under viable conditions.".to_string(),
                        compliance_obligations: vec![
                            "Registro civil obligatorio".to_string(),
                            "Protección del nasciturus".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Decreto 46-2002".to_string(),
                        reform_date: "2002-11-07".to_string(),
                        articles_modified: vec![173, 174, 175],
                        reform_purpose: "Ley de Desarrollo Social".to_string(),
                    },
                ],
            },
            criminal_code: CriminalCode {
                code_title: "Código Penal".to_string(),
                promulgation_date: "1973-07-17".to_string(),
                total_articles: 476,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Principio de legalidad".to_string(),
                        spanish_text: "Nadie podrá ser penado por hechos que no estén expresamente calificados, por ley anterior a su perpetración, como delito o falta, ni sancionado con pena que no esté establecida por ley anterior.".to_string(),
                        english_translation: "No one may be punished for acts that are not expressly classified, by law prior to their perpetration, as a crime or misdemeanor, nor sanctioned with a penalty that is not established by prior law.".to_string(),
                        compliance_obligations: vec![
                            "Aplicación estricta de la legalidad penal".to_string(),
                            "No retroactividad de la ley penal".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Decreto 9-2009".to_string(),
                        reform_date: "2009-05-08".to_string(),
                        articles_modified: vec![200, 201, 202],
                        reform_purpose: "Ley contra el Femicidio y otras Formas de Violencia contra la Mujer".to_string(),
                    },
                ],
            },
            commercial_code: CommercialCode {
                code_title: "Código de Comercio".to_string(),
                promulgation_date: "1970-01-28".to_string(),
                total_articles: 1024,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Actos de comercio".to_string(),
                        spanish_text: "Los comerciantes en sus operaciones mercantiles y los no comerciantes en las suyas que tengan carácter mercantil, se regirán por las disposiciones de este Código y en su defecto por las del Derecho Civil que se aplicarán e interpretarán de conformidad con los principios que inspiran el Derecho Mercantil.".to_string(),
                        english_translation: "Merchants in their commercial operations and non-merchants in their operations that have a commercial character, shall be governed by the provisions of this Code and, failing that, by those of Civil Law that shall be applied and interpreted in accordance with the principles that inspire Commercial Law.".to_string(),
                        compliance_obligations: vec![
                            "Registro mercantil obligatorio".to_string(),
                            "Aplicación de principios mercantiles".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![],
            },
            labor_code: LaborCode {
                code_title: "Código de Trabajo".to_string(),
                promulgation_date: "1961-04-18".to_string(),
                total_articles: 433,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Objeto del Código".to_string(),
                        spanish_text: "El presente Código regula los derechos y obligaciones de patronos y trabajadores, con ocasión del trabajo, y crea instituciones para resolver sus conflictos.".to_string(),
                        english_translation: "This Code regulates the rights and obligations of employers and workers, on the occasion of work, and creates institutions to resolve their conflicts.".to_string(),
                        compliance_obligations: vec![
                            "Cumplimiento de derechos laborales".to_string(),
                            "Instituciones de resolución de conflictos".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![
                    CodeReform {
                        reform_law: "Decreto 78-89".to_string(),
                        reform_date: "1989-11-24".to_string(),
                        articles_modified: vec![102, 103],
                        reform_purpose: "Ley de Bonificación Anual para Trabajadores del Sector Privado y Público".to_string(),
                    },
                ],
            },
            administrative_code: AdministrativeCode {
                code_title: "Ley del Organismo Judicial".to_string(),
                promulgation_date: "1989-03-10".to_string(),
                total_articles: 626,
                key_articles: vec![
                    CodeArticle {
                        article_number: 1,
                        article_title: "Fuentes del derecho".to_string(),
                        spanish_text: "La ley es la fuente del ordenamiento jurídico. La costumbre regirá sólo en defecto de ley aplicable o por delegación de la ley, siempre que no sea contraria a la moral o al orden público y que resulte probada.".to_string(),
                        english_translation: "Law is the source of the legal order. Custom shall govern only in the absence of applicable law or by delegation of law, provided it is not contrary to morality or public order and is proven.".to_string(),
                        compliance_obligations: vec![
                            "Supremacía de la ley".to_string(),
                            "Aplicación subsidiaria de la costumbre".to_string(),
                        ],
                        penalties: vec![],
                    },
                ],
                recent_reforms: vec![],
            },
        }
    }

    fn build_indigenous_jurisdiction() -> IndigenousJurisdiction {
        IndigenousJurisdiction {
            constitutional_recognition: "Artículo 66 de la Constitución Política".to_string(),
            indigenous_peoples: vec![
                IndigenousPeople {
                    people_name: "Maya K'iche'".to_string(),
                    language: "K'iche'".to_string(),
                    population: 1270953,
                    territory: "Altiplano occidental".to_string(),
                    traditional_authorities: vec![
                        "Principales".to_string(),
                        "Cofrades".to_string(),
                        "Alcaldes auxiliares".to_string(),
                    ],
                },
                IndigenousPeople {
                    people_name: "Maya Kaqchikel".to_string(),
                    language: "Kaqchikel".to_string(),
                    population: 832968,
                    territory: "Altiplano central".to_string(),
                    traditional_authorities: vec![
                        "Principales".to_string(),
                        "Cofradías".to_string(),
                        "Consejos de ancianos".to_string(),
                    ],
                },
                IndigenousPeople {
                    people_name: "Maya Mam".to_string(),
                    language: "Mam".to_string(),
                    population: 617171,
                    territory: "Sierra Madre de Chiapas".to_string(),
                    traditional_authorities: vec![
                        "Principales".to_string(),
                        "Alcaldes rezadores".to_string(),
                        "Cofrades".to_string(),
                    ],
                },
                IndigenousPeople {
                    people_name: "Maya Q'eqchi'".to_string(),
                    language: "Q'eqchi'".to_string(),
                    population: 852012,
                    territory: "Tierras bajas del norte".to_string(),
                    traditional_authorities: vec![
                        "Principales".to_string(),
                        "Alcaldes de vara".to_string(),
                        "Sacerdotes mayas".to_string(),
                    ],
                },
            ],
            customary_law_systems: vec![
                CustomaryLawSystem {
                    law_system_name: "Derecho Consuetudinario K'iche'".to_string(),
                    people: "Maya K'iche'".to_string(),
                    principles: vec![
                        "Complementariedad".to_string(),
                        "Equilibrio".to_string(),
                        "Armonía con la naturaleza".to_string(),
                        "Consenso comunitario".to_string(),
                    ],
                    dispute_resolution: vec![
                        "Asamblea comunitaria".to_string(),
                        "Mediación de principales".to_string(),
                        "Ceremonias de purificación".to_string(),
                        "Reparación del daño".to_string(),
                    ],
                    traditional_sanctions: vec![
                        "Trabajo comunitario".to_string(),
                        "Compensación económica".to_string(),
                        "Ceremonia de perdón".to_string(),
                        "Expulsión temporal".to_string(),
                    ],
                },
                CustomaryLawSystem {
                    law_system_name: "Derecho Consuetudinario Mam".to_string(),
                    people: "Maya Mam".to_string(),
                    principles: vec![
                        "Respeto a los mayores".to_string(),
                        "Solidaridad comunitaria".to_string(),
                        "Reciprocidad".to_string(),
                        "Espiritualidad maya".to_string(),
                    ],
                    dispute_resolution: vec![
                        "Consejo de principales".to_string(),
                        "Rituales de sanación".to_string(),
                        "Mediación familiar".to_string(),
                        "Intervención de cofradías".to_string(),
                    ],
                    traditional_sanctions: vec![
                        "Servicio a la comunidad".to_string(),
                        "Participación en cofradías".to_string(),
                        "Ceremonia de reconciliación".to_string(),
                        "Restitución de bienes".to_string(),
                    ],
                },
            ],
            coordination_mechanisms: vec![
                "Coordinación con juzgados de paz".to_string(),
                "Remisión de casos complejos".to_string(),
                "Reconocimiento de decisiones comunitarias".to_string(),
                "Capacitación intercultural".to_string(),
            ],
        }
    }

    fn build_api_integrations() -> Vec<APIIntegration> {
        vec![
            APIIntegration {
                institution_name: "Tribunal Supremo Electoral".to_string(),
                api_endpoint: "https://www.tse.org.gt/api".to_string(),
                update_frequency: "Real-time".to_string(),
                data_types: vec![
                    "Resultados electorales".to_string(),
                    "Registro de partidos políticos".to_string(),
                    "Padrón electoral".to_string(),
                    "Fiscalización de campañas".to_string(),
                ],
                authentication_method: "API Key".to_string(),
            },
            APIIntegration {
                institution_name: "Corte Suprema de Justicia".to_string(),
                api_endpoint: "https://www.oj.gob.gt/api".to_string(),
                update_frequency: "Daily".to_string(),
                data_types: vec![
                    "Jurisprudencia".to_string(),
                    "Sentencias".to_string(),
                    "Audiencias programadas".to_string(),
                    "Estadísticas judiciales".to_string(),
                ],
                authentication_method: "OAuth 2.0".to_string(),
            },
            APIIntegration {
                institution_name: "Congreso de la República".to_string(),
                api_endpoint: "https://www.congreso.gob.gt/api".to_string(),
                update_frequency: "Real-time".to_string(),
                data_types: vec![
                    "Iniciativas de ley".to_string(),
                    "Decretos aprobados".to_string(),
                    "Sesiones plenarias".to_string(),
                    "Votaciones".to_string(),
                ],
                authentication_method: "Bearer Token".to_string(),
            },
        ]
    }

    fn build_compliance_monitoring() -> ComplianceMonitoring {
        ComplianceMonitoring {
            monitoring_entities: vec![
                "Contraloría General de Cuentas".to_string(),
                "Ministerio Público".to_string(),
                "Procurador de los Derechos Humanos".to_string(),
                "Comisión Internacional contra la Impunidad en Guatemala (CICIG)".to_string(),
            ],
            compliance_indicators: vec![
                "Transparencia gubernamental".to_string(),
                "Acceso a la información pública".to_string(),
                "Rendición de cuentas".to_string(),
                "Respeto a derechos humanos".to_string(),
                "Lucha contra la corrupción".to_string(),
            ],
            reporting_mechanisms: vec![
                "Informes anuales de gestión".to_string(),
                "Portal de transparencia".to_string(),
                "Audiencias públicas".to_string(),
                "Sistema de denuncias ciudadanas".to_string(),
                "Informes internacionales".to_string(),
            ],
            enforcement_procedures: vec![
                "Procedimientos administrativos sancionadores".to_string(),
                "Procesos judiciales".to_string(),
                "Medidas cautelares".to_string(),
                "Reparación integral del daño".to_string(),
                "Investigaciones especiales".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guatemala_legal_system_creation() {
        let guatemala_system = GuatemalaLegalSystem::new();
        assert_eq!(guatemala_system.constitutional_framework.constitution_1985.total_articles, 281);
        assert_eq!(guatemala_system.departmental_governments.len(), 3); // Sample departments
        assert!(guatemala_system.departmental_governments.iter().any(|d| d.department_name == "Guatemala"));
    }

    #[test]
    fn test_constitutional_articles() {
        let guatemala_system = GuatemalaLegalSystem::new();
        let article_1 = &guatemala_system.constitutional_framework.constitutional_articles[0];
        assert_eq!(article_1.article_number, 1);
        assert!(article_1.spanish_text.contains("proteger a la persona"));
        assert!(article_1.english_translation.contains("protect the person"));
    }

    #[test]
    fn test_indigenous_recognition() {
        let guatemala_system = GuatemalaLegalSystem::new();
        let article_66 = &guatemala_system.constitutional_framework.constitutional_articles[2];
        assert_eq!(article_66.article_number, 66);
        assert!(article_66.spanish_text.contains("grupos étnicos"));
        assert!(article_66.mayan_text.is_some());
    }

    #[test]
    fn test_indigenous_jurisdiction() {
        let guatemala_system = GuatemalaLegalSystem::new();
        assert!(!guatemala_system.indigenous_jurisdiction.indigenous_peoples.is_empty());
        assert!(guatemala_system.indigenous_jurisdiction.indigenous_peoples.iter().any(|p| p.people_name == "Maya K'iche'"));
    }
}