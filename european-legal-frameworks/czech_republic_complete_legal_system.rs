use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete Czech Republic Legal System Implementation
/// Česká republika (Czech Republic)
/// Current President: Petr Pavel (2023-2028)
/// Current Prime Minister: Petr Fiala (ODS - Civic Democratic Party)
/// Current Government: Fiala's Cabinet (SPOLU + Pirates and Mayors coalition)
/// EU Member since 2004, Not in Eurozone (Czech koruna), Schengen since 2007

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechRepublicLegalSystem {
    pub constitutional_framework: CzechConstitution,
    pub government_structure: CzechGovernment,
    pub territorial_organization: CzechTerritorialOrganization,
    pub judicial_system: CzechJudicialSystem,
    pub legal_codes: CzechLegalCodes,
    pub european_integration: CzechEuropeanIntegration,
    pub regional_administration: CzechRegionalAdministration,
    pub local_government: CzechLocalGovernment,
    pub constitutional_court_jurisprudence: ConstitutionalCourtJurisprudence,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechConstitution {
    pub name: String,
    pub adoption_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub constitutional_acts: Vec<ConstitutionalAct>,
    pub fundamental_principles: Vec<ConstitutionalPrinciple>,
    pub rights_and_duties: Vec<ConstitutionalRight>,
    pub economic_system: EconomicConstitutionalFramework,
    pub key_articles: HashMap<String, String>,
    pub charter_of_rights: CharterOfRights,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalAct {
    pub number: String,
    pub name: String,
    pub adoption_date: String,
    pub subject_matter: String,
    pub key_provisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalPrinciple {
    pub article: String,
    pub title: String,
    pub content_czech: String,
    pub content_english: String,
    pub interpretation: String,
    pub constitutional_court_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalRight {
    pub article: String,
    pub category: String,
    pub right_name: String,
    pub content_czech: String,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
    pub european_convention_alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EconomicConstitutionalFramework {
    pub economic_system_type: String,
    pub property_rights: String,
    pub market_economy_principles: Vec<String>,
    pub state_intervention_limits: Vec<String>,
    pub public_sector_definition: String,
    pub privatization_framework: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharterOfRights {
    pub official_name: String,
    pub adoption_date: String,
    pub constitutional_status: String,
    pub fundamental_rights: Vec<FundamentalRight>,
    pub political_rights: Vec<PoliticalRight>,
    pub economic_social_cultural_rights: Vec<EconomicRight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundamentalRight {
    pub article: String,
    pub right_name: String,
    pub content: String,
    pub absolute_or_relative: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoliticalRight {
    pub article: String,
    pub right_name: String,
    pub content: String,
    pub exercise_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EconomicRight {
    pub article: String,
    pub right_name: String,
    pub content: String,
    pub state_obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechGovernment {
    pub president: PresidentOfRepublic,
    pub prime_minister: PrimeMinister,
    pub government_cabinet: GovernmentCabinet,
    pub chamber_of_deputies: ChamberOfDeputies,
    pub senate: Senate,
    pub current_legislature: String,
    pub coalition_government: CoalitionGovernment,
    pub political_parties: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresidentOfRepublic {
    pub name: String,
    pub term_start: String,
    pub term_end: String,
    pub election_date: String,
    pub vote_percentage_second_round: f64,
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
    pub coalition_partners: Vec<String>,
    pub reforms_agenda: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernmentCabinet {
    pub formation_date: String,
    pub total_ministers: u32,
    pub ministries: Vec<Ministry>,
    pub deputy_prime_ministers: Vec<DeputyPrimeMinister>,
    pub government_spokesperson: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ministry {
    pub name: String,
    pub minister_name: String,
    pub party: String,
    pub portfolio_areas: Vec<String>,
    pub budget_2024: f64,
    pub staff_count: u32,
    pub main_headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeputyPrimeMinister {
    pub name: String,
    pub party: String,
    pub portfolio: String,
    pub appointment_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChamberOfDeputies {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub speaker: String,
    pub deputy_speakers: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
    pub current_session: String,
    pub electoral_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Senate {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub president: String,
    pub vice_presidents: Vec<String>,
    pub electoral_system: String,
    pub term_length: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoalitionGovernment {
    pub coalition_name: String,
    pub participating_parties: Vec<String>,
    pub coalition_agreement: String,
    pub seat_distribution: HashMap<String, u32>,
    pub key_policy_agreements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParliamentaryGroup {
    pub party_name: String,
    pub leader: String,
    pub seats: u32,
    pub political_orientation: String,
    pub founded: String,
    pub european_affiliation: String,
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
    pub european_party_family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechTerritorialOrganization {
    pub administrative_division: AdministrativeDivision,
    pub regions: Vec<Region>,
    pub districts: Vec<District>,
    pub municipalities: Vec<Municipality>,
    pub prague_special_status: PragueSpecialStatus,
    pub territorial_statistics: TerritorialStatistics,
    pub territorial_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeDivision {
    pub total_area_km2: f64,
    pub population: u64,
    pub regions_count: u32,
    pub districts_count: u32,
    pub municipalities_count: u32,
    pub with_extended_powers_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Region {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub districts_count: u32,
    pub regional_governor: String,
    pub regional_assembly: RegionalAssembly,
    pub competencies: Vec<String>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalAssembly {
    pub members_count: u32,
    pub political_composition: HashMap<String, u32>,
    pub current_term: String,
    pub president: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct District {
    pub name: String,
    pub region: String,
    pub area_km2: f64,
    pub population: u64,
    pub administrative_center: String,
    pub municipalities_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Municipality {
    pub name: String,
    pub district: String,
    pub area_km2: f64,
    pub population: u64,
    pub mayor: String,
    pub mayor_party: String,
    pub municipal_council_seats: u32,
    pub municipal_status: String,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PragueSpecialStatus {
    pub constitutional_basis: String,
    pub special_competencies: Vec<String>,
    pub municipal_districts: u32,
    pub lord_mayor: String,
    pub prague_assembly: PragueAssembly,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PragueAssembly {
    pub members_count: u32,
    pub political_composition: HashMap<String, u32>,
    pub president: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerritorialStatistics {
    pub total_area_km2: f64,
    pub total_population: u64,
    pub population_density: f64,
    pub urban_population_percentage: f64,
    pub largest_cities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechJudicialSystem {
    pub supreme_court: SupremeCourt,
    pub supreme_administrative_court: SupremeAdministrativeCourt,
    pub constitutional_court: ConstitutionalCourt,
    pub high_courts: Vec<HighCourt>,
    pub regional_courts: Vec<RegionalCourt>,
    pub district_courts: Vec<DistrictCourt>,
    pub prosecution_service: ProsecutionService,
    pub judicial_independence: JudicialIndependence,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupremeCourt {
    pub president: String,
    pub vice_president: String,
    pub justices_count: u32,
    pub civil_chamber: String,
    pub criminal_chamber: String,
    pub competencies: Vec<String>,
    pub landmark_decisions: Vec<LandmarkDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupremeAdministrativeCourt {
    pub president: String,
    pub vice_president: String,
    pub justices_count: u32,
    pub competencies: Vec<String>,
    pub recent_decisions: Vec<AdministrativeDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalCourt {
    pub president: String,
    pub vice_president: String,
    pub justices_count: u32,
    pub appointment_method: String,
    pub term_years: u32,
    pub competencies: Vec<String>,
    pub recent_decisions: Vec<ConstitutionalDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighCourt {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub specializations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalCourt {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub annual_cases: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistrictCourt {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub annual_cases: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProsecutionService {
    pub supreme_prosecutor: String,
    pub high_prosecutors: Vec<String>,
    pub regional_prosecutors: Vec<String>,
    pub structure: Vec<String>,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialIndependence {
    pub constitutional_guarantees: Vec<String>,
    pub judicial_council: String,
    pub appointment_procedures: String,
    pub disciplinary_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LandmarkDecision {
    pub case_number: String,
    pub date: String,
    pub court: String,
    pub subject: String,
    pub legal_principle: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeDecision {
    pub case_number: String,
    pub date: String,
    pub subject: String,
    pub legal_reasoning: String,
    pub administrative_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalDecision {
    pub case_number: String,
    pub date: String,
    pub subject: String,
    pub constitutional_articles: Vec<String>,
    pub outcome: String,
    pub dissenting_opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechLegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub civil_procedure_code: CivilProcedureCode,
    pub criminal_procedure_code: CriminalProcedureCode,
    pub administrative_procedure_code: AdministrativeProcedureCode,
    pub labor_code: LaborCode,
    pub tax_codes: Vec<TaxCode>,
    pub commercial_code: CommercialCode,
    pub family_law: FamilyLaw,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilCode {
    pub official_name: String,
    pub approval_date: String,
    pub effective_date: String,
    pub total_provisions: u32,
    pub parts: Vec<CodePart>,
    pub key_principles: Vec<String>,
    pub recodification_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CriminalCode {
    pub official_name: String,
    pub approval_date: String,
    pub last_amendment: String,
    pub total_provisions: u32,
    pub general_part: CodePart,
    pub special_part: CodePart,
    pub penalty_system: PenaltySystem,
    pub recent_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilProcedureCode {
    pub official_name: String,
    pub approval_date: String,
    pub procedural_principles: Vec<String>,
    pub types_of_proceedings: Vec<String>,
    pub appeal_system: Vec<String>,
    pub electronic_procedures: Vec<String>,
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
    pub digital_administration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaborCode {
    pub official_name: String,
    pub approval_date: String,
    pub individual_labor_relations: String,
    pub collective_labor_relations: String,
    pub labor_inspection: String,
    pub social_security_coordination: String,
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
    pub commercial_transactions: String,
    pub insolvency_regulation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilyLaw {
    pub legal_framework: String,
    pub marriage_regulation: String,
    pub registered_partnership: String,
    pub parental_authority: String,
    pub adoption_procedures: String,
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
    pub alternative_sanctions: Vec<String>,
    pub rehabilitation_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechEuropeanIntegration {
    pub eu_membership: EUMembership,
    pub eurozone_status: EurozoneStatus,
    pub schengen_area: SchengenParticipation,
    pub eu_representation: EURepresentation,
    pub eu_legislation_transposition: EULegislationTransposition,
    pub eu_structural_funds: EUStructuralFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EUMembership {
    pub accession_date: String,
    pub accession_treaty: String,
    pub referendum_date: String,
    pub referendum_result: String,
    pub membership_benefits: Vec<String>,
    pub membership_obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EurozoneStatus {
    pub current_currency: String,
    pub euro_adoption_timeline: String,
    pub convergence_criteria: Vec<String>,
    pub political_position: String,
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
pub struct EUStructuralFunds {
    pub programming_period: String,
    pub total_allocation: f64,
    pub operational_programs: Vec<String>,
    pub absorption_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechRegionalAdministration {
    pub regional_system: RegionalSystem,
    pub regional_development: RegionalDevelopment,
    pub cohesion_regions: Vec<CohesionRegion>,
    pub cross_border_cooperation: CrossBorderCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalSystem {
    pub legal_framework: String,
    pub regional_competencies: Vec<String>,
    pub financial_framework: String,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalDevelopment {
    pub strategic_documents: Vec<String>,
    pub development_priorities: Vec<String>,
    pub innovation_support: Vec<String>,
    pub infrastructure_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CohesionRegion {
    pub name: String,
    pub nuts_code: String,
    pub component_regions: Vec<String>,
    pub gdp_per_capita: f64,
    pub development_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrossBorderCooperation {
    pub neighboring_countries: Vec<String>,
    pub cooperation_programs: Vec<String>,
    pub joint_projects: Vec<String>,
    pub legal_frameworks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CzechLocalGovernment {
    pub municipal_system: MunicipalSystem,
    pub micro_regions: Vec<MicroRegion>,
    pub voluntary_associations: Vec<VoluntaryAssociation>,
    pub local_finance: LocalFinance,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MunicipalSystem {
    pub legal_framework: String,
    pub municipal_assembly: String,
    pub mayor_powers: Vec<String>,
    pub competencies: Vec<String>,
    pub types_of_municipalities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MicroRegion {
    pub name: String,
    pub member_municipalities: Vec<String>,
    pub cooperation_areas: Vec<String>,
    pub governance_structure: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoluntaryAssociation {
    pub name: String,
    pub legal_form: String,
    pub purposes: Vec<String>,
    pub membership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalFinance {
    pub revenue_sources: Vec<String>,
    pub tax_sharing_system: String,
    pub grants_and_subsidies: Vec<String>,
    pub borrowing_rules: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CitizenParticipation {
    pub participatory_mechanisms: Vec<String>,
    pub local_referendums: String,
    pub public_consultations: Vec<String>,
    pub civic_initiatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalCourtJurisprudence {
    pub major_doctrines: Vec<ConstitutionalDoctrine>,
    pub landmark_cases: Vec<LandmarkConstitutionalCase>,
    pub interpretive_methods: Vec<String>,
    pub constitutional_protection: ConstitutionalProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalDoctrine {
    pub doctrine_name: String,
    pub key_decisions: Vec<String>,
    pub legal_principle: String,
    pub impact_on_legal_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LandmarkConstitutionalCase {
    pub case_number: String,
    pub date: String,
    pub case_name: String,
    pub legal_issue: String,
    pub decision: String,
    pub constitutional_significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalProtection {
    pub fundamental_rights_protection: Vec<String>,
    pub institutional_protection: Vec<String>,
    pub procedural_guarantees: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
}

impl Default for CzechRepublicLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: CzechConstitution {
                name: "Ústava České republiky (Constitution of the Czech Republic)".to_string(),
                adoption_date: "1992-12-16".to_string(),
                last_amendment: "2021-12-21".to_string(),
                total_articles: 113,
                constitutional_acts: vec![
                    ConstitutionalAct {
                        number: "1/1993 Sb.".to_string(),
                        name: "Ústava České republiky".to_string(),
                        adoption_date: "1992-12-16".to_string(),
                        subject_matter: "Fundamental constitutional framework".to_string(),
                        key_provisions: vec!["State structure".to_string(), "Fundamental rights".to_string()],
                    },
                    ConstitutionalAct {
                        number: "2/1993 Sb.".to_string(),
                        name: "Listina základních práv a svobod".to_string(),
                        adoption_date: "1991-01-09".to_string(),
                        subject_matter: "Charter of Fundamental Rights and Freedoms".to_string(),
                        key_provisions: vec!["Human rights".to_string(), "Civil liberties".to_string()],
                    },
                ],
                fundamental_principles: vec![
                    ConstitutionalPrinciple {
                        article: "Článek 1".to_string(),
                        title: "Demokratický právní stát".to_string(),
                        content_czech: "Česká republika je svrchovaný, jednotný a demokratický právní stát založený na úctě k právům a svobodám člověka a občana.".to_string(),
                        content_english: "The Czech Republic is a sovereign, unitary and democratic legal state founded on respect for the rights and freedoms of man and citizen.".to_string(),
                        interpretation: "Establishes Czech Republic as democratic rule of law state based on human rights".to_string(),
                        constitutional_court_decisions: vec!["Pl. ÚS 19/93 - Democratic state principle".to_string()],
                    },
                    ConstitutionalPrinciple {
                        article: "Článek 2".to_string(),
                        title: "Lidová svrchovanost".to_string(),
                        content_czech: "Lid je zdrojem vší státní moci; vykonává ji prostřednictvím orgánů moci zákonodárné, výkonné a soudní.".to_string(),
                        content_english: "The people are the source of all state power; they exercise it through the organs of legislative, executive and judicial power.".to_string(),
                        interpretation: "Establishes popular sovereignty and separation of powers".to_string(),
                        constitutional_court_decisions: vec!["Pl. ÚS 36/01 - Popular sovereignty".to_string()],
                    },
                    ConstitutionalPrinciple {
                        article: "Článek 9".to_string(),
                        title: "Ústavnost a zákonnost".to_string(),
                        content_czech: "Ústava může být měněna pouze ústavním zákonem. Při vydávání zákonů nesmí být překročeny meze stanovené ústavou.".to_string(),
                        content_english: "The Constitution may be changed only by constitutional law. In adopting laws, the limits set by the Constitution may not be exceeded.".to_string(),
                        interpretation: "Establishes constitutional supremacy and limits on legislative power".to_string(),
                        constitutional_court_decisions: vec!["Pl. ÚS 27/09 - Constitutional supremacy".to_string()],
                    },
                ],
                rights_and_duties: vec![
                    ConstitutionalRight {
                        article: "Článek 1 Listiny".to_string(),
                        category: "Základní lidská práva".to_string(),
                        right_name: "Human dignity".to_string(),
                        content_czech: "Lidé jsou svobodní a rovní v důstojnosti i v právech. Základní lidská práva a svobody jsou nezadatelné, nezcizitelné, nepromlčitelné a nezrušitelné.".to_string(),
                        limitations: vec!["No limitations on fundamental human dignity".to_string()],
                        jurisprudence: vec!["Pl. ÚS 4/94 - Human dignity as foundation".to_string()],
                        european_convention_alignment: "Fully aligned with ECHR Article 3".to_string(),
                    },
                    ConstitutionalRight {
                        article: "Článek 3 Listiny".to_string(),
                        category: "Rovnost".to_string(),
                        right_name: "Equality before the law".to_string(),
                        content_czech: "Základní lidská práva a svobody se zaručují všem bez rozdílu pohlaví, rasy, barvy pleti, jazyka, víry a náboženství, politického či jiného smýšlení, národního nebo sociálního původu, příslušnosti k národnostní nebo etnické menšině, majetku, rodu a dalších postavení.".to_string(),
                        limitations: vec!["Positive discrimination measures permitted".to_string()],
                        jurisprudence: vec!["Pl. ÚS 9/07 - Anti-discrimination".to_string()],
                        european_convention_alignment: "Consistent with ECHR Article 14".to_string(),
                    },
                ],
                economic_system: EconomicConstitutionalFramework {
                    economic_system_type: "Market economy with social elements".to_string(),
                    property_rights: "Private property guaranteed with social function".to_string(),
                    market_economy_principles: vec![
                        "Free competition".to_string(),
                        "Private enterprise freedom".to_string(),
                        "Consumer protection".to_string(),
                    ],
                    state_intervention_limits: vec![
                        "Public interest justification".to_string(),
                        "Proportionality principle".to_string(),
                        "Compensation for expropriation".to_string(),
                    ],
                    public_sector_definition: "State and public institutions economic activities".to_string(),
                    privatization_framework: "Post-1989 transformation from socialist to market economy".to_string(),
                },
                key_articles: {
                    let mut articles = HashMap::new();
                    articles.insert("Article_1".to_string(), "Česká republika je svrchovaný, jednotný a demokratický právní stát".to_string());
                    articles.insert("Article_2".to_string(), "Lid je zdrojem vší státní moci".to_string());
                    articles.insert("Article_15".to_string(), "Územní samospráva vykonána obcemi a vyššími územními samosprávnými celky".to_string());
                    articles.insert("Article_83".to_string(), "Prezident republiky je hlavou státu".to_string());
                    articles
                },
                charter_of_rights: CharterOfRights {
                    official_name: "Listina základních práv a svobod".to_string(),
                    adoption_date: "1991-01-09".to_string(),
                    constitutional_status: "Constitutional Act No. 2/1993 Coll.".to_string(),
                    fundamental_rights: vec![
                        FundamentalRight {
                            article: "Article 6".to_string(),
                            right_name: "Right to life".to_string(),
                            content: "Každý má právo na život. Lidský život je hoden ochrany již před narozením.".to_string(),
                            absolute_or_relative: "Absolute - death penalty prohibited".to_string(),
                        },
                        FundamentalRight {
                            article: "Article 7".to_string(),
                            right_name: "Inviolability of person".to_string(),
                            content: "Nedotknutelnost osoby je zaručena. Omezena může být pouze v případech stanovených zákonem.".to_string(),
                            absolute_or_relative: "Relative - subject to legal limitations".to_string(),
                        },
                    ],
                    political_rights: vec![
                        PoliticalRight {
                            article: "Article 18".to_string(),
                            right_name: "Freedom of thought, conscience and religion".to_string(),
                            content: "Svoboda myšlení, svědomí a náboženského vyznání je zaručena.".to_string(),
                            exercise_conditions: vec!["Public manifestation may be regulated".to_string()],
                        },
                        PoliticalRight {
                            article: "Article 20".to_string(),
                            right_name: "Right to vote".to_string(),
                            content: "Občané mají právo podílet se na správě veřejných věcí přímo nebo svobodnou volbou svých zástupců.".to_string(),
                            exercise_conditions: vec!["Age requirement".to_string(), "Legal capacity".to_string()],
                        },
                    ],
                    economic_social_cultural_rights: vec![
                        EconomicRight {
                            article: "Article 26".to_string(),
                            right_name: "Right to work".to_string(),
                            content: "Každý má právo na svobodnou volbu povolání a přípravu k němu.".to_string(),
                            state_obligations: vec!["Create employment opportunities".to_string(), "Provide vocational training".to_string()],
                        },
                        EconomicRight {
                            article: "Article 31".to_string(),
                            right_name: "Right to health care".to_string(),
                            content: "Každý má právo na ochranu zdraví.".to_string(),
                            state_obligations: vec!["Provide public health care".to_string(), "Ensure health insurance".to_string()],
                        },
                    ],
                },
            },
            government_structure: CzechGovernment {
                president: PresidentOfRepublic {
                    name: "Petr Pavel".to_string(),
                    term_start: "2023-03-09".to_string(),
                    term_end: "2028-03-09".to_string(),
                    election_date: "2023-01-27-28".to_string(),
                    vote_percentage_second_round: 58.32,
                    previous_occupation: "General, former Chairman of NATO Military Committee".to_string(),
                    constitutional_powers: vec![
                        "Appoint Prime Minister".to_string(),
                        "Dissolve Chamber of Deputies".to_string(),
                        "Sign or veto laws".to_string(),
                        "Supreme commander of armed forces".to_string(),
                        "Appoint judges".to_string(),
                    ],
                    residence: "Prague Castle".to_string(),
                },
                prime_minister: PrimeMinister {
                    name: "Petr Fiala".to_string(),
                    party: "ODS - Občanská demokratická strana".to_string(),
                    appointment_date: "2021-12-17".to_string(),
                    previous_positions: vec![
                        "Leader of ODS (2014-present)".to_string(),
                        "Minister of Education (2012-2013)".to_string(),
                        "Rector of Masaryk University (2004-2011)".to_string(),
                        "Political science professor".to_string(),
                    ],
                    government_program: vec![
                        "Economic recovery and competitiveness".to_string(),
                        "Digital transformation".to_string(),
                        "Climate action and green transition".to_string(),
                        "Rule of law strengthening".to_string(),
                        "EU integration deepening".to_string(),
                    ],
                    coalition_partners: vec![
                        "SPOLU (ODS, KDU-ČSL, TOP 09)".to_string(),
                        "Pirates and Mayors (Piráti, STAN)".to_string(),
                    ],
                    reforms_agenda: vec![
                        "Pension system reform".to_string(),
                        "Public administration digitalization".to_string(),
                        "Tax system simplification".to_string(),
                        "Healthcare system modernization".to_string(),
                    ],
                },
                government_cabinet: GovernmentCabinet {
                    formation_date: "2021-12-17".to_string(),
                    total_ministers: 19,
                    ministries: vec![
                        Ministry {
                            name: "Ministry of Interior".to_string(),
                            minister_name: "Vít Rakušan".to_string(),
                            party: "STAN".to_string(),
                            portfolio_areas: vec!["Public administration".to_string(), "Local government".to_string(), "Civil protection".to_string()],
                            budget_2024: 180_000_000_000.0,
                            staff_count: 35_000,
                            main_headquarters: "Nad Štolou, Prague".to_string(),
                        },
                        Ministry {
                            name: "Ministry of Finance".to_string(),
                            minister_name: "Zbyněk Stanjura".to_string(),
                            party: "ODS".to_string(),
                            portfolio_areas: vec!["Public finances".to_string(), "Tax policy".to_string(), "Budget".to_string()],
                            budget_2024: 220_000_000_000.0,
                            staff_count: 28_000,
                            main_headquarters: "Letenská, Prague".to_string(),
                        },
                        Ministry {
                            name: "Ministry of Defense".to_string(),
                            minister_name: "Jana Černochová".to_string(),
                            party: "ODS".to_string(),
                            portfolio_areas: vec!["Armed forces".to_string(), "Defense policy".to_string(), "NATO cooperation".to_string()],
                            budget_2024: 155_000_000_000.0,
                            staff_count: 65_000,
                            main_headquarters: "Tychonova, Prague".to_string(),
                        },
                    ],
                    deputy_prime_ministers: vec![
                        DeputyPrimeMinister {
                            name: "Vít Rakušan".to_string(),
                            party: "STAN".to_string(),
                            portfolio: "Deputy PM and Minister of Interior".to_string(),
                            appointment_date: "2021-12-17".to_string(),
                        },
                        DeputyPrimeMinister {
                            name: "Ivan Bartoš".to_string(),
                            party: "Piráti".to_string(),
                            portfolio: "Deputy PM for Digitalization and Minister of Regional Development".to_string(),
                            appointment_date: "2021-12-17".to_string(),
                        },
                    ],
                    government_spokesperson: "Václav Smolka".to_string(),
                },
                chamber_of_deputies: ChamberOfDeputies {
                    seats_total: 200,
                    current_composition: {
                        let mut composition = HashMap::new();
                        composition.insert("ANO".to_string(), 72);
                        composition.insert("ODS".to_string(), 34);
                        composition.insert("Piráti".to_string(), 33);
                        composition.insert("SPD".to_string(), 20);
                        composition.insert("KDU-ČSL".to_string(), 12);
                        composition.insert("STAN".to_string(), 12);
                        composition.insert("TOP09".to_string(), 10);
                        composition.insert("KSČM".to_string(), 7);
                        composition
                    },
                    speaker: "Markéta Pekarová Adamová".to_string(),
                    deputy_speakers: vec![
                        "Jan Skopeček".to_string(),
                        "Věra Kovářová".to_string(),
                        "Aleš Juchelka".to_string(),
                    ],
                    parliamentary_groups: vec![
                        ParliamentaryGroup {
                            party_name: "ANO 2011".to_string(),
                            leader: "Alena Schillerová".to_string(),
                            seats: 72,
                            political_orientation: "Centrist populist".to_string(),
                            founded: "2011".to_string(),
                            european_affiliation: "Renew Europe".to_string(),
                        },
                        ParliamentaryGroup {
                            party_name: "Občanská demokratická strana".to_string(),
                            leader: "Petr Fiala".to_string(),
                            seats: 34,
                            political_orientation: "Conservative liberal".to_string(),
                            founded: "1991".to_string(),
                            european_affiliation: "European Conservatives and Reformists".to_string(),
                        },
                    ],
                    current_session: "9th Chamber of Deputies (2021-2025)".to_string(),
                    electoral_system: "Proportional representation with D'Hondt method".to_string(),
                },
                senate: Senate {
                    seats_total: 81,
                    current_composition: {
                        let mut composition = HashMap::new();
                        composition.insert("ODS".to_string(), 25);
                        composition.insert("KDU-ČSL".to_string(), 12);
                        composition.insert("STAN".to_string(), 11);
                        composition.insert("ANO".to_string(), 7);
                        composition.insert("Others".to_string(), 26);
                        composition
                    },
                    president: "Miloš Vystrčil".to_string(),
                    vice_presidents: vec!["Jiří Růžička".to_string(), "Jiří Oberfalzer".to_string()],
                    electoral_system: "Two-round majority system".to_string(),
                    term_length: "6 years, one-third renewed every 2 years".to_string(),
                },
                current_legislature: "9th legislative period (2021-2025)".to_string(),
                coalition_government: CoalitionGovernment {
                    coalition_name: "SPOLU + Pirates and Mayors".to_string(),
                    participating_parties: vec!["ODS".to_string(), "KDU-ČSL".to_string(), "TOP 09".to_string(), "Piráti".to_string(), "STAN".to_string()],
                    coalition_agreement: "Coalition agreement of December 2021".to_string(),
                    seat_distribution: {
                        let mut distribution = HashMap::new();
                        distribution.insert("ODS".to_string(), 34);
                        distribution.insert("Piráti".to_string(), 33);
                        distribution.insert("KDU-ČSL".to_string(), 12);
                        distribution.insert("STAN".to_string(), 12);
                        distribution.insert("TOP09".to_string(), 10);
                        distribution
                    },
                    key_policy_agreements: vec![
                        "Pro-European orientation".to_string(),
                        "Rule of law strengthening".to_string(),
                        "Digital transformation priority".to_string(),
                        "Green transition commitment".to_string(),
                    ],
                },
                political_parties: vec![
                    PoliticalParty {
                        name: "ANO 2011".to_string(),
                        abbreviation: "ANO".to_string(),
                        leader: "Andrej Babiš".to_string(),
                        founded: "2011-05-11".to_string(),
                        ideology: vec!["Populism".to_string(), "Centrism".to_string(), "Technocracy".to_string()],
                        membership: 25_000,
                        headquarters: "Prague".to_string(),
                        european_party_family: "Renew Europe".to_string(),
                    },
                    PoliticalParty {
                        name: "Občanská demokratická strana".to_string(),
                        abbreviation: "ODS".to_string(),
                        leader: "Petr Fiala".to_string(),
                        founded: "1991-04-20".to_string(),
                        ideology: vec!["Conservative liberalism".to_string(), "Euroscepticism".to_string(), "Economic liberalism".to_string()],
                        membership: 17_000,
                        headquarters: "Prague".to_string(),
                        european_party_family: "European Conservatives and Reformists".to_string(),
                    },
                ],
            },
            territorial_organization: CzechTerritorialOrganization {
                administrative_division: AdministrativeDivision {
                    total_area_km2: 78_867.0,
                    population: 10_701_777,
                    regions_count: 14,
                    districts_count: 76,
                    municipalities_count: 6_258,
                    with_extended_powers_count: 205,
                },
                regions: vec![
                    Region {
                        name: "Praha (Prague)".to_string(),
                        capital: "Praha".to_string(),
                        area_km2: 496.0,
                        population: 1_335_084,
                        districts_count: 1,
                        regional_governor: "Bohuslav Svoboda".to_string(),
                        regional_assembly: RegionalAssembly {
                            members_count: 65,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("SPOLU".to_string(), 25);
                                composition.insert("ANO".to_string(), 17);
                                composition.insert("Pirates".to_string(), 13);
                                composition.insert("Others".to_string(), 10);
                                composition
                            },
                            current_term: "2020-2024".to_string(),
                            president: "Petr Hlubuček".to_string(),
                        },
                        competencies: vec!["Healthcare".to_string(), "Education".to_string(), "Transport".to_string(), "Regional development".to_string()],
                        budget_2024: 85_000_000_000.0,
                    },
                    Region {
                        name: "Středočeský kraj (Central Bohemia)".to_string(),
                        capital: "Praha".to_string(),
                        area_km2: 11_015.0,
                        population: 1_448_623,
                        districts_count: 12,
                        regional_governor: "Petra Pecková".to_string(),
                        regional_assembly: RegionalAssembly {
                            members_count: 65,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("ANO".to_string(), 23);
                                composition.insert("SPOLU".to_string(), 20);
                                composition.insert("Pirates_STAN".to_string(), 15);
                                composition.insert("Others".to_string(), 7);
                                composition
                            },
                            current_term: "2020-2024".to_string(),
                            president: "Jan Skopeček".to_string(),
                        },
                        competencies: vec!["Regional healthcare".to_string(), "Secondary education".to_string(), "Regional transport".to_string()],
                        budget_2024: 42_000_000_000.0,
                    },
                ],
                districts: vec![
                    District {
                        name: "Praha".to_string(),
                        region: "Praha".to_string(),
                        area_km2: 496.0,
                        population: 1_335_084,
                        administrative_center: "Praha".to_string(),
                        municipalities_count: 1,
                    },
                    District {
                        name: "Benešov".to_string(),
                        region: "Středočeský kraj".to_string(),
                        area_km2: 1_476.0,
                        population: 95_400,
                        administrative_center: "Benešov".to_string(),
                        municipalities_count: 118,
                    },
                ],
                municipalities: vec![
                    Municipality {
                        name: "Praha".to_string(),
                        district: "Praha".to_string(),
                        area_km2: 496.0,
                        population: 1_335_084,
                        mayor: "Bohuslav Svoboda".to_string(),
                        mayor_party: "ODS".to_string(),
                        municipal_council_seats: 65,
                        municipal_status: "Statutory city and region".to_string(),
                        budget_2024: 85_000_000_000.0,
                    },
                    Municipality {
                        name: "Brno".to_string(),
                        district: "Brno-město".to_string(),
                        area_km2: 230.0,
                        population: 380_681,
                        mayor: "Markéta Vaňková".to_string(),
                        mayor_party: "ODS".to_string(),
                        municipal_council_seats: 55,
                        municipal_status: "Statutory city".to_string(),
                        budget_2024: 18_500_000_000.0,
                    },
                ],
                prague_special_status: PragueSpecialStatus {
                    constitutional_basis: "Article 99 of Constitution - special status as capital".to_string(),
                    special_competencies: vec![
                        "Combined municipal and regional powers".to_string(),
                        "Metropolitan functions".to_string(),
                        "Special state functions".to_string(),
                    ],
                    municipal_districts: 22,
                    lord_mayor: "Bohuslav Svoboda".to_string(),
                    prague_assembly: PragueAssembly {
                        members_count: 65,
                        political_composition: {
                            let mut composition = HashMap::new();
                            composition.insert("SPOLU".to_string(), 25);
                            composition.insert("ANO".to_string(), 17);
                            composition.insert("Pirates".to_string(), 13);
                            composition.insert("Others".to_string(), 10);
                            composition
                        },
                        president: "Petr Hlubuček".to_string(),
                    },
                    budget_2024: 85_000_000_000.0,
                },
                territorial_statistics: TerritorialStatistics {
                    total_area_km2: 78_867.0,
                    total_population: 10_701_777,
                    population_density: 135.7,
                    urban_population_percentage: 74.0,
                    largest_cities: vec!["Prague".to_string(), "Brno".to_string(), "Ostrava".to_string(), "Plzeň".to_string()],
                },
                territorial_reforms: vec![
                    "1999-2000 - Creation of regions and district abolition".to_string(),
                    "2003 - Transfer of competencies to regions".to_string(),
                    "Municipal consolidation processes since 1990s".to_string(),
                ],
            },
            judicial_system: CzechJudicialSystem {
                supreme_court: SupremeCourt {
                    president: "Petr Angyalossy".to_string(),
                    vice_president: "Daniela Zemanová".to_string(),
                    justices_count: 82,
                    civil_chamber: "Civil and commercial law section".to_string(),
                    criminal_chamber: "Criminal law section".to_string(),
                    competencies: vec![
                        "Final appeals in civil and commercial matters".to_string(),
                        "Criminal cassation appeals".to_string(),
                        "Extraordinary remedies".to_string(),
                        "Disciplinary proceedings against judges".to_string(),
                    ],
                    landmark_decisions: vec![
                        LandmarkDecision {
                            case_number: "21 Cdo 2388/2017".to_string(),
                            date: "2018-04-26".to_string(),
                            court: "Supreme Court".to_string(),
                            subject: "Digital rights and privacy protection".to_string(),
                            legal_principle: "Balance between digital innovation and privacy rights".to_string(),
                            impact: "Shaped digital privacy jurisprudence".to_string(),
                        },
                    ],
                    headquarters: "Burešova 20, Brno".to_string(),
                },
                supreme_administrative_court: SupremeAdministrativeCourt {
                    president: "Michal Mazanec".to_string(),
                    vice_president: "Jarmila Pokorná".to_string(),
                    justices_count: 36,
                    competencies: vec![
                        "Administrative disputes final instance".to_string(),
                        "Election disputes".to_string(),
                        "Asylum and immigration appeals".to_string(),
                        "Public procurement disputes".to_string(),
                    ],
                    recent_decisions: vec![
                        AdministrativeDecision {
                            case_number: "6 As 55/2023".to_string(),
                            date: "2023-11-15".to_string(),
                            subject: "Digital administration and GDPR compliance".to_string(),
                            legal_reasoning: "Administrative acts must comply with data protection principles".to_string(),
                            administrative_impact: "Strengthened digital rights protection in administration".to_string(),
                        },
                    ],
                    headquarters: "Moravské náměstí 6, Brno".to_string(),
                },
                constitutional_court: ConstitutionalCourt {
                    president: "Pavel Rychetský".to_string(),
                    vice_president: "Ludvík David".to_string(),
                    justices_count: 15,
                    appointment_method: "Presidential nomination with Senate consent".to_string(),
                    term_years: 10,
                    competencies: vec![
                        "Constitutional review of laws".to_string(),
                        "Constitutional complaints".to_string(),
                        "Disputes between state organs".to_string(),
                        "Political party dissolution".to_string(),
                        "Presidential impeachment".to_string(),
                    ],
                    recent_decisions: vec![
                        ConstitutionalDecision {
                            case_number: "Pl. ÚS 44/17".to_string(),
                            date: "2019-01-31".to_string(),
                            subject: "EU law vs. constitutional identity".to_string(),
                            constitutional_articles: vec!["Article 1".to_string(), "Article 2".to_string()],
                            outcome: "Limits to EU law supremacy established".to_string(),
                            dissenting_opinions: vec!["Justice David - broader EU integration support".to_string()],
                        },
                    ],
                    headquarters: "Joštova 8, Brno".to_string(),
                },
                high_courts: vec![
                    HighCourt {
                        name: "High Court in Prague".to_string(),
                        location: "Prague".to_string(),
                        jurisdiction: "Central and Western Bohemia".to_string(),
                        judges_count: 55,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string(), "Commercial".to_string()],
                    },
                    HighCourt {
                        name: "High Court in Olomouc".to_string(),
                        location: "Olomouc".to_string(),
                        jurisdiction: "Moravia and Silesia".to_string(),
                        judges_count: 42,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string()],
                    },
                ],
                regional_courts: vec![
                    RegionalCourt {
                        name: "Municipal Court in Prague".to_string(),
                        location: "Prague".to_string(),
                        jurisdiction: "Prague and Central Bohemia".to_string(),
                        judges_count: 312,
                        annual_cases: 185_640,
                    },
                    RegionalCourt {
                        name: "Regional Court in Brno".to_string(),
                        location: "Brno".to_string(),
                        jurisdiction: "South Moravian Region".to_string(),
                        judges_count: 158,
                        annual_cases: 95_230,
                    },
                ],
                district_courts: vec![
                    DistrictCourt {
                        name: "District Court Prague 1".to_string(),
                        location: "Prague".to_string(),
                        jurisdiction: "Central Prague districts".to_string(),
                        judges_count: 45,
                        annual_cases: 28_560,
                    },
                ],
                prosecution_service: ProsecutionService {
                    supreme_prosecutor: "Igor Stříž".to_string(),
                    high_prosecutors: vec!["High Prosecutor Prague".to_string(), "High Prosecutor Olomouc".to_string()],
                    regional_prosecutors: vec!["Regional prosecutors in all regions".to_string()],
                    structure: vec![
                        "Hierarchical organization".to_string(),
                        "Functional independence".to_string(),
                        "Specialized departments".to_string(),
                    ],
                    competencies: vec![
                        "Criminal prosecution".to_string(),
                        "Public interest protection".to_string(),
                        "Supervision of legal compliance".to_string(),
                    ],
                },
                judicial_independence: JudicialIndependence {
                    constitutional_guarantees: vec![
                        "Independence in decision-making".to_string(),
                        "Irremovability during term".to_string(),
                        "Adequate remuneration".to_string(),
                    ],
                    judicial_council: "No separate judicial council - self-administration".to_string(),
                    appointment_procedures: "Presidential appointment with qualification requirements".to_string(),
                    disciplinary_system: "Disciplinary proceedings through judicial chambers".to_string(),
                },
            },
            legal_codes: CzechLegalCodes {
                civil_code: CivilCode {
                    official_name: "Občanský zákoník (Civil Code)".to_string(),
                    approval_date: "2012-03-22".to_string(),
                    effective_date: "2014-01-01".to_string(),
                    total_provisions: 3_046,
                    parts: vec![
                        CodePart {
                            name: "General Part".to_string(),
                            chapters: vec!["Basic provisions".to_string(), "Natural persons".to_string(), "Legal persons".to_string()],
                            main_subjects: vec!["Legal capacity".to_string(), "Legal acts".to_string(), "Representation".to_string()],
                        },
                        CodePart {
                            name: "Family Law".to_string(),
                            chapters: vec!["Marriage".to_string(), "Parenthood".to_string(), "Guardianship".to_string()],
                            main_subjects: vec!["Marriage and divorce".to_string(), "Parental rights".to_string(), "Child protection".to_string()],
                        },
                        CodePart {
                            name: "Absolute Property Rights".to_string(),
                            chapters: vec!["Ownership".to_string(), "Co-ownership".to_string(), "Limited real rights".to_string()],
                            main_subjects: vec!["Property protection".to_string(), "Real estate transactions".to_string()],
                        },
                        CodePart {
                            name: "Relative Property Rights".to_string(),
                            chapters: vec!["Obligations".to_string(), "Contracts".to_string(), "Torts".to_string()],
                            main_subjects: vec!["Contractual obligations".to_string(), "Liability for damages".to_string()],
                        },
                    ],
                    key_principles: vec![
                        "Human dignity".to_string(),
                        "Good faith".to_string(),
                        "Prohibition of abuse of rights".to_string(),
                        "Contractual freedom".to_string(),
                    ],
                    recodification_context: "Complete recodification after 2014 replacing socialist-era code".to_string(),
                },
                criminal_code: CriminalCode {
                    official_name: "Trestní zákoník (Criminal Code)".to_string(),
                    approval_date: "2009-02-17".to_string(),
                    last_amendment: "2023-12-31".to_string(),
                    total_provisions: 418,
                    general_part: CodePart {
                        name: "General Part".to_string(),
                        chapters: vec!["Basic provisions".to_string(), "Criminal liability".to_string(), "Penalties".to_string()],
                        main_subjects: vec!["Criminal law principles".to_string(), "Sanctions system".to_string()],
                    },
                    special_part: CodePart {
                        name: "Special Part".to_string(),
                        chapters: vec!["Crimes against life and health".to_string(), "Economic crimes".to_string(), "Public order crimes".to_string()],
                        main_subjects: vec!["Specific criminal offenses".to_string(), "Punishment ranges".to_string()],
                    },
                    penalty_system: PenaltySystem {
                        prison_sentences: vec!["Up to 20 years".to_string(), "Exceptional life imprisonment".to_string(), "Death penalty abolished".to_string()],
                        fines_system: "Fine system based on daily rates".to_string(),
                        alternative_sanctions: vec!["Community service".to_string(), "House arrest".to_string(), "Prohibition of activities".to_string()],
                        rehabilitation_programs: vec!["Restorative justice".to_string(), "Drug treatment".to_string(), "Probation supervision".to_string()],
                    },
                    recent_reforms: vec![
                        "2016 - Domestic violence criminalization".to_string(),
                        "2019 - Cybercrime legislation update".to_string(),
                        "2021 - Corporate criminal liability expansion".to_string(),
                    ],
                },
                civil_procedure_code: CivilProcedureCode {
                    official_name: "Občanský soudní řád (Civil Procedure Code)".to_string(),
                    approval_date: "1963-12-04".to_string(),
                    procedural_principles: vec![
                        "Adversarial principle".to_string(),
                        "Oral proceedings".to_string(),
                        "Immediate evidence taking".to_string(),
                        "Right to legal representation".to_string(),
                    ],
                    types_of_proceedings: vec![
                        "Contentious proceedings".to_string(),
                        "Non-contentious proceedings".to_string(),
                        "Enforcement proceedings".to_string(),
                        "Insolvency proceedings".to_string(),
                    ],
                    appeal_system: vec![
                        "Appeal to regional courts".to_string(),
                        "Cassation appeal to Supreme Court".to_string(),
                        "Extraordinary remedies".to_string(),
                    ],
                    electronic_procedures: vec![
                        "Electronic filing system".to_string(),
                        "Digital case management".to_string(),
                        "Online court proceedings".to_string(),
                    ],
                },
                criminal_procedure_code: CriminalProcedureCode {
                    official_name: "Trestní řád (Criminal Procedure Code)".to_string(),
                    approval_date: "1961-11-29".to_string(),
                    investigation_phase: "Police investigation under prosecutor supervision".to_string(),
                    trial_phase: "Public oral trial with adversarial proceedings".to_string(),
                    appeal_phase: "Right to appeal with full review of facts and law".to_string(),
                    special_procedures: vec![
                        "Simplified procedures for minor offenses".to_string(),
                        "Cooperation with justice (plea bargaining)".to_string(),
                        "Mediation in criminal matters".to_string(),
                    ],
                },
                administrative_procedure_code: AdministrativeProcedureCode {
                    official_name: "Správní řád (Administrative Procedure Code)".to_string(),
                    approval_date: "2004-06-24".to_string(),
                    general_principles: vec![
                        "Legality".to_string(),
                        "Objectivity and impartiality".to_string(),
                        "Proportionality".to_string(),
                        "Cooperation with parties".to_string(),
                        "Economy and efficiency".to_string(),
                    ],
                    administrative_acts: "Administrative decisions must be reasoned and subject to review".to_string(),
                    procedural_guarantees: vec![
                        "Right to be heard".to_string(),
                        "Right to legal counsel".to_string(),
                        "Right to interpretation".to_string(),
                        "Right to appeal".to_string(),
                    ],
                    digital_administration: vec![
                        "Digital by default principle".to_string(),
                        "Electronic signatures".to_string(),
                        "Data box system".to_string(),
                    ],
                },
                labor_code: LaborCode {
                    official_name: "Zákoník práce (Labor Code)".to_string(),
                    approval_date: "2006-05-26".to_string(),
                    individual_labor_relations: "Employment contracts, working time, wages, termination".to_string(),
                    collective_labor_relations: "Trade unions, collective bargaining, strikes".to_string(),
                    labor_inspection: "State oversight of labor law compliance".to_string(),
                    social_security_coordination: "Integration with social insurance system".to_string(),
                },
                tax_codes: vec![
                    TaxCode {
                        name: "Income Tax Act".to_string(),
                        tax_type: "Personal and Corporate Income Tax".to_string(),
                        approval_date: "1992-12-20".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Personal Income Tax".to_string(),
                                rate_percentage: 15.0,
                                threshold_amount: 48_840.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                            TaxRate {
                                category: "Corporate Income Tax".to_string(),
                                rate_percentage: 19.0,
                                threshold_amount: 0.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Tax-free allowance".to_string(), "Charity donations".to_string()],
                        collection_procedures: "Monthly advance payments with annual settlement".to_string(),
                    },
                    TaxCode {
                        name: "Value Added Tax Act".to_string(),
                        tax_type: "Value Added Tax".to_string(),
                        approval_date: "2003-03-26".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Standard VAT rate".to_string(),
                                rate_percentage: 21.0,
                                threshold_amount: 0.0,
                                applicable_from: "2013-01-01".to_string(),
                            },
                            TaxRate {
                                category: "Reduced VAT rate".to_string(),
                                rate_percentage: 12.0,
                                threshold_amount: 0.0,
                                applicable_from: "2015-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Healthcare services".to_string(), "Educational services".to_string()],
                        collection_procedures: "Monthly VAT returns for registered taxpayers".to_string(),
                    },
                ],
                commercial_code: CommercialCode {
                    official_name: "Zákon o obchodních korporacích (Business Corporations Act)".to_string(),
                    approval_date: "2012-01-25".to_string(),
                    company_types: vec![
                        "s.r.o. - Limited Liability Company".to_string(),
                        "a.s. - Joint Stock Company".to_string(),
                        "v.o.s. - General Partnership".to_string(),
                        "k.s. - Limited Partnership".to_string(),
                    ],
                    commercial_transactions: "Regulated by Civil Code since 2014".to_string(),
                    insolvency_regulation: "Insolvency Act - restructuring and bankruptcy procedures".to_string(),
                },
                family_law: FamilyLaw {
                    legal_framework: "Integrated in Civil Code Part Two".to_string(),
                    marriage_regulation: "Civil marriage between man and woman".to_string(),
                    registered_partnership: "Same-sex registered partnerships since 2006".to_string(),
                    parental_authority: "Joint parental responsibility with child's best interest principle".to_string(),
                    adoption_procedures: "Full and simple adoption with court approval".to_string(),
                },
            },
            european_integration: CzechEuropeanIntegration {
                eu_membership: EUMembership {
                    accession_date: "2004-05-01".to_string(),
                    accession_treaty: "Treaty of Accession 2003".to_string(),
                    referendum_date: "2003-06-13-14".to_string(),
                    referendum_result: "77.33% in favor".to_string(),
                    membership_benefits: vec![
                        "Single market access".to_string(),
                        "Structural and cohesion funds".to_string(),
                        "Freedom of movement".to_string(),
                        "Common policies participation".to_string(),
                    ],
                    membership_obligations: vec![
                        "EU law transposition".to_string(),
                        "Budget contributions".to_string(),
                        "Policy coordination".to_string(),
                        "Solidarity mechanisms".to_string(),
                    ],
                },
                eurozone_status: EurozoneStatus {
                    current_currency: "Czech koruna (CZK)".to_string(),
                    euro_adoption_timeline: "No official timeline - political decision pending".to_string(),
                    convergence_criteria: vec![
                        "Inflation criterion met".to_string(),
                        "Fiscal criterion challenging".to_string(),
                        "Exchange rate criterion not applicable".to_string(),
                    ],
                    political_position: "Eurosceptical approach to euro adoption".to_string(),
                },
                schengen_area: SchengenParticipation {
                    implementation_date: "2007-12-21".to_string(),
                    border_controls: "Free movement within Schengen Area".to_string(),
                    police_cooperation: "Enhanced police and judicial cooperation".to_string(),
                    information_systems: vec!["SIS II".to_string(), "VIS".to_string(), "EURODAC".to_string()],
                },
                eu_representation: EURepresentation {
                    european_parliament_meps: 21,
                    council_voting_weight: 12,
                    european_commission_commissioner: "Věra Jourová (Values and Transparency, 2019-2024)".to_string(),
                    permanent_representation: "Ambassador Tomáš Boček to EU".to_string(),
                },
                eu_legislation_transposition: EULegislationTransposition {
                    transposition_rate: 98.2,
                    infringement_procedures: 12,
                    recent_transpositions: vec![
                        "Digital Services Act".to_string(),
                        "Digital Markets Act".to_string(),
                        "European Climate Law".to_string(),
                    ],
                    pending_transpositions: vec![
                        "AI Act implementation".to_string(),
                        "Data Act transposition".to_string(),
                    ],
                },
                eu_structural_funds: EUStructuralFunds {
                    programming_period: "2021-2027".to_string(),
                    total_allocation: 22_500_000_000.0,
                    operational_programs: vec![
                        "Jan Amos Komenský (human resources)".to_string(),
                        "Technology and Applications for Competitiveness".to_string(),
                        "Integrated Regional OP".to_string(),
                    ],
                    absorption_rate: 85.2,
                },
            },
            regional_administration: CzechRegionalAdministration {
                regional_system: RegionalSystem {
                    legal_framework: "Act 129/2000 Coll. on Regions".to_string(),
                    regional_competencies: vec![
                        "Secondary and higher education".to_string(),
                        "Healthcare provision".to_string(),
                        "Regional transport".to_string(),
                        "Environmental protection".to_string(),
                        "Regional development".to_string(),
                    ],
                    financial_framework: "Own revenues, shared taxes, and state transfers".to_string(),
                    coordination_mechanisms: vec![
                        "Conference of Regional Presidents".to_string(),
                        "Sectoral coordination committees".to_string(),
                        "EU funds coordination".to_string(),
                    ],
                },
                regional_development: RegionalDevelopment {
                    strategic_documents: vec![
                        "Strategic Framework Czech Republic 2030".to_string(),
                        "Regional development strategies".to_string(),
                        "Innovation strategies for smart specialization".to_string(),
                    ],
                    development_priorities: vec![
                        "Digital transformation".to_string(),
                        "Green transition".to_string(),
                        "Innovation and competitiveness".to_string(),
                        "Territorial cohesion".to_string(),
                    ],
                    innovation_support: vec![
                        "Technology parks".to_string(),
                        "Innovation centers".to_string(),
                        "Cluster development".to_string(),
                    ],
                    infrastructure_projects: vec![
                        "High-speed internet coverage".to_string(),
                        "Transport infrastructure modernization".to_string(),
                        "Green energy projects".to_string(),
                    ],
                },
                cohesion_regions: vec![
                    CohesionRegion {
                        name: "Prague".to_string(),
                        nuts_code: "CZ01".to_string(),
                        component_regions: vec!["Prague".to_string()],
                        gdp_per_capita: 54_700.0,
                        development_status: "More developed region".to_string(),
                    },
                    CohesionRegion {
                        name: "Central Bohemia".to_string(),
                        nuts_code: "CZ02".to_string(),
                        component_regions: vec!["Central Bohemian Region".to_string()],
                        gdp_per_capita: 23_100.0,
                        development_status: "Transition region".to_string(),
                    },
                    CohesionRegion {
                        name: "Southwest".to_string(),
                        nuts_code: "CZ03".to_string(),
                        component_regions: vec!["Plzeň Region".to_string(), "South Bohemian Region".to_string()],
                        gdp_per_capita: 21_800.0,
                        development_status: "Less developed region".to_string(),
                    },
                ],
                cross_border_cooperation: CrossBorderCooperation {
                    neighboring_countries: vec!["Germany".to_string(), "Austria".to_string(), "Slovakia".to_string(), "Poland".to_string()],
                    cooperation_programs: vec![
                        "INTERREG Czech Republic-Austria".to_string(),
                        "INTERREG Czech Republic-Bavaria".to_string(),
                        "INTERREG Czech Republic-Poland".to_string(),
                        "INTERREG Czech Republic-Slovakia".to_string(),
                    ],
                    joint_projects: vec![
                        "Cross-border transport connections".to_string(),
                        "Environmental protection initiatives".to_string(),
                        "Tourism development".to_string(),
                        "Innovation cooperation".to_string(),
                    ],
                    legal_frameworks: vec![
                        "European Outline Convention on Transfrontier Co-operation".to_string(),
                        "Bilateral cooperation agreements".to_string(),
                        "EU cross-border cooperation regulation".to_string(),
                    ],
                },
            },
            local_government: CzechLocalGovernment {
                municipal_system: MunicipalSystem {
                    legal_framework: "Act 128/2000 Coll. on Municipalities".to_string(),
                    municipal_assembly: "Representative body elected by direct vote for 4 years".to_string(),
                    mayor_powers: vec![
                        "Executive authority".to_string(),
                        "Municipal administration".to_string(),
                        "External representation".to_string(),
                        "Emergency powers".to_string(),
                    ],
                    competencies: vec![
                        "Basic education".to_string(),
                        "Local transport".to_string(),
                        "Waste management".to_string(),
                        "Social services".to_string(),
                        "Local development".to_string(),
                    ],
                    types_of_municipalities: vec![
                        "Municipalities (obce)".to_string(),
                        "Municipalities with extended powers".to_string(),
                        "Municipalities with authorized municipal office".to_string(),
                        "Statutory cities".to_string(),
                    ],
                },
                micro_regions: vec![
                    MicroRegion {
                        name: "Blaník Microregion".to_string(),
                        member_municipalities: vec!["Louňovice pod Blaníkem".to_string(), "Načeradec".to_string()],
                        cooperation_areas: vec!["Tourism development".to_string(), "Environmental protection".to_string()],
                        governance_structure: "Voluntary association of municipalities".to_string(),
                    },
                ],
                voluntary_associations: vec![
                    VoluntaryAssociation {
                        name: "Association of Regions".to_string(),
                        legal_form: "Interest association of legal persons".to_string(),
                        purposes: vec!["Regional interests representation".to_string(), "Coordination".to_string()],
                        membership: vec!["All 14 regions".to_string()],
                    },
                    VoluntaryAssociation {
                        name: "Union of Cities and Municipalities".to_string(),
                        legal_form: "Civic association".to_string(),
                        purposes: vec!["Municipal interests advocacy".to_string(), "Information sharing".to_string()],
                        membership: vec!["Over 2,800 municipalities".to_string()],
                    },
                ],
                local_finance: LocalFinance {
                    revenue_sources: vec![
                        "Tax revenue sharing (RSD system)".to_string(),
                        "Local taxes and fees".to_string(),
                        "Property income".to_string(),
                        "Grants and subsidies".to_string(),
                    ],
                    tax_sharing_system: "Revenue determination and allocation based on fiscal rules".to_string(),
                    grants_and_subsidies: vec!["State subsidies".to_string(), "EU funds".to_string(), "Regional grants".to_string()],
                    borrowing_rules: "Debt brake rules and Ministry of Finance approval for large debts".to_string(),
                },
                citizen_participation: CitizenParticipation {
                    participatory_mechanisms: vec![
                        "Municipal assemblies".to_string(),
                        "Advisory committees".to_string(),
                        "Participatory budgeting".to_string(),
                        "Public consultations".to_string(),
                    ],
                    local_referendums: "Local referendums binding with qualified participation".to_string(),
                    public_consultations: vec!["Strategic planning".to_string(), "Major investment decisions".to_string()],
                    civic_initiatives: vec!["Civic movements".to_string(), "Local NGOs".to_string(), "Neighborhood committees".to_string()],
                },
            },
            constitutional_court_jurisprudence: ConstitutionalCourtJurisprudence {
                major_doctrines: vec![
                    ConstitutionalDoctrine {
                        doctrine_name: "Constitutional Identity Doctrine".to_string(),
                        key_decisions: vec!["Pl. ÚS 5/12 - Lisbon Treaty".to_string(), "Pl. ÚS 44/17 - EU quotas".to_string()],
                        legal_principle: "Core constitutional values cannot be transferred to EU".to_string(),
                        impact_on_legal_system: "Limits EU law supremacy in fundamental constitutional matters".to_string(),
                    },
                    ConstitutionalDoctrine {
                        doctrine_name: "Ultra Vires Doctrine".to_string(),
                        key_decisions: vec!["Pl. ÚS 44/17 - EU quotas decision".to_string()],
                        legal_principle: "EU institutions acting beyond competencies are ultra vires".to_string(),
                        impact_on_legal_system: "Establishes constitutional review of EU law implementation".to_string(),
                    },
                ],
                landmark_cases: vec![
                    LandmarkConstitutionalCase {
                        case_number: "Pl. ÚS 50/04".to_string(),
                        date: "2006-05-08".to_string(),
                        case_name: "EU Constitution case".to_string(),
                        legal_issue: "Constitutional limits to European integration".to_string(),
                        decision: "EU Constitution does not violate Czech Constitution".to_string(),
                        constitutional_significance: "First major EU integration case defining constitutional limits".to_string(),
                    },
                    LandmarkConstitutionalCase {
                        case_number: "Pl. ÚS 44/17".to_string(),
                        date: "2017-06-26".to_string(),
                        case_name: "EU Quotas case".to_string(),
                        legal_issue: "EU migration quotas and constitutional sovereignty".to_string(),
                        decision: "EU quotas decision exceeds EU competencies".to_string(),
                        constitutional_significance: "Major assertion of constitutional identity against EU law".to_string(),
                    },
                ],
                interpretive_methods: vec![
                    "Grammatical interpretation".to_string(),
                    "Systematic interpretation".to_string(),
                    "Teleological interpretation".to_string(),
                    "Historical interpretation".to_string(),
                    "Comparative law analysis".to_string(),
                ],
                constitutional_protection: ConstitutionalProtection {
                    fundamental_rights_protection: vec![
                        "Constitutional complaints procedure".to_string(),
                        "Abstract constitutional review".to_string(),
                        "Constitutional rights balancing".to_string(),
                    ],
                    institutional_protection: vec![
                        "Separation of powers enforcement".to_string(),
                        "Federal structure protection".to_string(),
                        "Democratic principles safeguarding".to_string(),
                    ],
                    procedural_guarantees: vec![
                        "Fair trial guarantees".to_string(),
                        "Due process requirements".to_string(),
                        "Access to justice".to_string(),
                    ],
                    enforcement_mechanisms: vec![
                        "Binding constitutional decisions".to_string(),
                        "Enforcement through ordinary courts".to_string(),
                        "Political enforcement through constitutional organs".to_string(),
                    ],
                },
            },
        }
    }
}