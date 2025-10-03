use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLegalSystem {
    pub constitutional_framework: SpainConstitutionalFramework,
    pub government_structure: SpainGovernmentStructure,
    pub territorial_organization: SpainTerritorialOrganization,
    pub judicial_system: SpainJudicialSystem,
    pub legal_codes: SpainLegalCodes,
    pub human_rights: SpainHumanRights,
    pub eu_integration: SpainEUIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalFramework {
    pub constitution_1978: SpainConstitution1978,
    pub constitutional_principles: SpainConstitutionalPrinciples,
    pub fundamental_rights: SpainFundamentalRights,
    pub constitutional_amendments: Vec<SpainConstitutionalAmendment>,
    pub constitutional_court_decisions: Vec<SpainConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitution1978 {
    pub preamble: String,
    pub preliminary_title: Vec<SpainConstitutionalArticle>,
    pub title_one_fundamental_rights: Vec<SpainConstitutionalArticle>,
    pub title_two_crown: Vec<SpainConstitutionalArticle>,
    pub title_three_cortes_generales: Vec<SpainConstitutionalArticle>,
    pub title_four_government_administration: Vec<SpainConstitutionalArticle>,
    pub title_eight_territorial_organization: Vec<SpainConstitutionalArticle>,
    pub title_nine_constitutional_court: Vec<SpainConstitutionalArticle>,
    pub adoption_date: String,
    pub referendum_date: String,
    pub ratification_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub interpretation_notes: Vec<String>,
    pub related_legislation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalPrinciples {
    pub constitutional_monarchy: String,
    pub democratic_state: String,
    pub rule_of_law: String,
    pub territorial_autonomy: String,
    pub political_pluralism: String,
    pub national_sovereignty: String,
    pub separation_powers: String,
    pub social_market_economy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainFundamentalRights {
    pub individual_rights: Vec<String>,
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub economic_social_rights: Vec<String>,
    pub cultural_linguistic_rights: Vec<String>,
    pub environmental_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainGovernmentStructure {
    pub executive_branch: SpainExecutiveBranch,
    pub legislative_branch: SpainLegislativeBranch,
    pub head_of_state: SpainHeadOfState,
    pub council_ministers: SpainCouncilMinisters,
    pub administrative_system: SpainAdministrativeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainExecutiveBranch {
    pub prime_minister: SpainPrimeMinister,
    pub deputy_prime_ministers: Vec<String>,
    pub ministers: Vec<SpainMinister>,
    pub secretaries_state: Vec<String>,
    pub government_formation: String,
    pub confidence_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainPrimeMinister {
    pub current_holder: String,
    pub appointment_process: String,
    pub powers_responsibilities: Vec<String>,
    pub term_duration: String,
    pub removal_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainMinister {
    pub ministry_name: String,
    pub current_minister: String,
    pub responsibilities: Vec<String>,
    pub budget_allocation: String,
    pub staff_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLegislativeBranch {
    pub cortes_generales: SpainCortesGenerales,
    pub congress_deputies: SpainCongressDeputies,
    pub senate: SpainSenate,
    pub legislative_process: SpainLegislativeProcess,
    pub parliamentary_groups: Vec<SpainParliamentaryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCortesGenerales {
    pub bicameral_system: String,
    pub session_duration: String,
    pub dissolution_rules: String,
    pub immunities_privileges: Vec<String>,
    pub current_legislature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCongressDeputies {
    pub total_seats: u32,
    pub current_composition: Vec<SpainPoliticalGroup>,
    pub speaker: String,
    pub electoral_system: String,
    pub term_length: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainSenate {
    pub total_seats: u32,
    pub current_composition: Vec<SpainPoliticalGroup>,
    pub president: String,
    pub electoral_system: String,
    pub territorial_representation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainPoliticalGroup {
    pub party_name: String,
    pub seats_count: u32,
    pub leader: String,
    pub political_orientation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainHeadOfState {
    pub king: SpainKing,
    pub powers: Vec<String>,
    pub ceremonial_functions: Vec<String>,
    pub constitutional_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainKing {
    pub current_king: String,
    pub succession_law: String,
    pub title: String,
    pub residence: String,
    pub constitutional_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainTerritorialOrganization {
    pub autonomous_communities: Vec<SpainAutonomousCommunity>,
    pub provinces: Vec<SpainProvince>,
    pub municipalities: Vec<SpainMunicipality>,
    pub autonomous_cities: Vec<SpainAutonomousCity>,
    pub territorial_cooperation: SpainTerritorialCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainAutonomousCommunity {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub area_km2: f64,
    pub gdp_euros: u64,
    pub autonomy_statute: String,
    pub regional_parliament: SpainRegionalParliament,
    pub regional_government: SpainRegionalGovernment,
    pub competencies: Vec<String>,
    pub co_official_languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainRegionalParliament {
    pub name: String,
    pub seats: u32,
    pub speaker: String,
    pub political_composition: Vec<SpainPoliticalGroup>,
    pub term_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainRegionalGovernment {
    pub president: String,
    pub council_government: Vec<String>,
    pub budget_euros: u64,
    pub executive_competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainProvince {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub autonomous_community: String,
    pub provincial_council: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainMunicipality {
    pub name: String,
    pub province: String,
    pub population: u64,
    pub mayor: String,
    pub municipal_council_seats: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainAutonomousCity {
    pub name: String,
    pub population: u64,
    pub president: String,
    pub special_statute: String,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainJudicialSystem {
    pub supreme_court: SpainSupremeCourt,
    pub constitutional_court: SpainConstitutionalCourt,
    pub general_council_judiciary: SpainGeneralCouncilJudiciary,
    pub national_court: SpainNationalCourt,
    pub ordinary_jurisdiction: SpainOrdinaryJurisdiction,
    pub specialized_jurisdictions: SpainSpecializedJurisdictions,
    pub prosecution_system: SpainProsecutionSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainSupremeCourt {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub total_judges: u32,
    pub chambers: Vec<SpainSupremeCourtChamber>,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainSupremeCourtChamber {
    pub chamber_name: String,
    pub president: String,
    pub specialization: String,
    pub judges_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalCourt {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub total_judges: u32,
    pub appointment_process: String,
    pub jurisdiction: Vec<String>,
    pub landmark_decisions: Vec<SpainConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalCourtDecision {
    pub decision_number: String,
    pub year: u32,
    pub case_title: String,
    pub constitutional_issue: String,
    pub ruling: String,
    pub legal_principle: String,
    pub dissenting_opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLegalCodes {
    pub civil_code: SpainCivilCode,
    pub criminal_code: SpainCriminalCode,
    pub commercial_code: SpainCommercialCode,
    pub labor_code: SpainLaborCode,
    pub administrative_code: SpainAdministrativeCode,
    pub tax_code: SpainTaxCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCivilCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<SpainCodeBook>,
    pub total_articles: u32,
    pub major_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCodeBook {
    pub book_number: u32,
    pub title: String,
    pub articles_range: String,
    pub main_topics: Vec<String>,
    pub key_articles: Vec<SpainCodeArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCodeArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub amendments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCriminalCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<SpainCodeBook>,
    pub total_articles: u32,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalAmendment {
    pub amendment_number: u32,
    pub year: u32,
    pub title: String,
    pub articles_modified: Vec<u32>,
    pub approval_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainHumanRights {
    pub constitutional_guarantees: Vec<String>,
    pub international_treaties: Vec<String>,
    pub ombudsman_system: SpainOmbudsmanSystem,
    pub anti_discrimination: Vec<String>,
    pub data_protection: SpainDataProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainOmbudsmanSystem {
    pub defensor_pueblo: String,
    pub regional_ombudsmen: Vec<String>,
    pub competencies: Vec<String>,
    pub reporting_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainDataProtection {
    pub authority: String,
    pub legal_framework: Vec<String>,
    pub individual_rights: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainEUIntegration {
    pub membership_date: String,
    pub euro_adoption: String,
    pub schengen_participation: String,
    pub eu_institutions_representation: SpainEURepresentation,
    pub european_law_implementation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainEURepresentation {
    pub european_parliament_seats: u32,
    pub council_votes: u32,
    pub commissioners: Vec<String>,
    pub permanent_representative: String,
}

impl Default for SpainLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SpainConstitutionalFramework {
                constitution_1978: SpainConstitution1978 {
                    preamble: "La Nación española, deseando establecer la justicia, la libertad y la seguridad y promover el bien de cuantos la integran, en uso de su soberanía, proclama su voluntad de: Garantizar la convivencia democrática dentro de la Constitución y de las leyes conforme a un orden económico y social justo".to_string(),
                    preliminary_title: vec![
                        SpainConstitutionalArticle {
                            article_number: 1,
                            title: "Estado social y democrático de Derecho".to_string(),
                            content: "España se constituye en un Estado social y democrático de Derecho, que propugna como valores superiores de su ordenamiento jurídico la libertad, la justicia, la igualdad y el pluralismo político.".to_string(),
                            interpretation_notes: vec!["Cláusula de Estado social".to_string(), "Valores superiores del ordenamiento".to_string()],
                            related_legislation: vec!["Ley Orgánica del Tribunal Constitucional".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 2,
                            title: "Unidad de España y autonomías".to_string(),
                            content: "La Constitución se fundamenta en la indisoluble unidad de la Nación española, patria común e indivisible de todos los españoles, y reconoce y garantiza el derecho a la autonomía de las nacionalidades y regiones que la integran y la solidaridad entre todas ellas.".to_string(),
                            interpretation_notes: vec!["Principio de unidad".to_string(), "Derecho a la autonomía".to_string()],
                            related_legislation: vec!["Estatutos de Autonomía".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 3,
                            title: "Lenguas oficiales".to_string(),
                            content: "El castellano es la lengua española oficial del Estado. Todos los españoles tienen el deber de conocerla y el derecho a usarla. Las demás lenguas españolas serán también oficiales en las respectivas Comunidades Autónomas de acuerdo con sus Estatutos.".to_string(),
                            interpretation_notes: vec!["Cooficialidad lingüística".to_string(), "Diversidad cultural".to_string()],
                            related_legislation: vec!["Leyes de normalización lingüística".to_string()],
                        },
                    ],
                    title_one_fundamental_rights: vec![
                        SpainConstitutionalArticle {
                            article_number: 14,
                            title: "Igualdad ante la ley".to_string(),
                            content: "Los españoles son iguales ante la ley, sin que pueda prevalecer discriminación alguna por razón de nacimiento, raza, sexo, religión, opinión o cualquier otra condición o circunstancia personal o social.".to_string(),
                            interpretation_notes: vec!["Principio general de igualdad".to_string(), "Prohibición de discriminación".to_string()],
                            related_legislation: vec!["Ley de Igualdad Efectiva".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 15,
                            title: "Derecho a la vida".to_string(),
                            content: "Todos tienen derecho a la vida y a la integridad física y moral, sin que, en ningún caso, puedan ser sometidos a tortura ni a penas o tratos inhumanos o degradantes. Queda abolida la pena de muerte, salvo lo que puedan disponer las leyes penales militares para tiempos de guerra.".to_string(),
                            interpretation_notes: vec!["Abolición pena de muerte".to_string(), "Prohibición tortura".to_string()],
                            related_legislation: vec!["Código Penal".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 16,
                            title: "Libertad religiosa".to_string(),
                            content: "Se garantiza la libertad ideológica, religiosa y de culto de los individuos y las comunidades sin más limitación, en sus manifestaciones, que la necesaria para el mantenimiento del orden público protegido por la ley.".to_string(),
                            interpretation_notes: vec!["Libertad de conciencia".to_string(), "Aconfesionalidad del Estado".to_string()],
                            related_legislation: vec!["Ley Orgánica de Libertad Religiosa".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 18,
                            title: "Honor, intimidad e imagen".to_string(),
                            content: "Se garantiza el derecho al honor, a la intimidad personal y familiar y a la propia imagen.".to_string(),
                            interpretation_notes: vec!["Derechos de la personalidad".to_string(), "Protección datos personales".to_string()],
                            related_legislation: vec!["Ley Orgánica de Protección de Datos".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 20,
                            title: "Libertad de expresión e información".to_string(),
                            content: "Se reconocen y protegen los derechos: a expresar y difundir libremente los pensamientos, ideas y opiniones mediante la palabra, el escrito o cualquier otro medio de reproducción.".to_string(),
                            interpretation_notes: vec!["Libertad de prensa".to_string(), "Derecho a la información".to_string()],
                            related_legislation: vec!["Ley de Prensa e Imprenta".to_string()],
                        },
                    ],
                    title_two_crown: vec![
                        SpainConstitutionalArticle {
                            article_number: 56,
                            title: "Funciones del Rey".to_string(),
                            content: "El Rey es el Jefe del Estado, símbolo de su unidad y permanencia, arbitra y modera el funcionamiento regular de las instituciones, asume la más alta representación del Estado español en las relaciones internacionales".to_string(),
                            interpretation_notes: vec!["Monarquía parlamentaria".to_string(), "Funciones simbólicas".to_string()],
                            related_legislation: vec!["Ley de la Casa Real".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 57,
                            title: "Sucesión en la Corona".to_string(),
                            content: "La Corona de España es hereditaria en los sucesores de S. M. Don Juan Carlos I de Borbón, legítimo heredero de la dinastía histórica.".to_string(),
                            interpretation_notes: vec!["Sucesión hereditaria".to_string(), "Orden de sucesión".to_string()],
                            related_legislation: vec!["Ley de Sucesión".to_string()],
                        },
                    ],
                    title_three_cortes_generales: vec![
                        SpainConstitutionalArticle {
                            article_number: 66,
                            title: "Cortes Generales".to_string(),
                            content: "Las Cortes Generales representan al pueblo español y están formadas por el Congreso de los Diputados y el Senado.".to_string(),
                            interpretation_notes: vec!["Bicameralismo".to_string(), "Representación popular".to_string()],
                            related_legislation: vec!["Reglamentos de las Cámaras".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 68,
                            title: "Congreso de los Diputados".to_string(),
                            content: "El Congreso se compone de un mínimo de 300 y un máximo de 400 Diputados, elegidos por sufragio universal, libre, igual, directo y secreto".to_string(),
                            interpretation_notes: vec!["Cámara baja".to_string(), "Elección directa".to_string()],
                            related_legislation: vec!["Ley Orgánica del Régimen Electoral General".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 69,
                            title: "Senado".to_string(),
                            content: "El Senado es la Cámara de representación territorial.".to_string(),
                            interpretation_notes: vec!["Cámara alta".to_string(), "Representación territorial".to_string()],
                            related_legislation: vec!["Ley Electoral".to_string()],
                        },
                    ],
                    title_four_government_administration: vec![
                        SpainConstitutionalArticle {
                            article_number: 97,
                            title: "Gobierno".to_string(),
                            content: "El Gobierno dirige la política interior y exterior, la Administración civil y militar y la defensa del Estado. Ejerce la función ejecutiva y la potestad reglamentaria de acuerdo con la Constitución y las leyes.".to_string(),
                            interpretation_notes: vec!["Función ejecutiva".to_string(), "Potestad reglamentaria".to_string()],
                            related_legislation: vec!["Ley del Gobierno".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 98,
                            title: "Composición del Gobierno".to_string(),
                            content: "El Gobierno se compone del Presidente, de los Vicepresidentes en su caso, de los Ministros y de los demás miembros que establezca la ley.".to_string(),
                            interpretation_notes: vec!["Estructura gubernamental".to_string()],
                            related_legislation: vec!["Ley de Organización del Gobierno".to_string()],
                        },
                    ],
                    title_eight_territorial_organization: vec![
                        SpainConstitutionalArticle {
                            article_number: 137,
                            title: "Organización territorial".to_string(),
                            content: "El Estado se organiza territorialmente en municipios, en provincias y en las Comunidades Autónomas que se constituyan. Todas estas entidades gozan de autonomía para la gestión de sus respectivos intereses.".to_string(),
                            interpretation_notes: vec!["Estado autonómico".to_string(), "Autonomía local".to_string()],
                            related_legislation: vec!["Estatutos de Autonomía".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 143,
                            title: "Constitución de Comunidades Autónomas".to_string(),
                            content: "En el ejercicio del derecho a la autonomía reconocido en el artículo 2 de la Constitución, las provincias limítrofes con características históricas, culturales y económicas comunes, los territorios insulares y las provincias con entidad regional histórica podrán acceder a su autogobierno".to_string(),
                            interpretation_notes: vec!["Proceso autonómico".to_string(), "Criterios constitución".to_string()],
                            related_legislation: vec!["Leyes Orgánicas de transferencias".to_string()],
                        },
                    ],
                    title_nine_constitutional_court: vec![
                        SpainConstitutionalArticle {
                            article_number: 159,
                            title: "Tribunal Constitucional".to_string(),
                            content: "El Tribunal Constitucional se compone de 12 miembros nombrados por el Rey; de ellos, cuatro a propuesta del Congreso por mayoría de tres quintos de sus miembros; cuatro a propuesta del Senado, con idéntica mayoría".to_string(),
                            interpretation_notes: vec!["Composición TC".to_string(), "Nombramiento magistrados".to_string()],
                            related_legislation: vec!["Ley Orgánica del Tribunal Constitucional".to_string()],
                        },
                        SpainConstitutionalArticle {
                            article_number: 161,
                            title: "Competencias del Tribunal Constitucional".to_string(),
                            content: "El Tribunal Constitucional tiene jurisdicción en todo el territorio español y es competente para conocer: del recurso de inconstitucionalidad contra leyes y disposiciones normativas con fuerza de ley".to_string(),
                            interpretation_notes: vec!["Control constitucionalidad".to_string(), "Jurisdicción nacional".to_string()],
                            related_legislation: vec!["LOTC".to_string()],
                        },
                    ],
                    adoption_date: "31 de octubre de 1978".to_string(),
                    referendum_date: "6 de diciembre de 1978".to_string(),
                    ratification_date: "27 de diciembre de 1978".to_string(),
                },
                constitutional_principles: SpainConstitutionalPrinciples {
                    constitutional_monarchy: "España se constituye en una Monarquía parlamentaria".to_string(),
                    democratic_state: "Estado social y democrático de Derecho".to_string(),
                    rule_of_law: "Imperio de la ley y jerarquía normativa".to_string(),
                    territorial_autonomy: "Estado autonómico descentralizado".to_string(),
                    political_pluralism: "Pluralismo político como valor superior".to_string(),
                    national_sovereignty: "La soberanía nacional reside en el pueblo español".to_string(),
                    separation_powers: "División de poderes legislativo, ejecutivo y judicial".to_string(),
                    social_market_economy: "Economía social de mercado".to_string(),
                },
                fundamental_rights: SpainFundamentalRights {
                    individual_rights: vec![
                        "Derecho a la vida (Art. 15)".to_string(),
                        "Libertad ideológica y religiosa (Art. 16)".to_string(),
                        "Derecho a la libertad y seguridad (Art. 17)".to_string(),
                        "Derecho al honor, intimidad e imagen (Art. 18)".to_string(),
                        "Libertad de residencia y circulación (Art. 19)".to_string(),
                    ],
                    civil_rights: vec![
                        "Libertad de expresión e información (Art. 20)".to_string(),
                        "Derecho de reunión (Art. 21)".to_string(),
                        "Derecho de asociación (Art. 22)".to_string(),
                        "Derecho de participación política (Art. 23)".to_string(),
                        "Derecho a la tutela judicial efectiva (Art. 24)".to_string(),
                    ],
                    political_rights: vec![
                        "Derecho de sufragio activo y pasivo".to_string(),
                        "Derecho de acceso a funciones públicas".to_string(),
                        "Derecho de petición".to_string(),
                        "Derecho a participar en asuntos públicos".to_string(),
                    ],
                    economic_social_rights: vec![
                        "Derecho al trabajo (Art. 35)".to_string(),
                        "Libertad de empresa (Art. 38)".to_string(),
                        "Derecho a la protección de la salud (Art. 43)".to_string(),
                        "Derecho a la educación (Art. 27)".to_string(),
                        "Derecho a la vivienda (Art. 47)".to_string(),
                    ],
                    cultural_linguistic_rights: vec![
                        "Derecho a la educación en lengua propia".to_string(),
                        "Protección del patrimonio cultural".to_string(),
                        "Promoción de la ciencia y la investigación".to_string(),
                    ],
                    environmental_rights: vec![
                        "Derecho a disfrutar de un medio ambiente adecuado (Art. 45)".to_string(),
                        "Deber de conservar el medio ambiente".to_string(),
                    ],
                },
                constitutional_amendments: vec![
                    SpainConstitutionalAmendment {
                        amendment_number: 1,
                        year: 1992,
                        title: "Reforma para el Tratado de Maastricht".to_string(),
                        articles_modified: vec![13],
                        approval_process: "Reforma por procedimiento agravado".to_string(),
                    },
                    SpainConstitutionalAmendment {
                        amendment_number: 2,
                        year: 2011,
                        title: "Estabilidad presupuestaria".to_string(),
                        articles_modified: vec![135],
                        approval_process: "Reforma por procedimiento ordinario".to_string(),
                    },
                ],
                constitutional_court_decisions: vec![
                    SpainConstitutionalCourtDecision {
                        decision_number: "STC 31/2010".to_string(),
                        year: 2010,
                        case_title: "Estatuto de Autonomía de Cataluña".to_string(),
                        constitutional_issue: "Límites del proceso autonómico".to_string(),
                        ruling: "Inconstitucionalidad parcial del Estatuto".to_string(),
                        legal_principle: "Principio de unidad constitucional".to_string(),
                        dissenting_opinions: vec!["Voto particular Rodríguez-Zapata".to_string()],
                    },
                    SpainConstitutionalCourtDecision {
                        decision_number: "STC 259/2015".to_string(),
                        year: 2015,
                        case_title: "Ley catalana de consultas populares".to_string(),
                        constitutional_issue: "Soberanía nacional y consultas territoriales".to_string(),
                        ruling: "Inconstitucionalidad de la ley".to_string(),
                        legal_principle: "Indivisibilidad de la soberanía nacional".to_string(),
                        dissenting_opinions: vec![],
                    },
                ],
            },
            government_structure: SpainGovernmentStructure {
                executive_branch: SpainExecutiveBranch {
                    prime_minister: SpainPrimeMinister {
                        current_holder: "Pedro Sánchez Pérez-Castejón".to_string(),
                        appointment_process: "Investidura por el Congreso de los Diputados".to_string(),
                        powers_responsibilities: vec![
                            "Dirección de la acción del Gobierno".to_string(),
                            "Coordinación de las funciones de los miembros del Gobierno".to_string(),
                            "Representación del Gobierno".to_string(),
                        ],
                        term_duration: "Coincide con la legislatura (4 años)".to_string(),
                        removal_process: "Moción de censura o cuestión de confianza".to_string(),
                    },
                    deputy_prime_ministers: vec![
                        "Yolanda Díaz Pérez (Segunda Vicepresidenta y Ministra de Trabajo)".to_string(),
                        "Teresa Ribera Rodríguez (Tercera Vicepresidenta y Ministra para la Transición Ecológica)".to_string(),
                    ],
                    ministers: vec![
                        SpainMinister {
                            ministry_name: "Ministerio del Interior".to_string(),
                            current_minister: "Fernando Grande-Marlaska Gómez".to_string(),
                            responsibilities: vec!["Seguridad ciudadana".to_string(), "Guardia Civil".to_string(), "Política migratoria".to_string()],
                            budget_allocation: "9.8 mil millones EUR".to_string(),
                            staff_count: 145000,
                        },
                        SpainMinister {
                            ministry_name: "Ministerio de Hacienda y Función Pública".to_string(),
                            current_minister: "María Jesús Montero Cuadrado".to_string(),
                            responsibilities: vec!["Política fiscal".to_string(), "Presupuestos".to_string(), "Función pública".to_string()],
                            budget_allocation: "520 mil millones EUR".to_string(),
                            staff_count: 185000,
                        },
                        SpainMinister {
                            ministry_name: "Ministerio de Justicia".to_string(),
                            current_minister: "Pilar Llop Cuenca".to_string(),
                            responsibilities: vec!["Administración de justicia".to_string(), "Registros y notariado".to_string()],
                            budget_allocation: "1.8 mil millones EUR".to_string(),
                            staff_count: 35000,
                        },
                    ],
                    secretaries_state: vec![
                        "Francesc Vallès i Vives (Secretario de Estado de Comunicación)".to_string(),
                        "Isabel Rodríguez García (Secretaria de Estado de Relaciones con las Cortes)".to_string(),
                    ],
                    government_formation: "Investidura presidencial con mayoría absoluta o simple".to_string(),
                    confidence_mechanism: "Cuestión de confianza y moción de censura".to_string(),
                },
                legislative_branch: SpainLegislativeBranch {
                    cortes_generales: SpainCortesGenerales {
                        bicameral_system: "Congreso de los Diputados y Senado".to_string(),
                        session_duration: "4 años".to_string(),
                        dissolution_rules: "Disolución anticipada posible".to_string(),
                        immunities_privileges: vec!["Inmunidad parlamentaria".to_string(), "Inviolabilidad".to_string()],
                        current_legislature: "XV Legislatura (2023-2027)".to_string(),
                    },
                    congress_deputies: SpainCongressDeputies {
                        total_seats: 350,
                        current_composition: vec![
                            SpainPoliticalGroup {
                                party_name: "Partido Socialista Obrero Español (PSOE)".to_string(),
                                seats_count: 121,
                                leader: "Pedro Sánchez".to_string(),
                                political_orientation: "Socialdemocracia".to_string(),
                            },
                            SpainPoliticalGroup {
                                party_name: "Partido Popular (PP)".to_string(),
                                seats_count: 137,
                                leader: "Alberto Núñez Feijóo".to_string(),
                                political_orientation: "Centro-derecha conservador".to_string(),
                            },
                            SpainPoliticalGroup {
                                party_name: "Vox".to_string(),
                                seats_count: 33,
                                leader: "Santiago Abascal Conde".to_string(),
                                political_orientation: "Derecha nacional-conservadora".to_string(),
                            },
                            SpainPoliticalGroup {
                                party_name: "Sumar".to_string(),
                                seats_count: 31,
                                leader: "Yolanda Díaz".to_string(),
                                political_orientation: "Izquierda alternativa".to_string(),
                            },
                        ],
                        speaker: "Francina Armengol Socías".to_string(),
                        electoral_system: "Sistema proporcional D'Hondt con circunscripciones provinciales".to_string(),
                        term_length: "4 años".to_string(),
                    },
                    senate: SpainSenate {
                        total_seats: 266,
                        current_composition: vec![
                            SpainPoliticalGroup {
                                party_name: "Partido Popular".to_string(),
                                seats_count: 140,
                                leader: "Alberto Núñez Feijóo".to_string(),
                                political_orientation: "Centro-derecha".to_string(),
                            },
                            SpainPoliticalGroup {
                                party_name: "Partido Socialista Obrero Español".to_string(),
                                seats_count: 89,
                                leader: "Pedro Sánchez".to_string(),
                                political_orientation: "Socialdemocracia".to_string(),
                            },
                        ],
                        president: "Pedro Rollán Ojeda".to_string(),
                        electoral_system: "Elección directa por provincias y designación autonómica".to_string(),
                        territorial_representation: "4 senadores por provincia y representantes autonómicos".to_string(),
                    },
                    legislative_process: SpainLegislativeProcess {
                        ordinary_procedure: "Tramitación bicameral".to_string(),
                        priority_procedures: "Lectura única y procedimiento de urgencia".to_string(),
                        organic_laws: "Mayoría absoluta del Congreso".to_string(),
                        constitutional_reform: "Procedimientos agravados según Art. 167 y 168".to_string(),
                    },
                    parliamentary_groups: vec![
                        SpainParliamentaryGroup {
                            name: "Grupo Parlamentario Socialista".to_string(),
                            chamber: "Congreso".to_string(),
                            spokesperson: "Patxi López Álvarez".to_string(),
                            members: 121,
                        },
                        SpainParliamentaryGroup {
                            name: "Grupo Parlamentario Popular".to_string(),
                            chamber: "Congreso".to_string(),
                            spokesperson: "Miguel Tellado Rey".to_string(),
                            members: 137,
                        },
                    ],
                },
                head_of_state: SpainHeadOfState {
                    king: SpainKing {
                        current_king: "Felipe VI de Borbón y Grecia".to_string(),
                        succession_law: "Sucesión hereditaria por orden de primogenitura y representación".to_string(),
                        title: "Rey de España".to_string(),
                        residence: "Palacio de la Zarzuela, Madrid".to_string(),
                        constitutional_functions: vec![
                            "Sancionar y promulgar las leyes".to_string(),
                            "Convocar y disolver las Cortes Generales".to_string(),
                            "Proponer candidato a Presidente del Gobierno".to_string(),
                            "Nombrar y separar miembros del Gobierno".to_string(),
                        ],
                    },
                    powers: vec![
                        "Jefe del Estado".to_string(),
                        "Símbolo de unidad y permanencia".to_string(),
                        "Árbitro y moderador del funcionamiento regular de las instituciones".to_string(),
                        "Representación del Estado en relaciones internacionales".to_string(),
                    ],
                    ceremonial_functions: vec![
                        "Actos oficiales y ceremonias de Estado".to_string(),
                        "Imposición de condecoraciones".to_string(),
                        "Recepción de cartas credenciales".to_string(),
                    ],
                    constitutional_role: "Monarquía parlamentaria con funciones tasadas".to_string(),
                },
                council_ministers: SpainCouncilMinisters {
                    composition: "Presidente, Vicepresidentes y Ministros".to_string(),
                    meeting_frequency: "Semanal los viernes".to_string(),
                    decision_making: "Deliberación colegiada".to_string(),
                    secretariat: "Secretaría del Consejo de Ministros".to_string(),
                },
                administrative_system: SpainAdministrativeSystem {
                    central_administration: "Ministerios y organismos públicos".to_string(),
                    peripheral_administration: "Delegaciones y subdelegaciones del Gobierno".to_string(),
                    autonomous_administration: "Administraciones autonómicas".to_string(),
                    local_administration: "Entidades locales territoriales".to_string(),
                },
            },
            territorial_organization: SpainTerritorialOrganization {
                autonomous_communities: vec![
                    SpainAutonomousCommunity {
                        name: "Comunidad de Madrid".to_string(),
                        capital: "Madrid".to_string(),
                        population: 6779888,
                        area_km2: 8028.0,
                        gdp_euros: 230000000000,
                        autonomy_statute: "Ley Orgánica 3/1983, Estatuto de Autonomía de la Comunidad de Madrid".to_string(),
                        regional_parliament: SpainRegionalParliament {
                            name: "Asamblea de Madrid".to_string(),
                            seats: 136,
                            speaker: "Enrique Ossorio Crespo".to_string(),
                            political_composition: vec![
                                SpainPoliticalGroup {
                                    party_name: "Partido Popular".to_string(),
                                    seats_count: 65,
                                    leader: "Isabel Díaz Ayuso".to_string(),
                                    political_orientation: "Centro-derecha".to_string(),
                                },
                            ],
                            term_duration: "4 años".to_string(),
                        },
                        regional_government: SpainRegionalGovernment {
                            president: "Isabel Díaz Ayuso".to_string(),
                            council_government: vec![
                                "Enrique López (Consejero de Justicia)".to_string(),
                                "Rocío Albert (Consejera de Educación)".to_string(),
                            ],
                            budget_euros: 24000000000,
                            executive_competencies: vec!["Sanidad".to_string(), "Educación".to_string(), "Transportes".to_string()],
                        },
                        competencies: vec!["Organización institucional autonómica".to_string(), "Sanidad e higiene".to_string(), "Educación".to_string()],
                        co_official_languages: vec![],
                    },
                    SpainAutonomousCommunity {
                        name: "Cataluña".to_string(),
                        capital: "Barcelona".to_string(),
                        population: 7727029,
                        area_km2: 32114.0,
                        gdp_euros: 261000000000,
                        autonomy_statute: "Ley Orgánica 6/2006, Estatuto de Autonomía de Cataluña".to_string(),
                        regional_parliament: SpainRegionalParliament {
                            name: "Parlament de Catalunya".to_string(),
                            seats: 135,
                            speaker: "Anna Erra i Solà".to_string(),
                            political_composition: vec![
                                SpainPoliticalGroup {
                                    party_name: "Junts per Catalunya".to_string(),
                                    seats_count: 32,
                                    leader: "Carles Puigdemont".to_string(),
                                    political_orientation: "Independentismo catalán".to_string(),
                                },
                            ],
                            term_duration: "4 años".to_string(),
                        },
                        regional_government: SpainRegionalGovernment {
                            president: "Pere Aragonès i Garcia".to_string(),
                            council_government: vec![
                                "Laura Vilagrà (Consellera de Presidencia)".to_string(),
                                "Josep González-Cambray (Conseller de Educación)".to_string(),
                            ],
                            budget_euros: 38000000000,
                            executive_competencies: vec!["Lengua catalana".to_string(), "Cultura".to_string(), "Enseñanza".to_string()],
                        },
                        competencies: vec!["Lengua propia".to_string(), "Cultura catalana".to_string(), "Comunicaciones internas".to_string()],
                        co_official_languages: vec!["Catalán".to_string(), "Aranés".to_string()],
                    },
                    SpainAutonomousCommunity {
                        name: "Andalucía".to_string(),
                        capital: "Sevilla".to_string(),
                        population: 8464411,
                        area_km2: 87268.0,
                        gdp_euros: 167000000000,
                        autonomy_statute: "Ley Orgánica 2/2007, Estatuto de Autonomía para Andalucía".to_string(),
                        regional_parliament: SpainRegionalParliament {
                            name: "Parlamento de Andalucía".to_string(),
                            seats: 109,
                            speaker: "Jesús Aguirre Muñoz".to_string(),
                            political_composition: vec![
                                SpainPoliticalGroup {
                                    party_name: "Partido Popular".to_string(),
                                    seats_count: 58,
                                    leader: "Juanma Moreno".to_string(),
                                    political_orientation: "Centro-derecha".to_string(),
                                },
                            ],
                            term_duration: "4 años".to_string(),
                        },
                        regional_government: SpainRegionalGovernment {
                            president: "Juan Manuel Moreno Bonilla".to_string(),
                            council_government: vec![
                                "Antonio Sanz (Consejero de Presidencia)".to_string(),
                                "Carmen Crespo (Consejera de Agricultura)".to_string(),
                            ],
                            budget_euros: 42000000000,
                            executive_competencies: vec!["Agricultura".to_string(), "Pesca".to_string(), "Patrimonio histórico".to_string()],
                        },
                        competencies: vec!["Ordenación territorial".to_string(), "Urbanismo".to_string(), "Vivienda".to_string()],
                        co_official_languages: vec![],
                    },
                ],
                provinces: vec![
                    SpainProvince {
                        name: "Madrid".to_string(),
                        capital: "Madrid".to_string(),
                        population: 6779888,
                        autonomous_community: "Comunidad de Madrid".to_string(),
                        provincial_council: "Diputación Provincial suprimida".to_string(),
                    },
                    SpainProvince {
                        name: "Barcelona".to_string(),
                        capital: "Barcelona".to_string(),
                        population: 5743402,
                        autonomous_community: "Cataluña".to_string(),
                        provincial_council: "Diputació de Barcelona".to_string(),
                    },
                ],
                municipalities: vec![
                    SpainMunicipality {
                        name: "Madrid".to_string(),
                        province: "Madrid".to_string(),
                        population: 3334730,
                        mayor: "José Luis Martínez-Almeida Navasqüés".to_string(),
                        municipal_council_seats: 57,
                    },
                    SpainMunicipality {
                        name: "Barcelona".to_string(),
                        province: "Barcelona".to_string(),
                        population: 1636762,
                        mayor: "Jaume Collboni i Cuadrado".to_string(),
                        municipal_council_seats: 41,
                    },
                ],
                autonomous_cities: vec![
                    SpainAutonomousCity {
                        name: "Ceuta".to_string(),
                        population: 82376,
                        president: "Juan Jesús Vivas Lara".to_string(),
                        special_statute: "Ley Orgánica 1/1995, Estatuto de Autonomía de Ceuta".to_string(),
                        competencies: vec!["Asistencia social".to_string(), "Sanidad e higiene".to_string(), "Educación".to_string()],
                    },
                    SpainAutonomousCity {
                        name: "Melilla".to_string(),
                        population: 86487,
                        president: "Eduardo de Castro González".to_string(),
                        special_statute: "Ley Orgánica 2/1995, Estatuto de Autonomía de Melilla".to_string(),
                        competencies: vec!["Promoción y fomento de la cultura".to_string(), "Turismo".to_string(), "Deportes".to_string()],
                    },
                ],
                territorial_cooperation: SpainTerritorialCooperation {
                    cooperation_agreements: "Convenios de colaboración entre administraciones".to_string(),
                    sectoral_conferences: "Conferencias sectoriales ministeriales-autonómicas".to_string(),
                    solidarity_fund: "Fondo de Compensación Interterritorial".to_string(),
                    fiscal_coordination: "Consejo de Política Fiscal y Financiera".to_string(),
                },
            },
            judicial_system: SpainJudicialSystem {
                supreme_court: SpainSupremeCourt {
                    official_name: "Tribunal Supremo".to_string(),
                    location: "Madrid".to_string(),
                    president: "Carlos Lesmes Serrano".to_string(),
                    total_judges: 79,
                    chambers: vec![
                        SpainSupremeCourtChamber {
                            chamber_name: "Sala de lo Civil".to_string(),
                            president: "Francisco Javier Orduña Moreno".to_string(),
                            specialization: "Derecho civil".to_string(),
                            judges_count: 13,
                        },
                        SpainSupremeCourtChamber {
                            chamber_name: "Sala de lo Penal".to_string(),
                            president: "Manuel Marchena Gómez".to_string(),
                            specialization: "Derecho penal".to_string(),
                            judges_count: 15,
                        },
                        SpainSupremeCourtChamber {
                            chamber_name: "Sala de lo Contencioso-Administrativo".to_string(),
                            president: "César Tolosa Tribiño".to_string(),
                            specialization: "Derecho administrativo".to_string(),
                            judges_count: 32,
                        },
                    ],
                    jurisdiction: vec![
                        "Recurso de casación".to_string(),
                        "Recurso extraordinario por infracción procesal".to_string(),
                        "Aforamientos especiales".to_string(),
                    ],
                },
                constitutional_court: SpainConstitutionalCourt {
                    official_name: "Tribunal Constitucional".to_string(),
                    location: "Madrid".to_string(),
                    president: "Pedro González-Trevijano Sánchez".to_string(),
                    total_judges: 12,
                    appointment_process: "4 por Congreso, 4 por Senado, 2 por Gobierno, 2 por CGPJ".to_string(),
                    jurisdiction: vec![
                        "Recurso de inconstitucionalidad".to_string(),
                        "Cuestión de inconstitucionalidad".to_string(),
                        "Recurso de amparo".to_string(),
                        "Conflictos de competencias".to_string(),
                    ],
                    landmark_decisions: vec![
                        SpainConstitutionalCourtDecision {
                            decision_number: "STC 76/1983".to_string(),
                            year: 1983,
                            case_title: "Ley de despenalización del aborto".to_string(),
                            constitutional_issue: "Derecho a la vida vs. derechos de la mujer".to_string(),
                            ruling: "Constitucionalidad con condiciones".to_string(),
                            legal_principle: "Ponderación de derechos fundamentales".to_string(),
                            dissenting_opinions: vec!["Voto particular Rubio Llorente".to_string()],
                        },
                        SpainConstitutionalCourtDecision {
                            decision_number: "STC 198/2012".to_string(),
                            year: 2012,
                            case_title: "Estatuto de Autonomía de Cataluña".to_string(),
                            constitutional_issue: "Límites de la autonomía territorial".to_string(),
                            ruling: "Inconstitucionalidad parcial".to_string(),
                            legal_principle: "Principio de unidad constitucional".to_string(),
                            dissenting_opinions: vec![],
                        },
                    ],
                },
                general_council_judiciary: SpainGeneralCouncilJudiciary {
                    official_name: "Consejo General del Poder Judicial".to_string(),
                    president: "Carlos Lesmes Serrano".to_string(),
                    composition: "20 vocales más Presidente del Tribunal Supremo".to_string(),
                    functions: vec![
                        "Gobierno del Poder Judicial".to_string(),
                        "Nombramiento de jueces y magistrados".to_string(),
                        "Inspección y disciplina judicial".to_string(),
                    ],
                    renewable_mandate: "5 años".to_string(),
                },
                national_court: SpainNationalCourt {
                    official_name: "Audiencia Nacional".to_string(),
                    location: "Madrid".to_string(),
                    president: "José Ramón Navarro Miranda".to_string(),
                    specialized_chambers: vec![
                        "Sala de lo Penal".to_string(),
                        "Sala de lo Contencioso-Administrativo".to_string(),
                        "Sala de Apelación".to_string(),
                    ],
                    jurisdiction: vec![
                        "Delitos de terrorismo".to_string(),
                        "Delitos contra la Corona".to_string(),
                        "Tráfico de drogas a gran escala".to_string(),
                        "Delitos monetarios".to_string(),
                    ],
                },
                ordinary_jurisdiction: SpainOrdinaryJurisdiction {
                    first_instance: "Juzgados de Primera Instancia e Instrucción".to_string(),
                    provincial_courts: "Audiencias Provinciales".to_string(),
                    superior_courts: "Tribunales Superiores de Justicia de las CCAA".to_string(),
                    supreme_court: "Tribunal Supremo".to_string(),
                },
                specialized_jurisdictions: SpainSpecializedJurisdictions {
                    administrative_jurisdiction: "Juzgados de lo Contencioso-Administrativo".to_string(),
                    labor_jurisdiction: "Juzgados de lo Social".to_string(),
                    juvenile_jurisdiction: "Juzgados de Menores".to_string(),
                    violence_against_women: "Juzgados de Violencia sobre la Mujer".to_string(),
                },
                prosecution_system: SpainProsecutionSystem {
                    attorney_general: "Álvaro García Ortiz".to_string(),
                    structure: "Ministerio Fiscal".to_string(),
                    specialized_prosecutors: vec![
                        "Fiscalía Anticorrupción".to_string(),
                        "Fiscalía Antidroga".to_string(),
                        "Fiscalía de Menores".to_string(),
                        "Fiscalía de Violencia sobre la Mujer".to_string(),
                    ],
                    principles: vec!["Legalidad".to_string(), "Imparcialidad".to_string(), "Unidad de actuación".to_string()],
                },
            },
            legal_codes: SpainLegalCodes {
                civil_code: SpainCivilCode {
                    official_name: "Código Civil".to_string(),
                    promulgation_date: "24 de julio de 1889".to_string(),
                    structure: vec![
                        SpainCodeBook {
                            book_number: 1,
                            title: "De las personas".to_string(),
                            articles_range: "Art. 17-332".to_string(),
                            main_topics: vec!["Personalidad jurídica".to_string(), "Matrimonio".to_string(), "Filiación".to_string()],
                            key_articles: vec![
                                SpainCodeArticle {
                                    article_number: 29,
                                    title: "Nacimiento con vida".to_string(),
                                    content: "El nacimiento determina la personalidad; pero el concebido se tiene por nacido para todos los efectos que le sean favorables, siempre que nazca con las condiciones que expresa el artículo siguiente.".to_string(),
                                    amendments: vec!["Ley 20/2011 del Registro Civil".to_string()],
                                },
                                SpainCodeArticle {
                                    article_number: 44,
                                    title: "Matrimonio".to_string(),
                                    content: "El hombre y la mujer tienen derecho a contraer matrimonio conforme a las disposiciones de este Código.".to_string(),
                                    amendments: vec!["Ley 13/2005 de matrimonio homosexual".to_string()],
                                },
                            ],
                        },
                        SpainCodeBook {
                            book_number: 4,
                            title: "De las obligaciones y contratos".to_string(),
                            articles_range: "Art. 1088-1976".to_string(),
                            main_topics: vec!["Obligaciones".to_string(), "Contratos".to_string(), "Responsabilidad civil".to_string()],
                            key_articles: vec![
                                SpainCodeArticle {
                                    article_number: 1254,
                                    title: "Fuerza vinculante del contrato".to_string(),
                                    content: "El contrato existe desde que una o varias personas consienten en obligarse, respecto de otra u otras, a dar alguna cosa o prestar algún servicio.".to_string(),
                                    amendments: vec![],
                                },
                                SpainCodeArticle {
                                    article_number: 1902,
                                    title: "Responsabilidad extracontractual".to_string(),
                                    content: "El que por acción u omisión causa daño a otro, interviniendo culpa o negligencia, está obligado a reparar el daño causado.".to_string(),
                                    amendments: vec!["Ley 1/1991 de responsabilidad civil".to_string()],
                                },
                            ],
                        },
                    ],
                    total_articles: 1976,
                    major_reforms: vec![
                        "Reforma del derecho de familia (1981)".to_string(),
                        "Ley de matrimonio igualitario (2005)".to_string(),
                        "Nueva Ley del Registro Civil (2011)".to_string(),
                    ],
                },
                criminal_code: SpainCriminalCode {
                    official_name: "Código Penal".to_string(),
                    promulgation_date: "23 de noviembre de 1995".to_string(),
                    structure: vec![
                        SpainCodeBook {
                            book_number: 1,
                            title: "Disposiciones generales sobre los delitos y las faltas, las personas responsables, las penas, medidas de seguridad y demás consecuencias de la infracción penal".to_string(),
                            articles_range: "Art. 1-137".to_string(),
                            main_topics: vec!["Principios generales".to_string(), "Penas".to_string(), "Medidas de seguridad".to_string()],
                            key_articles: vec![
                                SpainCodeArticle {
                                    article_number: 1,
                                    title: "Principio de legalidad".to_string(),
                                    content: "No será castigado ningún delito ni falta con pena que no se halle prevista por Ley anterior a su perpetración.".to_string(),
                                    amendments: vec![],
                                },
                            ],
                        },
                    ],
                    total_articles: 639,
                    recent_reforms: vec![
                        "LO 1/2015 reforma Código Penal".to_string(),
                        "LO 8/2021 protección integral infancia y adolescencia".to_string(),
                        "LO 10/2022 garantía integral libertad sexual".to_string(),
                    ],
                },
                commercial_code: SpainCommercialCode {
                    official_name: "Código de Comercio".to_string(),
                    promulgation_date: "22 de agosto de 1885".to_string(),
                    complementary_legislation: vec![
                        "Ley de Sociedades de Capital".to_string(),
                        "Ley Concursal".to_string(),
                        "Ley de Auditoría de Cuentas".to_string(),
                    ],
                    recent_updates: vec![
                        "Digitalización empresarial".to_string(),
                        "Sostenibilidad corporativa".to_string(),
                    ],
                },
                labor_code: SpainLaborCode {
                    main_legislation: "Real Decreto Legislativo 2/2015, Estatuto de los Trabajadores".to_string(),
                    complementary_laws: vec![
                        "Ley 31/1995 de Prevención de Riesgos Laborales".to_string(),
                        "Real Decreto Legislativo 8/2015, Ley General de la Seguridad Social".to_string(),
                    ],
                    recent_reforms: vec![
                        "Real Decreto-ley 32/2021 plan de choque empleo joven".to_string(),
                        "Ley 15/2022 integral para la igualdad de trato".to_string(),
                    ],
                    key_principles: vec![
                        "Derecho al trabajo".to_string(),
                        "Libertad sindical".to_string(),
                        "Negociación colectiva".to_string(),
                        "Derecho de huelga".to_string(),
                    ],
                },
                administrative_code: SpainAdministrativeCode {
                    main_legislation: "Ley 39/2015 del Procedimiento Administrativo Común".to_string(),
                    organic_law: "Ley 40/2015 de Régimen Jurídico del Sector Público".to_string(),
                    principles: vec![
                        "Legalidad".to_string(),
                        "Jerarquía".to_string(),
                        "Descentralización".to_string(),
                        "Desconcentración".to_string(),
                        "Coordinación".to_string(),
                    ],
                    citizen_rights: vec![
                        "Derecho a la información".to_string(),
                        "Derecho de audiencia".to_string(),
                        "Derecho de participación".to_string(),
                    ],
                },
                tax_code: SpainTaxCode {
                    general_tax_law: "Ley 58/2003, Ley General Tributaria".to_string(),
                    main_taxes: vec![
                        "Impuesto sobre la Renta de las Personas Físicas (IRPF)".to_string(),
                        "Impuesto sobre Sociedades (IS)".to_string(),
                        "Impuesto sobre el Valor Añadido (IVA)".to_string(),
                        "Impuesto sobre Transmisiones Patrimoniales y Actos Jurídicos Documentados".to_string(),
                    ],
                    tax_administration: "Agencia Estatal de Administración Tributaria (AEAT)".to_string(),
                    recent_reforms: vec![
                        "Ley 11/2021 medidas prevención y lucha fraude fiscal".to_string(),
                        "Real Decreto-ley 13/2021 nuevas medidas apoyo empresarial".to_string(),
                    ],
                },
            },
            human_rights: SpainHumanRights {
                constitutional_guarantees: vec![
                    "Igualdad ante la ley (Art. 14 CE)".to_string(),
                    "Derecho a la vida e integridad física (Art. 15 CE)".to_string(),
                    "Libertad religiosa (Art. 16 CE)".to_string(),
                    "Derecho a la libertad y seguridad (Art. 17 CE)".to_string(),
                    "Derecho al honor, intimidad e imagen (Art. 18 CE)".to_string(),
                ],
                international_treaties: vec![
                    "Convenio Europeo de Derechos Humanos (1979)".to_string(),
                    "Pacto Internacional de Derechos Civiles y Políticos (1977)".to_string(),
                    "Convención contra la Tortura (1987)".to_string(),
                    "Convención sobre los Derechos del Niño (1990)".to_string(),
                ],
                ombudsman_system: SpainOmbudsmanSystem {
                    defensor_pueblo: "Ángel Gabilondo Pujol".to_string(),
                    regional_ombudsmen: vec![
                        "Síndic de Greuges de Catalunya".to_string(),
                        "Ararteko del País Vasco".to_string(),
                        "Valedor do Pobo de Galicia".to_string(),
                        "Justicia de Aragón".to_string(),
                    ],
                    competencies: vec![
                        "Supervisión de la actividad de las Administraciones Públicas".to_string(),
                        "Defensa de los derechos fundamentales".to_string(),
                        "Promoción de reformas legislativas".to_string(),
                    ],
                    reporting_mechanism: "Informe anual a las Cortes Generales".to_string(),
                },
                anti_discrimination: vec![
                    "Ley 62/2003 medidas fiscales, administrativas y orden social (igualdad)".to_string(),
                    "Ley Orgánica 3/2007 para la igualdad efectiva de mujeres y hombres".to_string(),
                    "Ley 15/2022 integral para la igualdad de trato y la no discriminación".to_string(),
                ],
                data_protection: SpainDataProtection {
                    authority: "Agencia Española de Protección de Datos (AEPD)".to_string(),
                    legal_framework: vec![
                        "Reglamento UE 2016/679 (RGPD)".to_string(),
                        "Ley Orgánica 3/2018 de Protección de Datos Personales y garantía de los derechos digitales".to_string(),
                    ],
                    individual_rights: vec![
                        "Derecho de información".to_string(),
                        "Derecho de acceso".to_string(),
                        "Derecho de rectificación".to_string(),
                        "Derecho de supresión".to_string(),
                        "Derecho a la portabilidad de los datos".to_string(),
                    ],
                    enforcement_mechanisms: vec![
                        "Procedimiento sancionador".to_string(),
                        "Medidas correctivas".to_string(),
                        "Derecho de reclamación".to_string(),
                    ],
                },
            },
            eu_integration: SpainEUIntegration {
                membership_date: "1 de enero de 1986".to_string(),
                euro_adoption: "1 de enero de 1999 (circulación 1 de enero de 2002)".to_string(),
                schengen_participation: "26 de marzo de 1995".to_string(),
                eu_institutions_representation: SpainEURepresentation {
                    european_parliament_seats: 59,
                    council_votes: 27,
                    commissioners: vec!["Josep Borrell Fontelles (Alto Representante)".to_string()],
                    permanent_representative: "Marcos Alonso Alonso".to_string(),
                },
                european_law_implementation: vec![
                    "Ley 2/1997 por la que se regula la Conferencia para Asuntos relacionados con las Comunidades Europeas".to_string(),
                    "Ley 8/1994 por la que se regula la Comisión Mixta para la Unión Europea".to_string(),
                    "Disposiciones de transposición de directivas europeas".to_string(),
                ],
            },
        }
    }
}

// Additional implementation structs needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLegislativeProcess {
    pub ordinary_procedure: String,
    pub priority_procedures: String,
    pub organic_laws: String,
    pub constitutional_reform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainParliamentaryGroup {
    pub name: String,
    pub chamber: String,
    pub spokesperson: String,
    pub members: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCouncilMinisters {
    pub composition: String,
    pub meeting_frequency: String,
    pub decision_making: String,
    pub secretariat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainAdministrativeSystem {
    pub central_administration: String,
    pub peripheral_administration: String,
    pub autonomous_administration: String,
    pub local_administration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainTerritorialCooperation {
    pub cooperation_agreements: String,
    pub sectoral_conferences: String,
    pub solidarity_fund: String,
    pub fiscal_coordination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainGeneralCouncilJudiciary {
    pub official_name: String,
    pub president: String,
    pub composition: String,
    pub functions: Vec<String>,
    pub renewable_mandate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainNationalCourt {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub specialized_chambers: Vec<String>,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainOrdinaryJurisdiction {
    pub first_instance: String,
    pub provincial_courts: String,
    pub superior_courts: String,
    pub supreme_court: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainSpecializedJurisdictions {
    pub administrative_jurisdiction: String,
    pub labor_jurisdiction: String,
    pub juvenile_jurisdiction: String,
    pub violence_against_women: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainProsecutionSystem {
    pub attorney_general: String,
    pub structure: String,
    pub specialized_prosecutors: Vec<String>,
    pub principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainCommercialCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub complementary_legislation: Vec<String>,
    pub recent_updates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLaborCode {
    pub main_legislation: String,
    pub complementary_laws: Vec<String>,
    pub recent_reforms: Vec<String>,
    pub key_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainAdministrativeCode {
    pub main_legislation: String,
    pub organic_law: String,
    pub principles: Vec<String>,
    pub citizen_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainTaxCode {
    pub general_tax_law: String,
    pub main_taxes: Vec<String>,
    pub tax_administration: String,
    pub recent_reforms: Vec<String>,
}

impl Default for SpainLegislativeProcess {
    fn default() -> Self {
        Self {
            ordinary_procedure: "Tramitación bicameral".to_string(),
            priority_procedures: "Lectura única y procedimiento de urgencia".to_string(),
            organic_laws: "Mayoría absoluta del Congreso".to_string(),
            constitutional_reform: "Procedimientos agravados según Art. 167 y 168".to_string(),
        }
    }
}

impl Default for SpainParliamentaryGroup {
    fn default() -> Self {
        Self {
            name: String::new(),
            chamber: String::new(),
            spokesperson: String::new(),
            members: 0,
        }
    }
}

impl Default for SpainCouncilMinisters {
    fn default() -> Self {
        Self {
            composition: "Presidente, Vicepresidentes y Ministros".to_string(),
            meeting_frequency: "Semanal los viernes".to_string(),
            decision_making: "Deliberación colegiada".to_string(),
            secretariat: "Secretaría del Consejo de Ministros".to_string(),
        }
    }
}

impl Default for SpainAdministrativeSystem {
    fn default() -> Self {
        Self {
            central_administration: "Ministerios y organismos públicos".to_string(),
            peripheral_administration: "Delegaciones y subdelegaciones del Gobierno".to_string(),
            autonomous_administration: "Administraciones autonómicas".to_string(),
            local_administration: "Entidades locales territoriales".to_string(),
        }
    }
}

impl Default for SpainTerritorialCooperation {
    fn default() -> Self {
        Self {
            cooperation_agreements: "Convenios de colaboración entre administraciones".to_string(),
            sectoral_conferences: "Conferencias sectoriales ministeriales-autonómicas".to_string(),
            solidarity_fund: "Fondo de Compensación Interterritorial".to_string(),
            fiscal_coordination: "Consejo de Política Fiscal y Financiera".to_string(),
        }
    }
}

impl Default for SpainGeneralCouncilJudiciary {
    fn default() -> Self {
        Self {
            official_name: "Consejo General del Poder Judicial".to_string(),
            president: String::new(),
            composition: "20 vocales más Presidente del Tribunal Supremo".to_string(),
            functions: vec![],
            renewable_mandate: "5 años".to_string(),
        }
    }
}

impl Default for SpainNationalCourt {
    fn default() -> Self {
        Self {
            official_name: "Audiencia Nacional".to_string(),
            location: "Madrid".to_string(),
            president: String::new(),
            specialized_chambers: vec![],
            jurisdiction: vec![],
        }
    }
}

impl Default for SpainOrdinaryJurisdiction {
    fn default() -> Self {
        Self {
            first_instance: "Juzgados de Primera Instancia e Instrucción".to_string(),
            provincial_courts: "Audiencias Provinciales".to_string(),
            superior_courts: "Tribunales Superiores de Justicia de las CCAA".to_string(),
            supreme_court: "Tribunal Supremo".to_string(),
        }
    }
}

impl Default for SpainSpecializedJurisdictions {
    fn default() -> Self {
        Self {
            administrative_jurisdiction: "Juzgados de lo Contencioso-Administrativo".to_string(),
            labor_jurisdiction: "Juzgados de lo Social".to_string(),
            juvenile_jurisdiction: "Juzgados de Menores".to_string(),
            violence_against_women: "Juzgados de Violencia sobre la Mujer".to_string(),
        }
    }
}

impl Default for SpainProsecutionSystem {
    fn default() -> Self {
        Self {
            attorney_general: String::new(),
            structure: "Ministerio Fiscal".to_string(),
            specialized_prosecutors: vec![],
            principles: vec![],
        }
    }
}

impl Default for SpainCommercialCode {
    fn default() -> Self {
        Self {
            official_name: "Código de Comercio".to_string(),
            promulgation_date: "22 de agosto de 1885".to_string(),
            complementary_legislation: vec![],
            recent_updates: vec![],
        }
    }
}

impl Default for SpainLaborCode {
    fn default() -> Self {
        Self {
            main_legislation: "Real Decreto Legislativo 2/2015, Estatuto de los Trabajadores".to_string(),
            complementary_laws: vec![],
            recent_reforms: vec![],
            key_principles: vec![],
        }
    }
}

impl Default for SpainAdministrativeCode {
    fn default() -> Self {
        Self {
            main_legislation: "Ley 39/2015 del Procedimiento Administrativo Común".to_string(),
            organic_law: "Ley 40/2015 de Régimen Jurídico del Sector Público".to_string(),
            principles: vec![],
            citizen_rights: vec![],
        }
    }
}

impl Default for SpainTaxCode {
    fn default() -> Self {
        Self {
            general_tax_law: "Ley 58/2003, Ley General Tributaria".to_string(),
            main_taxes: vec![],
            tax_administration: "Agencia Estatal de Administración Tributaria (AEAT)".to_string(),
            recent_reforms: vec![],
        }
    }
}

pub fn create_spain_legal_system() -> SpainLegalSystem {
    SpainLegalSystem::default()
}