use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete Romanian Legal System Implementation
/// România (Romania)
/// Current President: Klaus Iohannis (2014-2024) - Final term
/// Current Prime Minister: Marcel Ciolacu (PSD - Social Democratic Party)
/// Current Government: Ciolacu Cabinet (PSD-PNL grand coalition)
/// EU Member since 2007, Not in Eurozone (Romanian leu), Schengen candidate

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomaniaLegalSystem {
    pub constitutional_framework: RomanianConstitution,
    pub government_structure: RomanianGovernment,
    pub territorial_organization: RomanianTerritorialOrganization,
    pub judicial_system: RomanianJudicialSystem,
    pub legal_codes: RomanianLegalCodes,
    pub european_integration: RomanianEuropeanIntegration,
    pub regional_administration: RomanianRegionalAdministration,
    pub local_government: RomanianLocalGovernment,
    pub cvm_mechanism: CVMMechanism,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomanianConstitution {
    pub name: String,
    pub adoption_date: String,
    pub last_revision: String,
    pub total_articles: u32,
    pub fundamental_principles: Vec<ConstitutionalPrinciple>,
    pub rights_and_duties: Vec<ConstitutionalRight>,
    pub state_organization: Vec<StateOrganizationArticle>,
    pub key_articles: HashMap<String, String>,
    pub constitutional_court_role: ConstitutionalCourtRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalPrinciple {
    pub article: String,
    pub title: String,
    pub content_romanian: String,
    pub content_english: String,
    pub interpretation: String,
    pub constitutional_court_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalRight {
    pub article: String,
    pub category: String,
    pub right_name: String,
    pub content_romanian: String,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
    pub european_standards_alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StateOrganizationArticle {
    pub article: String,
    pub institution: String,
    pub content_romanian: String,
    pub competencies: Vec<String>,
    pub relations_with_other_organs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalCourtRole {
    pub constitutional_basis: String,
    pub competencies: Vec<String>,
    pub composition: String,
    pub appointment_procedure: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomanianGovernment {
    pub president: PresidentOfRepublic,
    pub prime_minister: PrimeMinister,
    pub government_cabinet: GovernmentCabinet,
    pub parliament: Parliament,
    pub current_legislature: String,
    pub political_parties: Vec<PoliticalParty>,
    pub coalition_government: CoalitionGovernment,
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
pub struct Parliament {
    pub chamber_of_deputies: ChamberOfDeputies,
    pub senate: Senate,
    pub bicameral_system: BicameralSystem,
    pub current_legislature: String,
    pub electoral_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChamberOfDeputies {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub speaker: String,
    pub deputy_speakers: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Senate {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub president: String,
    pub vice_presidents: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BicameralSystem {
    pub legislative_procedure: String,
    pub competencies_distribution: Vec<String>,
    pub joint_sessions: Vec<String>,
    pub conflict_resolution: String,
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
pub struct RomanianTerritorialOrganization {
    pub administrative_division: AdministrativeDivision,
    pub development_regions: Vec<DevelopmentRegion>,
    pub counties: Vec<County>,
    pub municipalities: Vec<Municipality>,
    pub communes: Vec<Commune>,
    pub bucharest_special_status: BucharestSpecialStatus,
    pub territorial_statistics: TerritorialStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeDivision {
    pub total_area_km2: f64,
    pub population: u64,
    pub development_regions_count: u32,
    pub counties_count: u32,
    pub municipalities_count: u32,
    pub communes_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevelopmentRegion {
    pub name: String,
    pub code: String,
    pub counties: Vec<String>,
    pub area_km2: f64,
    pub population: u64,
    pub gdp_per_capita: f64,
    pub development_agency: RegionalDevelopmentAgency,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalDevelopmentAgency {
    pub name: String,
    pub headquarters: String,
    pub competencies: Vec<String>,
    pub eu_funds_management: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct County {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub municipalities_count: u32,
    pub communes_count: u32,
    pub county_council: CountyCouncil,
    pub prefect: String,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountyCouncil {
    pub president: String,
    pub members_count: u32,
    pub political_composition: HashMap<String, u32>,
    pub current_term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Municipality {
    pub name: String,
    pub county: String,
    pub area_km2: f64,
    pub population: u64,
    pub mayor: String,
    pub mayor_party: String,
    pub local_council_seats: u32,
    pub municipal_status: String,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Commune {
    pub name: String,
    pub county: String,
    pub villages: Vec<String>,
    pub area_km2: f64,
    pub population: u64,
    pub mayor: String,
    pub local_council_seats: u32,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucharestSpecialStatus {
    pub constitutional_basis: String,
    pub special_competencies: Vec<String>,
    pub sectors_count: u32,
    pub general_mayor: String,
    pub general_council: BucharestGeneralCouncil,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucharestGeneralCouncil {
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
pub struct RomanianJudicialSystem {
    pub high_court_of_cassation_and_justice: HighCourtOfCassationAndJustice,
    pub constitutional_court: ConstitutionalCourt,
    pub courts_of_appeal: Vec<CourtOfAppeal>,
    pub county_courts: Vec<CountyCourt>,
    pub local_courts: Vec<LocalCourt>,
    pub prosecution_service: ProsecutionService,
    pub superior_council_of_magistracy: SuperiorCouncilOfMagistracy,
    pub judicial_inspection: JudicialInspection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighCourtOfCassationAndJustice {
    pub president: String,
    pub vice_president: String,
    pub judges_count: u32,
    pub sections: Vec<String>,
    pub competencies: Vec<String>,
    pub landmark_decisions: Vec<LandmarkDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalCourt {
    pub president: String,
    pub vice_president: String,
    pub judges_count: u32,
    pub appointment_method: String,
    pub term_years: u32,
    pub competencies: Vec<String>,
    pub recent_decisions: Vec<ConstitutionalDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourtOfAppeal {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub specializations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountyCourt {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub annual_cases: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalCourt {
    pub name: String,
    pub location: String,
    pub jurisdiction: String,
    pub judges_count: u32,
    pub annual_cases: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProsecutionService {
    pub prosecutor_general: String,
    pub first_deputy_prosecutor_general: String,
    pub dna: NationalAnticorruptionDirectorate,
    pub diicot: DirectorateForInvestigatingOrganizedCrime,
    pub structure: Vec<String>,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NationalAnticorruptionDirectorate {
    pub chief_prosecutor: String,
    pub competencies: Vec<String>,
    pub major_cases: Vec<String>,
    pub european_cooperation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectorateForInvestigatingOrganizedCrime {
    pub chief_prosecutor: String,
    pub competencies: Vec<String>,
    pub specialized_units: Vec<String>,
    pub international_cooperation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuperiorCouncilOfMagistracy {
    pub president: String,
    pub composition: String,
    pub competencies: Vec<String>,
    pub judicial_independence_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialInspection {
    pub general_inspector: String,
    pub competencies: Vec<String>,
    pub disciplinary_procedures: String,
    pub quality_assurance: String,
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
pub struct ConstitutionalDecision {
    pub decision_number: String,
    pub date: String,
    pub subject: String,
    pub constitutional_articles: Vec<String>,
    pub outcome: String,
    pub dissenting_opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomanianLegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub civil_procedure_code: CivilProcedureCode,
    pub criminal_procedure_code: CriminalProcedureCode,
    pub administrative_code: AdministrativeCode,
    pub labor_code: LaborCode,
    pub tax_codes: Vec<TaxCode>,
    pub family_code: FamilyCode,
    pub commercial_law: CommercialLaw,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilCode {
    pub official_name: String,
    pub approval_date: String,
    pub effective_date: String,
    pub total_provisions: u32,
    pub books: Vec<CodeBook>,
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
pub struct AdministrativeCode {
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
    pub social_dialogue: String,
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
pub struct FamilyCode {
    pub legal_framework: String,
    pub marriage_regulation: String,
    pub civil_partnership: String,
    pub parental_authority: String,
    pub adoption_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommercialLaw {
    pub main_legislation: String,
    pub company_types: Vec<String>,
    pub commercial_registration: String,
    pub insolvency_law: String,
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
    pub alternative_sanctions: Vec<String>,
    pub rehabilitation_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomanianEuropeanIntegration {
    pub eu_membership: EUMembership,
    pub eurozone_status: EurozoneStatus,
    pub schengen_candidacy: SchengenCandidacy,
    pub eu_representation: EURepresentation,
    pub eu_legislation_transposition: EULegislationTransposition,
    pub cvm_mechanism: CVMStatus,
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
    pub political_commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchengenCandidacy {
    pub candidacy_status: String,
    pub evaluation_process: String,
    pub outstanding_conditions: Vec<String>,
    pub expected_timeline: String,
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
pub struct CVMStatus {
    pub establishment_date: String,
    pub current_status: String,
    pub benchmarks: Vec<String>,
    pub progress_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomanianRegionalAdministration {
    pub development_regions: DevelopmentRegionsSystem,
    pub regional_development: RegionalDevelopment,
    pub nuts_classification: NutsClassification,
    pub cross_border_cooperation: CrossBorderCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevelopmentRegionsSystem {
    pub legal_framework: String,
    pub coordination_role: String,
    pub eu_funds_management: String,
    pub statistical_purposes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalDevelopment {
    pub strategic_documents: Vec<String>,
    pub development_priorities: Vec<String>,
    pub innovation_support: Vec<String>,
    pub infrastructure_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NutsClassification {
    pub nuts_2_regions: Vec<Nuts2Region>,
    pub nuts_3_counties: Vec<String>,
    pub statistical_coordination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Nuts2Region {
    pub name: String,
    pub nuts_code: String,
    pub counties: Vec<String>,
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
pub struct RomanianLocalGovernment {
    pub municipal_system: MunicipalSystem,
    pub commune_system: CommuneSystem,
    pub inter_community_associations: Vec<InterCommunityAssociation>,
    pub local_finance: LocalFinance,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MunicipalSystem {
    pub legal_framework: String,
    pub local_council: String,
    pub mayor_powers: Vec<String>,
    pub competencies: Vec<String>,
    pub types_of_municipalities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommuneSystem {
    pub legal_framework: String,
    pub commune_council: String,
    pub mayor_competencies: Vec<String>,
    pub village_organization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterCommunityAssociation {
    pub name: String,
    pub legal_form: String,
    pub member_localities: Vec<String>,
    pub cooperation_areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalFinance {
    pub revenue_sources: Vec<String>,
    pub fiscal_equalization: String,
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
pub struct CVMMechanism {
    pub establishment_context: String,
    pub benchmarks: Vec<CVMBenchmark>,
    pub progress_reports: Vec<ProgressReport>,
    pub current_status: String,
    pub termination_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CVMBenchmark {
    pub benchmark_number: u32,
    pub description: String,
    pub focus_area: String,
    pub progress_status: String,
    pub remaining_challenges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgressReport {
    pub report_date: String,
    pub overall_assessment: String,
    pub key_developments: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for RomaniaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: RomanianConstitution {
                name: "Constituția României (Constitution of Romania)".to_string(),
                adoption_date: "1991-11-21".to_string(),
                last_revision: "2003-10-29".to_string(),
                total_articles: 156,
                fundamental_principles: vec![
                    ConstitutionalPrinciple {
                        article: "Articolul 1".to_string(),
                        title: "Statul român".to_string(),
                        content_romanian: "România este stat național, suveran și independent, unitar și indivizibil. Forma de guvernământ a României este republica. România este stat de drept, democratic și social, în care demnitatea omului, drepturile și libertățile cetățenilor, libera dezvoltare a personalității umane, dreptatea și pluralismul politic reprezintă valori supreme, în spiritul tradițiilor democratice ale poporului român și idealurilor Revoluției din decembrie 1989, și sunt garantate.".to_string(),
                        content_english: "Romania is a sovereign, independent, unitary and indivisible national state. Romania's form of government is the republic. Romania is a rule of law, democratic and social state, in which human dignity, citizens' rights and freedoms, free development of human personality, justice and political pluralism represent supreme values, in the spirit of the democratic traditions of the Romanian people and the ideals of the December 1989 Revolution, and are guaranteed.".to_string(),
                        interpretation: "Establishes Romania as unitary democratic state based on rule of law and human dignity".to_string(),
                        constitutional_court_decisions: vec!["Decision 148/2003 on national state character".to_string()],
                    },
                    ConstitutionalPrinciple {
                        article: "Articolul 2".to_string(),
                        title: "Suveranitatea".to_string(),
                        content_romanian: "Suveranitatea națională aparține poporului român, care o exercită prin organele sale reprezentative, constituite prin alegeri libere, periodice și corecte, precum și prin referendum. Nici un grup și nici o persoană nu pot exercita suveranitatea în nume propriu.".to_string(),
                        content_english: "National sovereignty belongs to the Romanian people, who exercise it through their representative organs, constituted through free, periodic and fair elections, as well as through referendum. No group and no person may exercise sovereignty in their own name.".to_string(),
                        interpretation: "Establishes popular sovereignty exercised through democratic elections and referendums".to_string(),
                        constitutional_court_decisions: vec!["Decision 51/1995 on popular sovereignty".to_string()],
                    },
                    ConstitutionalPrinciple {
                        article: "Articolul 16".to_string(),
                        title: "Egalitatea în drepturi".to_string(),
                        content_romanian: "Cetățenii sunt egali în fața legii și a autorităților publice, fără privilegii și fără discriminări. Nimeni nu este mai presus de lege. Funcțiile și demnităților publice, civile sau militare, pot fi ocupate, în condițiile legii, de persoanele care au cetățenia română și domiciliul în țară. Statul român garantează egalitatea de șanse între femei și bărbați pentru ocuparea funcțiilor și demnităților publice, civile și militare.".to_string(),
                        content_english: "Citizens are equal before the law and public authorities, without privileges and without discrimination. No one is above the law. Public, civil or military functions and dignities may be occupied, under the conditions of the law, by persons who have Romanian citizenship and domicile in the country. The Romanian state guarantees equality of opportunities between women and men for occupying public, civil and military functions and dignities.".to_string(),
                        interpretation: "Establishes equality before law and equal opportunities principle".to_string(),
                        constitutional_court_decisions: vec!["Decision 62/2007 on gender equality".to_string()],
                    },
                ],
                rights_and_duties: vec![
                    ConstitutionalRight {
                        article: "Articolul 22".to_string(),
                        category: "Dreptul la viață și la integritate fizică și psihică".to_string(),
                        right_name: "Right to life and physical and mental integrity".to_string(),
                        content_romanian: "Dreptul la viață, precum și dreptul la integritate fizică și psihică ale persoanei sunt garantate. Nimeni nu poate fi supus torturii și nici unui fel de pedepse sau de tratamente inumane ori degradante. Pedeapsa cu moartea este interzisă.".to_string(),
                        limitations: vec!["No limitations - absolute prohibition of death penalty and torture".to_string()],
                        jurisprudence: vec!["Decision 144/1999 on right to life protection".to_string()],
                        european_standards_alignment: "Fully aligned with ECHR Articles 2 and 3".to_string(),
                    },
                    ConstitutionalRight {
                        article: "Articolul 23".to_string(),
                        category: "Libertatea individuală".to_string(),
                        right_name: "Individual liberty".to_string(),
                        content_romanian: "Libertatea individuală și siguranța persoanei sunt inviolabile. Percheziționarea, reținerea sau arestarea unei persoane sunt permise numai în cazurile și cu procedura prevăzute de lege. Reținerea nu poate depăși 24 de ore. Arestarea preventivă se dispune de instanța judecătorească și numai în cursul procesului penal.".to_string(),
                        limitations: vec!["Criminal procedure limitations".to_string(), "Judicial authorization required".to_string()],
                        jurisprudence: vec!["Decision 69/2005 on preventive arrest conditions".to_string()],
                        european_standards_alignment: "Consistent with ECHR Article 5".to_string(),
                    },
                ],
                state_organization: vec![
                    StateOrganizationArticle {
                        article: "Articolul 80".to_string(),
                        institution: "Președintele României".to_string(),
                        content_romanian: "Președintele României reprezintă statul român și este garantul independenței naționale, al unității și integrității teritoriale a țării. Președintele României veghează la respectarea Constituției și la buna funcționare a autorităților publice. În acest scop, Președintele exercită funcția de mediere între puterile statului, precum și între stat și societate.".to_string(),
                        competencies: vec![
                            "State representation".to_string(),
                            "National independence guarantor".to_string(),
                            "Constitutional guardian".to_string(),
                            "Mediation between state powers".to_string(),
                        ],
                        relations_with_other_organs: vec!["Appoints Prime Minister".to_string(), "Dissolves Parliament".to_string()],
                    },
                    StateOrganizationArticle {
                        article: "Articolul 102".to_string(),
                        institution: "Guvernul".to_string(),
                        content_romanian: "Guvernul, în conformitate cu programul de guvernare acceptat de Parlament, asigură realizarea politicii interne și externe a țării și exercită conducerea generală a administrației publice. În îndeplinirea atribuțiilor sale, Guvernul se conduce după programul de guvernare și după legile țării.".to_string(),
                        competencies: vec![
                            "Internal and external policy implementation".to_string(),
                            "General public administration leadership".to_string(),
                            "Government program execution".to_string(),
                        ],
                        relations_with_other_organs: vec!["Accountable to Parliament".to_string(), "Appointed by President".to_string()],
                    },
                ],
                key_articles: {
                    let mut articles = HashMap::new();
                    articles.insert("Article_1".to_string(), "România este stat național, suveran și independent, unitar și indivizibil".to_string());
                    articles.insert("Article_2".to_string(), "Suveranitatea națională aparține poporului român".to_string());
                    articles.insert("Article_148".to_string(), "Integrarea în Uniunea Europeană".to_string());
                    articles.insert("Article_11".to_string(), "Dreptul internațional și dreptul intern".to_string());
                    articles
                },
                constitutional_court_role: ConstitutionalCourtRole {
                    constitutional_basis: "Title IV - Constitutional Court (Articles 142-147)".to_string(),
                    competencies: vec![
                        "Constitutional review of laws".to_string(),
                        "Constitutional conflicts resolution".to_string(),
                        "Constitutional exceptions".to_string(),
                        "Presidential impeachment".to_string(),
                    ],
                    composition: "9 judges appointed for 9-year non-renewable terms".to_string(),
                    appointment_procedure: "3 by Chamber of Deputies, 3 by Senate, 3 by President".to_string(),
                },
            },
            government_structure: RomanianGovernment {
                president: PresidentOfRepublic {
                    name: "Klaus Werner Iohannis".to_string(),
                    term_start: "2014-12-21".to_string(),
                    term_end: "2024-12-21".to_string(),
                    election_date: "2019-11-24".to_string(),
                    vote_percentage_second_round: 66.09,
                    previous_occupation: "Mayor of Sibiu, Physics teacher".to_string(),
                    constitutional_powers: vec![
                        "Designate Prime Minister candidate".to_string(),
                        "Dissolve Parliament".to_string(),
                        "Promulgate or return laws".to_string(),
                        "Supreme commander of armed forces".to_string(),
                        "International treaties negotiation".to_string(),
                    ],
                    residence: "Cotroceni Palace, Bucharest".to_string(),
                },
                prime_minister: PrimeMinister {
                    name: "Marcel Ciolacu".to_string(),
                    party: "PSD - Partidul Social Democrat".to_string(),
                    appointment_date: "2023-06-15".to_string(),
                    previous_positions: vec![
                        "President of Chamber of Deputies (2019-2023)".to_string(),
                        "PSD President (2020-present)".to_string(),
                        "Deputy in Parliament (2004-present)".to_string(),
                        "County Council President Buzău (2008-2012)".to_string(),
                    ],
                    government_program: vec![
                        "Economic recovery and social cohesion".to_string(),
                        "Infrastructure development".to_string(),
                        "Healthcare system strengthening".to_string(),
                        "Education modernization".to_string(),
                        "European integration acceleration".to_string(),
                    ],
                    coalition_partners: vec!["PNL - Partidul Național Liberal".to_string(), "UDMR - Uniunea Democrată Maghiară din România".to_string()],
                    reforms_agenda: vec![
                        "Pension system reform".to_string(),
                        "Healthcare system reform".to_string(),
                        "Public administration modernization".to_string(),
                        "Justice system strengthening".to_string(),
                    ],
                },
                government_cabinet: GovernmentCabinet {
                    formation_date: "2023-06-15".to_string(),
                    total_ministers: 20,
                    ministries: vec![
                        Ministry {
                            name: "Ministry of Internal Affairs".to_string(),
                            minister_name: "Cătălin Predoiu".to_string(),
                            party: "PNL".to_string(),
                            portfolio_areas: vec!["Public order".to_string(), "Emergency situations".to_string(), "Public administration".to_string()],
                            budget_2024: 12_500_000_000.0,
                            staff_count: 125_000,
                            main_headquarters: "Piața Revoluției 1A, Bucharest".to_string(),
                        },
                        Ministry {
                            name: "Ministry of Public Finance".to_string(),
                            minister_name: "Marcel Boloș".to_string(),
                            party: "PNL".to_string(),
                            portfolio_areas: vec!["Public finances".to_string(), "Tax policy".to_string(), "Budget management".to_string()],
                            budget_2024: 15_800_000_000.0,
                            staff_count: 45_000,
                            main_headquarters: "Strada Apolodor 17, Bucharest".to_string(),
                        },
                        Ministry {
                            name: "Ministry of National Defense".to_string(),
                            minister_name: "Angel Tîlvăr".to_string(),
                            party: "PSD".to_string(),
                            portfolio_areas: vec!["Armed forces".to_string(), "Defense policy".to_string(), "NATO commitments".to_string()],
                            budget_2024: 18_200_000_000.0,
                            staff_count: 85_000,
                            main_headquarters: "Strada Izvor 7-9, Bucharest".to_string(),
                        },
                    ],
                    deputy_prime_ministers: vec![
                        DeputyPrimeMinister {
                            name: "Hunor Kelemen".to_string(),
                            party: "UDMR".to_string(),
                            portfolio: "Deputy PM and Minister of Development".to_string(),
                            appointment_date: "2023-06-15".to_string(),
                        },
                    ],
                    government_spokesperson: "Mihai Constantin".to_string(),
                },
                parliament: Parliament {
                    chamber_of_deputies: ChamberOfDeputies {
                        seats_total: 330,
                        current_composition: {
                            let mut composition = HashMap::new();
                            composition.insert("PSD".to_string(), 110);
                            composition.insert("PNL".to_string(), 93);
                            composition.insert("AUR".to_string(), 47);
                            composition.insert("USR".to_string(), 55);
                            composition.insert("UDMR".to_string(), 21);
                            composition.insert("Others".to_string(), 4);
                            composition
                        },
                        speaker: "Alfred Simonis".to_string(),
                        deputy_speakers: vec![
                            "Gheorghe Pecingină".to_string(),
                            "Varujan Vosganian".to_string(),
                            "Călin Potor".to_string(),
                        ],
                        parliamentary_groups: vec![
                            ParliamentaryGroup {
                                party_name: "Partidul Social Democrat".to_string(),
                                leader: "Alfred Simonis".to_string(),
                                seats: 110,
                                political_orientation: "Social democratic".to_string(),
                                founded: "1993".to_string(),
                                european_affiliation: "Party of European Socialists".to_string(),
                            },
                        ],
                    },
                    senate: Senate {
                        seats_total: 136,
                        current_composition: {
                            let mut composition = HashMap::new();
                            composition.insert("PSD".to_string(), 45);
                            composition.insert("PNL".to_string(), 42);
                            composition.insert("AUR".to_string(), 19);
                            composition.insert("USR".to_string(), 21);
                            composition.insert("UDMR".to_string(), 9);
                            composition
                        },
                        president: "Nicolae Ciucă".to_string(),
                        vice_presidents: vec!["Alina Gorghiu".to_string(), "Robert Cazanciuc".to_string()],
                        parliamentary_groups: vec![
                            ParliamentaryGroup {
                                party_name: "Partidul Național Liberal".to_string(),
                                leader: "Daniel Fenechiu".to_string(),
                                seats: 42,
                                political_orientation: "Liberal conservative".to_string(),
                                founded: "1875".to_string(),
                                european_affiliation: "European People's Party".to_string(),
                            },
                        ],
                    },
                    bicameral_system: BicameralSystem {
                        legislative_procedure: "Perfect bicameralism with equal powers".to_string(),
                        competencies_distribution: vec!["Both chambers have equal legislative powers".to_string()],
                        joint_sessions: vec!["Presidential oath".to_string(), "Presidential addresses".to_string()],
                        conflict_resolution: "Mediation committee for conflicting versions".to_string(),
                    },
                    current_legislature: "2020-2024 Legislature".to_string(),
                    electoral_system: "Proportional representation with closed lists".to_string(),
                },
                current_legislature: "Ciolacu Cabinet (PSD-PNL-UDMR coalition)".to_string(),
                coalition_government: CoalitionGovernment {
                    coalition_name: "PSD-PNL-UDMR Coalition".to_string(),
                    participating_parties: vec!["PSD".to_string(), "PNL".to_string(), "UDMR".to_string()],
                    coalition_agreement: "Coalition agreement of June 2023".to_string(),
                    seat_distribution: {
                        let mut distribution = HashMap::new();
                        distribution.insert("PSD".to_string(), 155);
                        distribution.insert("PNL".to_string(), 135);
                        distribution.insert("UDMR".to_string(), 30);
                        distribution
                    },
                    key_policy_agreements: vec![
                        "Pro-European orientation".to_string(),
                        "NATO commitment".to_string(),
                        "Economic recovery focus".to_string(),
                        "Social cohesion priority".to_string(),
                    ],
                },
                political_parties: vec![
                    PoliticalParty {
                        name: "Partidul Social Democrat".to_string(),
                        abbreviation: "PSD".to_string(),
                        leader: "Marcel Ciolacu".to_string(),
                        founded: "1993-07-10".to_string(),
                        ideology: vec!["Social democracy".to_string(), "Pro-European".to_string()],
                        membership: 500_000,
                        headquarters: "Kiseleff 10, Bucharest".to_string(),
                        european_party_family: "Party of European Socialists".to_string(),
                    },
                    PoliticalParty {
                        name: "Partidul Național Liberal".to_string(),
                        abbreviation: "PNL".to_string(),
                        leader: "Nicolae Ciucă".to_string(),
                        founded: "1875".to_string(),
                        ideology: vec!["Liberal conservatism".to_string(), "Pro-European".to_string()],
                        membership: 250_000,
                        headquarters: "Aleea Modrogan 1, Bucharest".to_string(),
                        european_party_family: "European People's Party".to_string(),
                    },
                ],
            },
            territorial_organization: RomanianTerritorialOrganization {
                administrative_division: AdministrativeDivision {
                    total_area_km2: 238_391.0,
                    population: 19_053_815,
                    development_regions_count: 8,
                    counties_count: 41,
                    municipalities_count: 320,
                    communes_count: 2_861,
                },
                development_regions: vec![
                    DevelopmentRegion {
                        name: "București - Ilfov".to_string(),
                        code: "RO32".to_string(),
                        counties: vec!["Bucharest".to_string(), "Ilfov".to_string()],
                        area_km2: 1_811.0,
                        population: 2_272_163,
                        gdp_per_capita: 34_500.0,
                        development_agency: RegionalDevelopmentAgency {
                            name: "Bucharest-Ilfov Regional Development Agency".to_string(),
                            headquarters: "Bucharest".to_string(),
                            competencies: vec!["EU funds management".to_string(), "Regional development planning".to_string()],
                            eu_funds_management: "Managing ERDF and ESF programs".to_string(),
                        },
                    },
                    DevelopmentRegion {
                        name: "Sud - Muntenia".to_string(),
                        code: "RO31".to_string(),
                        counties: vec!["Argeș".to_string(), "Călărași".to_string(), "Dâmbovița".to_string(), "Giurgiu".to_string(), "Ialomița".to_string(), "Prahova".to_string(), "Teleorman".to_string()],
                        area_km2: 34_489.0,
                        population: 3_136_446,
                        gdp_per_capita: 18_200.0,
                        development_agency: RegionalDevelopmentAgency {
                            name: "Sud-Muntenia Regional Development Agency".to_string(),
                            headquarters: "Călărași".to_string(),
                            competencies: vec!["Regional development coordination".to_string(), "EU programs implementation".to_string()],
                            eu_funds_management: "Regional Operational Program implementation".to_string(),
                        },
                    },
                ],
                counties: vec![
                    County {
                        name: "Alba".to_string(),
                        capital: "Alba Iulia".to_string(),
                        area_km2: 6_242.0,
                        population: 342_376,
                        municipalities_count: 4,
                        communes_count: 67,
                        county_council: CountyCouncil {
                            president: "Ion Dumitrel".to_string(),
                            members_count: 33,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("PNL".to_string(), 15);
                                composition.insert("PSD".to_string(), 12);
                                composition.insert("USR".to_string(), 4);
                                composition.insert("Others".to_string(), 2);
                                composition
                            },
                            current_term: "2020-2024".to_string(),
                        },
                        prefect: "Alexandru Moldovan".to_string(),
                        budget_2024: 850_000_000.0,
                    },
                    County {
                        name: "Ilfov".to_string(),
                        capital: "Bucharest (administrative)".to_string(),
                        area_km2: 1_583.0,
                        population: 542_686,
                        municipalities_count: 8,
                        communes_count: 32,
                        county_council: CountyCouncil {
                            president: "Hubert Thuma".to_string(),
                            members_count: 33,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("PNL".to_string(), 18);
                                composition.insert("PSD".to_string(), 10);
                                composition.insert("USR".to_string(), 3);
                                composition.insert("Others".to_string(), 2);
                                composition
                            },
                            current_term: "2020-2024".to_string(),
                        },
                        prefect: "Alin Cristian Stoica".to_string(),
                        budget_2024: 1_200_000_000.0,
                    },
                ],
                municipalities: vec![
                    Municipality {
                        name: "Bucharest".to_string(),
                        county: "Bucharest Municipality".to_string(),
                        area_km2: 228.0,
                        population: 1_716_983,
                        mayor: "Nicușor Dan".to_string(),
                        mayor_party: "Independent (USR support)".to_string(),
                        local_council_seats: 55,
                        municipal_status: "Capital and municipality".to_string(),
                        budget_2024: 8_500_000_000.0,
                    },
                    Municipality {
                        name: "Cluj-Napoca".to_string(),
                        county: "Cluj".to_string(),
                        area_km2: 179.5,
                        population: 286_598,
                        mayor: "Emil Boc".to_string(),
                        mayor_party: "PNL".to_string(),
                        local_council_seats: 27,
                        municipal_status: "Municipality".to_string(),
                        budget_2024: 1_850_000_000.0,
                    },
                ],
                communes: vec![
                    Commune {
                        name: "Voluntari".to_string(),
                        county: "Ilfov".to_string(),
                        villages: vec!["Voluntari".to_string(), "Pipera".to_string()],
                        area_km2: 46.1,
                        population: 42_673,
                        mayor: "Florentin Pandele".to_string(),
                        local_council_seats: 19,
                        budget_2024: 180_000_000.0,
                    },
                ],
                bucharest_special_status: BucharestSpecialStatus {
                    constitutional_basis: "Article 122 - Special administrative status as capital".to_string(),
                    special_competencies: vec![
                        "Capital city functions".to_string(),
                        "Metropolitan area coordination".to_string(),
                        "Special state representation functions".to_string(),
                    ],
                    sectors_count: 6,
                    general_mayor: "Nicușor Dan".to_string(),
                    bucharest_general_council: BucharestGeneralCouncil {
                        members_count: 55,
                        political_composition: {
                            let mut composition = HashMap::new();
                            composition.insert("USR".to_string(), 22);
                            composition.insert("PSD".to_string(), 17);
                            composition.insert("PNL".to_string(), 12);
                            composition.insert("Others".to_string(), 4);
                            composition
                        },
                        president: "Ana Ciceală".to_string(),
                    },
                    budget_2024: 8_500_000_000.0,
                },
                territorial_statistics: TerritorialStatistics {
                    total_area_km2: 238_391.0,
                    total_population: 19_053_815,
                    population_density: 79.9,
                    urban_population_percentage: 56.4,
                    largest_cities: vec!["Bucharest".to_string(), "Cluj-Napoca".to_string(), "Timișoara".to_string(), "Iași".to_string()],
                },
            },
            judicial_system: RomanianJudicialSystem {
                high_court_of_cassation_and_justice: HighCourtOfCassationAndJustice {
                    president: "Corina Corbu".to_string(),
                    vice_president: "Simona Marcu".to_string(),
                    judges_count: 111,
                    sections: vec!["Civil Section".to_string(), "Criminal Section".to_string(), "Administrative and Tax Section".to_string()],
                    competencies: vec![
                        "Final appeals in civil and criminal matters".to_string(),
                        "Administrative litigation review".to_string(),
                        "Legal uniformity decisions".to_string(),
                        "Disciplinary cases against magistrates".to_string(),
                    ],
                    landmark_decisions: vec![
                        LandmarkDecision {
                            case_number: "Decision 1537/2019".to_string(),
                            date: "2019-04-15".to_string(),
                            court: "High Court of Cassation and Justice".to_string(),
                            subject: "Data protection and judicial independence".to_string(),
                            legal_principle: "Balance between transparency and judicial independence in magistrate evaluation".to_string(),
                            impact: "Shaped magistrate evaluation procedures".to_string(),
                        },
                    ],
                    headquarters: "Strada Blanduziei 2, Bucharest".to_string(),
                },
                constitutional_court: ConstitutionalCourt {
                    president: "Marian Enache".to_string(),
                    vice_president: "Laura-Iuliana Scântei".to_string(),
                    judges_count: 9,
                    appointment_method: "3 by Chamber of Deputies, 3 by Senate, 3 by President".to_string(),
                    term_years: 9,
                    competencies: vec![
                        "Constitutional review of laws before promulgation".to_string(),
                        "Constitutional exceptions during judicial proceedings".to_string(),
                        "Conflicts of constitutional nature between public authorities".to_string(),
                        "Presidential impeachment procedures".to_string(),
                        "Constitutional review of referendum initiatives".to_string(),
                    ],
                    recent_decisions: vec![
                        ConstitutionalDecision {
                            decision_number: "Decision 358/2022".to_string(),
                            date: "2022-06-07".to_string(),
                            subject: "EU Recovery and Resilience Plan constitutionality".to_string(),
                            constitutional_articles: vec!["Article 148".to_string(), "Article 11".to_string()],
                            outcome: "Constitutional with interpretation".to_string(),
                            dissenting_opinions: vec!["Judge Deliorga - concerns about sovereignty limits".to_string()],
                        },
                    ],
                    headquarters: "Strada Panduri 2, Bucharest".to_string(),
                },
                courts_of_appeal: vec![
                    CourtOfAppeal {
                        name: "Bucharest Court of Appeal".to_string(),
                        location: "Bucharest".to_string(),
                        jurisdiction: "Bucharest and surrounding counties".to_string(),
                        judges_count: 187,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string(), "Administrative".to_string()],
                    },
                    CourtOfAppeal {
                        name: "Cluj Court of Appeal".to_string(),
                        location: "Cluj-Napoca".to_string(),
                        jurisdiction: "Northwestern Romania".to_string(),
                        judges_count: 98,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string()],
                    },
                ],
                county_courts: vec![
                    CountyCourt {
                        name: "Bucharest County Court".to_string(),
                        location: "Bucharest".to_string(),
                        jurisdiction: "Bucharest Municipality".to_string(),
                        judges_count: 245,
                        annual_cases: 125_640,
                    },
                ],
                local_courts: vec![
                    LocalCourt {
                        name: "Bucharest District 1 Local Court".to_string(),
                        location: "Bucharest".to_string(),
                        jurisdiction: "Sector 1 Bucharest".to_string(),
                        judges_count: 42,
                        annual_cases: 34_560,
                    },
                ],
                prosecution_service: ProsecutionService {
                    prosecutor_general: "Alex Florența".to_string(),
                    first_deputy_prosecutor_general: "Marius Iacob".to_string(),
                    dna: NationalAnticorruptionDirectorate {
                        chief_prosecutor: "Laurențiu Viorel Dinică".to_string(),
                        competencies: vec![
                            "High-level corruption cases".to_string(),
                            "EU funds fraud investigation".to_string(),
                            "Public procurement corruption".to_string(),
                            "Money laundering related to corruption".to_string(),
                        ],
                        major_cases: vec!["Teleorman County Council corruption case".to_string(), "Microsoft Romania licensing case".to_string()],
                        european_cooperation: vec!["EPPO cooperation".to_string(), "OLAF joint investigations".to_string()],
                    },
                    diicot: DirectorateForInvestigatingOrganizedCrime {
                        chief_prosecutor: "Giorgiana Hosu".to_string(),
                        competencies: vec![
                            "Organized crime investigation".to_string(),
                            "Drug trafficking cases".to_string(),
                            "Human trafficking".to_string(),
                            "Terrorism and extremism".to_string(),
                        ],
                        specialized_units: vec!["Anti-drug unit".to_string(), "Anti-terrorism unit".to_string(), "Cybercrime unit".to_string()],
                        international_cooperation: vec!["Europol cooperation".to_string(), "Joint investigation teams".to_string()],
                    },
                    structure: vec![
                        "Hierarchical organization".to_string(),
                        "Prosecutor General at the apex".to_string(),
                        "Specialized directorates".to_string(),
                    ],
                    competencies: vec![
                        "Criminal prosecution".to_string(),
                        "Legality supervision".to_string(),
                        "Public interest protection".to_string(),
                    ],
                },
                superior_council_of_magistracy: SuperiorCouncilOfMagistracy {
                    president: "Nicoleta Margareta Tint".to_string(),
                    composition: "19 members - 14 elected magistrates, 2 civil society representatives, 3 ex officio".to_string(),
                    competencies: vec![
                        "Judicial independence guarantee".to_string(),
                        "Magistrate appointment and promotion".to_string(),
                        "Judicial administration".to_string(),
                        "Disciplinary proceedings".to_string(),
                    ],
                    judicial_independence_role: "Constitutional guarantee of judicial independence".to_string(),
                },
                judicial_inspection: JudicialInspection {
                    general_inspector: "Ioana Bogdan".to_string(),
                    competencies: vec![
                        "Judicial activity monitoring".to_string(),
                        "Disciplinary investigations".to_string(),
                        "Quality control".to_string(),
                    ],
                    disciplinary_procedures: "Investigation and prosecution of magistrate misconduct".to_string(),
                    quality_assurance: "Judicial activity quality assessment and improvement".to_string(),
                },
            },
            legal_codes: RomanianLegalCodes {
                civil_code: CivilCode {
                    official_name: "Codul civil (Law 287/2009)".to_string(),
                    approval_date: "2009-07-17".to_string(),
                    effective_date: "2011-10-01".to_string(),
                    total_provisions: 2_664,
                    books: vec![
                        CodeBook {
                            number: 1,
                            title: "On persons".to_string(),
                            chapters: vec!["Natural persons".to_string(), "Legal persons".to_string(), "Family relations".to_string()],
                            articles_range: "Articles 1-597".to_string(),
                        },
                        CodeBook {
                            number: 2,
                            title: "On goods".to_string(),
                            chapters: vec!["Classification of goods".to_string(), "Property".to_string(), "Possession".to_string()],
                            articles_range: "Articles 598-835".to_string(),
                        },
                        CodeBook {
                            number: 3,
                            title: "On ways of acquiring goods".to_string(),
                            chapters: vec!["Obligations".to_string(), "Contracts".to_string(), "Succession".to_string()],
                            articles_range: "Articles 836-1.649".to_string(),
                        },
                        CodeBook {
                            number: 4,
                            title: "On obligations".to_string(),
                            chapters: vec!["General provisions".to_string(), "Contractual obligations".to_string(), "Non-contractual obligations".to_string()],
                            articles_range: "Articles 1.650-2.664".to_string(),
                        },
                    ],
                    key_principles: vec![
                        "Good faith".to_string(),
                        "Prohibition of abuse of rights".to_string(),
                        "Contractual freedom".to_string(),
                        "Legal certainty".to_string(),
                    ],
                    recodification_context: "Complete recodification replacing communist-era civil code".to_string(),
                },
                criminal_code: CriminalCode {
                    official_name: "Codul penal (Law 286/2009)".to_string(),
                    approval_date: "2009-07-17".to_string(),
                    last_amendment: "2023-11-15".to_string(),
                    total_provisions: 446,
                    general_part: CodePart {
                        name: "General Part".to_string(),
                        chapters: vec!["Criminal law".to_string(), "Infraction".to_string(), "Penalty".to_string(), "Safety measures".to_string()],
                        main_subjects: vec!["Criminal liability".to_string(), "Sanctions system".to_string()],
                    },
                    special_part: CodePart {
                        name: "Special Part".to_string(),
                        chapters: vec!["Crimes against person".to_string(), "Crimes against property".to_string(), "Crimes against public authority".to_string()],
                        main_subjects: vec!["Specific criminal offenses".to_string(), "Penalty ranges".to_string()],
                    },
                    penalty_system: PenaltySystem {
                        prison_sentences: vec!["Imprisonment from 6 months to 30 years".to_string(), "Life imprisonment".to_string(), "Death penalty abolished".to_string()],
                        fines_system: "Fine between 100-600,000 lei or day-fine system".to_string(),
                        alternative_sanctions: vec!["Community service".to_string(), "House arrest".to_string(), "Deprivation of rights".to_string()],
                        rehabilitation_programs: vec!["Probation supervision".to_string(), "Social reintegration".to_string(), "Victim-offender mediation".to_string()],
                    },
                    recent_reforms: vec![
                        "2020 - Enhanced anti-corruption provisions".to_string(),
                        "2021 - Domestic violence criminalization strengthening".to_string(),
                        "2023 - Cybercrime legislation update".to_string(),
                    ],
                },
                civil_procedure_code: CivilProcedureCode {
                    official_name: "Codul de procedură civilă (Law 134/2010)".to_string(),
                    approval_date: "2010-07-01".to_string(),
                    procedural_principles: vec![
                        "Adversarial principle".to_string(),
                        "Right to defense".to_string(),
                        "Oral proceedings".to_string(),
                        "Public proceedings".to_string(),
                    ],
                    types_of_proceedings: vec![
                        "First instance proceedings".to_string(),
                        "Appeal proceedings".to_string(),
                        "Supreme Court appeals".to_string(),
                        "Special proceedings".to_string(),
                    ],
                    appeal_system: vec![
                        "Appeal to higher court".to_string(),
                        "Supreme Court review".to_string(),
                        "Extraordinary remedies".to_string(),
                    ],
                    electronic_procedures: vec![
                        "Electronic filing (ECRIS)".to_string(),
                        "Video conferencing".to_string(),
                        "Digital case management".to_string(),
                    ],
                },
                criminal_procedure_code: CriminalProcedureCode {
                    official_name: "Codul de procedură penală (Law 135/2010)".to_string(),
                    approval_date: "2010-07-01".to_string(),
                    investigation_phase: "Prosecutor-led investigation with judicial control".to_string(),
                    trial_phase: "Public oral trial with adversarial proceedings".to_string(),
                    appeal_phase: "Right to appeal with full review of facts and law".to_string(),
                    special_procedures: vec![
                        "Simplified procedure for flagrant crimes".to_string(),
                        "Plea bargaining (agreement on punishment recognition)".to_string(),
                        "Mediation in criminal matters".to_string(),
                    ],
                },
                administrative_code: AdministrativeCode {
                    official_name: "Codul administrativ (Law 57/2019)".to_string(),
                    approval_date: "2019-04-03".to_string(),
                    general_principles: vec![
                        "Legality".to_string(),
                        "Impartiality".to_string(),
                        "Proportionality".to_string(),
                        "Transparency".to_string(),
                        "Efficiency".to_string(),
                    ],
                    administrative_acts: "Administrative acts must be motivated and subject to judicial review".to_string(),
                    procedural_guarantees: vec![
                        "Right to be heard".to_string(),
                        "Right to legal assistance".to_string(),
                        "Right to access administrative file".to_string(),
                        "Right to appeal".to_string(),
                    ],
                    digital_administration: vec![
                        "Digital by default principle".to_string(),
                        "Electronic signature".to_string(),
                        "Online public services".to_string(),
                    ],
                },
                labor_code: LaborCode {
                    official_name: "Codul muncii (Law 53/2003)".to_string(),
                    approval_date: "2003-01-24".to_string(),
                    individual_labor_relations: "Employment contracts, working time, wages, disciplinary measures, termination".to_string(),
                    collective_labor_relations: "Trade unions, employers' organizations, collective bargaining, strikes".to_string(),
                    labor_inspection: "Labor Inspectorate supervision with sanctioning powers".to_string(),
                    social_dialogue: "Tripartite social dialogue at national and sectoral levels".to_string(),
                },
                tax_codes: vec![
                    TaxCode {
                        name: "Income Tax Code".to_string(),
                        tax_type: "Personal Income Tax".to_string(),
                        approval_date: "2015-12-23".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Salary income".to_string(),
                                rate_percentage: 10.0,
                                threshold_amount: 0.0,
                                applicable_from: "2018-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Minimum wage exemption".to_string(), "Meal vouchers".to_string()],
                        collection_procedures: "Monthly withholding with annual declaration for certain incomes".to_string(),
                    },
                    TaxCode {
                        name: "Corporate Income Tax".to_string(),
                        tax_type: "Corporate Income Tax".to_string(),
                        approval_date: "2015-12-23".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Standard corporate rate".to_string(),
                                rate_percentage: 16.0,
                                threshold_amount: 0.0,
                                applicable_from: "2005-01-01".to_string(),
                            },
                            TaxRate {
                                category: "Micro-enterprises".to_string(),
                                rate_percentage: 1.0,
                                threshold_amount: 500_000.0,
                                applicable_from: "2018-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Reinvested profit".to_string(), "Research and development".to_string()],
                        collection_procedures: "Quarterly declarations with annual settlement".to_string(),
                    },
                ],
                family_code: FamilyCode {
                    legal_framework: "Family law integrated in Civil Code Book I".to_string(),
                    marriage_regulation: "Civil marriage between man and woman".to_string(),
                    civil_partnership: "No civil unions - marriage only recognized form".to_string(),
                    parental_authority: "Joint parental authority with child's best interest priority".to_string(),
                    adoption_procedures: "Full adoption with court approval and social services assessment".to_string(),
                },
                commercial_law: CommercialLaw {
                    main_legislation: "Companies Law 31/1990 and subsequent amendments".to_string(),
                    company_types: vec![
                        "SRL - Limited Liability Company".to_string(),
                        "SA - Joint Stock Company".to_string(),
                        "SCS - Limited Partnership".to_string(),
                        "SNC - General Partnership".to_string(),
                        "PFA - Authorized Natural Person".to_string(),
                    ],
                    commercial_registration: "National Trade Register Office (ONRC) registration".to_string(),
                    insolvency_law: "Insolvency Law 85/2014 with preventive procedures".to_string(),
                },
            },
            european_integration: RomanianEuropeanIntegration {
                eu_membership: EUMembership {
                    accession_date: "2007-01-01".to_string(),
                    accession_treaty: "Treaty of Accession 2005".to_string(),
                    referendum_date: "2003-05-18-19".to_string(),
                    referendum_result: "89.7% in favor".to_string(),
                    membership_benefits: vec![
                        "Single market access".to_string(),
                        "Structural and cohesion funds".to_string(),
                        "Freedom of movement".to_string(),
                        "Agricultural support (CAP)".to_string(),
                        "Regional development funds".to_string(),
                    ],
                    membership_obligations: vec![
                        "EU law transposition".to_string(),
                        "Budget contributions".to_string(),
                        "CVM compliance".to_string(),
                        "Rule of law respect".to_string(),
                    ],
                },
                eurozone_status: EurozoneStatus {
                    current_currency: "Romanian leu (RON)".to_string(),
                    euro_adoption_timeline: "Target 2029-2030 (subject to convergence criteria)".to_string(),
                    convergence_criteria: vec![
                        "Inflation criterion - progress needed".to_string(),
                        "Fiscal criterion - occasional breaches".to_string(),
                        "ERM II participation - not yet joined".to_string(),
                        "Long-term interest rates - generally met".to_string(),
                    ],
                    political_commitment: "Government committed to euro adoption".to_string(),
                },
                schengen_candidacy: SchengenCandidacy {
                    candidacy_status: "Candidate since 2011, partial accession achieved 2024".to_string(),
                    evaluation_process: "Schengen evaluation completed with positive assessment".to_string(),
                    outstanding_conditions: vec!["Full air and sea border controls acceptance achieved".to_string(), "Land borders still pending".to_string()],
                    expected_timeline: "Full Schengen membership expected 2024-2025".to_string(),
                },
                eu_representation: EURepresentation {
                    european_parliament_meps: 33,
                    council_voting_weight: 14,
                    european_commission_commissioner: "Adina Vălean (Transport)".to_string(),
                    permanent_representation: "Ambassador Luca Niculescu to EU".to_string(),
                },
                eu_legislation_transposition: EULegislationTransposition {
                    transposition_rate: 97.1,
                    infringement_procedures: 42,
                    recent_transpositions: vec![
                        "European Green Deal legislation".to_string(),
                        "Digital Services Act".to_string(),
                        "Whistleblower Protection Directive".to_string(),
                    ],
                    pending_transpositions: vec![
                        "AI Act implementation".to_string(),
                        "Data Act transposition".to_string(),
                    ],
                },
                cvm_mechanism: CVMStatus {
                    establishment_date: "2007-01-01".to_string(),
                    current_status: "Still active - significant progress but benchmarks not fully met".to_string(),
                    benchmarks: vec![
                        "Judicial independence".to_string(),
                        "Anti-corruption framework".to_string(),
                        "Integrity agencies efficiency".to_string(),
                    ],
                    progress_assessment: "Substantial progress with remaining concerns".to_string(),
                },
            },
            regional_administration: RomanianRegionalAdministration {
                development_regions: DevelopmentRegionsSystem {
                    legal_framework: "Law 315/2004 on regional development".to_string(),
                    coordination_role: "Statistical and EU programming purposes only".to_string(),
                    eu_funds_management: "Regional Operational Programs management".to_string(),
                    statistical_purposes: "NUTS 2 statistical regions for EU purposes".to_string(),
                },
                regional_development: RegionalDevelopment {
                    strategic_documents: vec![
                        "National Strategy for Regional Development 2021-2027".to_string(),
                        "National Recovery and Resilience Plan".to_string(),
                        "Smart Specialization Strategy".to_string(),
                    ],
                    development_priorities: vec![
                        "Economic competitiveness".to_string(),
                        "Infrastructure connectivity".to_string(),
                        "Digital transformation".to_string(),
                        "Green transition".to_string(),
                        "Social inclusion".to_string(),
                    ],
                    innovation_support: vec![
                        "Innovation poles".to_string(),
                        "Technology parks".to_string(),
                        "Cluster development".to_string(),
                        "Start-up ecosystems".to_string(),
                    ],
                    infrastructure_projects: vec![
                        "Transport infrastructure modernization".to_string(),
                        "Digital infrastructure development".to_string(),
                        "Energy infrastructure upgrade".to_string(),
                        "Healthcare infrastructure improvement".to_string(),
                    ],
                },
                nuts_classification: NutsClassification {
                    nuts_2_regions: vec![
                        Nuts2Region {
                            name: "Bucharest-Ilfov".to_string(),
                            nuts_code: "RO32".to_string(),
                            counties: vec!["Bucharest".to_string(), "Ilfov".to_string()],
                            gdp_per_capita: 34_500.0,
                            development_status: "More developed region".to_string(),
                        },
                        Nuts2Region {
                            name: "South-East".to_string(),
                            nuts_code: "RO22".to_string(),
                            counties: vec!["Brăila".to_string(), "Buzău".to_string(), "Constanța".to_string(), "Galați".to_string(), "Tulcea".to_string(), "Vrancea".to_string()],
                            gdp_per_capita: 16_800.0,
                            development_status: "Less developed region".to_string(),
                        },
                    ],
                    nuts_3_counties: vec!["All 41 counties plus Bucharest municipality".to_string()],
                    statistical_coordination: "National Institute of Statistics coordination".to_string(),
                },
                cross_border_cooperation: CrossBorderCooperation {
                    neighboring_countries: vec!["Bulgaria".to_string(), "Serbia".to_string(), "Hungary".to_string(), "Ukraine".to_string(), "Moldova".to_string()],
                    cooperation_programs: vec![
                        "INTERREG Romania-Bulgaria".to_string(),
                        "INTERREG Romania-Serbia".to_string(),
                        "INTERREG Romania-Hungary".to_string(),
                        "Black Sea Basin Programme".to_string(),
                        "Danube Transnational Programme".to_string(),
                    ],
                    joint_projects: vec![
                        "Cross-border transport infrastructure".to_string(),
                        "Environmental protection initiatives".to_string(),
                        "Cultural heritage preservation".to_string(),
                        "Economic cooperation projects".to_string(),
                    ],
                    legal_frameworks: vec![
                        "European Outline Convention on Transfrontier Co-operation".to_string(),
                        "Bilateral cooperation agreements".to_string(),
                        "INTERREG regulations".to_string(),
                    ],
                },
            },
            local_government: RomanianLocalGovernment {
                municipal_system: MunicipalSystem {
                    legal_framework: "Law 215/2001 on local public administration".to_string(),
                    local_council: "Representative body elected by direct vote for 4 years".to_string(),
                    mayor_powers: vec![
                        "Executive authority".to_string(),
                        "Local administration leadership".to_string(),
                        "Local council chairmanship".to_string(),
                        "Local development planning".to_string(),
                    ],
                    competencies: vec![
                        "Local public services".to_string(),
                        "Urban planning".to_string(),
                        "Local infrastructure".to_string(),
                        "Social assistance".to_string(),
                        "Environmental protection".to_string(),
                    ],
                    types_of_municipalities: vec![
                        "Municipalities (municipii)".to_string(),
                        "Towns (orașe)".to_string(),
                        "Communes (comune)".to_string(),
                    ],
                },
                commune_system: CommuneSystem {
                    legal_framework: "Same as municipalities - Law 215/2001".to_string(),
                    commune_council: "Representative body for rural administration".to_string(),
                    mayor_competencies: vec![
                        "Rural development coordination".to_string(),
                        "Agricultural support services".to_string(),
                        "Village infrastructure maintenance".to_string(),
                    ],
                    village_organization: "Villages as component settlements of communes".to_string(),
                },
                inter_community_associations: vec![
                    InterCommunityAssociation {
                        name: "Bucharest Metropolitan Area".to_string(),
                        legal_form: "Inter-community development association".to_string(),
                        member_localities: vec!["Bucharest".to_string(), "Voluntary associations from Ilfov".to_string()],
                        cooperation_areas: vec!["Transport".to_string(), "Environment".to_string(), "Economic development".to_string()],
                    },
                ],
                local_finance: LocalFinance {
                    revenue_sources: vec![
                        "Local taxes and fees".to_string(),
                        "Share from income tax".to_string(),
                        "Property taxes".to_string(),
                        "Grants and subsidies".to_string(),
                        "EU funds".to_string(),
                    ],
                    fiscal_equalization: "Equalization grants for less developed localities".to_string(),
                    grants_and_subsidies: vec!["Investment grants".to_string(), "Sectoral subsidies".to_string(), "Emergency funds".to_string()],
                    borrowing_rules: "Local debt limited to 30% of own revenues".to_string(),
                },
                citizen_participation: CitizenParticipation {
                    participatory_mechanisms: vec![
                        "Public council meetings".to_string(),
                        "Citizens' advisory committees".to_string(),
                        "Public consultations".to_string(),
                        "Participatory budgeting pilots".to_string(),
                    ],
                    local_referendums: "Local referendums binding with qualified participation".to_string(),
                    public_consultations: vec!["Urban planning".to_string(), "Budget adoption".to_string(), "Major local investments".to_string()],
                    civic_initiatives: vec!["Civic organizations".to_string(), "Neighborhood associations".to_string(), "Local development groups".to_string()],
                },
            },
            cvm_mechanism: CVMMechanism {
                establishment_context: "Established upon EU accession due to concerns about judicial reforms and anti-corruption efforts".to_string(),
                benchmarks: vec![
                    CVMBenchmark {
                        benchmark_number: 1,
                        description: "Ensure a more transparent and efficient judicial process notably by enhancing the capacity and accountability of the Superior Council of Magistracy".to_string(),
                        focus_area: "Judicial independence and efficiency".to_string(),
                        progress_status: "Substantial progress but concerns remain".to_string(),
                        remaining_challenges: vec!["Full implementation of SCM reforms".to_string(), "Consistent application of standards".to_string()],
                    },
                    CVMBenchmark {
                        benchmark_number: 2,
                        description: "Establish, as foreseen, an integrity agency with responsibilities for verifying assets, incompatibilities and potential conflicts of interest and for issuing mandatory decisions on the basis of which dissuasive sanctions can be taken".to_string(),
                        focus_area: "Integrity framework".to_string(),
                        progress_status: "Good progress with operational ANI".to_string(),
                        remaining_challenges: vec!["Enhanced enforcement capacity".to_string(), "Consistent sanctioning".to_string()],
                    },
                    CVMBenchmark {
                        benchmark_number: 3,
                        description: "Building on progress already made, continue to conduct professional, non-partisan investigations into allegations of high-level corruption".to_string(),
                        focus_area: "Anti-corruption investigations".to_string(),
                        progress_status: "Significant achievements by DNA".to_string(),
                        remaining_challenges: vec!["Maintaining investigative capacity".to_string(), "Final court decisions".to_string()],
                    },
                    CVMBenchmark {
                        benchmark_number: 4,
                        description: "Take further measures to prevent and fight against corruption, in particular within the local government".to_string(),
                        focus_area: "Local-level corruption prevention".to_string(),
                        progress_status: "Progress in prevention mechanisms".to_string(),
                        remaining_challenges: vec!["Enhanced local oversight".to_string(), "Systematic prevention".to_string()],
                    },
                ],
                progress_reports: vec![
                    ProgressReport {
                        report_date: "2023-07-05".to_string(),
                        overall_assessment: "Continued progress with remaining areas for improvement".to_string(),
                        key_developments: vec![
                            "Constitutional Court decisions on justice laws".to_string(),
                            "DNA continued effective work".to_string(),
                            "SCM reforms implementation".to_string(),
                        ],
                        recommendations: vec![
                            "Maintain judicial independence guarantees".to_string(),
                            "Ensure consistent implementation of integrity framework".to_string(),
                            "Strengthen anti-corruption coordination".to_string(),
                        ],
                    },
                ],
                current_status: "Active monitoring with significant progress but CVM not yet lifted".to_string(),
                termination_conditions: vec![
                    "Full satisfaction of all benchmarks".to_string(),
                    "Irreversible and sustainable progress".to_string(),
                    "Consistent track record over time".to_string(),
                ],
            },
        }
    }
}