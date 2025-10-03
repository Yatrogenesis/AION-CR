use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLegalSystem {
    pub constitutional_framework: ItalyConstitutionalFramework,
    pub government_structure: ItalyGovernmentStructure,
    pub territorial_organization: ItalyTerritorialOrganization,
    pub judicial_system: ItalyJudicialSystem,
    pub legal_codes: ItalyLegalCodes,
    pub human_rights: ItalyHumanRights,
    pub eu_integration: ItalyEUIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalFramework {
    pub constitution_1948: ItalyConstitution1948,
    pub constitutional_principles: ItalyConstitutionalPrinciples,
    pub fundamental_rights: ItalyFundamentalRights,
    pub constitutional_amendments: Vec<ItalyConstitutionalAmendment>,
    pub constitutional_court_decisions: Vec<ItalyConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitution1948 {
    pub preamble: String,
    pub fundamental_principles: Vec<ItalyConstitutionalArticle>,
    pub part_one_rights_duties: Vec<ItalyConstitutionalArticle>,
    pub part_two_organization_republic: Vec<ItalyConstitutionalArticle>,
    pub transitional_provisions: Vec<ItalyConstitutionalArticle>,
    pub adoption_date: String,
    pub effective_date: String,
    pub constituent_assembly: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub interpretation_notes: Vec<String>,
    pub related_legislation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalPrinciples {
    pub sovereignty_principle: String,
    pub democratic_principle: String,
    pub rule_of_law: String,
    pub separation_powers: String,
    pub republican_form: String,
    pub regional_autonomy: String,
    pub international_cooperation: String,
    pub pacifist_principle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyFundamentalRights {
    pub individual_rights: Vec<String>,
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub economic_social_rights: Vec<String>,
    pub cultural_rights: Vec<String>,
    pub environmental_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyGovernmentStructure {
    pub executive_branch: ItalyExecutiveBranch,
    pub legislative_branch: ItalyLegislativeBranch,
    pub head_of_state: ItalyHeadOfState,
    pub council_ministers: ItalyCouncilMinisters,
    pub administrative_system: ItalyAdministrativeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyExecutiveBranch {
    pub prime_minister: ItalyPrimeMinister,
    pub deputy_prime_ministers: Vec<String>,
    pub ministers: Vec<ItalyMinister>,
    pub undersecretaries: Vec<String>,
    pub government_formation: String,
    pub confidence_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyPrimeMinister {
    pub current_holder: String,
    pub appointment_process: String,
    pub powers_responsibilities: Vec<String>,
    pub term_duration: String,
    pub removal_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyMinister {
    pub ministry_name: String,
    pub current_minister: String,
    pub responsibilities: Vec<String>,
    pub budget_allocation: String,
    pub staff_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLegislativeBranch {
    pub parliament: ItalyParliament,
    pub chamber_deputies: ItalyChamberDeputies,
    pub senate: ItalySenate,
    pub legislative_process: ItalyLegislativeProcess,
    pub parliamentary_committees: Vec<ItalyParliamentaryCommittee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyParliament {
    pub bicameral_system: String,
    pub session_duration: String,
    pub dissolution_rules: String,
    pub immunities_privileges: Vec<String>,
    pub current_legislature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyChamberDeputies {
    pub total_seats: u32,
    pub current_composition: Vec<ItalyPoliticalGroup>,
    pub speaker: String,
    pub electoral_system: String,
    pub term_length: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalySenate {
    pub total_seats: u32,
    pub current_composition: Vec<ItalyPoliticalGroup>,
    pub president: String,
    pub electoral_system: String,
    pub senators_for_life: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyPoliticalGroup {
    pub party_name: String,
    pub seats_count: u32,
    pub leader: String,
    pub political_orientation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyHeadOfState {
    pub president: ItalyPresident,
    pub powers: Vec<String>,
    pub ceremonial_functions: Vec<String>,
    pub emergency_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyPresident {
    pub current_president: String,
    pub election_process: String,
    pub term_duration: String,
    pub eligibility_requirements: Vec<String>,
    pub impeachment_process: String,
    pub residence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyTerritorialOrganization {
    pub regions: Vec<ItalyRegion>,
    pub provinces: Vec<ItalyProvince>,
    pub municipalities: Vec<ItalyMunicipality>,
    pub metropolitan_cities: Vec<ItalyMetropolitanCity>,
    pub special_autonomy: Vec<ItalySpecialAutonomy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyRegion {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub area_km2: f64,
    pub gdp_euros: u64,
    pub regional_council: ItalyRegionalCouncil,
    pub regional_government: ItalyRegionalGovernment,
    pub statute: String,
    pub legislative_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyRegionalCouncil {
    pub seats: u32,
    pub president: String,
    pub political_composition: Vec<ItalyPoliticalGroup>,
    pub term_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyRegionalGovernment {
    pub regional_president: String,
    pub regional_ministers: Vec<String>,
    pub budget_euros: u64,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyProvince {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub municipalities_count: u32,
    pub president: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyMunicipality {
    pub name: String,
    pub province: String,
    pub population: u64,
    pub mayor: String,
    pub municipal_council_seats: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyMetropolitanCity {
    pub name: String,
    pub metropolitan_mayor: String,
    pub population: u64,
    pub municipalities_included: Vec<String>,
    pub special_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalySpecialAutonomy {
    pub territory_name: String,
    pub autonomy_type: String,
    pub special_statute: String,
    pub legislative_powers: Vec<String>,
    pub financial_autonomy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyJudicialSystem {
    pub supreme_court: ItalySupremeCourt,
    pub constitutional_court: ItalyConstitutionalCourt,
    pub council_state: ItalyCouncilOfState,
    pub court_audit: ItalyCourtOfAudit,
    pub ordinary_jurisdiction: ItalyOrdinaryJurisdiction,
    pub administrative_jurisdiction: ItalyAdministrativeJurisdiction,
    pub prosecution_system: ItalyProsecutionSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalySupremeCourt {
    pub official_name: String,
    pub location: String,
    pub chief_justice: String,
    pub total_judges: u32,
    pub sections: Vec<ItalySupremeCourtSection>,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalySupremeCourtSection {
    pub section_name: String,
    pub president: String,
    pub specialization: String,
    pub judges_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalCourt {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub total_judges: u32,
    pub appointment_process: String,
    pub jurisdiction: Vec<String>,
    pub landmark_decisions: Vec<ItalyConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalCourtDecision {
    pub decision_number: String,
    pub year: u32,
    pub case_title: String,
    pub constitutional_issue: String,
    pub ruling: String,
    pub legal_principle: String,
    pub dissenting_opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCouncilOfState {
    pub official_name: String,
    pub president: String,
    pub advisory_functions: Vec<String>,
    pub judicial_functions: Vec<String>,
    pub sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLegalCodes {
    pub civil_code: ItalyCivilCode,
    pub criminal_code: ItalyCriminalCode,
    pub administrative_code: ItalyAdministrativeCode,
    pub commercial_code: ItalyCommercialCode,
    pub labor_code: ItalyLaborCode,
    pub tax_code: ItalyTaxCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCivilCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<ItalyCodeBook>,
    pub total_articles: u32,
    pub major_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCodeBook {
    pub book_number: u32,
    pub title: String,
    pub articles_range: String,
    pub main_topics: Vec<String>,
    pub key_articles: Vec<ItalyCodeArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCodeArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub amendments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCriminalCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<ItalyCodeBook>,
    pub total_articles: u32,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalAmendment {
    pub amendment_number: u32,
    pub year: u32,
    pub title: String,
    pub articles_modified: Vec<u32>,
    pub referendum_required: bool,
    pub approval_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyHumanRights {
    pub constitutional_guarantees: Vec<String>,
    pub international_treaties: Vec<String>,
    pub ombudsman_system: ItalyOmbudsmanSystem,
    pub anti_discrimination: Vec<String>,
    pub data_protection: ItalyDataProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyOmbudsmanSystem {
    pub national_ombudsman: String,
    pub regional_ombudsmen: Vec<String>,
    pub competencies: Vec<String>,
    pub reporting_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyDataProtection {
    pub authority: String,
    pub legal_framework: Vec<String>,
    pub individual_rights: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyEUIntegration {
    pub membership_date: String,
    pub euro_adoption: String,
    pub schengen_participation: String,
    pub eu_institutions_representation: ItalyEURepresentation,
    pub european_law_implementation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyEURepresentation {
    pub european_parliament_seats: u32,
    pub council_votes: u32,
    pub commissioners: Vec<String>,
    pub permanent_representative: String,
}

impl Default for ItalyLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ItalyConstitutionalFramework {
                constitution_1948: ItalyConstitution1948 {
                    preamble: "La Repubblica italiana, fondata sul lavoro, riconosce e garantisce i diritti inviolabili dell'uomo, sia come singolo sia nelle formazioni sociali ove si svolge la sua personalità".to_string(),
                    fundamental_principles: vec![
                        ItalyConstitutionalArticle {
                            article_number: 1,
                            title: "Fondamento democratico".to_string(),
                            content: "L'Italia è una Repubblica democratica, fondata sul lavoro. La sovranità appartiene al popolo, che la esercita nelle forme e nei limiti della Costituzione.".to_string(),
                            interpretation_notes: vec!["Principio democratico fondamentale".to_string(), "Sovranità popolare".to_string()],
                            related_legislation: vec!["Legge elettorale".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 2,
                            title: "Diritti inviolabili".to_string(),
                            content: "La Repubblica riconosce e garantisce i diritti inviolabili dell'uomo, sia come singolo sia nelle formazioni sociali ove si svolge la sua personalità, e richiede l'adempimento dei doveri inderogabili di solidarietà politica, economica e sociale.".to_string(),
                            interpretation_notes: vec!["Diritti fondamentali".to_string(), "Doveri costituzionali".to_string()],
                            related_legislation: vec!["Codice civile".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 3,
                            title: "Uguaglianza".to_string(),
                            content: "Tutti i cittadini hanno pari dignità sociale e sono eguali davanti alla legge, senza distinzione di sesso, di razza, di lingua, di religione, di opinioni politiche, di condizioni personali e sociali.".to_string(),
                            interpretation_notes: vec!["Principio di uguaglianza formale e sostanziale".to_string()],
                            related_legislation: vec!["Leggi antidiscriminazione".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 11,
                            title: "Ripudio della guerra".to_string(),
                            content: "L'Italia ripudia la guerra come strumento di offesa alla libertà degli altri popoli e come mezzo di risoluzione delle controversie internazionali".to_string(),
                            interpretation_notes: vec!["Principio pacifista".to_string()],
                            related_legislation: vec!["Trattati internazionali".to_string()],
                        },
                    ],
                    part_one_rights_duties: vec![
                        ItalyConstitutionalArticle {
                            article_number: 13,
                            title: "Libertà personale".to_string(),
                            content: "La libertà personale è inviolabile. Non è ammessa forma alcuna di detenzione, di ispezione o perquisizione personale, né qualsiasi altra restrizione della libertà personale, se non per atto motivato dell'autorità giudiziaria".to_string(),
                            interpretation_notes: vec!["Habeas corpus italiano".to_string()],
                            related_legislation: vec!["Codice di procedura penale".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 21,
                            title: "Libertà di manifestazione del pensiero".to_string(),
                            content: "Tutti hanno diritto di manifestare liberamente il proprio pensiero con la parola, lo scritto e ogni altro mezzo di diffusione.".to_string(),
                            interpretation_notes: vec!["Libertà di espressione".to_string()],
                            related_legislation: vec!["Legge sulla stampa".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 36,
                            title: "Diritto al lavoro".to_string(),
                            content: "Il lavoratore ha diritto ad una retribuzione proporzionata alla quantità e qualità del suo lavoro e in ogni caso sufficiente ad assicurare a sé e alla famiglia un'esistenza libera e dignitosa.".to_string(),
                            interpretation_notes: vec!["Salario minimo costituzionale".to_string()],
                            related_legislation: vec!["Statuto dei lavoratori".to_string()],
                        },
                    ],
                    part_two_organization_republic: vec![
                        ItalyConstitutionalArticle {
                            article_number: 83,
                            title: "Elezione del Presidente".to_string(),
                            content: "Il Presidente della Repubblica è eletto dal Parlamento in seduta comune dei suoi membri.".to_string(),
                            interpretation_notes: vec!["Sistema di elezione presidenziale".to_string()],
                            related_legislation: vec!["Regolamenti parlamentari".to_string()],
                        },
                        ItalyConstitutionalArticle {
                            article_number: 94,
                            title: "Fiducia parlamentare".to_string(),
                            content: "Il Governo deve avere la fiducia delle due Camere.".to_string(),
                            interpretation_notes: vec!["Sistema parlamentare".to_string()],
                            related_legislation: vec!["Legge sulla formazione del governo".to_string()],
                        },
                    ],
                    transitional_provisions: vec![
                        ItalyConstitutionalArticle {
                            article_number: 1,
                            title: "Disposizione transitoria".to_string(),
                            content: "Con l'entrata in vigore della Costituzione il Capo provvisorio dello Stato esercita le attribuzioni di Presidente della Repubblica".to_string(),
                            interpretation_notes: vec!["Transizione monarchia-repubblica".to_string()],
                            related_legislation: vec!["Decreto luogotenenziale".to_string()],
                        },
                    ],
                    adoption_date: "22 dicembre 1947".to_string(),
                    effective_date: "1º gennaio 1948".to_string(),
                    constituent_assembly: "Assemblea Costituente eletta il 2 giugno 1946".to_string(),
                },
                constitutional_principles: ItalyConstitutionalPrinciples {
                    sovereignty_principle: "La sovranità appartiene al popolo, che la esercita nelle forme e nei limiti della Costituzione".to_string(),
                    democratic_principle: "L'Italia è una Repubblica democratica, fondata sul lavoro".to_string(),
                    rule_of_law: "Tutti sono soggetti alla legge".to_string(),
                    separation_powers: "Distinzione tra potere legislativo, esecutivo e giudiziario".to_string(),
                    republican_form: "Forma repubblicana non può essere oggetto di revisione costituzionale".to_string(),
                    regional_autonomy: "Le Regioni sono enti autonomi con propri poteri e funzioni".to_string(),
                    international_cooperation: "L'Italia consente limitazioni di sovranità per assicurare la pace tra le Nazioni".to_string(),
                    pacifist_principle: "L'Italia ripudia la guerra come strumento di offesa alla libertà degli altri popoli".to_string(),
                },
                fundamental_rights: ItalyFundamentalRights {
                    individual_rights: vec![
                        "Diritto alla vita".to_string(),
                        "Libertà personale".to_string(),
                        "Libertà di domicilio".to_string(),
                        "Libertà di comunicazione".to_string(),
                        "Libertà di circolazione".to_string(),
                    ],
                    civil_rights: vec![
                        "Libertà di manifestazione del pensiero".to_string(),
                        "Libertà di riunione".to_string(),
                        "Libertà di associazione".to_string(),
                        "Libertà religiosa".to_string(),
                        "Diritto di petizione".to_string(),
                    ],
                    political_rights: vec![
                        "Diritto di voto".to_string(),
                        "Diritto di accesso agli uffici pubblici".to_string(),
                        "Diritto di associazione politica".to_string(),
                    ],
                    economic_social_rights: vec![
                        "Diritto al lavoro".to_string(),
                        "Diritto allo sciopero".to_string(),
                        "Diritto alla salute".to_string(),
                        "Diritto all'istruzione".to_string(),
                        "Diritto alla previdenza sociale".to_string(),
                    ],
                    cultural_rights: vec![
                        "Libertà dell'arte e della scienza".to_string(),
                        "Diritto all'istruzione".to_string(),
                        "Tutela del paesaggio e patrimonio storico-artistico".to_string(),
                    ],
                    environmental_rights: vec![
                        "Tutela dell'ambiente".to_string(),
                        "Diritto alle generazioni future".to_string(),
                        "Sviluppo sostenibile".to_string(),
                    ],
                },
                constitutional_amendments: vec![
                    ItalyConstitutionalAmendment {
                        amendment_number: 1,
                        year: 2001,
                        title: "Riforma del Titolo V".to_string(),
                        articles_modified: vec![114, 117, 118, 119, 120, 127],
                        referendum_required: false,
                        approval_process: "Approvata a maggioranza dei due terzi".to_string(),
                    },
                    ItalyConstitutionalAmendment {
                        amendment_number: 2,
                        year: 2020,
                        title: "Riduzione del numero dei parlamentari".to_string(),
                        articles_modified: vec![56, 57],
                        referendum_required: true,
                        approval_process: "Confermata da referendum popolare".to_string(),
                    },
                ],
                constitutional_court_decisions: vec![
                    ItalyConstitutionalCourtDecision {
                        decision_number: "Sentenza n. 1146/1988".to_string(),
                        year: 1988,
                        case_title: "Caso Granital".to_string(),
                        constitutional_issue: "Rapporti tra diritto comunitario e diritto interno".to_string(),
                        ruling: "Il diritto comunitario ha efficacia diretta nell'ordinamento italiano".to_string(),
                        legal_principle: "Principio dei controlimiti costituzionali".to_string(),
                        dissenting_opinions: vec![],
                    },
                    ItalyConstitutionalCourtDecision {
                        decision_number: "Sentenza n. 242/2019".to_string(),
                        year: 2019,
                        case_title: "Caso Cappato".to_string(),
                        constitutional_issue: "Aiuto al suicidio assistito".to_string(),
                        ruling: "Non punibilità in determinate condizioni".to_string(),
                        legal_principle: "Bilanciamento tra diritto alla vita e autodeterminazione".to_string(),
                        dissenting_opinions: vec!["Opinione dissenziente del giudice Cartabia".to_string()],
                    },
                ],
            },
            government_structure: ItalyGovernmentStructure {
                executive_branch: ItalyExecutiveBranch {
                    prime_minister: ItalyPrimeMinister {
                        current_holder: "Giorgia Meloni".to_string(),
                        appointment_process: "Nominata dal Presidente della Repubblica".to_string(),
                        powers_responsibilities: vec![
                            "Direzione della politica generale del Governo".to_string(),
                            "Coordinamento dell'attività dei ministri".to_string(),
                            "Rappresentanza del Governo".to_string(),
                        ],
                        term_duration: "Coincide con la legislatura".to_string(),
                        removal_process: "Sfiducia parlamentare o dimissioni".to_string(),
                    },
                    deputy_prime_ministers: vec![
                        "Matteo Salvini (Ministro delle Infrastrutture)".to_string(),
                        "Antonio Tajani (Ministro degli Esteri)".to_string(),
                    ],
                    ministers: vec![
                        ItalyMinister {
                            ministry_name: "Ministero dell'Interno".to_string(),
                            current_minister: "Matteo Piantedosi".to_string(),
                            responsibilities: vec!["Sicurezza pubblica".to_string(), "Immigrazione".to_string(), "Enti locali".to_string()],
                            budget_allocation: "13.2 miliardi EUR".to_string(),
                            staff_count: 125000,
                        },
                        ItalyMinister {
                            ministry_name: "Ministero dell'Economia e delle Finanze".to_string(),
                            current_minister: "Giancarlo Giorgetti".to_string(),
                            responsibilities: vec!["Politica fiscale".to_string(), "Bilancio pubblico".to_string(), "Debito pubblico".to_string()],
                            budget_allocation: "850 miliardi EUR".to_string(),
                            staff_count: 45000,
                        },
                        ItalyMinister {
                            ministry_name: "Ministero della Giustizia".to_string(),
                            current_minister: "Carlo Nordio".to_string(),
                            responsibilities: vec!["Amministrazione della giustizia".to_string(), "Sistema penitenziario".to_string()],
                            budget_allocation: "8.5 miliardi EUR".to_string(),
                            staff_count: 75000,
                        },
                    ],
                    undersecretaries: vec![
                        "Alfredo Mantovano (Sottosegretario alla Presidenza)".to_string(),
                        "Giovanbattista Fazzolari (Sottosegretario alla Presidenza)".to_string(),
                    ],
                    government_formation: "Nomina presidenziale con fiducia parlamentare".to_string(),
                    confidence_mechanism: "Fiducia di entrambe le Camere necessaria".to_string(),
                },
                legislative_branch: ItalyLegislativeBranch {
                    parliament: ItalyParliament {
                        bicameral_system: "Camera dei Deputati e Senato della Repubblica".to_string(),
                        session_duration: "5 anni".to_string(),
                        dissolution_rules: "Scioglimento anticipato possibile".to_string(),
                        immunities_privileges: vec!["Immunità parlamentare".to_string(), "Indennità parlamentare".to_string()],
                        current_legislature: "XIX Legislatura (2022-2027)".to_string(),
                    },
                    chamber_deputies: ItalyChamberDeputies {
                        total_seats: 400,
                        current_composition: vec![
                            ItalyPoliticalGroup {
                                party_name: "Fratelli d'Italia".to_string(),
                                seats_count: 119,
                                leader: "Giorgia Meloni".to_string(),
                                political_orientation: "Destra conservatrice".to_string(),
                            },
                            ItalyPoliticalGroup {
                                party_name: "Partito Democratico".to_string(),
                                seats_count: 69,
                                leader: "Elly Schlein".to_string(),
                                political_orientation: "Centro-sinistra".to_string(),
                            },
                            ItalyPoliticalGroup {
                                party_name: "Movimento 5 Stelle".to_string(),
                                seats_count: 52,
                                leader: "Giuseppe Conte".to_string(),
                                political_orientation: "Populista".to_string(),
                            },
                            ItalyPoliticalGroup {
                                party_name: "Lega".to_string(),
                                seats_count: 66,
                                leader: "Matteo Salvini".to_string(),
                                political_orientation: "Destra populista".to_string(),
                            },
                        ],
                        speaker: "Lorenzo Fontana".to_string(),
                        electoral_system: "Sistema misto maggioritario-proporzionale".to_string(),
                        term_length: "5 anni".to_string(),
                    },
                    senate: ItalySenate {
                        total_seats: 200,
                        current_composition: vec![
                            ItalyPoliticalGroup {
                                party_name: "Fratelli d'Italia".to_string(),
                                seats_count: 65,
                                leader: "Giorgia Meloni".to_string(),
                                political_orientation: "Destra conservatrice".to_string(),
                            },
                            ItalyPoliticalGroup {
                                party_name: "Partito Democratico".to_string(),
                                seats_count: 39,
                                leader: "Elly Schlein".to_string(),
                                political_orientation: "Centro-sinistra".to_string(),
                            },
                            ItalyPoliticalGroup {
                                party_name: "Lega".to_string(),
                                seats_count: 29,
                                leader: "Matteo Salvini".to_string(),
                                political_orientation: "Destra populista".to_string(),
                            },
                        ],
                        president: "Ignazio La Russa".to_string(),
                        electoral_system: "Sistema misto maggioritario-proporzionale".to_string(),
                        senators_for_life: vec![
                            "Giorgio Napolitano".to_string(),
                            "Mario Monti".to_string(),
                            "Renzo Piano".to_string(),
                            "Carlo Rubbia".to_string(),
                            "Elena Cattaneo".to_string(),
                        ],
                    },
                    legislative_process: ItalyLegislativeProcess {
                        ordinary_procedure: "Approvazione bicamerale".to_string(),
                        committee_procedure: "Deliberazione in sede legislativa".to_string(),
                        constitutional_laws: "Doppia deliberazione e possibile referendum".to_string(),
                        government_bills: "Disegni di legge governativi".to_string(),
                    },
                    parliamentary_committees: vec![
                        ItalyParliamentaryCommittee {
                            name: "Commissione Giustizia".to_string(),
                            chamber: "Camera dei Deputati".to_string(),
                            president: "Ciro Maschio".to_string(),
                            competencies: vec!["Legislazione penale".to_string(), "Ordinamento giudiziario".to_string()],
                        },
                        ItalyParliamentaryCommittee {
                            name: "Commissione Bilancio".to_string(),
                            chamber: "Senato".to_string(),
                            president: "Nicola Calandrini".to_string(),
                            competencies: vec!["Legge di bilancio".to_string(), "Finanza pubblica".to_string()],
                        },
                    ],
                },
                head_of_state: ItalyHeadOfState {
                    president: ItalyPresident {
                        current_president: "Sergio Mattarella".to_string(),
                        election_process: "Elezione parlamentare con rappresentanti regionali".to_string(),
                        term_duration: "7 anni, rieleggibile".to_string(),
                        eligibility_requirements: vec![
                            "Cittadinanza italiana".to_string(),
                            "Età minima 50 anni".to_string(),
                            "Godimento diritti civili e politici".to_string(),
                        ],
                        impeachment_process: "Messa in stato d'accusa per alto tradimento".to_string(),
                        residence: "Palazzo del Quirinale, Roma".to_string(),
                    },
                    powers: vec![
                        "Nomina del Presidente del Consiglio".to_string(),
                        "Scioglimento delle Camere".to_string(),
                        "Promulgazione delle leggi".to_string(),
                        "Nomina di alti funzionari".to_string(),
                        "Grazia e commutazione pene".to_string(),
                    ],
                    ceremonial_functions: vec![
                        "Rappresentanza dell'unità nazionale".to_string(),
                        "Cerimonie di Stato".to_string(),
                        "Conferimento onorificenze".to_string(),
                    ],
                    emergency_powers: vec![
                        "Decreti-legge in caso di necessità e urgenza".to_string(),
                        "Poteri di emergenza costituzionale".to_string(),
                    ],
                },
                council_ministers: ItalyCouncilMinisters {
                    composition: "Presidente del Consiglio e Ministri".to_string(),
                    meeting_frequency: "Settimanale o su convocazione".to_string(),
                    decision_making: "Deliberazioni collegiali".to_string(),
                    secretariat: "Segretariato Generale della Presidenza del Consiglio".to_string(),
                },
                administrative_system: ItalyAdministrativeSystem {
                    central_administration: "Ministeri e agenzie centrali".to_string(),
                    peripheral_administration: "Prefetture e uffici territoriali".to_string(),
                    independent_authorities: vec![
                        "Banca d'Italia".to_string(),
                        "CONSOB".to_string(),
                        "Autorità Antitrust".to_string(),
                        "Garante Privacy".to_string(),
                    ],
                    public_employment: "Contratti collettivi pubblici".to_string(),
                },
            },
            territorial_organization: ItalyTerritorialOrganization {
                regions: vec![
                    ItalyRegion {
                        name: "Lombardia".to_string(),
                        capital: "Milano".to_string(),
                        population: 10027602,
                        area_km2: 23844.0,
                        gdp_euros: 400000000000,
                        regional_council: ItalyRegionalCouncil {
                            seats: 80,
                            president: "Alessandro Fermi".to_string(),
                            political_composition: vec![
                                ItalyPoliticalGroup {
                                    party_name: "Lega Lombarda".to_string(),
                                    seats_count: 34,
                                    leader: "Attilio Fontana".to_string(),
                                    political_orientation: "Centro-destra".to_string(),
                                },
                            ],
                            term_duration: "5 anni".to_string(),
                        },
                        regional_government: ItalyRegionalGovernment {
                            regional_president: "Attilio Fontana".to_string(),
                            regional_ministers: vec![
                                "Guido Bertolaso (Welfare)".to_string(),
                                "Franco Lucente (Bilancio)".to_string(),
                            ],
                            budget_euros: 22000000000,
                            competencies: vec!["Sanità".to_string(), "Trasporti".to_string(), "Ambiente".to_string()],
                        },
                        statute: "Statuto della Regione Lombardia".to_string(),
                        legislative_powers: vec!["Sanità".to_string(), "Istruzione".to_string(), "Trasporti locali".to_string()],
                    },
                    ItalyRegion {
                        name: "Lazio".to_string(),
                        capital: "Roma".to_string(),
                        population: 5755700,
                        area_km2: 17232.0,
                        gdp_euros: 195000000000,
                        regional_council: ItalyRegionalCouncil {
                            seats: 50,
                            president: "Marco Vincenzi".to_string(),
                            political_composition: vec![
                                ItalyPoliticalGroup {
                                    party_name: "Partito Democratico".to_string(),
                                    seats_count: 18,
                                    leader: "Nicola Zingaretti".to_string(),
                                    political_orientation: "Centro-sinistra".to_string(),
                                },
                            ],
                            term_duration: "5 anni".to_string(),
                        },
                        regional_government: ItalyRegionalGovernment {
                            regional_president: "Francesco Rocca".to_string(),
                            regional_ministers: vec![
                                "Massimiliano Maselli (Bilancio)".to_string(),
                                "Elena Palazzo (Sanità)".to_string(),
                            ],
                            budget_euros: 15000000000,
                            competencies: vec!["Sanità".to_string(), "Trasporti".to_string(), "Turismo".to_string()],
                        },
                        statute: "Statuto della Regione Lazio".to_string(),
                        legislative_powers: vec!["Sanità".to_string(), "Istruzione".to_string(), "Ambiente".to_string()],
                    },
                    ItalyRegion {
                        name: "Sicilia".to_string(),
                        capital: "Palermo".to_string(),
                        population: 4875290,
                        area_km2: 25711.0,
                        gdp_euros: 89000000000,
                        regional_council: ItalyRegionalCouncil {
                            seats: 70,
                            president: "Gaetano Galvagno".to_string(),
                            political_composition: vec![
                                ItalyPoliticalGroup {
                                    party_name: "Fratelli d'Italia".to_string(),
                                    seats_count: 15,
                                    leader: "Renato Schifani".to_string(),
                                    political_orientation: "Centro-destra".to_string(),
                                },
                            ],
                            term_duration: "5 anni".to_string(),
                        },
                        regional_government: ItalyRegionalGovernment {
                            regional_president: "Renato Schifani".to_string(),
                            regional_ministers: vec![
                                "Marco Falcone (Economia)".to_string(),
                                "Giovanna Volo (Sanità)".to_string(),
                            ],
                            budget_euros: 8500000000,
                            competencies: vec!["Sanità".to_string(), "Agricoltura".to_string(), "Trasporti".to_string()],
                        },
                        statute: "Statuto Speciale della Regione Siciliana".to_string(),
                        legislative_powers: vec!["Autonomia legislativa speciale".to_string(), "Finanze regionali".to_string()],
                    },
                ],
                provinces: vec![
                    ItalyProvince {
                        name: "Milano".to_string(),
                        capital: "Milano".to_string(),
                        population: 3250315,
                        municipalities_count: 133,
                        president: "Arianna Censi".to_string(),
                    },
                    ItalyProvince {
                        name: "Roma".to_string(),
                        capital: "Roma".to_string(),
                        population: 4342212,
                        municipalities_count: 121,
                        president: "Pietro Tidei".to_string(),
                    },
                ],
                municipalities: vec![
                    ItalyMunicipality {
                        name: "Roma".to_string(),
                        province: "Roma Capitale".to_string(),
                        population: 2761632,
                        mayor: "Roberto Gualtieri".to_string(),
                        municipal_council_seats: 48,
                    },
                    ItalyMunicipality {
                        name: "Milano".to_string(),
                        province: "Milano".to_string(),
                        population: 1396059,
                        mayor: "Giuseppe Sala".to_string(),
                        municipal_council_seats: 48,
                    },
                ],
                metropolitan_cities: vec![
                    ItalyMetropolitanCity {
                        name: "Città Metropolitana di Roma Capitale".to_string(),
                        metropolitan_mayor: "Roberto Gualtieri".to_string(),
                        population: 4342212,
                        municipalities_included: vec!["Roma".to_string(), "Fiumicino".to_string(), "Tivoli".to_string()],
                        special_powers: vec!["Pianificazione territoriale".to_string(), "Trasporti metropolitani".to_string()],
                    },
                    ItalyMetropolitanCity {
                        name: "Città Metropolitana di Milano".to_string(),
                        metropolitan_mayor: "Giuseppe Sala".to_string(),
                        population: 3250315,
                        municipalities_included: vec!["Milano".to_string(), "Monza".to_string(), "Bergamo".to_string()],
                        special_powers: vec!["Trasporti pubblici".to_string(), "Ambiente".to_string()],
                    },
                ],
                special_autonomy: vec![
                    ItalySpecialAutonomy {
                        territory_name: "Valle d'Aosta".to_string(),
                        autonomy_type: "Regione a Statuto Speciale".to_string(),
                        special_statute: "Statuto Speciale per la Valle d'Aosta".to_string(),
                        legislative_powers: vec!["Bilinguismo franco-italiano".to_string(), "Autonomia fiscale".to_string()],
                        financial_autonomy: "Trattiene 90% delle imposte erogate".to_string(),
                    },
                    ItalySpecialAutonomy {
                        territory_name: "Trentino-Alto Adige".to_string(),
                        autonomy_type: "Regione a Statuto Speciale".to_string(),
                        special_statute: "Statuto Speciale per il Trentino-Alto Adige".to_string(),
                        legislative_powers: vec!["Trilinguismo italiano-tedesco-ladino".to_string(), "Autonomia provinciale".to_string()],
                        financial_autonomy: "Trattenimento 90% imposte dirette".to_string(),
                    },
                ],
            },
            judicial_system: ItalyJudicialSystem {
                supreme_court: ItalySupremeCourt {
                    official_name: "Corte Suprema di Cassazione".to_string(),
                    location: "Palazzo di Giustizia, Roma".to_string(),
                    chief_justice: "Pietro Curzio".to_string(),
                    total_judges: 350,
                    sections: vec![
                        ItalySupremeCourtSection {
                            section_name: "Sezioni Unite Civili".to_string(),
                            president: "Giuseppe Acierno".to_string(),
                            specialization: "Diritto civile".to_string(),
                            judges_count: 15,
                        },
                        ItalySupremeCourtSection {
                            section_name: "Sezioni Unite Penali".to_string(),
                            president: "Elisabetta Cesqui".to_string(),
                            specialization: "Diritto penale".to_string(),
                            judges_count: 15,
                        },
                    ],
                    jurisdiction: vec![
                        "Ricorsi per Cassazione".to_string(),
                        "Regolamento di giurisdizione".to_string(),
                        "Ricorsi per conflitto di competenza".to_string(),
                    ],
                },
                constitutional_court: ItalyConstitutionalCourt {
                    official_name: "Corte Costituzionale".to_string(),
                    location: "Palazzo della Consulta, Roma".to_string(),
                    president: "Augusto Barbera".to_string(),
                    total_judges: 15,
                    appointment_process: "5 nominati dal Presidente della Repubblica, 5 dal Parlamento, 5 dalle magistrature superiori".to_string(),
                    jurisdiction: vec![
                        "Controllo costituzionalità leggi".to_string(),
                        "Conflitti tra poteri dello Stato".to_string(),
                        "Conflitti tra Stato e Regioni".to_string(),
                        "Giudizi penali contro Presidente della Repubblica".to_string(),
                    ],
                    landmark_decisions: vec![
                        ItalyConstitutionalCourtDecision {
                            decision_number: "Sentenza n. 170/1984".to_string(),
                            year: 1984,
                            case_title: "Caso Granital".to_string(),
                            constitutional_issue: "Rapporti tra ordinamento comunitario e costituzionale".to_string(),
                            ruling: "Prevalenza del diritto comunitario con riserva dei principi supremi".to_string(),
                            legal_principle: "Teoria dei controlimiti".to_string(),
                            dissenting_opinions: vec![],
                        },
                        ItalyConstitutionalCourtDecision {
                            decision_number: "Sentenza n. 242/2019".to_string(),
                            year: 2019,
                            case_title: "Caso Cappato - DJ Fabo".to_string(),
                            constitutional_issue: "Aiuto al suicidio assistito".to_string(),
                            ruling: "Non punibilità nelle condizioni specifiche indicate".to_string(),
                            legal_principle: "Bilanciamento tra tutela vita e autodeterminazione".to_string(),
                            dissenting_opinions: vec!["Riserve sulla discrezionalità legislativa".to_string()],
                        },
                    ],
                },
                council_state: ItalyCouncilOfState {
                    official_name: "Consiglio di Stato".to_string(),
                    president: "Filippo Patroni Griffi".to_string(),
                    advisory_functions: vec![
                        "Pareri su schemi di regolamenti".to_string(),
                        "Pareri su ricorsi straordinari".to_string(),
                    ],
                    judicial_functions: vec![
                        "Giudice d'appello amministrativo".to_string(),
                        "Giurisdizione esclusiva in materie specifiche".to_string(),
                    ],
                    sections: vec![
                        "Sezioni consultive".to_string(),
                        "Sezioni giurisdizionali".to_string(),
                    ],
                },
                court_audit: ItalyCourtOfAudit {
                    official_name: "Corte dei Conti".to_string(),
                    president: "Guido Carlino".to_string(),
                    control_functions: vec![
                        "Controllo preventivo legittimità".to_string(),
                        "Controllo successivo gestione".to_string(),
                        "Controllo sistemi contabili".to_string(),
                    ],
                    judicial_functions: vec![
                        "Giudizio responsabilità amministrativa".to_string(),
                        "Giudizio pensioni".to_string(),
                    ],
                    regional_sections: 20,
                },
                ordinary_jurisdiction: ItalyOrdinaryJurisdiction {
                    first_instance: "Tribunali ordinari e specializzati".to_string(),
                    appeal: "Corti d'Appello".to_string(),
                    cassation: "Corte Suprema di Cassazione".to_string(),
                    specialized_courts: vec![
                        "Tribunali per i minorenni".to_string(),
                        "Tribunali di sorveglianza".to_string(),
                        "Tribunali del lavoro".to_string(),
                    ],
                },
                administrative_jurisdiction: ItalyAdministrativeJurisdiction {
                    first_instance: "Tribunali Amministrativi Regionali (TAR)".to_string(),
                    appeal: "Consiglio di Stato".to_string(),
                    specialized_jurisdiction: vec![
                        "Appalti pubblici".to_string(),
                        "Servizi pubblici".to_string(),
                        "Urbanistica".to_string(),
                    ],
                },
                prosecution_system: ItalyProsecutionSystem {
                    prosecutor_general: "Giovanni Salvi".to_string(),
                    structure: "Procure della Repubblica presso i Tribunali".to_string(),
                    specialized_prosecutors: vec![
                        "Direzione Nazionale Antimafia".to_string(),
                        "Direzione Investigativa Antimafia".to_string(),
                        "Procura Nazionale Anticorruzione".to_string(),
                    ],
                    independence_guarantees: "Autogoverno della magistratura tramite CSM".to_string(),
                },
            },
            legal_codes: ItalyLegalCodes {
                civil_code: ItalyCivilCode {
                    official_name: "Codice Civile del 1942".to_string(),
                    promulgation_date: "16 marzo 1942".to_string(),
                    structure: vec![
                        ItalyCodeBook {
                            book_number: 1,
                            title: "Delle Persone e della Famiglia".to_string(),
                            articles_range: "Art. 1-455".to_string(),
                            main_topics: vec!["Capacità giuridica".to_string(), "Matrimonio".to_string(), "Filiazione".to_string()],
                            key_articles: vec![
                                ItalyCodeArticle {
                                    article_number: 1,
                                    title: "Capacità giuridica".to_string(),
                                    content: "La capacità giuridica si acquista dal momento della nascita. I diritti che la legge riconosce a favore del concepito sono subordinati all'evento della nascita.".to_string(),
                                    amendments: vec!["Riforma del diritto di famiglia 1975".to_string()],
                                },
                                ItalyCodeArticle {
                                    article_number: 143,
                                    title: "Diritti e doveri reciproci dei coniugi".to_string(),
                                    content: "Con il matrimonio il marito e la moglie acquistano gli stessi diritti e assumono i medesimi doveri.".to_string(),
                                    amendments: vec!["Legge 19 maggio 1975, n. 151".to_string()],
                                },
                            ],
                        },
                        ItalyCodeBook {
                            book_number: 4,
                            title: "Delle Obbligazioni".to_string(),
                            articles_range: "Art. 1173-2059".to_string(),
                            main_topics: vec!["Contratti".to_string(), "Responsabilità extracontrattuale".to_string(), "Garanzie".to_string()],
                            key_articles: vec![
                                ItalyCodeArticle {
                                    article_number: 1321,
                                    title: "Nozione di contratto".to_string(),
                                    content: "Il contratto è l'accordo di due o più parti per costituire, regolare o estinguere tra loro un rapporto giuridico patrimoniale.".to_string(),
                                    amendments: vec!["Codice del consumo 2005".to_string()],
                                },
                                ItalyCodeArticle {
                                    article_number: 2043,
                                    title: "Risarcimento per fatto illecito".to_string(),
                                    content: "Qualunque fatto doloso o colposo, che cagiona ad altri un danno ingiusto, obbliga colui che ha commesso il fatto a risarcire il danno.".to_string(),
                                    amendments: vec![],
                                },
                            ],
                        },
                    ],
                    total_articles: 2969,
                    major_reforms: vec![
                        "Riforma del diritto di famiglia (1975)".to_string(),
                        "Riforma della filiazione (2012-2013)".to_string(),
                        "Unioni civili (2016)".to_string(),
                    ],
                },
                criminal_code: ItalyCriminalCode {
                    official_name: "Codice Penale del 1930 (Codice Rocco)".to_string(),
                    promulgation_date: "19 ottobre 1930".to_string(),
                    structure: vec![
                        ItalyCodeBook {
                            book_number: 1,
                            title: "Dei reati in generale".to_string(),
                            articles_range: "Art. 1-240".to_string(),
                            main_topics: vec!["Principi generali".to_string(), "Elemento soggettivo".to_string(), "Circostanze".to_string()],
                            key_articles: vec![
                                ItalyCodeArticle {
                                    article_number: 1,
                                    title: "Reati e pene: disposizione espressa di legge".to_string(),
                                    content: "Nessuno può essere punito per un fatto che non sia espressamente preveduto come reato dalla legge, né con pene che non siano da essa stabilite.".to_string(),
                                    amendments: vec![],
                                },
                            ],
                        },
                    ],
                    total_articles: 734,
                    recent_reforms: vec![
                        "Riforma Cartabia (2021-2022)".to_string(),
                        "Codice Rosso - violenza di genere (2019)".to_string(),
                        "Riforma prescrizione (2020)".to_string(),
                    ],
                },
                administrative_code: ItalyAdministrativeCode {
                    official_name: "Codice dell'Amministrazione Digitale".to_string(),
                    promulgation_date: "7 marzo 2005".to_string(),
                    main_principles: vec![
                        "Digitalizzazione amministrazione".to_string(),
                        "Diritti digitali cittadini".to_string(),
                        "Semplificazione procedimenti".to_string(),
                    ],
                },
                commercial_code: ItalyCommercialCode {
                    integration_civil_code: "Disposizioni integrate nel Codice Civile".to_string(),
                    commercial_law_sources: vec![
                        "Codice Civile - Libro V".to_string(),
                        "Decreto Legislativo 385/1993 (TUB)".to_string(),
                        "Decreto Legislativo 58/1998 (TUF)".to_string(),
                    ],
                },
                labor_code: ItalyLaborCode {
                    main_legislation: "Statuto dei Lavoratori (Legge 300/1970)".to_string(),
                    recent_reforms: vec![
                        "Jobs Act (2014-2015)".to_string(),
                        "Decreto Dignità (2018)".to_string(),
                    ],
                    key_principles: vec![
                        "Tutela del posto di lavoro".to_string(),
                        "Libertà sindacale".to_string(),
                        "Diritto di sciopero".to_string(),
                    ],
                },
                tax_code: ItalyTaxCode {
                    main_legislation: "Decreto del Presidente della Repubblica 600/1973".to_string(),
                    structure: vec![
                        "Imposte dirette".to_string(),
                        "Imposte indirette".to_string(),
                        "Procedimenti tributari".to_string(),
                    ],
                    recent_reforms: vec![
                        "Riforma fiscale 2022".to_string(),
                        "Decreto sostegni bis".to_string(),
                    ],
                },
            },
            human_rights: ItalyHumanRights {
                constitutional_guarantees: vec![
                    "Diritti inviolabili dell'uomo (Art. 2 Cost.)".to_string(),
                    "Principio di uguaglianza (Art. 3 Cost.)".to_string(),
                    "Libertà personale (Art. 13 Cost.)".to_string(),
                    "Libertà di manifestazione del pensiero (Art. 21 Cost.)".to_string(),
                ],
                international_treaties: vec![
                    "Convenzione Europea dei Diritti dell'Uomo".to_string(),
                    "Patto Internazionale sui Diritti Civili e Politici".to_string(),
                    "Convenzione contro la Tortura".to_string(),
                    "Convenzione sui Diritti del Bambino".to_string(),
                ],
                ombudsman_system: ItalyOmbudsmanSystem {
                    national_ombudsman: "Autorità Garante per l'Infanzia e l'Adolescenza".to_string(),
                    regional_ombudsmen: vec![
                        "Difensore Civico Regione Lazio".to_string(),
                        "Difensore Civico Regione Lombardia".to_string(),
                    ],
                    competencies: vec![
                        "Tutela diritti cittadini".to_string(),
                        "Controllo amministrazione pubblica".to_string(),
                    ],
                    reporting_mechanism: "Relazione annuale al Parlamento".to_string(),
                },
                anti_discrimination: vec![
                    "Decreto Legislativo 215/2003 (discriminazione razziale)".to_string(),
                    "Decreto Legislativo 216/2003 (discriminazione lavoro)".to_string(),
                    "Legge Mancino contro odio razziale".to_string(),
                ],
                data_protection: ItalyDataProtection {
                    authority: "Garante per la Protezione dei Dati Personali".to_string(),
                    legal_framework: vec![
                        "Regolamento UE 2016/679 (GDPR)".to_string(),
                        "Decreto Legislativo 196/2003 (Codice Privacy)".to_string(),
                    ],
                    individual_rights: vec![
                        "Diritto di accesso".to_string(),
                        "Diritto di rettifica".to_string(),
                        "Diritto alla cancellazione".to_string(),
                        "Diritto alla portabilità".to_string(),
                    ],
                    enforcement_mechanisms: vec![
                        "Sanzioni amministrative".to_string(),
                        "Ordini di cessazione trattamento".to_string(),
                    ],
                },
            },
            eu_integration: ItalyEUIntegration {
                membership_date: "25 marzo 1957 (membro fondatore CEE)".to_string(),
                euro_adoption: "1º gennaio 1999 (1º gennaio 2002 circolazione)".to_string(),
                schengen_participation: "26 ottobre 1997".to_string(),
                eu_institutions_representation: ItalyEURepresentation {
                    european_parliament_seats: 76,
                    council_votes: 29,
                    commissioners: vec!["Paolo Gentiloni (Economia)".to_string()],
                    permanent_representative: "Ambasciatore Luca Gori".to_string(),
                },
                european_law_implementation: vec![
                    "Legge La Pergola (1989)".to_string(),
                    "Legge Buttiglione (2005)".to_string(),
                    "Legge 234/2012 (nuove norme partecipazione UE)".to_string(),
                ],
            },
        }
    }
}

// Additional implementation structs needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLegislativeProcess {
    pub ordinary_procedure: String,
    pub committee_procedure: String,
    pub constitutional_laws: String,
    pub government_bills: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyParliamentaryCommittee {
    pub name: String,
    pub chamber: String,
    pub president: String,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCouncilMinisters {
    pub composition: String,
    pub meeting_frequency: String,
    pub decision_making: String,
    pub secretariat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyAdministrativeSystem {
    pub central_administration: String,
    pub peripheral_administration: String,
    pub independent_authorities: Vec<String>,
    pub public_employment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCourtOfAudit {
    pub official_name: String,
    pub president: String,
    pub control_functions: Vec<String>,
    pub judicial_functions: Vec<String>,
    pub regional_sections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyOrdinaryJurisdiction {
    pub first_instance: String,
    pub appeal: String,
    pub cassation: String,
    pub specialized_courts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyAdministrativeJurisdiction {
    pub first_instance: String,
    pub appeal: String,
    pub specialized_jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyProsecutionSystem {
    pub prosecutor_general: String,
    pub structure: String,
    pub specialized_prosecutors: Vec<String>,
    pub independence_guarantees: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyAdministrativeCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub main_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyCommercialCode {
    pub integration_civil_code: String,
    pub commercial_law_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLaborCode {
    pub main_legislation: String,
    pub recent_reforms: Vec<String>,
    pub key_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyTaxCode {
    pub main_legislation: String,
    pub structure: Vec<String>,
    pub recent_reforms: Vec<String>,
}

impl Default for ItalyLegislativeProcess {
    fn default() -> Self {
        Self {
            ordinary_procedure: "Approvazione bicamerale".to_string(),
            committee_procedure: "Deliberazione in sede legislativa".to_string(),
            constitutional_laws: "Doppia deliberazione e possibile referendum".to_string(),
            government_bills: "Disegni di legge governativi".to_string(),
        }
    }
}

impl Default for ItalyParliamentaryCommittee {
    fn default() -> Self {
        Self {
            name: String::new(),
            chamber: String::new(),
            president: String::new(),
            competencies: vec![],
        }
    }
}

impl Default for ItalyCouncilMinisters {
    fn default() -> Self {
        Self {
            composition: "Presidente del Consiglio e Ministri".to_string(),
            meeting_frequency: "Settimanale o su convocazione".to_string(),
            decision_making: "Deliberazioni collegiali".to_string(),
            secretariat: "Segretariato Generale della Presidenza del Consiglio".to_string(),
        }
    }
}

impl Default for ItalyAdministrativeSystem {
    fn default() -> Self {
        Self {
            central_administration: "Ministeri e agenzie centrali".to_string(),
            peripheral_administration: "Prefetture e uffici territoriali".to_string(),
            independent_authorities: vec![],
            public_employment: "Contratti collettivi pubblici".to_string(),
        }
    }
}

impl Default for ItalyCourtOfAudit {
    fn default() -> Self {
        Self {
            official_name: "Corte dei Conti".to_string(),
            president: String::new(),
            control_functions: vec![],
            judicial_functions: vec![],
            regional_sections: 0,
        }
    }
}

impl Default for ItalyOrdinaryJurisdiction {
    fn default() -> Self {
        Self {
            first_instance: "Tribunali ordinari e specializzati".to_string(),
            appeal: "Corti d'Appello".to_string(),
            cassation: "Corte Suprema di Cassazione".to_string(),
            specialized_courts: vec![],
        }
    }
}

impl Default for ItalyAdministrativeJurisdiction {
    fn default() -> Self {
        Self {
            first_instance: "Tribunali Amministrativi Regionali (TAR)".to_string(),
            appeal: "Consiglio di Stato".to_string(),
            specialized_jurisdiction: vec![],
        }
    }
}

impl Default for ItalyProsecutionSystem {
    fn default() -> Self {
        Self {
            prosecutor_general: String::new(),
            structure: "Procure della Repubblica presso i Tribunali".to_string(),
            specialized_prosecutors: vec![],
            independence_guarantees: "Autogoverno della magistratura tramite CSM".to_string(),
        }
    }
}

impl Default for ItalyAdministrativeCode {
    fn default() -> Self {
        Self {
            official_name: "Codice dell'Amministrazione Digitale".to_string(),
            promulgation_date: "7 marzo 2005".to_string(),
            main_principles: vec![],
        }
    }
}

impl Default for ItalyCommercialCode {
    fn default() -> Self {
        Self {
            integration_civil_code: "Disposizioni integrate nel Codice Civile".to_string(),
            commercial_law_sources: vec![],
        }
    }
}

impl Default for ItalyLaborCode {
    fn default() -> Self {
        Self {
            main_legislation: "Statuto dei Lavoratori (Legge 300/1970)".to_string(),
            recent_reforms: vec![],
            key_principles: vec![],
        }
    }
}

impl Default for ItalyTaxCode {
    fn default() -> Self {
        Self {
            main_legislation: "Decreto del Presidente della Repubblica 600/1973".to_string(),
            structure: vec![],
            recent_reforms: vec![],
        }
    }
}

pub fn create_italy_legal_system() -> ItalyLegalSystem {
    ItalyLegalSystem::default()
}