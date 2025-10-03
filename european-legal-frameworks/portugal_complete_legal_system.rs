use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete Portuguese Legal System Implementation
/// República Portuguesa - Constitutional Republic
/// Current President: Marcelo Rebelo de Sousa (2016-2026)
/// Current Prime Minister: António Costa (PS - Partido Socialista)
/// Current Government: XXIII Constitutional Government
/// EU Member since 1986, Eurozone since 1999, Schengen since 1995

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugalLegalSystem {
    pub constitutional_framework: PortugueseConstitution,
    pub government_structure: PortugueseGovernment,
    pub territorial_organization: PortugueseTerritorialOrganization,
    pub judicial_system: PortugueseJudicialSystem,
    pub legal_codes: PortugueseLegalCodes,
    pub european_integration: PortugueseEuropeanIntegration,
    pub autonomous_regions: PortugueseAutonomousRegions,
    pub local_government: PortugueseLocalGovernment,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseConstitution {
    pub name: String,
    pub adoption_date: String,
    pub last_revision: String,
    pub total_articles: u32,
    pub fundamental_principles: Vec<ConstitutionalPrinciple>,
    pub rights_and_duties: Vec<ConstitutionalRight>,
    pub economic_system: EconomicConstitutionalFramework,
    pub key_articles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalPrinciple {
    pub article: String,
    pub title: String,
    pub content_portuguese: String,
    pub content_english: String,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalRight {
    pub article: String,
    pub category: String,
    pub right_name: String,
    pub content_portuguese: String,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EconomicConstitutionalFramework {
    pub economic_system_type: String,
    pub property_rights: String,
    pub social_market_principles: Vec<String>,
    pub state_intervention_limits: Vec<String>,
    pub public_sector_definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseGovernment {
    pub president: PresidentOfRepublic,
    pub prime_minister: PrimeMinister,
    pub council_of_ministers: CouncilOfMinisters,
    pub assembly_of_republic: AssemblyOfRepublic,
    pub current_legislature: String,
    pub government_formation: String,
    pub political_parties: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresidentOfRepublic {
    pub name: String,
    pub term_start: String,
    pub term_end: String,
    pub election_date: String,
    pub vote_percentage: f64,
    pub previous_occupation: String,
    pub constitutional_powers: Vec<String>,
    pub residence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrimeMinister {
    pub name: String,
    pub party: String,
    pub appointment_date: String,
    pub previous_positions: Vec<String>,
    pub government_program: Vec<String>,
    pub coalition_agreements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilOfMinisters {
    pub formation_date: String,
    pub total_ministers: u32,
    pub ministries: Vec<Ministry>,
    pub state_secretaries: Vec<StateSecretary>,
    pub weekly_meetings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ministry {
    pub name: String,
    pub minister_name: String,
    pub portfolio_areas: Vec<String>,
    pub budget_2024: f64,
    pub staff_count: u32,
    pub main_headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StateSecretary {
    pub name: String,
    pub ministry: String,
    pub specific_area: String,
    pub appointment_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssemblyOfRepublic {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub president_assembly: String,
    pub vice_presidents: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
    pub current_session: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParliamentaryGroup {
    pub party_name: String,
    pub leader: String,
    pub seats: u32,
    pub political_orientation: String,
    pub founded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoliticalParty {
    pub name: String,
    pub abbreviation: String,
    pub leader: String,
    pub founded: String,
    pub ideology: Vec<String>,
    pub membership: u32,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseTerritorialOrganization {
    pub continental_portugal: ContinentalPortugal,
    pub autonomous_regions: Vec<AutonomousRegion>,
    pub municipalities: Vec<Municipality>,
    pub parishes: Vec<Parish>,
    pub territorial_statistics: TerritorialStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContinentalPortugal {
    pub total_area_km2: f64,
    pub population: u64,
    pub districts: Vec<District>,
    pub nuts_regions: Vec<NutsRegion>,
    pub ccdr_regions: Vec<CCDRRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct District {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub municipalities_count: u32,
    pub governor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NutsRegion {
    pub name: String,
    pub nuts_code: String,
    pub level: String,
    pub area_km2: f64,
    pub population: u64,
    pub gdp_per_capita: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CCDRRegion {
    pub name: String,
    pub president: String,
    pub headquarters: String,
    pub competencies: Vec<String>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutonomousRegion {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub autonomy_statute: String,
    pub regional_government: RegionalGovernment,
    pub special_competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalGovernment {
    pub president: String,
    pub party: String,
    pub assembly_seats: u32,
    pub regional_secretaries: Vec<String>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Municipality {
    pub name: String,
    pub district: String,
    pub area_km2: f64,
    pub population: u64,
    pub mayor: String,
    pub mayor_party: String,
    pub assembly_seats: u32,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Parish {
    pub name: String,
    pub municipality: String,
    pub area_km2: f64,
    pub population: u64,
    pub parish_council: String,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerritorialStatistics {
    pub total_area_km2: f64,
    pub total_population: u64,
    pub population_density: f64,
    pub districts_total: u32,
    pub municipalities_total: u32,
    pub parishes_total: u32,
    pub urban_population_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseJudicialSystem {
    pub supreme_court_justice: SupremeCourt,
    pub constitutional_court: ConstitutionalCourt,
    pub supreme_administrative_court: SupremeAdministrativeCourt,
    pub court_of_auditors: CourtOfAuditors,
    pub judicial_courts: Vec<JudicialCourt>,
    pub administrative_courts: Vec<AdministrativeCourt>,
    pub prosecution_service: ProsecutionService,
    pub judicial_police: JudicialPolice,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupremeCourt {
    pub president: String,
    pub vice_president: String,
    pub judges_count: u32,
    pub civil_section: String,
    pub criminal_section: String,
    pub social_section: String,
    pub landmark_decisions: Vec<LandmarkDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalCourt {
    pub president: String,
    pub judges_count: u32,
    pub appointment_method: String,
    pub term_years: u32,
    pub key_competencies: Vec<String>,
    pub recent_decisions: Vec<ConstitutionalDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupremeAdministrativeCourt {
    pub president: String,
    pub sections: Vec<String>,
    pub judges_count: u32,
    pub administrative_litigation: String,
    pub tax_litigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourtOfAuditors {
    pub president: String,
    pub counselors_count: u32,
    pub audit_competencies: Vec<String>,
    pub annual_reports: Vec<String>,
    pub budget_oversight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialCourt {
    pub name: String,
    pub level: String,
    pub jurisdiction: String,
    pub location: String,
    pub judges_count: u32,
    pub annual_cases: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeCourt {
    pub name: String,
    pub jurisdiction: String,
    pub location: String,
    pub specialization: Vec<String>,
    pub judges_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProsecutionService {
    pub prosecutor_general: String,
    pub deputy_prosecutors_general: Vec<String>,
    pub republic_prosecutors: Vec<String>,
    pub structure: Vec<String>,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialPolice {
    pub national_director: String,
    pub regional_directories: Vec<String>,
    pub specialized_units: Vec<String>,
    pub investigation_competencies: Vec<String>,
    pub staff_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LandmarkDecision {
    pub case_number: String,
    pub date: String,
    pub subject: String,
    pub legal_principle: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalDecision {
    pub decision_number: String,
    pub date: String,
    pub subject: String,
    pub constitutional_articles: Vec<String>,
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseLegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub civil_procedure_code: CivilProcedureCode,
    pub criminal_procedure_code: CriminalProcedureCode,
    pub administrative_procedure_code: AdministrativeProcedureCode,
    pub labor_code: LaborCode,
    pub tax_codes: Vec<TaxCode>,
    pub commercial_code: CommercialCode,
    pub family_code: FamilyCode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilCode {
    pub official_name: String,
    pub approval_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub books: Vec<CodeBook>,
    pub key_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CriminalCode {
    pub official_name: String,
    pub approval_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub parts: Vec<CodePart>,
    pub penalty_system: PenaltySystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilProcedureCode {
    pub official_name: String,
    pub approval_date: String,
    pub procedural_principles: Vec<String>,
    pub types_of_proceedings: Vec<String>,
    pub appeal_system: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CriminalProcedureCode {
    pub official_name: String,
    pub approval_date: String,
    pub investigation_phase: String,
    pub trial_phase: String,
    pub appeal_phase: String,
    pub special_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeProcedureCode {
    pub official_name: String,
    pub approval_date: String,
    pub general_principles: Vec<String>,
    pub administrative_acts: String,
    pub procedural_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaborCode {
    pub official_name: String,
    pub approval_date: String,
    pub individual_labor_relations: String,
    pub collective_labor_relations: String,
    pub labor_tribunals: String,
    pub social_security_integration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxCode {
    pub name: String,
    pub tax_type: String,
    pub approval_date: String,
    pub tax_rates: Vec<TaxRate>,
    pub exemptions: Vec<String>,
    pub collection_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxRate {
    pub category: String,
    pub rate_percentage: f64,
    pub threshold_amount: f64,
    pub applicable_from: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommercialCode {
    pub official_name: String,
    pub approval_date: String,
    pub company_types: Vec<String>,
    pub commercial_acts: Vec<String>,
    pub insolvency_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilyCode {
    pub official_name: String,
    pub approval_date: String,
    pub marriage_regulation: String,
    pub civil_union_regulation: String,
    pub parental_responsibility: String,
    pub adoption_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodeBook {
    pub number: u32,
    pub title: String,
    pub chapters: Vec<String>,
    pub articles_range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodePart {
    pub name: String,
    pub chapters: Vec<String>,
    pub main_subjects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PenaltySystem {
    pub prison_sentences: Vec<String>,
    pub fines_system: String,
    pub alternative_measures: Vec<String>,
    pub rehabilitation_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseEuropeanIntegration {
    pub eu_membership: EUMembership,
    pub eurozone_participation: EurozoneParticipation,
    pub schengen_area: SchengenParticipation,
    pub eu_representation: EURepresentation,
    pub eu_legislation_transposition: EULegislationTransposition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EUMembership {
    pub accession_date: String,
    pub accession_treaty: String,
    pub membership_benefits: Vec<String>,
    pub membership_obligations: Vec<String>,
    pub opt_outs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EurozoneParticipation {
    pub euro_adoption_date: String,
    pub escudo_exchange_rate: String,
    pub ecb_representation: String,
    pub fiscal_rules_compliance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchengenParticipation {
    pub implementation_date: String,
    pub border_controls: String,
    pub police_cooperation: String,
    pub information_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EURepresentation {
    pub european_parliament_meps: u32,
    pub council_voting_weight: u32,
    pub european_commission_commissioner: String,
    pub permanent_representation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EULegislationTransposition {
    pub transposition_rate: f64,
    pub infringement_procedures: u32,
    pub recent_transpositions: Vec<String>,
    pub pending_transpositions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseAutonomousRegions {
    pub azores: AzoresRegion,
    pub madeira: MadeiraRegion,
    pub autonomy_constitutional_basis: String,
    pub special_competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzoresRegion {
    pub official_name: String,
    pub capital: String,
    pub islands: Vec<AzoresIsland>,
    pub regional_government: AzoresGovernment,
    pub autonomy_statute: String,
    pub special_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzoresIsland {
    pub name: String,
    pub area_km2: f64,
    pub population: u64,
    pub main_city: String,
    pub economic_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzoresGovernment {
    pub president: String,
    pub party: String,
    pub regional_assembly_seats: u32,
    pub regional_secretaries: Vec<RegionalSecretary>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MadeiraRegion {
    pub official_name: String,
    pub capital: String,
    pub islands: Vec<MadeiraIsland>,
    pub regional_government: MadeiraGovernment,
    pub autonomy_statute: String,
    pub special_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MadeiraIsland {
    pub name: String,
    pub area_km2: f64,
    pub population: u64,
    pub main_city: String,
    pub economic_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MadeiraGovernment {
    pub president: String,
    pub party: String,
    pub regional_assembly_seats: u32,
    pub regional_secretaries: Vec<RegionalSecretary>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalSecretary {
    pub name: String,
    pub portfolio: String,
    pub appointment_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortugueseLocalGovernment {
    pub municipal_system: MunicipalSystem,
    pub parish_system: ParishSystem,
    pub intermunicipal_entities: Vec<IntermunicipalEntity>,
    pub metropolitan_areas: Vec<MetropolitanArea>,
    pub local_finance: LocalFinance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MunicipalSystem {
    pub legal_framework: String,
    pub municipal_assembly: String,
    pub municipal_chamber: String,
    pub competencies: Vec<String>,
    pub financial_autonomy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParishSystem {
    pub legal_framework: String,
    pub parish_assembly: String,
    pub parish_council: String,
    pub competencies: Vec<String>,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntermunicipalEntity {
    pub name: String,
    pub entity_type: String,
    pub member_municipalities: Vec<String>,
    pub competencies: Vec<String>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetropolitanArea {
    pub name: String,
    pub municipalities: Vec<String>,
    pub population: u64,
    pub governance_structure: String,
    pub special_competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalFinance {
    pub revenue_sources: Vec<String>,
    pub transfer_system: String,
    pub borrowing_rules: String,
    pub fiscal_supervision: String,
}

impl Default for PortugalLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: PortugueseConstitution {
                name: "Constituição da República Portuguesa".to_string(),
                adoption_date: "1976-04-02".to_string(),
                last_revision: "2005-07-12".to_string(),
                total_articles: 296,
                fundamental_principles: vec![
                    ConstitutionalPrinciple {
                        article: "Artigo 1º".to_string(),
                        title: "República soberana".to_string(),
                        content_portuguese: "Portugal é uma República soberana, baseada na dignidade da pessoa humana e na vontade popular e empenhada na construção de uma sociedade livre, justa e solidária.".to_string(),
                        content_english: "Portugal is a sovereign Republic, based on human dignity and popular will and committed to building a free, just and supportive society.".to_string(),
                        interpretation: "Fundamental principle establishing Portugal as a sovereign republic based on human dignity".to_string(),
                    },
                    ConstitutionalPrinciple {
                        article: "Artigo 2º".to_string(),
                        title: "Estado de direito democrático".to_string(),
                        content_portuguese: "A República Portuguesa é um Estado de direito democrático, baseado na soberania popular, no pluralismo de expressão e organização política democráticas, no respeito e na garantia de efetivação dos direitos e liberdades fundamentais e na separação e interdependência de poderes, visando a realização da democracia económica, social e cultural e o aprofundamento da democracia participativa.".to_string(),
                        content_english: "The Portuguese Republic is a democratic rule of law state, based on popular sovereignty, pluralism of democratic political expression and organization, respect for and guarantee of fundamental rights and freedoms, and separation and interdependence of powers, aimed at achieving economic, social and cultural democracy and deepening participatory democracy.".to_string(),
                        interpretation: "Establishes Portugal as a democratic rule of law state with separation of powers and fundamental rights protection".to_string(),
                    },
                    ConstitutionalPrinciple {
                        article: "Artigo 3º".to_string(),
                        title: "Soberania e legalidade".to_string(),
                        content_portuguese: "1. A soberania, una e indivisível, reside no povo, que a exerce segundo as formas previstas na Constituição. 2. O Estado subordina-se à Constituição e funda-se na legalidade democrática. 3. A validade das leis e dos demais atos do Estado, das regiões autónomas, do poder local e de quaisquer outras entidades públicas depende da sua conformidade com a Constituição.".to_string(),
                        content_english: "1. Sovereignty, one and indivisible, resides in the people, who exercise it according to the forms provided for in the Constitution. 2. The State is subordinated to the Constitution and is founded on democratic legality. 3. The validity of laws and other acts of the State, autonomous regions, local authorities and any other public entities depends on their conformity with the Constitution.".to_string(),
                        interpretation: "Establishes popular sovereignty and constitutional supremacy over all state acts".to_string(),
                    },
                ],
                rights_and_duties: vec![
                    ConstitutionalRight {
                        article: "Artigo 13º".to_string(),
                        category: "Princípio da igualdade".to_string(),
                        right_name: "Equality principle".to_string(),
                        content_portuguese: "1. Todos os cidadãos têm a mesma dignidade social e são iguais perante a lei. 2. Ninguém pode ser privilegiado, beneficiado, prejudicado, privado de qualquer direito ou isento de qualquer dever em razão de ascendência, sexo, raça, língua, território de origem, religião, convicções políticas ou ideológicas, instrução, situação económica, condição social ou orientação sexual.".to_string(),
                        limitations: vec!["Temporary special measures for equality promotion allowed".to_string()],
                        jurisprudence: vec!["Constitutional Court Decision 359/2009 on positive discrimination".to_string()],
                    },
                    ConstitutionalRight {
                        article: "Artigo 24º".to_string(),
                        category: "Direito à vida".to_string(),
                        right_name: "Right to life".to_string(),
                        content_portuguese: "1. A vida humana é inviolável. 2. Em caso algum haverá pena de morte.".to_string(),
                        limitations: vec!["No limitations - absolute prohibition of death penalty".to_string()],
                        jurisprudence: vec!["Constitutional Court landmark decision abolishing death penalty".to_string()],
                    },
                ],
                economic_system: EconomicConstitutionalFramework {
                    economic_system_type: "Mixed economy with social market characteristics".to_string(),
                    property_rights: "Private property and inheritance guaranteed with social function".to_string(),
                    social_market_principles: vec![
                        "Market economy with social correction".to_string(),
                        "State intervention in strategic sectors".to_string(),
                        "Public service provision guarantee".to_string(),
                    ],
                    state_intervention_limits: vec![
                        "Proportionality principle".to_string(),
                        "Subsidiarity in economic intervention".to_string(),
                        "Respect for private initiative".to_string(),
                    ],
                    public_sector_definition: "State, autonomous regions, and local authorities economic activities".to_string(),
                },
                key_articles: {
                    let mut articles = HashMap::new();
                    articles.insert("Article_1".to_string(), "Portugal é uma República soberana, baseada na dignidade da pessoa humana".to_string());
                    articles.insert("Article_2".to_string(), "Estado de direito democrático baseado na soberania popular".to_string());
                    articles.insert("Article_6".to_string(), "Estado unitário com autonomias regionais nos Açores e Madeira".to_string());
                    articles.insert("Article_81".to_string(), "Incumbências do Estado na economia social de mercado".to_string());
                    articles
                },
            },
            government_structure: PortugueseGovernment {
                president: PresidentOfRepublic {
                    name: "Marcelo Rebelo de Sousa".to_string(),
                    term_start: "2016-03-09".to_string(),
                    term_end: "2026-03-09".to_string(),
                    election_date: "2021-01-24".to_string(),
                    vote_percentage: 60.70,
                    previous_occupation: "Constitutional Law Professor, Journalist, Politician".to_string(),
                    constitutional_powers: vec![
                        "Appoint Prime Minister".to_string(),
                        "Dissolve Assembly of the Republic".to_string(),
                        "Promulgate or veto laws".to_string(),
                        "Commander-in-Chief of Armed Forces".to_string(),
                        "Representative of Portugal in international relations".to_string(),
                    ],
                    residence: "Palácio de Belém, Lisboa".to_string(),
                },
                prime_minister: PrimeMinister {
                    name: "António Costa".to_string(),
                    party: "PS - Partido Socialista".to_string(),
                    appointment_date: "2015-11-26".to_string(),
                    previous_positions: vec![
                        "Mayor of Lisbon (2007-2015)".to_string(),
                        "Minister of Parliamentary Affairs (1995-1997)".to_string(),
                        "Minister of Justice (1999-2002)".to_string(),
                        "Minister of Internal Administration (2005-2007)".to_string(),
                    ],
                    government_program: vec![
                        "Economic recovery and job creation".to_string(),
                        "Public service strengthening".to_string(),
                        "Climate action and green transition".to_string(),
                        "Digital transformation".to_string(),
                        "Social cohesion and territorial development".to_string(),
                    ],
                    coalition_agreements: vec![
                        "Confidence and supply agreement with Left Bloc (2015-2019)".to_string(),
                        "Parliamentary support from PCP and PEV (2015-2019)".to_string(),
                        "Absolute majority since 2022".to_string(),
                    ],
                },
                council_of_ministers: CouncilOfMinisters {
                    formation_date: "2022-03-30".to_string(),
                    total_ministers: 18,
                    ministries: vec![
                        Ministry {
                            name: "Ministério da Presidência".to_string(),
                            minister_name: "Mariana Vieira da Silva".to_string(),
                            portfolio_areas: vec!["Government coordination".to_string(), "Modernization".to_string()],
                            budget_2024: 1_200_000_000.0,
                            staff_count: 850,
                            main_headquarters: "Rua da Imprensa à Estrela, Lisboa".to_string(),
                        },
                        Ministry {
                            name: "Ministério das Finanças".to_string(),
                            minister_name: "Fernando Medina".to_string(),
                            portfolio_areas: vec!["Public finances".to_string(), "Tax policy".to_string(), "Budget".to_string()],
                            budget_2024: 2_500_000_000.0,
                            staff_count: 12_500,
                            main_headquarters: "Rua da Alfândega, Lisboa".to_string(),
                        },
                        Ministry {
                            name: "Ministério da Defesa Nacional".to_string(),
                            minister_name: "Helena Carreiras".to_string(),
                            portfolio_areas: vec!["Armed Forces".to_string(), "Defense policy".to_string(), "Security".to_string()],
                            budget_2024: 3_100_000_000.0,
                            staff_count: 32_000,
                            main_headquarters: "Avenida da Ilha da Madeira, Lisboa".to_string(),
                        },
                    ],
                    state_secretaries: vec![
                        StateSecretary {
                            name: "José Luís Carneiro".to_string(),
                            ministry: "Ministério da Administração Interna".to_string(),
                            specific_area: "Internal Administration".to_string(),
                            appointment_date: "2022-03-30".to_string(),
                        },
                    ],
                    weekly_meetings: "Every Thursday at 10:00 AM at São Bento Palace".to_string(),
                },
                assembly_of_republic: AssemblyOfRepublic {
                    seats_total: 230,
                    current_composition: {
                        let mut composition = HashMap::new();
                        composition.insert("PS".to_string(), 120);
                        composition.insert("PSD".to_string(), 77);
                        composition.insert("Chega".to_string(), 12);
                        composition.insert("IL".to_string(), 8);
                        composition.insert("BE".to_string(), 5);
                        composition.insert("PCP".to_string(), 4);
                        composition.insert("PAN".to_string(), 1);
                        composition.insert("Livre".to_string(), 1);
                        composition.insert("PEV".to_string(), 2);
                        composition
                    },
                    president_assembly: "Augusto Santos Silva".to_string(),
                    vice_presidents: vec![
                        "Edite Estrela".to_string(),
                        "José Manuel Pureza".to_string(),
                        "Teresa Caeiro".to_string(),
                        "António Filipe".to_string(),
                    ],
                    parliamentary_groups: vec![
                        ParliamentaryGroup {
                            party_name: "Partido Socialista".to_string(),
                            leader: "Eurico Brilhante Dias".to_string(),
                            seats: 120,
                            political_orientation: "Social Democratic".to_string(),
                            founded: "1973".to_string(),
                        },
                        ParliamentaryGroup {
                            party_name: "Partido Social Democrata".to_string(),
                            leader: "Hugo Soares".to_string(),
                            seats: 77,
                            political_orientation: "Liberal Conservative".to_string(),
                            founded: "1974".to_string(),
                        },
                    ],
                    current_session: "XV Legislature (2022-2026)".to_string(),
                },
                current_legislature: "XV Constitutional Government".to_string(),
                government_formation: "Socialist Party absolute majority since January 2022".to_string(),
                political_parties: vec![
                    PoliticalParty {
                        name: "Partido Socialista".to_string(),
                        abbreviation: "PS".to_string(),
                        leader: "António Costa".to_string(),
                        founded: "1973-04-19".to_string(),
                        ideology: vec!["Social democracy".to_string(), "Pro-European".to_string()],
                        membership: 45_000,
                        headquarters: "Largo do Rato, Lisboa".to_string(),
                    },
                    PoliticalParty {
                        name: "Partido Social Democrata".to_string(),
                        abbreviation: "PSD".to_string(),
                        leader: "Luís Montenegro".to_string(),
                        founded: "1974-05-06".to_string(),
                        ideology: vec!["Liberal conservatism".to_string(), "Christian democracy".to_string()],
                        membership: 40_000,
                        headquarters: "Rua de São Caetano, Lisboa".to_string(),
                    },
                ],
            },
            territorial_organization: PortugueseTerritorialOrganization {
                continental_portugal: ContinentalPortugal {
                    total_area_km2: 89_015.0,
                    population: 8_918_123,
                    districts: vec![
                        District {
                            name: "Lisboa".to_string(),
                            capital: "Lisboa".to_string(),
                            area_km2: 2_761.0,
                            population: 2_308_718,
                            municipalities_count: 16,
                            governor: "Jorge Seguro Sanches".to_string(),
                        },
                        District {
                            name: "Porto".to_string(),
                            capital: "Porto".to_string(),
                            area_km2: 2_395.0,
                            population: 1_817_117,
                            municipalities_count: 18,
                            governor: "Pedro Alves".to_string(),
                        },
                    ],
                    nuts_regions: vec![
                        NutsRegion {
                            name: "Norte".to_string(),
                            nuts_code: "PT11".to_string(),
                            level: "NUTS II".to_string(),
                            area_km2: 21_278.0,
                            population: 3_572_583,
                            gdp_per_capita: 18_500.0,
                        },
                        NutsRegion {
                            name: "Centro".to_string(),
                            nuts_code: "PT16".to_string(),
                            level: "NUTS II".to_string(),
                            area_km2: 28_462.0,
                            population: 2_219_478,
                            gdp_per_capita: 17_200.0,
                        },
                    ],
                    ccdr_regions: vec![
                        CCDRRegion {
                            name: "CCDR Norte".to_string(),
                            president: "António Cunha".to_string(),
                            headquarters: "Porto".to_string(),
                            competencies: vec!["Regional development".to_string(), "EU funds management".to_string()],
                            budget_2024: 850_000_000.0,
                        },
                    ],
                },
                autonomous_regions: vec![
                    AutonomousRegion {
                        name: "Região Autónoma dos Açores".to_string(),
                        capital: "Angra do Heroísmo".to_string(),
                        area_km2: 2_322.0,
                        population: 236_657,
                        autonomy_statute: "Political-Administrative Statute of the Azores".to_string(),
                        regional_government: RegionalGovernment {
                            president: "José Manuel Bolieiro".to_string(),
                            party: "PSD".to_string(),
                            assembly_seats: 57,
                            regional_secretaries: vec!["Regional Secretary for Finance".to_string()],
                            budget_2024: 1_800_000_000.0,
                        },
                        special_competencies: vec![
                            "Exclusive legislative competency in specific areas".to_string(),
                            "Tax autonomy".to_string(),
                            "Administrative autonomy".to_string(),
                        ],
                    },
                    AutonomousRegion {
                        name: "Região Autónoma da Madeira".to_string(),
                        capital: "Funchal".to_string(),
                        area_km2: 801.0,
                        population: 251_060,
                        autonomy_statute: "Political-Administrative Statute of Madeira".to_string(),
                        regional_government: RegionalGovernment {
                            president: "Miguel Albuquerque".to_string(),
                            party: "PSD".to_string(),
                            assembly_seats: 47,
                            regional_secretaries: vec!["Regional Secretary for Tourism".to_string()],
                            budget_2024: 1_500_000_000.0,
                        },
                        special_competencies: vec![
                            "Legislative autonomy in regional matters".to_string(),
                            "Financial autonomy".to_string(),
                            "Administrative self-government".to_string(),
                        ],
                    },
                ],
                municipalities: vec![
                    Municipality {
                        name: "Lisboa".to_string(),
                        district: "Lisboa".to_string(),
                        area_km2: 100.1,
                        population: 544_851,
                        mayor: "Carlos Moedas".to_string(),
                        mayor_party: "PSD".to_string(),
                        assembly_seats: 17,
                        budget_2024: 950_000_000.0,
                    },
                    Municipality {
                        name: "Porto".to_string(),
                        district: "Porto".to_string(),
                        area_km2: 41.4,
                        population: 231_962,
                        mayor: "Rui Moreira".to_string(),
                        mayor_party: "Independent".to_string(),
                        assembly_seats: 13,
                        budget_2024: 420_000_000.0,
                    },
                ],
                parishes: vec![
                    Parish {
                        name: "Santa Maria Maior".to_string(),
                        municipality: "Lisboa".to_string(),
                        area_km2: 1.5,
                        population: 12_456,
                        parish_council: "Junta de Freguesia de Santa Maria Maior".to_string(),
                        budget_2024: 2_500_000.0,
                    },
                ],
                territorial_statistics: TerritorialStatistics {
                    total_area_km2: 92_212.0,
                    total_population: 10_347_892,
                    population_density: 112.3,
                    districts_total: 18,
                    municipalities_total: 308,
                    parishes_total: 3_091,
                    urban_population_percentage: 66.3,
                },
            },
            judicial_system: PortugueseJudicialSystem {
                supreme_court_justice: SupremeCourt {
                    president: "Henrique Araújo".to_string(),
                    vice_president: "Maria José Costeira".to_string(),
                    judges_count: 70,
                    civil_section: "1st Section - Civil and Commercial Law".to_string(),
                    criminal_section: "2nd Section - Criminal Law".to_string(),
                    social_section: "3rd Section - Social Law".to_string(),
                    landmark_decisions: vec![
                        LandmarkDecision {
                            case_number: "STJ 156/2018".to_string(),
                            date: "2018-03-15".to_string(),
                            subject: "Constitutional interpretation of property rights".to_string(),
                            legal_principle: "Property rights must be balanced with social function".to_string(),
                            impact: "Established framework for expropriation procedures".to_string(),
                        },
                    ],
                    headquarters: "Largo do Linhó, Lisboa".to_string(),
                },
                constitutional_court: ConstitutionalCourt {
                    president: "João Pedro Caupers".to_string(),
                    judges_count: 13,
                    appointment_method: "10 by Assembly, 3 by co-optation".to_string(),
                    term_years: 9,
                    key_competencies: vec![
                        "Constitutional review".to_string(),
                        "Electoral disputes".to_string(),
                        "Presidential and government member trials".to_string(),
                    ],
                    recent_decisions: vec![
                        ConstitutionalDecision {
                            decision_number: "TC 123/2023".to_string(),
                            date: "2023-02-10".to_string(),
                            subject: "Constitutionality of tax measures".to_string(),
                            constitutional_articles: vec!["Article 103".to_string()],
                            outcome: "Partially unconstitutional".to_string(),
                        },
                    ],
                },
                supreme_administrative_court: SupremeAdministrativeCourt {
                    president: "Maria José Rangel de Mesquita".to_string(),
                    sections: vec!["Administrative Litigation".to_string(), "Tax Litigation".to_string()],
                    judges_count: 48,
                    administrative_litigation: "Administrative acts review and public contracts".to_string(),
                    tax_litigation: "Tax disputes and fiscal procedures".to_string(),
                },
                court_of_auditors: CourtOfAuditors {
                    president: "José Tavares".to_string(),
                    counselors_count: 19,
                    audit_competencies: vec![
                        "Public accounts audit".to_string(),
                        "Budget execution control".to_string(),
                        "Public procurement oversight".to_string(),
                    ],
                    annual_reports: vec!["State Budget Execution Report 2023".to_string()],
                    budget_oversight: "Parliamentary budget control support".to_string(),
                },
                judicial_courts: vec![
                    JudicialCourt {
                        name: "Tribunal da Relação de Lisboa".to_string(),
                        level: "Appeal Court".to_string(),
                        jurisdiction: "Lisbon Appeal District".to_string(),
                        location: "Lisboa".to_string(),
                        judges_count: 95,
                        annual_cases: 15_642,
                    },
                ],
                administrative_courts: vec![
                    AdministrativeCourt {
                        name: "Tribunal Central Administrativo Norte".to_string(),
                        jurisdiction: "Northern Portugal".to_string(),
                        location: "Porto".to_string(),
                        specialization: vec!["Public contracts".to_string(), "Urban planning".to_string()],
                        judges_count: 22,
                    },
                ],
                prosecution_service: ProsecutionService {
                    prosecutor_general: "Lucília Gago".to_string(),
                    deputy_prosecutors_general: vec!["José Eduardo Moreira".to_string()],
                    republic_prosecutors: vec!["Multiple district prosecutors".to_string()],
                    structure: vec!["Hierarchical organization".to_string(), "Functional autonomy".to_string()],
                    competencies: vec!["Criminal prosecution".to_string(), "Law enforcement".to_string()],
                },
                judicial_police: JudicialPolice {
                    national_director: "Luís Neves".to_string(),
                    regional_directories: vec!["Porto".to_string(), "Coimbra".to_string(), "Lisboa".to_string()],
                    specialized_units: vec!["Economic crimes".to_string(), "Cybercrime".to_string()],
                    investigation_competencies: vec!["Serious crime investigation".to_string()],
                    staff_count: 4_500,
                },
            },
            legal_codes: PortugueseLegalCodes {
                civil_code: CivilCode {
                    official_name: "Código Civil Português".to_string(),
                    approval_date: "1966-11-25".to_string(),
                    last_amendment: "2023-09-15".to_string(),
                    total_articles: 2_334,
                    books: vec![
                        CodeBook {
                            number: 1,
                            title: "General Part".to_string(),
                            chapters: vec!["Legal sources".to_string(), "Application of law".to_string()],
                            articles_range: "Articles 1-396".to_string(),
                        },
                        CodeBook {
                            number: 2,
                            title: "Law of Obligations".to_string(),
                            chapters: vec!["Contracts".to_string(), "Non-contractual obligations".to_string()],
                            articles_range: "Articles 397-1250".to_string(),
                        },
                    ],
                    key_principles: vec![
                        "Good faith principle".to_string(),
                        "Autonomy of will".to_string(),
                        "Legal security".to_string(),
                    ],
                },
                criminal_code: CriminalCode {
                    official_name: "Código Penal Português".to_string(),
                    approval_date: "1982-09-15".to_string(),
                    last_amendment: "2023-07-20".to_string(),
                    total_articles: 386,
                    parts: vec![
                        CodePart {
                            name: "General Part".to_string(),
                            chapters: vec!["Criminal law application".to_string(), "Crime".to_string()],
                            main_subjects: vec!["Criminal liability".to_string(), "Penalties".to_string()],
                        },
                        CodePart {
                            name: "Special Part".to_string(),
                            chapters: vec!["Crimes against persons".to_string(), "Crimes against property".to_string()],
                            main_subjects: vec!["Specific criminal types".to_string(), "Aggravating circumstances".to_string()],
                        },
                    ],
                    penalty_system: PenaltySystem {
                        prison_sentences: vec!["Up to 25 years maximum".to_string(), "Life imprisonment abolished".to_string()],
                        fines_system: "Day-fine system based on defendant's income".to_string(),
                        alternative_measures: vec!["Community service".to_string(), "Electronic monitoring".to_string()],
                        rehabilitation_programs: vec!["Drug treatment programs".to_string(), "Professional training".to_string()],
                    },
                },
                civil_procedure_code: CivilProcedureCode {
                    official_name: "Código de Processo Civil".to_string(),
                    approval_date: "2013-02-26".to_string(),
                    procedural_principles: vec!["Adversarial principle".to_string(), "Cooperation principle".to_string()],
                    types_of_proceedings: vec!["Declaratory actions".to_string(), "Executive actions".to_string()],
                    appeal_system: vec!["Appeal to higher courts".to_string(), "Supreme Court review".to_string()],
                },
                criminal_procedure_code: CriminalProcedureCode {
                    official_name: "Código de Processo Penal".to_string(),
                    approval_date: "1987-02-17".to_string(),
                    investigation_phase: "Judicial investigation with prosecution service leadership".to_string(),
                    trial_phase: "Public oral trial with presumption of innocence".to_string(),
                    appeal_phase: "Right to appeal to higher instances".to_string(),
                    special_procedures: vec!["Simplified procedures".to_string(), "Plea bargaining".to_string()],
                },
                administrative_procedure_code: AdministrativeProcedureCode {
                    official_name: "Código do Procedimento Administrativo".to_string(),
                    approval_date: "2015-04-07".to_string(),
                    general_principles: vec!["Legality".to_string(), "Equality".to_string(), "Proportionality".to_string()],
                    administrative_acts: "Administrative decisions binding on citizens".to_string(),
                    procedural_guarantees: vec!["Right to be heard".to_string(), "Right to legal representation".to_string()],
                },
                labor_code: LaborCode {
                    official_name: "Código do Trabalho".to_string(),
                    approval_date: "2009-02-12".to_string(),
                    individual_labor_relations: "Employment contracts and worker rights".to_string(),
                    collective_labor_relations: "Trade unions and collective bargaining".to_string(),
                    labor_tribunals: "Specialized labor courts for employment disputes".to_string(),
                    social_security_integration: "Coordination with social security system".to_string(),
                },
                tax_codes: vec![
                    TaxCode {
                        name: "Código do IRS".to_string(),
                        tax_type: "Personal Income Tax".to_string(),
                        approval_date: "1988-12-30".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Category A - Employment income".to_string(),
                                rate_percentage: 48.0,
                                threshold_amount: 80_640.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Minimum subsistence level".to_string()],
                        collection_procedures: "Monthly withholding and annual declaration".to_string(),
                    },
                ],
                commercial_code: CommercialCode {
                    official_name: "Código das Sociedades Comerciais".to_string(),
                    approval_date: "1986-09-02".to_string(),
                    company_types: vec!["SA - Sociedade Anónima".to_string(), "Lda - Sociedade por Quotas".to_string()],
                    commercial_acts: vec!["Commercial transactions".to_string(), "Business registration".to_string()],
                    insolvency_procedures: "Código da Insolvência e da Recuperação de Empresas".to_string(),
                },
                family_code: FamilyCode {
                    official_name: "Regime Jurídico da Família".to_string(),
                    approval_date: "1977-05-25".to_string(),
                    marriage_regulation: "Civil and religious marriage recognition".to_string(),
                    civil_union_regulation: "União de facto legal recognition".to_string(),
                    parental_responsibility: "Joint parental authority principle".to_string(),
                    adoption_procedures: "Full and simple adoption types".to_string(),
                },
            },
            european_integration: PortugueseEuropeanIntegration {
                eu_membership: EUMembership {
                    accession_date: "1986-01-01".to_string(),
                    accession_treaty: "Treaty of Accession 1985".to_string(),
                    membership_benefits: vec![
                        "Single market access".to_string(),
                        "Structural funds".to_string(),
                        "Freedom of movement".to_string(),
                    ],
                    membership_obligations: vec![
                        "EU law transposition".to_string(),
                        "Budget contribution".to_string(),
                        "Policy coordination".to_string(),
                    ],
                    opt_outs: vec![],
                },
                eurozone_participation: EurozoneParticipation {
                    euro_adoption_date: "1999-01-01".to_string(),
                    escudo_exchange_rate: "200.482 escudos = 1 euro".to_string(),
                    ecb_representation: "Carlos Costa (former Governor, Bank of Portugal)".to_string(),
                    fiscal_rules_compliance: vec!["Stability and Growth Pact".to_string(), "Fiscal Compact".to_string()],
                },
                schengen_area: SchengenParticipation {
                    implementation_date: "1995-03-26".to_string(),
                    border_controls: "Free movement within Schengen Area".to_string(),
                    police_cooperation: "European police cooperation enhanced".to_string(),
                    information_systems: vec!["SIS II".to_string(), "VIS".to_string()],
                },
                eu_representation: EURepresentation {
                    european_parliament_meps: 21,
                    council_voting_weight: 12,
                    european_commission_commissioner: "Elisa Ferreira (Cohesion and Reforms)".to_string(),
                    permanent_representation: "Ambassador Nuno Brito to EU".to_string(),
                },
                eu_legislation_transposition: EULegislationTransposition {
                    transposition_rate: 98.5,
                    infringement_procedures: 15,
                    recent_transpositions: vec!["Digital Services Act".to_string(), "Digital Markets Act".to_string()],
                    pending_transpositions: vec!["AI Act implementation".to_string()],
                },
            },
            autonomous_regions: PortugueseAutonomousRegions {
                azores: AzoresRegion {
                    official_name: "Região Autónoma dos Açores".to_string(),
                    capital: "Angra do Heroísmo".to_string(),
                    islands: vec![
                        AzoresIsland {
                            name: "São Miguel".to_string(),
                            area_km2: 744.0,
                            population: 137_856,
                            main_city: "Ponta Delgada".to_string(),
                            economic_activities: vec!["Tourism".to_string(), "Agriculture".to_string(), "Fishing".to_string()],
                        },
                        AzoresIsland {
                            name: "Terceira".to_string(),
                            area_km2: 402.0,
                            population: 53_311,
                            main_city: "Angra do Heroísmo".to_string(),
                            economic_activities: vec!["Military base".to_string(), "Tourism".to_string(), "Dairy".to_string()],
                        },
                    ],
                    regional_government: AzoresGovernment {
                        president: "José Manuel Bolieiro".to_string(),
                        party: "PSD/CDS-PP/PPM Coalition".to_string(),
                        regional_assembly_seats: 57,
                        regional_secretaries: vec![
                            RegionalSecretary {
                                name: "Berta Cabral".to_string(),
                                portfolio: "Regional Secretary for Finance, Planning and Public Administration".to_string(),
                                appointment_date: "2020-11-24".to_string(),
                            },
                        ],
                        budget_2024: 1_800_000_000.0,
                    },
                    autonomy_statute: "Political-Administrative Statute approved by Law 39/80".to_string(),
                    special_powers: vec![
                        "Legislative autonomy in regional matters".to_string(),
                        "Administrative autonomy".to_string(),
                        "Financial autonomy with own revenues".to_string(),
                    ],
                },
                madeira: MadeiraRegion {
                    official_name: "Região Autónoma da Madeira".to_string(),
                    capital: "Funchal".to_string(),
                    islands: vec![
                        MadeiraIsland {
                            name: "Madeira".to_string(),
                            area_km2: 741.0,
                            population: 251_060,
                            main_city: "Funchal".to_string(),
                            economic_activities: vec!["Tourism".to_string(), "Wine production".to_string(), "Embroidery".to_string()],
                        },
                        MadeiraIsland {
                            name: "Porto Santo".to_string(),
                            area_km2: 42.0,
                            population: 5_483,
                            main_city: "Vila Baleira".to_string(),
                            economic_activities: vec!["Tourism".to_string(), "Agriculture".to_string()],
                        },
                    ],
                    regional_government: MadeiraGovernment {
                        president: "Miguel Albuquerque".to_string(),
                        party: "PSD".to_string(),
                        regional_assembly_seats: 47,
                        regional_secretaries: vec![
                            RegionalSecretary {
                                name: "Rogério Gouveia".to_string(),
                                portfolio: "Regional Secretary for Economy, Tourism and Culture".to_string(),
                                appointment_date: "2019-09-24".to_string(),
                            },
                        ],
                        budget_2024: 1_500_000_000.0,
                    },
                    autonomy_statute: "Political-Administrative Statute approved by Law 13/91".to_string(),
                    special_powers: vec![
                        "Exclusive legislative competency in regional matters".to_string(),
                        "Administrative self-government".to_string(),
                        "Financial autonomy and fiscal benefits".to_string(),
                    ],
                },
                autonomy_constitutional_basis: "Article 6 and Title VII of the Constitution".to_string(),
                special_competencies: vec![
                    "Regional legislation in areas not reserved to the State".to_string(),
                    "Administrative autonomy in regional matters".to_string(),
                    "Financial autonomy with regional taxes".to_string(),
                    "Participation in European Union affairs affecting the regions".to_string(),
                ],
            },
            local_government: PortugueseLocalGovernment {
                municipal_system: MunicipalSystem {
                    legal_framework: "Law 75/2013 - Local Authorities Statute".to_string(),
                    municipal_assembly: "Deliberative body elected by universal suffrage".to_string(),
                    municipal_chamber: "Executive body with mayor and municipal councillors".to_string(),
                    competencies: vec![
                        "Urban planning and management".to_string(),
                        "Local infrastructure".to_string(),
                        "Social services".to_string(),
                        "Environmental protection".to_string(),
                    ],
                    financial_autonomy: "Own revenues and state transfers".to_string(),
                },
                parish_system: ParishSystem {
                    legal_framework: "Law 75/2013 - Local Authorities Statute".to_string(),
                    parish_assembly: "Local deliberative body".to_string(),
                    parish_council: "Executive body with president and councillors".to_string(),
                    competencies: vec![
                        "Local services provision".to_string(),
                        "Cultural and recreational activities".to_string(),
                        "Local development support".to_string(),
                    ],
                    recent_reforms: vec![
                        "Parish aggregation reform 2013".to_string(),
                        "Competencies transfer to parishes".to_string(),
                    ],
                },
                intermunicipal_entities: vec![
                    IntermunicipalEntity {
                        name: "Área Metropolitana de Lisboa".to_string(),
                        entity_type: "Metropolitan Area".to_string(),
                        member_municipalities: vec!["Lisboa".to_string(), "Sintra".to_string(), "Cascais".to_string()],
                        competencies: vec!["Metropolitan transport".to_string(), "Spatial planning".to_string()],
                        headquarters: "Lisboa".to_string(),
                    },
                ],
                metropolitan_areas: vec![
                    MetropolitanArea {
                        name: "Área Metropolitana do Porto".to_string(),
                        municipalities: vec!["Porto".to_string(), "Vila Nova de Gaia".to_string(), "Matosinhos".to_string()],
                        population: 1_722_374,
                        governance_structure: "Metropolitan Council with municipal representatives".to_string(),
                        special_competencies: vec!["Public transport coordination".to_string(), "Waste management".to_string()],
                    },
                ],
                local_finance: LocalFinance {
                    revenue_sources: vec!["Municipal taxes".to_string(), "State transfers".to_string(), "EU funds".to_string()],
                    transfer_system: "General Grant for Municipalities (FGM)".to_string(),
                    borrowing_rules: "Debt limit and authorization procedures".to_string(),
                    fiscal_supervision: "Court of Auditors oversight and reporting".to_string(),
                },
            },
        }
    }
}