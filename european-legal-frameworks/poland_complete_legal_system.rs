use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandLegalSystem {
    pub constitutional_framework: PolandConstitutionalFramework,
    pub government_structure: PolandGovernmentStructure,
    pub territorial_organization: PolandTerritorialOrganization,
    pub judicial_system: PolandJudicialSystem,
    pub legal_codes: PolandLegalCodes,
    pub human_rights: PolandHumanRights,
    pub eu_integration: PolandEUIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalFramework {
    pub constitution_1997: PolandConstitution1997,
    pub constitutional_principles: PolandConstitutionalPrinciples,
    pub fundamental_rights: PolandFundamentalRights,
    pub constitutional_amendments: Vec<PolandConstitutionalAmendment>,
    pub constitutional_court_decisions: Vec<PolandConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitution1997 {
    pub preamble: String,
    pub chapter_one_republic: Vec<PolandConstitutionalArticle>,
    pub chapter_two_freedoms_rights_duties: Vec<PolandConstitutionalArticle>,
    pub chapter_three_sources_law: Vec<PolandConstitutionalArticle>,
    pub chapter_four_sejm_senate: Vec<PolandConstitutionalArticle>,
    pub chapter_five_president: Vec<PolandConstitutionalArticle>,
    pub chapter_six_council_ministers: Vec<PolandConstitutionalArticle>,
    pub chapter_eight_courts_tribunals: Vec<PolandConstitutionalArticle>,
    pub adoption_date: String,
    pub referendum_date: String,
    pub effective_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub interpretation_notes: Vec<String>,
    pub related_legislation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalPrinciples {
    pub democratic_state: String,
    pub rule_of_law: String,
    pub separation_powers: String,
    pub decentralization: String,
    pub social_market_economy: String,
    pub human_dignity: String,
    pub political_pluralism: String,
    pub subsidiarity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandFundamentalRights {
    pub individual_rights: Vec<String>,
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub economic_social_rights: Vec<String>,
    pub cultural_rights: Vec<String>,
    pub environmental_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandGovernmentStructure {
    pub executive_branch: PolandExecutiveBranch,
    pub legislative_branch: PolandLegislativeBranch,
    pub head_of_state: PolandHeadOfState,
    pub council_ministers: PolandCouncilMinisters,
    pub administrative_system: PolandAdministrativeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandExecutiveBranch {
    pub prime_minister: PolandPrimeMinister,
    pub deputy_prime_ministers: Vec<String>,
    pub ministers: Vec<PolandMinister>,
    pub government_formation: String,
    pub confidence_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandPrimeMinister {
    pub current_holder: String,
    pub appointment_process: String,
    pub powers_responsibilities: Vec<String>,
    pub term_duration: String,
    pub removal_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandMinister {
    pub ministry_name: String,
    pub current_minister: String,
    pub responsibilities: Vec<String>,
    pub budget_allocation: String,
    pub staff_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandLegislativeBranch {
    pub national_assembly: PolandNationalAssembly,
    pub sejm: PolandSejm,
    pub senate: PolandSenate,
    pub legislative_process: PolandLegislativeProcess,
    pub parliamentary_groups: Vec<PolandParliamentaryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandNationalAssembly {
    pub bicameral_system: String,
    pub session_duration: String,
    pub dissolution_rules: String,
    pub immunities_privileges: Vec<String>,
    pub current_term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandSejm {
    pub total_seats: u32,
    pub current_composition: Vec<PolandPoliticalGroup>,
    pub speaker: String,
    pub electoral_system: String,
    pub term_length: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandSenate {
    pub total_seats: u32,
    pub current_composition: Vec<PolandPoliticalGroup>,
    pub speaker: String,
    pub electoral_system: String,
    pub territorial_representation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandPoliticalGroup {
    pub party_name: String,
    pub seats_count: u32,
    pub leader: String,
    pub political_orientation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandHeadOfState {
    pub president: PolandPresident,
    pub powers: Vec<String>,
    pub ceremonial_functions: Vec<String>,
    pub emergency_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandPresident {
    pub current_president: String,
    pub election_process: String,
    pub term_duration: String,
    pub eligibility_requirements: Vec<String>,
    pub impeachment_process: String,
    pub residence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandTerritorialOrganization {
    pub voivodeships: Vec<PolandVoivodeship>,
    pub counties: Vec<PolandCounty>,
    pub municipalities: Vec<PolandMunicipality>,
    pub territorial_self_government: PolandTerritorialSelfGovernment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandVoivodeship {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub area_km2: f64,
    pub gdp_pln: u64,
    pub voivode: String,
    pub regional_assembly: PolandRegionalAssembly,
    pub regional_government: PolandRegionalGovernment,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandRegionalAssembly {
    pub name: String,
    pub seats: u32,
    pub speaker: String,
    pub political_composition: Vec<PolandPoliticalGroup>,
    pub term_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandRegionalGovernment {
    pub marshal: String,
    pub board_members: Vec<String>,
    pub budget_pln: u64,
    pub executive_competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCounty {
    pub name: String,
    pub seat: String,
    pub population: u64,
    pub voivodeship: String,
    pub starosta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandMunicipality {
    pub name: String,
    pub county: String,
    pub population: u64,
    pub mayor_type: String,
    pub mayor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandJudicialSystem {
    pub supreme_court: PolandSupremeCourt,
    pub constitutional_tribunal: PolandConstitutionalTribunal,
    pub supreme_administrative_court: PolandSupremeAdministrativeCourt,
    pub national_council_judiciary: PolandNationalCouncilJudiciary,
    pub ordinary_courts: PolandOrdinaryCourts,
    pub administrative_courts: PolandAdministrativeCourts,
    pub prosecution_system: PolandProsecutionSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandSupremeCourt {
    pub official_name: String,
    pub location: String,
    pub first_president: String,
    pub total_judges: u32,
    pub chambers: Vec<PolandSupremeCourtChamber>,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandSupremeCourtChamber {
    pub chamber_name: String,
    pub president: String,
    pub specialization: String,
    pub judges_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalTribunal {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub total_judges: u32,
    pub appointment_process: String,
    pub jurisdiction: Vec<String>,
    pub landmark_decisions: Vec<PolandConstitutionalCourtDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalCourtDecision {
    pub decision_number: String,
    pub year: u32,
    pub case_title: String,
    pub constitutional_issue: String,
    pub ruling: String,
    pub legal_principle: String,
    pub dissenting_opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandLegalCodes {
    pub civil_code: PolandCivilCode,
    pub criminal_code: PolandCriminalCode,
    pub administrative_code: PolandAdministrativeCode,
    pub labor_code: PolandLaborCode,
    pub commercial_code: PolandCommercialCode,
    pub tax_code: PolandTaxCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCivilCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<PolandCodeBook>,
    pub total_articles: u32,
    pub major_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCodeBook {
    pub book_number: u32,
    pub title: String,
    pub articles_range: String,
    pub main_topics: Vec<String>,
    pub key_articles: Vec<PolandCodeArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCodeArticle {
    pub article_number: u32,
    pub title: String,
    pub content: String,
    pub amendments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCriminalCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub structure: Vec<PolandCodeBook>,
    pub total_articles: u32,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandConstitutionalAmendment {
    pub amendment_number: u32,
    pub year: u32,
    pub title: String,
    pub articles_modified: Vec<u32>,
    pub approval_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandHumanRights {
    pub constitutional_guarantees: Vec<String>,
    pub international_treaties: Vec<String>,
    pub ombudsman_system: PolandOmbudsmanSystem,
    pub anti_discrimination: Vec<String>,
    pub data_protection: PolandDataProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandOmbudsmanSystem {
    pub human_rights_defender: String,
    pub specialized_ombudsmen: Vec<String>,
    pub competencies: Vec<String>,
    pub reporting_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandDataProtection {
    pub authority: String,
    pub legal_framework: Vec<String>,
    pub individual_rights: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandEUIntegration {
    pub membership_date: String,
    pub euro_adoption_status: String,
    pub schengen_participation: String,
    pub eu_institutions_representation: PolandEURepresentation,
    pub european_law_implementation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandEURepresentation {
    pub european_parliament_seats: u32,
    pub council_votes: u32,
    pub commissioners: Vec<String>,
    pub permanent_representative: String,
}

impl Default for PolandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: PolandConstitutionalFramework {
                constitution_1997: PolandConstitution1997 {
                    preamble: "My, Naród Polski - wszyscy obywatele Rzeczypospolitej, zarówno wierzący w Boga będącego źródłem prawdy, sprawiedliwości, dobra i piękna, jak i nie podzielający tej wiary, a te uniwersalne wartości wywodzący z innych źródeł, równi w prawach i w powinnościach wobec dobra wspólnego - Polski".to_string(),
                    chapter_one_republic: vec![
                        PolandConstitutionalArticle {
                            article_number: 1,
                            title: "Rzeczpospolita Polska".to_string(),
                            content: "Rzeczpospolita Polska jest dobrem wspólnym wszystkich obywateli.".to_string(),
                            interpretation_notes: vec!["Dobro wspólne jako podstawa państwa".to_string()],
                            related_legislation: vec!["Ustawa o obywatelstwie polskim".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 2,
                            title: "Demokratyczne państwo prawne".to_string(),
                            content: "Rzeczpospolita Polska jest demokratycznym państwem prawnym, urzeczywistniającym zasady sprawiedliwości społecznej.".to_string(),
                            interpretation_notes: vec!["Zasada demokratycznego państwa prawnego".to_string(), "Sprawiedliwość społeczna".to_string()],
                            related_legislation: vec!["Ustawa o Trybunale Konstytucyjnym".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 3,
                            title: "Ustrój republikański".to_string(),
                            content: "Rzeczpospolita Polska jest państwem jednolitym.".to_string(),
                            interpretation_notes: vec!["Państwo unitarne".to_string(), "Zasada jedności".to_string()],
                            related_legislation: vec!["Ustawa o samorządzie terytorialnym".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 4,
                            title: "Władza zwierzchnia".to_string(),
                            content: "Władza zwierzchnia w Rzeczypospolitej Polskiej należy do Narodu.".to_string(),
                            interpretation_notes: vec!["Suwerenność narodu".to_string(), "Demokracja".to_string()],
                            related_legislation: vec!["Kodeks wyborczy".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 20,
                            title: "Gospodarka społeczna rynkowa".to_string(),
                            content: "Społeczna gospodarka rynkowa oparta na wolności działalności gospodarczej, własności prywatnej oraz solidarności, dialogu i współpracy partnerów społecznych stanowi podstawę ustroju gospodarczego Rzeczypospolitej Polskiej.".to_string(),
                            interpretation_notes: vec!["Model gospodarczy".to_string(), "Własność prywatna".to_string()],
                            related_legislation: vec!["Ustawa o działalności gospodarczej".to_string()],
                        },
                    ],
                    chapter_two_freedoms_rights_duties: vec![
                        PolandConstitutionalArticle {
                            article_number: 30,
                            title: "Godność człowieka".to_string(),
                            content: "Przyrodzona i niezbywalna godność człowieka stanowi źródło wolności i praw człowieka i obywatela. Jest ona nienaruszalna, a jej poszanowanie i ochrona jest obowiązkiem władz publicznych.".to_string(),
                            interpretation_notes: vec!["Godność jako źródło praw".to_string(), "Obowiązek władz publicznych".to_string()],
                            related_legislation: vec!["Kodeks karny".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 32,
                            title: "Równość wobec prawa".to_string(),
                            content: "Wszyscy są wobec prawa równi. Wszyscy mają prawo do równego traktowania przez władze publiczne.".to_string(),
                            interpretation_notes: vec!["Zasada równości".to_string(), "Równe traktowanie".to_string()],
                            related_legislation: vec!["Ustawa o równym traktowaniu".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 38,
                            title: "Ochrona życia".to_string(),
                            content: "Rzeczpospolita Polska zapewnia każdemu człowiekowi prawną ochronę życia.".to_string(),
                            interpretation_notes: vec!["Prawo do życia".to_string(), "Ochrona prawna".to_string()],
                            related_legislation: vec!["Ustawa o ochronie zdrowia".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 41,
                            title: "Nietykalność osobista".to_string(),
                            content: "Nietykalność osobista jest zapewniona każdemu. Jakiekolwiek jej ograniczenie może nastąpić jedynie na podstawie ustawy.".to_string(),
                            interpretation_notes: vec!["Wolność osobista".to_string(), "Zasada legalności".to_string()],
                            related_legislation: vec!["Kodeks postępowania karnego".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 54,
                            title: "Wolność wypowiadania poglądów".to_string(),
                            content: "Każdemu zapewnia się wolność wyrażania swoich poglądów oraz pozyskiwania i rozpowszechniania informacji.".to_string(),
                            interpretation_notes: vec!["Wolność słowa".to_string(), "Prawo do informacji".to_string()],
                            related_legislation: vec!["Ustawa Prawo prasowe".to_string()],
                        },
                    ],
                    chapter_three_sources_law: vec![
                        PolandConstitutionalArticle {
                            article_number: 87,
                            title: "Źródła prawa powszechnie obowiązującego".to_string(),
                            content: "Źródłami prawa powszechnie obowiązującego Rzeczypospolitej Polskiej są: Konstytucja, ustawy, ratyfikowane umowy międzynarodowe oraz rozporządzenia.".to_string(),
                            interpretation_notes: vec!["Hierarchia źródeł prawa".to_string(), "Prawo powszechnie obowiązujące".to_string()],
                            related_legislation: vec!["Ustawa o technice prawodawczej".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 91,
                            title: "Umowy międzynarodowe".to_string(),
                            content: "Ratyfikowana umowa międzynarodowa, po jej ogłoszeniu w Dzienniku Ustaw Rzeczypospolitej Polskiej, stanowi część krajowego porządku prawnego i jest bezpośrednio stosowana, chyba że jej stosowanie jest uzależnione od wydania ustawy.".to_string(),
                            interpretation_notes: vec!["Monizm prawny".to_string(), "Bezpośrednie stosowanie".to_string()],
                            related_legislation: vec!["Ustawa o zawieraniu umów międzynarodowych".to_string()],
                        },
                    ],
                    chapter_four_sejm_senate: vec![
                        PolandConstitutionalArticle {
                            article_number: 95,
                            title: "Władza ustawodawcza".to_string(),
                            content: "Władzę ustawodawczą w Rzeczypospolitej Polskiej sprawują Sejm i Senat.".to_string(),
                            interpretation_notes: vec!["Bikameralizm".to_string(), "Władza ustawodawcza".to_string()],
                            related_legislation: vec!["Regulamin Sejmu".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 96,
                            title: "Sejm".to_string(),
                            content: "Sejm składa się z 460 posłów wybieranych na cztery lata w wyborach powszechnych, równych, bezpośrednich i proporcjonalnych przy głosowaniu tajnym.".to_string(),
                            interpretation_notes: vec!["Skład Sejmu".to_string(), "System wyborczy".to_string()],
                            related_legislation: vec!["Kodeks wyborczy".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 97,
                            title: "Senat".to_string(),
                            content: "Senat składa się ze 100 senatorów wybieranych na cztery lata w wyborach powszechnych, równych, bezpośrednich przy głosowaniu tajnym.".to_string(),
                            interpretation_notes: vec!["Skład Senatu".to_string(), "Wybory senatorskie".to_string()],
                            related_legislation: vec!["Ustawa o Senacie".to_string()],
                        },
                    ],
                    chapter_five_president: vec![
                        PolandConstitutionalArticle {
                            article_number: 126,
                            title: "Prezydent Rzeczypospolitej".to_string(),
                            content: "Prezydent Rzeczypospolitej Polskiej jest najwyższym przedstawicielem Rzeczypospolitej Polskiej i gwarantem ciągłości władzy państwowej.".to_string(),
                            interpretation_notes: vec!["Funkcje Prezydenta".to_string(), "Reprezentacja państwa".to_string()],
                            related_legislation: vec!["Ustawa o wyborze Prezydenta".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 127,
                            title: "Wybór Prezydenta".to_string(),
                            content: "Prezydent Rzeczypospolitej jest wybierany przez Naród w wyborach powszechnych, równych, bezpośrednich przy głosowaniu tajnym.".to_string(),
                            interpretation_notes: vec!["Demokratyczna legitymacja".to_string(), "Wybory prezydenckie".to_string()],
                            related_legislation: vec!["Kodeks wyborczy".to_string()],
                        },
                    ],
                    chapter_six_council_ministers: vec![
                        PolandConstitutionalArticle {
                            article_number: 146,
                            title: "Rada Ministrów".to_string(),
                            content: "Rada Ministrów sprawuje władzę wykonawczą w Rzeczypospolitej Polskiej.".to_string(),
                            interpretation_notes: vec!["Władza wykonawcza".to_string(), "Rząd".to_string()],
                            related_legislation: vec!["Ustawa o Radzie Ministrów".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 154,
                            title: "Prezes Rady Ministrów".to_string(),
                            content: "Prezes Rady Ministrów kieruje pracami Rady Ministrów.".to_string(),
                            interpretation_notes: vec!["Premier".to_string(), "Kierowanie rządem".to_string()],
                            related_legislation: vec!["Ustawa o organizacji rządu".to_string()],
                        },
                    ],
                    chapter_eight_courts_tribunals: vec![
                        PolandConstitutionalArticle {
                            article_number: 173,
                            title: "Sądy i trybunały".to_string(),
                            content: "Sądy i trybunały są władzą odrębną i niezależną od innych władz.".to_string(),
                            interpretation_notes: vec!["Niezależność sądów".to_string(), "Trójpodział władzy".to_string()],
                            related_legislation: vec!["Ustawa Prawo o ustroju sądów powszechnych".to_string()],
                        },
                        PolandConstitutionalArticle {
                            article_number: 188,
                            title: "Trybunał Konstytucyjny".to_string(),
                            content: "Trybunał Konstytucyjny orzeka w sprawach: zgodności ustaw i umów międzynarodowych z Konstytucją".to_string(),
                            interpretation_notes: vec!["Kontrola konstytucyjności".to_string(), "Sądownictwo konstytucyjne".to_string()],
                            related_legislation: vec!["Ustawa o Trybunale Konstytucyjnym".to_string()],
                        },
                    ],
                    adoption_date: "2 kwietnia 1997".to_string(),
                    referendum_date: "25 maja 1997".to_string(),
                    effective_date: "17 października 1997".to_string(),
                },
                constitutional_principles: PolandConstitutionalPrinciples {
                    democratic_state: "Rzeczpospolita Polska jest demokratycznym państwem prawnym".to_string(),
                    rule_of_law: "Zasada państwa prawnego i hierarchii źródeł prawa".to_string(),
                    separation_powers: "Trójpodział władzy na ustawodawczą, wykonawczą i sądowniczą".to_string(),
                    decentralization: "Decentralizacja władzy publicznej".to_string(),
                    social_market_economy: "Społeczna gospodarka rynkowa".to_string(),
                    human_dignity: "Godność człowieka jako źródło praw i wolności".to_string(),
                    political_pluralism: "Pluralizm polityczny i demokratyczne procedury".to_string(),
                    subsidiarity: "Zasada pomocniczości w działaniu władz publicznych".to_string(),
                },
                fundamental_rights: PolandFundamentalRights {
                    individual_rights: vec![
                        "Prawo do życia (art. 38)".to_string(),
                        "Nietykalność osobista (art. 41)".to_string(),
                        "Nietykalność mieszkania (art. 50)".to_string(),
                        "Wolność poruszania się (art. 52)".to_string(),
                        "Ochrona danych osobowych (art. 51)".to_string(),
                    ],
                    civil_rights: vec![
                        "Wolność wypowiadania poglądów (art. 54)".to_string(),
                        "Wolność sumienia i religii (art. 53)".to_string(),
                        "Prawo do sądu (art. 45)".to_string(),
                        "Wolność zgromadzeń (art. 57)".to_string(),
                        "Wolność zrzeszania się (art. 58)".to_string(),
                    ],
                    political_rights: vec![
                        "Prawo wybierania i bycia wybieranym (art. 62)".to_string(),
                        "Prawo dostępu do służby publicznej (art. 60)".to_string(),
                        "Prawo petycji (art. 63)".to_string(),
                        "Prawo do informacji o działalności władz (art. 61)".to_string(),
                    ],
                    economic_social_rights: vec![
                        "Prawo do pracy (art. 65)".to_string(),
                        "Prawo do zabezpieczenia społecznego (art. 67)".to_string(),
                        "Prawo do ochrony zdrowia (art. 68)".to_string(),
                        "Prawo do nauki (art. 70)".to_string(),
                        "Prawo do mieszkania (art. 75)".to_string(),
                    ],
                    cultural_rights: vec![
                        "Wolność twórczości artystycznej (art. 73)".to_string(),
                        "Prawo do nauki i korzystania z dóbr kultury (art. 73)".to_string(),
                        "Ochrona dziedzictwa narodowego (art. 5)".to_string(),
                    ],
                    environmental_rights: vec![
                        "Prawo do życia w czystym środowisku (art. 74)".to_string(),
                        "Obowiązek ochrony środowiska (art. 74)".to_string(),
                        "Zrównoważony rozwój (art. 74)".to_string(),
                    ],
                },
                constitutional_amendments: vec![
                    PolandConstitutionalAmendment {
                        amendment_number: 1,
                        year: 2006,
                        title: "Europejski Nakaz Aresztowania".to_string(),
                        articles_modified: vec![55],
                        approval_process: "Ustawa zmieniająca Konstytucję".to_string(),
                    },
                    PolandConstitutionalAmendment {
                        amendment_number: 2,
                        year: 2009,
                        title: "Przewodniczący Krajowej Rady Sądownictwa".to_string(),
                        articles_modified: vec![187],
                        approval_process: "Referendum konstytucyjne".to_string(),
                    },
                ],
                constitutional_court_decisions: vec![
                    PolandConstitutionalCourtDecision {
                        decision_number: "K 18/04".to_string(),
                        year: 2005,
                        case_title: "Zgodność Traktatu akcesyjnego UE z Konstytucją".to_string(),
                        constitutional_issue: "Transfery kompetencji do UE".to_string(),
                        ruling: "Zgodność z zastrzeżeniami".to_string(),
                        legal_principle: "Granice integracji europejskiej".to_string(),
                        dissenting_opinions: vec![],
                    },
                    PolandConstitutionalCourtDecision {
                        decision_number: "K 32/09".to_string(),
                        year: 2010,
                        case_title: "Traktat lizboński".to_string(),
                        constitutional_issue: "Zgodność Traktatu lizbońskiego z Konstytucją".to_string(),
                        ruling: "Zgodność pod warunkiem zachowania tożsamości konstytucyjnej".to_string(),
                        legal_principle: "Tożsamość konstytucyjna Polski".to_string(),
                        dissenting_opinions: vec!["Zdanie odrębne sędziego Wróblewskiego".to_string()],
                    },
                ],
            },
            government_structure: PolandGovernmentStructure {
                executive_branch: PolandExecutiveBranch {
                    prime_minister: PolandPrimeMinister {
                        current_holder: "Donald Tusk".to_string(),
                        appointment_process: "Wybór przez Sejm na wniosek Prezydenta".to_string(),
                        powers_responsibilities: vec![
                            "Kierowanie pracami Rady Ministrów".to_string(),
                            "Wydawanie zarządzeń".to_string(),
                            "Zapewnianie wykonywania polityki Rady Ministrów".to_string(),
                        ],
                        term_duration: "Kadencja Sejmu (4 lata)".to_string(),
                        removal_process: "Wotum nieufności Sejmu".to_string(),
                    },
                    deputy_prime_ministers: vec![
                        "Władysław Kosiniak-Kamysz (Wicepremier, Minister Obrony Narodowej)".to_string(),
                        "Krzysztof Gawkowski (Wicepremier, Minister Cyfryzacji)".to_string(),
                    ],
                    ministers: vec![
                        PolandMinister {
                            ministry_name: "Ministerstwo Spraw Wewnętrznych i Administracji".to_string(),
                            current_minister: "Tomasz Siemoniak".to_string(),
                            responsibilities: vec!["Bezpieczeństwo wewnętrzne".to_string(), "Administracja publiczna".to_string(), "Samorząd terytorialny".to_string()],
                            budget_allocation: "15.2 mld PLN".to_string(),
                            staff_count: 85000,
                        },
                        PolandMinister {
                            ministry_name: "Ministerstwo Finansów".to_string(),
                            current_minister: "Andrzej Domański".to_string(),
                            responsibilities: vec!["Polityka fiskalna".to_string(), "Budżet państwa".to_string(), "System podatkowy".to_string()],
                            budget_allocation: "520 mld PLN".to_string(),
                            staff_count: 55000,
                        },
                        PolandMinister {
                            ministry_name: "Ministerstwo Sprawiedliwości".to_string(),
                            current_minister: "Adam Bodnar".to_string(),
                            responsibilities: vec!["Sądownictwo".to_string(), "Prokuratura".to_string(), "Więziennictwo".to_string()],
                            budget_allocation: "8.5 mld PLN".to_string(),
                            staff_count: 45000,
                        },
                    ],
                    government_formation: "Wybór przez Sejm na wniosek Prezydenta".to_string(),
                    confidence_mechanism: "Wotum zaufania i wotum nieufności".to_string(),
                },
                legislative_branch: PolandLegislativeBranch {
                    national_assembly: PolandNationalAssembly {
                        bicameral_system: "Sejm i Senat".to_string(),
                        session_duration: "4 lata".to_string(),
                        dissolution_rules: "Skrócenie kadencji możliwe".to_string(),
                        immunities_privileges: vec!["Immunitet parlamentarny".to_string(), "Indemnitet".to_string()],
                        current_term: "X kadencja Sejmu (2023-2027)".to_string(),
                    },
                    sejm: PolandSejm {
                        total_seats: 460,
                        current_composition: vec![
                            PolandPoliticalGroup {
                                party_name: "Koalicja Obywatelska".to_string(),
                                seats_count: 157,
                                leader: "Donald Tusk".to_string(),
                                political_orientation: "Centrum-liberalna".to_string(),
                            },
                            PolandPoliticalGroup {
                                party_name: "Prawo i Sprawiedliwość".to_string(),
                                seats_count: 194,
                                leader: "Jarosław Kaczyński".to_string(),
                                political_orientation: "Prawica konserwatywna".to_string(),
                            },
                            PolandPoliticalGroup {
                                party_name: "Trzecia Droga".to_string(),
                                seats_count: 65,
                                leader: "Władysław Kosiniak-Kamysz".to_string(),
                                political_orientation: "Centrum-prawica".to_string(),
                            },
                            PolandPoliticalGroup {
                                party_name: "Lewica".to_string(),
                                seats_count: 26,
                                leader: "Włodzimierz Czarzasty".to_string(),
                                political_orientation: "Socjaldemokracja".to_string(),
                            },
                        ],
                        speaker: "Szymon Hołownia".to_string(),
                        electoral_system: "System proporcjonalny D'Hondta z progiem wyborczym".to_string(),
                        term_length: "4 lata".to_string(),
                    },
                    senate: PolandSenate {
                        total_seats: 100,
                        current_composition: vec![
                            PolandPoliticalGroup {
                                party_name: "Koalicja Obywatelska".to_string(),
                                seats_count: 66,
                                leader: "Donald Tusk".to_string(),
                                political_orientation: "Centrum-liberalna".to_string(),
                            },
                            PolandPoliticalGroup {
                                party_name: "Prawo i Sprawiedliwość".to_string(),
                                seats_count: 34,
                                leader: "Jarosław Kaczyński".to_string(),
                                political_orientation: "Prawica konserwatywna".to_string(),
                            },
                        ],
                        speaker: "Małgorzata Kidawa-Błońska".to_string(),
                        electoral_system: "System większościowy w okręgach jednomandatowych".to_string(),
                        territorial_representation: "Reprezentacja województw".to_string(),
                    },
                    legislative_process: PolandLegislativeProcess {
                        ordinary_procedure: "Trzyczytaniowy tryb legislacyjny".to_string(),
                        urgency_procedure: "Tryb pilny z skróceniami terminów".to_string(),
                        senate_veto: "Senat może odrzucić ustawę większością bezwzględną".to_string(),
                        presidential_veto: "Prezydent może zawetować ustawę".to_string(),
                    },
                    parliamentary_groups: vec![
                        PolandParliamentaryGroup {
                            name: "Klub Parlamentarny Koalicja Obywatelska".to_string(),
                            chamber: "Sejm".to_string(),
                            leader: "Borys Budka".to_string(),
                            members: 157,
                        },
                        PolandParliamentaryGroup {
                            name: "Klub Parlamentarny Prawo i Sprawiedliwość".to_string(),
                            chamber: "Sejm".to_string(),
                            leader: "Mariusz Błaszczak".to_string(),
                            members: 194,
                        },
                    ],
                },
                head_of_state: PolandHeadOfState {
                    president: PolandPresident {
                        current_president: "Andrzej Duda".to_string(),
                        election_process: "Wybory powszechne w dwóch turach".to_string(),
                        term_duration: "5 lat, maksymalnie dwie kadencje".to_string(),
                        eligibility_requirements: vec![
                            "Obywatelstwo polskie".to_string(),
                            "Ukończone 35 lat".to_string(),
                            "Korzystanie z pełni praw wyborczych".to_string(),
                        ],
                        impeachment_process: "Stawienie przed Trybunałem Stanu".to_string(),
                        residence: "Pałac Prezydencki w Warszawie".to_string(),
                    },
                    powers: vec![
                        "Reprezentacja państwa na arenie międzynarodowej".to_string(),
                        "Naczelne dowództwo nad siłami zbrojnymi".to_string(),
                        "Mianowanie Prezesa Rady Ministrów".to_string(),
                        "Prawo łaski".to_string(),
                        "Prawo veta ustawodawczego".to_string(),
                    ],
                    ceremonial_functions: vec![
                        "Uroczystości państwowe".to_string(),
                        "Nadawanie orderów i odznaczeń".to_string(),
                        "Przyjmowanie listów uwierzytelniających".to_string(),
                    ],
                    emergency_powers: vec![
                        "Wprowadzenie stanu wojennego".to_string(),
                        "Wprowadzenie stanu wyjątkowego".to_string(),
                        "Wprowadzenie stanu klęski żywiołowej".to_string(),
                    ],
                },
                council_ministers: PolandCouncilMinisters {
                    composition: "Prezes Rady Ministrów, wicepremierzy i ministrowie".to_string(),
                    meeting_frequency: "Co najmniej raz w tygodniu".to_string(),
                    decision_making: "Uchwały podejmowane kolegialnie".to_string(),
                    secretariat: "Kancelaria Prezesa Rady Ministrów".to_string(),
                },
                administrative_system: PolandAdministrativeSystem {
                    central_administration: "Ministerstwa i urzędy centralne".to_string(),
                    field_administration: "Wojewodowie i administracja zespolona".to_string(),
                    local_self_government: "Samorząd terytorialny trzystopniowy".to_string(),
                    independent_institutions: vec!["NBP".to_string(), "NIK".to_string(), "RPO".to_string()],
                },
            },
            territorial_organization: PolandTerritorialOrganization {
                voivodeships: vec![
                    PolandVoivodeship {
                        name: "Mazowieckie".to_string(),
                        capital: "Warszawa".to_string(),
                        population: 5403412,
                        area_km2: 35558.0,
                        gdp_pln: 450000000000,
                        voivode: "Konstanty Radziwiłł".to_string(),
                        regional_assembly: PolandRegionalAssembly {
                            name: "Sejmik Województwa Mazowieckiego".to_string(),
                            seats: 51,
                            speaker: "Ludwik Rakowski".to_string(),
                            political_composition: vec![
                                PolandPoliticalGroup {
                                    party_name: "Koalicja Obywatelska".to_string(),
                                    seats_count: 22,
                                    leader: "Rafał Trzaskowski".to_string(),
                                    political_orientation: "Centrum-liberalna".to_string(),
                                },
                            ],
                            term_duration: "5 lat".to_string(),
                        },
                        regional_government: PolandRegionalGovernment {
                            marshal: "Adam Struzik".to_string(),
                            board_members: vec![
                                "Sylwester Dąbrowski (Wicemarszałek)".to_string(),
                                "Elżbieta Lanc (Wicemarszałek)".to_string(),
                            ],
                            budget_pln: 3200000000,
                            executive_competencies: vec!["Edukacja".to_string(), "Ochrona zdrowia".to_string(), "Transport".to_string()],
                        },
                        competencies: vec!["Polityka rozwoju regionalnego".to_string(), "Edukacja publiczna".to_string(), "Ochrona środowiska".to_string()],
                    },
                    PolandVoivodeship {
                        name: "Śląskie".to_string(),
                        capital: "Katowice".to_string(),
                        population: 4492330,
                        area_km2: 12333.0,
                        gdp_pln: 320000000000,
                        voivode: "Marek Wójcik".to_string(),
                        regional_assembly: PolandRegionalAssembly {
                            name: "Sejmik Województwa Śląskiego".to_string(),
                            seats: 45,
                            speaker: "Jerzy Buzek".to_string(),
                            political_composition: vec![
                                PolandPoliticalGroup {
                                    party_name: "Koalicja Obywatelska".to_string(),
                                    seats_count: 19,
                                    leader: "Wojciech Saługa".to_string(),
                                    political_orientation: "Centrum-liberalna".to_string(),
                                },
                            ],
                            term_duration: "5 lat".to_string(),
                        },
                        regional_government: PolandRegionalGovernment {
                            marshal: "Wojciech Saługa".to_string(),
                            board_members: vec![
                                "Beata Moskal-Słaniewska (Wicemarszałek)".to_string(),
                                "Jakub Chełstowski (Wicemarszałek)".to_string(),
                            ],
                            budget_pln: 2800000000,
                            executive_competencies: vec!["Restrukturyzacja górnictwa".to_string(), "Innowacje".to_string(), "Kultura".to_string()],
                        },
                        competencies: vec!["Transformacja energetyczna".to_string(), "Polityka innowacji".to_string(), "Rewitalizacja miast".to_string()],
                    },
                    PolandVoivodeship {
                        name: "Wielkopolskie".to_string(),
                        capital: "Poznań".to_string(),
                        population: 3496450,
                        area_km2: 29826.0,
                        gdp_pln: 285000000000,
                        voivode: "Michał Zieliński".to_string(),
                        regional_assembly: PolandRegionalAssembly {
                            name: "Sejmik Województwa Wielkopolskiego".to_string(),
                            seats: 39,
                            speaker: "Marcin Wroński".to_string(),
                            political_composition: vec![
                                PolandPoliticalGroup {
                                    party_name: "Koalicja Obywatelska".to_string(),
                                    seats_count: 18,
                                    leader: "Marek Woźniak".to_string(),
                                    political_orientation: "Centrum-liberalna".to_string(),
                                },
                            ],
                            term_duration: "5 lat".to_string(),
                        },
                        regional_government: PolandRegionalGovernment {
                            marshal: "Marek Woźniak".to_string(),
                            board_members: vec![
                                "Krzysztof Grabowski (Wicemarszałek)".to_string(),
                                "Paulina Stochniałek (Wicemarszałek)".to_string(),
                            ],
                            budget_pln: 2500000000,
                            executive_competencies: vec!["Rolnictwo".to_string(), "Przemysł".to_string(), "Turystyka".to_string()],
                        },
                        competencies: vec!["Rozwój wsi".to_string(), "Polityka przemysłowa".to_string(), "Współpraca międzynarodowa".to_string()],
                    },
                ],
                counties: vec![
                    PolandCounty {
                        name: "Powiat warszawski zachodni".to_string(),
                        seat: "Ożarów Mazowiecki".to_string(),
                        population: 115000,
                        voivodeship: "Mazowieckie".to_string(),
                        starosta: "Anna Brzezińska".to_string(),
                    },
                    PolandCounty {
                        name: "Powiat krakowski".to_string(),
                        seat: "Kraków".to_string(),
                        population: 280000,
                        voivodeship: "Małopolskie".to_string(),
                        starosta: "Józef Gawron".to_string(),
                    },
                ],
                municipalities: vec![
                    PolandMunicipality {
                        name: "Warszawa".to_string(),
                        county: "miasto na prawach powiatu".to_string(),
                        population: 1765000,
                        mayor_type: "Prezydent miasta".to_string(),
                        mayor: "Rafał Trzaskowski".to_string(),
                    },
                    PolandMunicipality {
                        name: "Kraków".to_string(),
                        county: "miasto na prawach powiatu".to_string(),
                        population: 779000,
                        mayor_type: "Prezydent miasta".to_string(),
                        mayor: "Aleksander Miszalski".to_string(),
                    },
                ],
                territorial_self_government: PolandTerritorialSelfGovernment {
                    three_tier_system: "Gmina, powiat, województwo".to_string(),
                    municipal_level: "Gminy - 2477 jednostek".to_string(),
                    county_level: "Powiaty - 314 jednostek".to_string(),
                    regional_level: "Województwa - 16 jednostek".to_string(),
                    competencies_distribution: "Podział kompetencji według zasady subsydiarności".to_string(),
                },
            },
            judicial_system: PolandJudicialSystem {
                supreme_court: PolandSupremeCourt {
                    official_name: "Sąd Najwyższy".to_string(),
                    location: "Warszawa".to_string(),
                    first_president: "Małgorzata Manowska".to_string(),
                    total_judges: 97,
                    chambers: vec![
                        PolandSupremeCourtChamber {
                            chamber_name: "Izba Cywilna".to_string(),
                            president: "Krzysztof Strzelczyk".to_string(),
                            specialization: "Sprawy cywilne".to_string(),
                            judges_count: 32,
                        },
                        PolandSupremeCourtChamber {
                            chamber_name: "Izba Karna".to_string(),
                            president: "Zbigniew Puszkarski".to_string(),
                            specialization: "Sprawy karne".to_string(),
                            judges_count: 28,
                        },
                        PolandSupremeCourtChamber {
                            chamber_name: "Izba Pracy, Ubezpieczeń Społecznych i Spraw Publicznych".to_string(),
                            president: "Dawid Miąsik".to_string(),
                            specialization: "Sprawy pracy i ubezpieczeń".to_string(),
                            judges_count: 25,
                        },
                    ],
                    jurisdiction: vec![
                        "Kasacja od wyroków sądów apelacyjnych".to_string(),
                        "Zagadnienia prawne o szczególnej wadze".to_string(),
                        "Orzeczenia w sprawie ważności wyborów".to_string(),
                    ],
                },
                constitutional_tribunal: PolandConstitutionalTribunal {
                    official_name: "Trybunał Konstytucyjny".to_string(),
                    location: "Warszawa".to_string(),
                    president: "Julia Przyłębska".to_string(),
                    total_judges: 15,
                    appointment_process: "Wybór przez Sejm na 9-letnią kadencję".to_string(),
                    jurisdiction: vec![
                        "Kontrola zgodności ustaw z Konstytucją".to_string(),
                        "Kontrola zgodności umów międzynarodowych z Konstytucją".to_string(),
                        "Skargi konstytucyjne".to_string(),
                        "Spory kompetencyjne między organami państwa".to_string(),
                    ],
                    landmark_decisions: vec![
                        PolandConstitutionalCourtDecision {
                            decision_number: "K 18/04".to_string(),
                            year: 2005,
                            case_title: "Zgodność Traktatu akcesyjnego z Konstytucją".to_string(),
                            constitutional_issue: "Transfery kompetencji do UE".to_string(),
                            ruling: "Zgodność z zastrzeżeniami".to_string(),
                            legal_principle: "Granice integracji europejskiej".to_string(),
                            dissenting_opinions: vec![],
                        },
                        PolandConstitutionalCourtDecision {
                            decision_number: "K 3/21".to_string(),
                            year: 2021,
                            case_title: "Pierwszeństwo prawa UE".to_string(),
                            constitutional_issue: "Kolizja prawa UE z Konstytucją RP".to_string(),
                            ruling: "Pierwszeństwo Konstytucji RP".to_string(),
                            legal_principle: "Tożsamość konstytucyjna Polski".to_string(),
                            dissenting_opinions: vec!["Zdanie odrębne sędziego Piotrowskiego".to_string()],
                        },
                    ],
                },
                supreme_administrative_court: PolandSupremeAdministrativeCourt {
                    official_name: "Naczelny Sąd Administracyjny".to_string(),
                    location: "Warszawa".to_string(),
                    president: "Sylwester Marciniak".to_string(),
                    total_judges: 78,
                    jurisdiction: vec![
                        "Kasacja od wyroków wojewódzkich sądów administracyjnych".to_string(),
                        "Kontrola aktów administracyjnych".to_string(),
                        "Zagadnienia prawne o szczególnej wadze w prawie administracyjnym".to_string(),
                    ],
                },
                national_council_judiciary: PolandNationalCouncilJudiciary {
                    official_name: "Krajowa Rada Sądownictwa".to_string(),
                    president: "Dagmara Pawełczyk-Woicka".to_string(),
                    composition: "25 członków: sędziowie, przedstawiciele innych władz".to_string(),
                    functions: vec![
                        "Powoływanie sędziów".to_string(),
                        "Stażenie niezależności sądów".to_string(),
                        "Orzekanie w sprawach dyscyplinarnych sędziów".to_string(),
                    ],
                    controversy: "Reforma z 2017 roku budzi kontrowersje".to_string(),
                },
                ordinary_courts: PolandOrdinaryCourts {
                    district_courts: "Sądy rejonowe - 318 sądów".to_string(),
                    regional_courts: "Sądy okręgowe - 45 sądów".to_string(),
                    courts_of_appeal: "Sądy apelacyjne - 11 sądów".to_string(),
                    specialized_courts: vec![
                        "Sądy pracy i ubezpieczeń społecznych".to_string(),
                        "Sądy gospodarcze".to_string(),
                        "Sądy rodzinne i nieletnich".to_string(),
                    ],
                },
                administrative_courts: PolandAdministrativeCourts {
                    voivodeship_courts: "Wojewódzkie sądy administracyjne - 16 sądów".to_string(),
                    supreme_administrative_court: "Naczelny Sąd Administracyjny".to_string(),
                    jurisdiction: vec![
                        "Kontrola legalności aktów administracyjnych".to_string(),
                        "Skargi na bezczynność organów".to_string(),
                        "Sprawy podatkowe".to_string(),
                    ],
                },
                prosecution_system: PolandProsecutionSystem {
                    prosecutor_general: "Adam Bodnar".to_string(),
                    structure: "Prokuratura hierarchicznie zorganizowana".to_string(),
                    levels: vec![
                        "Prokuratura Krajowa".to_string(),
                        "Prokuratury regionalne".to_string(),
                        "Prokuratury okręgowe".to_string(),
                        "Prokuratury rejonowe".to_string(),
                    ],
                    specialized_units: vec![
                        "Prokuratura do spraw Zorganizowanej Przestępczości i Korupcji".to_string(),
                        "Prokuratura Wojskowa".to_string(),
                        "Instytut Pamięci Narodowej - Komisja Ścigania Zbrodni przeciwko Narodowi Polskiemu".to_string(),
                    ],
                },
            },
            legal_codes: PolandLegalCodes {
                civil_code: PolandCivilCode {
                    official_name: "Kodeks cywilny".to_string(),
                    promulgation_date: "23 kwietnia 1964".to_string(),
                    structure: vec![
                        PolandCodeBook {
                            book_number: 1,
                            title: "Część ogólna".to_string(),
                            articles_range: "Art. 1-125".to_string(),
                            main_topics: vec!["Osoby fizyczne i prawne".to_string(), "Czynności prawne".to_string(), "Przedawnienie".to_string()],
                            key_articles: vec![
                                PolandCodeArticle {
                                    article_number: 1,
                                    title: "Zasady współżycia społecznego".to_string(),
                                    content: "Nie można czynić ze swego prawa użytku, który by był sprzeczny ze społeczno-gospodarczym przeznaczeniem tego prawa lub z zasadami współżycia społecznego.".to_string(),
                                    amendments: vec!["Nowelizacja z 2003 roku".to_string()],
                                },
                                PolandCodeArticle {
                                    article_number: 8,
                                    title: "Osobowość prawna".to_string(),
                                    content: "Każdy człowiek ma zdolność prawną od chwili urodzenia.".to_string(),
                                    amendments: vec![],
                                },
                            ],
                        },
                        PolandCodeBook {
                            book_number: 3,
                            title: "Obligacje".to_string(),
                            articles_range: "Art. 353-921".to_string(),
                            main_topics: vec!["Zobowiązania".to_string(), "Umowy".to_string(), "Odpowiedzialność deliktowa".to_string()],
                            key_articles: vec![
                                PolandCodeArticle {
                                    article_number: 353,
                                    title: "Zasada pacta sunt servanda".to_string(),
                                    content: "Dłużnik powinien wykonać zobowiązanie zgodnie z jego treścią i w sposób odpowiadający jego celowi społeczno-gospodarczemu oraz zasadom współżycia społecznego".to_string(),
                                    amendments: vec!["Nowelizacja z 1990 roku".to_string()],
                                },
                                PolandCodeArticle {
                                    article_number: 415,
                                    title: "Odpowiedzialność deliktowa".to_string(),
                                    content: "Kto by działaniem lub zaniechaniem, zawinionym albo za które ponosi odpowiedzialność bez względu na winę, wyrządził drugiemu szkodę, obowiązany jest do jej naprawienia.".to_string(),
                                    amendments: vec!["Nowelizacja z 2008 roku".to_string()],
                                },
                            ],
                        },
                    ],
                    total_articles: 1088,
                    major_reforms: vec![
                        "Transformacja ustrojowa 1989-1990".to_string(),
                        "Europeizacja prawa cywilnego 2003-2004".to_string(),
                        "Implementacja dyrektyw UE 2014-2016".to_string(),
                    ],
                },
                criminal_code: PolandCriminalCode {
                    official_name: "Kodeks karny".to_string(),
                    promulgation_date: "6 czerwca 1997".to_string(),
                    structure: vec![
                        PolandCodeBook {
                            book_number: 1,
                            title: "Przepisy ogólne".to_string(),
                            articles_range: "Art. 1-116".to_string(),
                            main_topics: vec!["Zasady odpowiedzialności karnej".to_string(), "Kary i środki karne".to_string(), "Warunkowe umorzenie".to_string()],
                            key_articles: vec![
                                PolandCodeArticle {
                                    article_number: 1,
                                    title: "Zasada nullum crimen sine lege".to_string(),
                                    content: "Odpowiedzialności karnej podlega ten tylko, kto popełnia czyn zabroniony pod groźbą kary przez ustawę obowiązującą w czasie jego popełnienia.".to_string(),
                                    amendments: vec![],
                                },
                            ],
                        },
                    ],
                    total_articles: 363,
                    recent_reforms: vec![
                        "Ustawa z 2022 roku o ochronie dzieci".to_string(),
                        "Kodeks Drogowy - nowe przepisy karne 2022".to_string(),
                        "Zmiany w zakresie cyberprzestępczości 2021".to_string(),
                    ],
                },
                administrative_code: PolandAdministrativeCode {
                    main_legislation: "Kodeks postępowania administracyjnego z 1960 roku".to_string(),
                    supporting_acts: vec![
                        "Ustawa o samorządzie terytorialnym".to_string(),
                        "Ustawa o dostępie do informacji publicznej".to_string(),
                        "Ustawa o petycjach".to_string(),
                    ],
                    principles: vec![
                        "Zasada legalizmu".to_string(),
                        "Zasada prawdy obiektywnej".to_string(),
                        "Zasada dwuinstancyjności".to_string(),
                        "Zasada czynnego udziału stron".to_string(),
                    ],
                    recent_reforms: vec![
                        "Cyfryzacja administracji 2021".to_string(),
                        "Procedura uproszczona 2020".to_string(),
                    ],
                },
                labor_code: PolandLaborCode {
                    official_name: "Kodeks pracy".to_string(),
                    promulgation_date: "26 czerwca 1974".to_string(),
                    main_principles: vec![
                        "Ochrona praw pracownika".to_string(),
                        "Równouprawnienie w zatrudnieniu".to_string(),
                        "Bezpieczeństwo i higiena pracy".to_string(),
                        "Prawo do wypoczynku".to_string(),
                    ],
                    recent_reforms: vec![
                        "Kodeks pracy 4.0 - elastyczne formy zatrudnienia 2023".to_string(),
                        "Prawo do odłączenia się 2022".to_string(),
                        "Praca zdalna podczas pandemii 2020-2021".to_string(),
                    ],
                    trade_union_law: "Ustawa o związkach zawodowych z 1991 roku".to_string(),
                },
                commercial_code: PolandCommercialCode {
                    company_law: "Kodeks spółek handlowych z 2000 roku".to_string(),
                    bankruptcy_law: "Prawo restrukturyzacyjne z 2015 roku".to_string(),
                    competition_law: "Ustawa o ochronie konkurencji i konsumentów z 2007 roku".to_string(),
                    recent_changes: vec![
                        "Prosta spółka akcyjna 2021".to_string(),
                        "Cyfryzacja rejestrów gospodarczych 2020".to_string(),
                        "Nowe regulacje ESG 2023".to_string(),
                    ],
                },
                tax_code: PolandTaxCode {
                    general_tax_law: "Ordynacja podatkowa z 1997 roku".to_string(),
                    main_taxes: vec![
                        "Podatek dochodowy od osób fizycznych (PIT)".to_string(),
                        "Podatek dochodowy od osób prawnych (CIT)".to_string(),
                        "Podatek od towarów i usług (VAT)".to_string(),
                        "Podatek akcyzowy".to_string(),
                    ],
                    tax_administration: "Krajowa Administracja Skarbowa".to_string(),
                    recent_reforms: vec![
                        "Polski Ład - reforma podatkowa 2022".to_string(),
                        "Slim VAT - uproszczenia 2020-2021".to_string(),
                        "Pakiet paliwowy 2022".to_string(),
                    ],
                },
            },
            human_rights: PolandHumanRights {
                constitutional_guarantees: vec![
                    "Godność człowieka (art. 30 Konstytucji)".to_string(),
                    "Równość wobec prawa (art. 32 Konstytucji)".to_string(),
                    "Prawo do życia (art. 38 Konstytucji)".to_string(),
                    "Nietykalność osobista (art. 41 Konstytucji)".to_string(),
                    "Wolność słowa (art. 54 Konstytucji)".to_string(),
                ],
                international_treaties: vec![
                    "Europejska Konwencja Praw Człowieka (1993)".to_string(),
                    "Międzynarodowy Pakt Praw Obywatelskich i Politycznych (1977)".to_string(),
                    "Konwencja przeciwko Torturom (1989)".to_string(),
                    "Konwencja o Prawach Dziecka (1991)".to_string(),
                ],
                ombudsman_system: PolandOmbudsmanSystem {
                    human_rights_defender: "Marcin Wiącek".to_string(),
                    specialized_ombudsmen: vec![
                        "Rzecznik Praw Dziecka".to_string(),
                        "Rzecznik Praw Pacjenta".to_string(),
                        "Rzecznik Małych i Średnich Przedsiębiorców".to_string(),
                    ],
                    competencies: vec![
                        "Ochrona wolności i praw człowieka".to_string(),
                        "Badanie skarg na działanie organów publicznych".to_string(),
                        "Inicjatywy ustawodawcze".to_string(),
                    ],
                    reporting_mechanism: "Sprawozdanie roczne składane Sejmowi i Senatowi".to_string(),
                },
                anti_discrimination: vec![
                    "Ustawa o wdrożeniu niektórych przepisów UE w zakresie równego traktowania (2010)".to_string(),
                    "Kodeks pracy - zakaz dyskryminacji w zatrudnieniu".to_string(),
                    "Ustawa o równym traktowaniu w dostępie do edukacji (2015)".to_string(),
                ],
                data_protection: PolandDataProtection {
                    authority: "Urząd Ochrony Danych Osobowych (UODO)".to_string(),
                    legal_framework: vec![
                        "Rozporządzenie RODO (2018)".to_string(),
                        "Ustawa o ochronie danych osobowych (2018)".to_string(),
                    ],
                    individual_rights: vec![
                        "Prawo dostępu do danych".to_string(),
                        "Prawo do sprostowania".to_string(),
                        "Prawo do usunięcia danych".to_string(),
                        "Prawo do przenoszenia danych".to_string(),
                    ],
                    enforcement_mechanisms: vec![
                        "Kary administracyjne".to_string(),
                        "Środki naprawcze".to_string(),
                        "Skarga do UODO".to_string(),
                    ],
                },
            },
            eu_integration: PolandEUIntegration {
                membership_date: "1 maja 2004".to_string(),
                euro_adoption_status: "Nie przyjęła euro, obowiązek prawny przyjęcia".to_string(),
                schengen_participation: "21 grudnia 2007".to_string(),
                eu_institutions_representation: PolandEURepresentation {
                    european_parliament_seats: 52,
                    council_votes: 27,
                    commissioners: vec!["Janusz Wojciechowski (Rolnictwo)".to_string()],
                    permanent_representative: "Andrzej Sadoś".to_string(),
                },
                european_law_implementation: vec![
                    "Ustawa o współpracy Rady Ministrów z Sejmem i Senatem w sprawach UE".to_string(),
                    "Ustawa o zasadach uczestniczenia jednostek samorządu terytorialnego w programach UE".to_string(),
                    "Procedury implementacji dyrektyw UE".to_string(),
                ],
            },
        }
    }
}

// Additional implementation structs needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandLegislativeProcess {
    pub ordinary_procedure: String,
    pub urgency_procedure: String,
    pub senate_veto: String,
    pub presidential_veto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandParliamentaryGroup {
    pub name: String,
    pub chamber: String,
    pub leader: String,
    pub members: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCouncilMinisters {
    pub composition: String,
    pub meeting_frequency: String,
    pub decision_making: String,
    pub secretariat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandAdministrativeSystem {
    pub central_administration: String,
    pub field_administration: String,
    pub local_self_government: String,
    pub independent_institutions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandTerritorialSelfGovernment {
    pub three_tier_system: String,
    pub municipal_level: String,
    pub county_level: String,
    pub regional_level: String,
    pub competencies_distribution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandSupremeAdministrativeCourt {
    pub official_name: String,
    pub location: String,
    pub president: String,
    pub total_judges: u32,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandNationalCouncilJudiciary {
    pub official_name: String,
    pub president: String,
    pub composition: String,
    pub functions: Vec<String>,
    pub controversy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandOrdinaryCourts {
    pub district_courts: String,
    pub regional_courts: String,
    pub courts_of_appeal: String,
    pub specialized_courts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandAdministrativeCourts {
    pub voivodeship_courts: String,
    pub supreme_administrative_court: String,
    pub jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandProsecutionSystem {
    pub prosecutor_general: String,
    pub structure: String,
    pub levels: Vec<String>,
    pub specialized_units: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandAdministrativeCode {
    pub main_legislation: String,
    pub supporting_acts: Vec<String>,
    pub principles: Vec<String>,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandLaborCode {
    pub official_name: String,
    pub promulgation_date: String,
    pub main_principles: Vec<String>,
    pub recent_reforms: Vec<String>,
    pub trade_union_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandCommercialCode {
    pub company_law: String,
    pub bankruptcy_law: String,
    pub competition_law: String,
    pub recent_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolandTaxCode {
    pub general_tax_law: String,
    pub main_taxes: Vec<String>,
    pub tax_administration: String,
    pub recent_reforms: Vec<String>,
}

impl Default for PolandLegislativeProcess {
    fn default() -> Self {
        Self {
            ordinary_procedure: "Trzyczytaniowy tryb legislacyjny".to_string(),
            urgency_procedure: "Tryb pilny z skróceniami terminów".to_string(),
            senate_veto: "Senat może odrzucić ustawę większością bezwzględną".to_string(),
            presidential_veto: "Prezydent może zawetować ustawę".to_string(),
        }
    }
}

impl Default for PolandParliamentaryGroup {
    fn default() -> Self {
        Self {
            name: String::new(),
            chamber: String::new(),
            leader: String::new(),
            members: 0,
        }
    }
}

impl Default for PolandCouncilMinisters {
    fn default() -> Self {
        Self {
            composition: "Prezes Rady Ministrów, wicepremierzy i ministrowie".to_string(),
            meeting_frequency: "Co najmniej raz w tygodniu".to_string(),
            decision_making: "Uchwały podejmowane kolegialnie".to_string(),
            secretariat: "Kancelaria Prezesa Rady Ministrów".to_string(),
        }
    }
}

impl Default for PolandAdministrativeSystem {
    fn default() -> Self {
        Self {
            central_administration: "Ministerstwa i urzędy centralne".to_string(),
            field_administration: "Wojewodowie i administracja zespolona".to_string(),
            local_self_government: "Samorząd terytorialny trzystopniowy".to_string(),
            independent_institutions: vec![],
        }
    }
}

impl Default for PolandTerritorialSelfGovernment {
    fn default() -> Self {
        Self {
            three_tier_system: "Gmina, powiat, województwo".to_string(),
            municipal_level: "Gminy - 2477 jednostek".to_string(),
            county_level: "Powiaty - 314 jednostek".to_string(),
            regional_level: "Województwa - 16 jednostek".to_string(),
            competencies_distribution: "Podział kompetencji według zasady subsydiarności".to_string(),
        }
    }
}

impl Default for PolandSupremeAdministrativeCourt {
    fn default() -> Self {
        Self {
            official_name: "Naczelny Sąd Administracyjny".to_string(),
            location: "Warszawa".to_string(),
            president: String::new(),
            total_judges: 0,
            jurisdiction: vec![],
        }
    }
}

impl Default for PolandNationalCouncilJudiciary {
    fn default() -> Self {
        Self {
            official_name: "Krajowa Rada Sądownictwa".to_string(),
            president: String::new(),
            composition: "25 członków: sędziowie, przedstawiciele innych władz".to_string(),
            functions: vec![],
            controversy: "Reforma z 2017 roku budzi kontrowersje".to_string(),
        }
    }
}

impl Default for PolandOrdinaryCourts {
    fn default() -> Self {
        Self {
            district_courts: "Sądy rejonowe - 318 sądów".to_string(),
            regional_courts: "Sądy okręgowe - 45 sądów".to_string(),
            courts_of_appeal: "Sądy apelacyjne - 11 sądów".to_string(),
            specialized_courts: vec![],
        }
    }
}

impl Default for PolandAdministrativeCourts {
    fn default() -> Self {
        Self {
            voivodeship_courts: "Wojewódzkie sądy administracyjne - 16 sądów".to_string(),
            supreme_administrative_court: "Naczelny Sąd Administracyjny".to_string(),
            jurisdiction: vec![],
        }
    }
}

impl Default for PolandProsecutionSystem {
    fn default() -> Self {
        Self {
            prosecutor_general: String::new(),
            structure: "Prokuratura hierarchicznie zorganizowana".to_string(),
            levels: vec![],
            specialized_units: vec![],
        }
    }
}

impl Default for PolandAdministrativeCode {
    fn default() -> Self {
        Self {
            main_legislation: "Kodeks postępowania administracyjnego z 1960 roku".to_string(),
            supporting_acts: vec![],
            principles: vec![],
            recent_reforms: vec![],
        }
    }
}

impl Default for PolandLaborCode {
    fn default() -> Self {
        Self {
            official_name: "Kodeks pracy".to_string(),
            promulgation_date: "26 czerwca 1974".to_string(),
            main_principles: vec![],
            recent_reforms: vec![],
            trade_union_law: "Ustawa o związkach zawodowych z 1991 roku".to_string(),
        }
    }
}

impl Default for PolandCommercialCode {
    fn default() -> Self {
        Self {
            company_law: "Kodeks spółek handlowych z 2000 roku".to_string(),
            bankruptcy_law: "Prawo restrukturyzacyjne z 2015 roku".to_string(),
            competition_law: "Ustawa o ochronie konkurencji i konsumentów z 2007 roku".to_string(),
            recent_changes: vec![],
        }
    }
}

impl Default for PolandTaxCode {
    fn default() -> Self {
        Self {
            general_tax_law: "Ordynacja podatkowa z 1997 roku".to_string(),
            main_taxes: vec![],
            tax_administration: "Krajowa Administracja Skarbowa".to_string(),
            recent_reforms: vec![],
        }
    }
}

pub fn create_poland_legal_system() -> PolandLegalSystem {
    PolandLegalSystem::default()
}