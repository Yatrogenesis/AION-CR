use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete Hungarian Legal System Implementation
/// Magyarország (Hungary)
/// Current President: Katalin Novák (2022-2027)
/// Current Prime Minister: Viktor Orbán (Fidesz - Hungarian Civic Alliance)
/// Current Government: Fourth Orbán Government (Fidesz-KDNP coalition)
/// EU Member since 2004, Not in Eurozone (Hungarian forint), Schengen since 2007

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HungaryLegalSystem {
    pub constitutional_framework: HungarianConstitution,
    pub government_structure: HungarianGovernment,
    pub territorial_organization: HungarianTerritorialOrganization,
    pub judicial_system: HungarianJudicialSystem,
    pub legal_codes: HungarianLegalCodes,
    pub european_integration: HungarianEuropeanIntegration,
    pub regional_administration: HungarianRegionalAdministration,
    pub local_government: HungarianLocalGovernment,
    pub fundamental_law_system: FundamentalLawSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HungarianConstitution {
    pub name: String,
    pub adoption_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub foundation_articles: Vec<FoundationArticle>,
    pub freedom_and_responsibility: Vec<FreedomResponsibilityArticle>,
    pub state_structure: Vec<StateStructureArticle>,
    pub key_articles: HashMap<String, String>,
    pub constitutional_identity: ConstitutionalIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FoundationArticle {
    pub article: String,
    pub title: String,
    pub content_hungarian: String,
    pub content_english: String,
    pub interpretation: String,
    pub constitutional_court_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FreedomResponsibilityArticle {
    pub article: String,
    pub category: String,
    pub right_name: String,
    pub content_hungarian: String,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
    pub european_standards_alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StateStructureArticle {
    pub article: String,
    pub institution: String,
    pub content_hungarian: String,
    pub competencies: Vec<String>,
    pub relations_with_other_organs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalIdentity {
    pub christian_culture: String,
    pub national_sovereignty: String,
    pub traditional_family: String,
    pub hungarian_nation_concept: String,
    pub historical_constitution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HungarianGovernment {
    pub president: PresidentOfRepublic,
    pub prime_minister: PrimeMinister,
    pub government_cabinet: GovernmentCabinet,
    pub national_assembly: NationalAssembly,
    pub council_of_ministers: CouncilOfMinisters,
    pub current_legislature: String,
    pub political_parties: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresidentOfRepublic {
    pub name: String,
    pub term_start: String,
    pub term_end: String,
    pub election_date: String,
    pub parliamentary_votes: u32,
    pub previous_occupation: String,
    pub constitutional_powers: Vec<String>,
    pub residence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrimeMinister {
    pub name: String,
    pub party: String,
    pub appointment_date: String,
    pub terms_served: u32,
    pub government_program: Vec<String>,
    pub coalition_partners: Vec<String>,
    pub reforms_agenda: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernmentCabinet {
    pub formation_date: String,
    pub total_ministers: u32,
    pub ministries: Vec<Ministry>,
    pub state_secretaries: Vec<StateSecretary>,
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
pub struct StateSecretary {
    pub name: String,
    pub ministry: String,
    pub specific_area: String,
    pub appointment_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NationalAssembly {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub speaker: String,
    pub deputy_speakers: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
    pub current_session: String,
    pub electoral_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilOfMinisters {
    pub formation_basis: String,
    pub decision_making: String,
    pub coordination_role: String,
    pub european_affairs: String,
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
pub struct HungarianTerritorialOrganization {
    pub administrative_division: AdministrativeDivision,
    pub counties: Vec<County>,
    pub districts: Vec<District>,
    pub municipalities: Vec<Municipality>,
    pub budapest_special_status: BudapestSpecialStatus,
    pub territorial_statistics: TerritorialStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeDivision {
    pub total_area_km2: f64,
    pub population: u64,
    pub counties_count: u32,
    pub districts_count: u32,
    pub municipalities_count: u32,
    pub cities_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct County {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub districts_count: u32,
    pub county_assembly: CountyAssembly,
    pub competencies: Vec<String>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountyAssembly {
    pub members_count: u32,
    pub political_composition: HashMap<String, u32>,
    pub current_term: String,
    pub president: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct District {
    pub name: String,
    pub county: String,
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
    pub municipal_assembly_seats: u32,
    pub municipal_status: String,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BudapestSpecialStatus {
    pub constitutional_basis: String,
    pub special_competencies: Vec<String>,
    pub districts_count: u32,
    pub lord_mayor: String,
    pub budapest_assembly: BudapestAssembly,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BudapestAssembly {
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
pub struct HungarianJudicialSystem {
    pub curia: Curia,
    pub constitutional_court: ConstitutionalCourt,
    pub supreme_administrative_court: SupremeAdministrativeCourt,
    pub appellate_courts: Vec<AppellateCourt>,
    pub regional_courts: Vec<RegionalCourt>,
    pub district_courts: Vec<DistrictCourt>,
    pub prosecution_service: ProsecutionService,
    pub judicial_administration: JudicialAdministration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Curia {
    pub president: String,
    pub vice_president: String,
    pub justices_count: u32,
    pub chambers: Vec<String>,
    pub competencies: Vec<String>,
    pub landmark_decisions: Vec<LandmarkDecision>,
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
pub struct SupremeAdministrativeCourt {
    pub president: String,
    pub vice_president: String,
    pub justices_count: u32,
    pub competencies: Vec<String>,
    pub recent_decisions: Vec<AdministrativeDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppellateCourt {
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
    pub prosecutor_general: String,
    pub chief_prosecutors: Vec<String>,
    pub regional_prosecutors: Vec<String>,
    pub structure: Vec<String>,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialAdministration {
    pub national_judicial_office: NationalJudicialOffice,
    pub judicial_council: JudicialCouncil,
    pub court_administration: Vec<String>,
    pub judicial_independence_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NationalJudicialOffice {
    pub president: String,
    pub competencies: Vec<String>,
    pub administrative_role: String,
    pub budget_management: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudicialCouncil {
    pub composition: String,
    pub competencies: Vec<String>,
    pub role_in_appointments: String,
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
    pub case_number: String,
    pub date: String,
    pub subject: String,
    pub constitutional_articles: Vec<String>,
    pub outcome: String,
    pub dissenting_opinions: Vec<String>,
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
pub struct HungarianLegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub civil_procedure_code: CivilProcedureCode,
    pub criminal_procedure_code: CriminalProcedureCode,
    pub administrative_procedure_code: AdministrativeProcedureCode,
    pub labor_code: LaborCode,
    pub tax_codes: Vec<TaxCode>,
    pub family_code: FamilyCode,
    pub company_law: CompanyLaw,
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
pub struct FamilyCode {
    pub legal_framework: String,
    pub marriage_regulation: String,
    pub registered_partnership: String,
    pub parental_authority: String,
    pub adoption_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyLaw {
    pub main_act: String,
    pub company_types: Vec<String>,
    pub commercial_registration: String,
    pub corporate_governance: String,
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
pub struct HungarianEuropeanIntegration {
    pub eu_membership: EUMembership,
    pub eurozone_status: EurozoneStatus,
    pub schengen_area: SchengenParticipation,
    pub eu_representation: EURepresentation,
    pub eu_legislation_transposition: EULegislationTransposition,
    pub article_7_procedure: Article7Procedure,
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
pub struct Article7Procedure {
    pub procedure_status: String,
    pub issues_raised: Vec<String>,
    pub hungarian_response: Vec<String>,
    pub current_developments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HungarianRegionalAdministration {
    pub county_system: CountySystem,
    pub regional_development: RegionalDevelopment,
    pub nuts_regions: Vec<NutsRegion>,
    pub development_programs: Vec<DevelopmentProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountySystem {
    pub legal_framework: String,
    pub county_competencies: Vec<String>,
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
pub struct NutsRegion {
    pub name: String,
    pub nuts_code: String,
    pub counties_included: Vec<String>,
    pub gdp_per_capita: f64,
    pub development_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevelopmentProgram {
    pub name: String,
    pub funding_source: String,
    pub budget: f64,
    pub objectives: Vec<String>,
    pub implementation_period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HungarianLocalGovernment {
    pub municipal_system: MunicipalSystem,
    pub local_associations: Vec<LocalAssociation>,
    pub inter_municipal_cooperation: InterMunicipalCooperation,
    pub local_finance: LocalFinance,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MunicipalSystem {
    pub legal_framework: String,
    pub municipal_assembly: String,
    pub mayor_powers: Vec<String>,
    pub competencies: Vec<String>,
    pub types_of_settlements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalAssociation {
    pub name: String,
    pub legal_form: String,
    pub purposes: Vec<String>,
    pub membership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterMunicipalCooperation {
    pub cooperation_forms: Vec<String>,
    pub joint_offices: Vec<String>,
    pub service_associations: Vec<String>,
    pub development_councils: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalFinance {
    pub revenue_sources: Vec<String>,
    pub normative_financing: String,
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
pub struct FundamentalLawSystem {
    pub adoption_process: AdoptionProcess,
    pub constitutional_identity_doctrine: ConstitutionalIdentityDoctrine,
    pub amendments: Vec<ConstitutionalAmendment>,
    pub interpretation_principles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdoptionProcess {
    pub constitutional_assembly: String,
    pub adoption_date: String,
    pub public_consultation: String,
    pub political_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalIdentityDoctrine {
    pub core_elements: Vec<String>,
    pub protection_mechanisms: Vec<String>,
    pub eu_law_relations: String,
    pub constitutional_court_jurisprudence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalAmendment {
    pub amendment_number: String,
    pub date: String,
    pub subject_matter: String,
    pub political_controversy: Option<String>,
}

impl Default for HungaryLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: HungarianConstitution {
                name: "Magyarország Alaptörvénye (Fundamental Law of Hungary)".to_string(),
                adoption_date: "2011-04-25".to_string(),
                last_amendment: "2023-12-15".to_string(),
                total_articles: 54,
                foundation_articles: vec![
                    FoundationArticle {
                        article: "A) cikk".to_string(),
                        title: "Közös örökség".to_string(),
                        content_hungarian: "HAZÁNK neve Magyarország. Magyarország független, demokratikus jogállam. Magyarország államformája köztársaság. A magyar nép tagjai a magyar nemzet. Magyarország fővárosa Budapest.".to_string(),
                        content_english: "OUR HOMELAND's name is Hungary. Hungary is an independent, democratic state governed by the rule of law. Hungary's form of government is a republic. The members of the Hungarian people are the Hungarian nation. Hungary's capital is Budapest.".to_string(),
                        interpretation: "Establishes Hungary as independent democratic state and defines Hungarian nation".to_string(),
                        constitutional_court_decisions: vec!["22/2016. (XII. 5.) AB határozat - National identity".to_string()],
                    },
                    FoundationArticle {
                        article: "B) cikk".to_string(),
                        title: "Népszuverenitás".to_string(),
                        content_hungarian: "Magyarország független és demokratikus állam. A hatalom forrása a nép. A nép a hatalmát választott képviselői útján, kivételesen közvetlenül gyakorolja.".to_string(),
                        content_english: "Hungary is an independent and democratic state. The source of public power shall be the people. The people shall exercise their power through their elected representatives or, in exceptional cases, directly.".to_string(),
                        interpretation: "Establishes popular sovereignty and democratic governance principle".to_string(),
                        constitutional_court_decisions: vec!["2/2019. (III. 5.) AB határozat - Popular sovereignty".to_string()],
                    },
                    FoundationArticle {
                        article: "C) cikk".to_string(),
                        title: "Állami szervezet".to_string(),
                        content_hungarian: "Az állam működése a hatalom megosztásának elvén alapul. Magyarország alkotmányos rendjének alapja a jogállamiság.".to_string(),
                        content_english: "The operation of the Hungarian State shall be based on the principle of the separation of powers. The foundation of Hungary's constitutional order is the rule of law.".to_string(),
                        interpretation: "Establishes separation of powers and rule of law as constitutional foundations".to_string(),
                        constitutional_court_decisions: vec!["13/2013. (VI. 17.) AB határozat - Rule of law".to_string()],
                    },
                ],
                freedom_and_responsibility: vec![
                    FreedomResponsibilityArticle {
                        article: "I. cikk".to_string(),
                        category: "Emberi méltóság".to_string(),
                        right_name: "Human dignity".to_string(),
                        content_hungarian: "Az ember sérthetetlen és elidegeníthetetlen alapvető jogai vannak. Ezek tiszteletben tartása és védelme az állam elsőrendű kötelezettsége. Magyarország elismeri az ember alapvető jogait és szabadságait.".to_string(),
                        limitations: vec!["No limitations on fundamental human dignity".to_string()],
                        jurisprudence: vec!["64/1991. (XII. 17.) AB határozat - Human dignity foundation".to_string()],
                        european_standards_alignment: "Fully aligned with ECHR Article 3 and EU Charter Article 1".to_string(),
                    },
                    FreedomResponsibilityArticle {
                        article: "II. cikk".to_string(),
                        category: "Élethez való jog".to_string(),
                        right_name: "Right to life and human dignity".to_string(),
                        content_hungarian: "Az emberi méltóság sérthetetlen. Minden embernek joga van az élethez és az emberi méltósághoz, a magzat életét a fogantatástól kezdve védelem illeti meg.".to_string(),
                        limitations: vec!["Death penalty prohibited".to_string(), "Fetal life protection from conception".to_string()],
                        jurisprudence: vec!["48/1998. (XI. 23.) AB határozat - Right to life".to_string()],
                        european_standards_alignment: "Consistent with ECHR Article 2 with additional fetal protection".to_string(),
                    },
                    FreedomResponsibilityArticle {
                        article: "XV. cikk".to_string(),
                        category: "Egyenlőség".to_string(),
                        right_name: "Equality before the law".to_string(),
                        content_hungarian: "A törvény előtt mindenki egyenlő. Minden ember jogképes. Magyarország az alapvető jogokat mindenkinek bármely megkülönböztetés, nevezetesen faj, szín, nem, fogyatékosság, nyelv, vallás, politikai vagy más vélemény, nemzeti vagy társadalmi származás, vagyoni, születési vagy egyéb helyzet szerinti különbségtétel nélkül biztosítja.".to_string(),
                        limitations: vec!["Positive discrimination measures permitted for historical disadvantages".to_string()],
                        jurisprudence: vec!["9/1990. (IV. 25.) AB határozat - Equality principle".to_string()],
                        european_standards_alignment: "Aligned with EU Charter Article 20 and ECHR Article 14".to_string(),
                    },
                ],
                state_structure: vec![
                    StateStructureArticle {
                        article: "9. cikk".to_string(),
                        institution: "Köztársasági elnök".to_string(),
                        content_hungarian: "A köztársasági elnök Magyarország államfője. A köztársasági elnök a magyar állam egységének megtestesítője, őrködik az államszervezet demokratikus működése felett.".to_string(),
                        competencies: vec![
                            "State representation".to_string(),
                            "Government formation oversight".to_string(),
                            "Constitutional guardian role".to_string(),
                        ],
                        relations_with_other_organs: vec!["Appoints Prime Minister".to_string(), "Dissolves Parliament".to_string()],
                    },
                    StateStructureArticle {
                        article: "15. cikk".to_string(),
                        institution: "Miniszterelnök".to_string(),
                        content_hungarian: "A miniszterelnök a végrehajtó hatalom általános irányítója. A miniszterelnök meghatározza a Kormány általános politikáját.".to_string(),
                        competencies: vec![
                            "Executive power leadership".to_string(),
                            "Government policy determination".to_string(),
                            "Ministers coordination".to_string(),
                        ],
                        relations_with_other_organs: vec!["Accountable to Parliament".to_string(), "Appointed by President".to_string()],
                    },
                ],
                key_articles: {
                    let mut articles = HashMap::new();
                    articles.insert("Article_A".to_string(), "Magyarország független, demokratikus jogállam".to_string());
                    articles.insert("Article_B".to_string(), "A hatalom forrása a nép".to_string());
                    articles.insert("Article_D".to_string(), "Magyarország felelősséget visel a határain kívül élő magyarokért".to_string());
                    articles.insert("Article_R".to_string(), "Magyarország védelmezi a Magyarországi Református Egyházat és a Magyarországi Evangélikus Egyházat".to_string());
                    articles
                },
                constitutional_identity: ConstitutionalIdentity {
                    christian_culture: "Magyarország keresztény kultúrájának védelme az Alaptörvény alapja".to_string(),
                    national_sovereignty: "Nemzeti szuverenitás védelme az európai integrációban".to_string(),
                    traditional_family: "Házasság mint férfi és nő közti szövetség alkotmányos védelme".to_string(),
                    hungarian_nation_concept: "Egységes magyar nemzet határon túli magyarokkal".to_string(),
                    historical_constitution: "Szent Korona-tan és történeti alkotmány folytonossága".to_string(),
                },
            },
            government_structure: HungarianGovernment {
                president: PresidentOfRepublic {
                    name: "Katalin Novák".to_string(),
                    term_start: "2022-05-10".to_string(),
                    term_end: "2027-05-10".to_string(),
                    election_date: "2022-03-10".to_string(),
                    parliamentary_votes: 137,
                    previous_occupation: "Minister of Family Affairs, State Secretary for Family and Youth Policy".to_string(),
                    constitutional_powers: vec![
                        "Appoint Prime Minister".to_string(),
                        "Dissolve National Assembly".to_string(),
                        "Sign or return laws".to_string(),
                        "Commander-in-chief of armed forces".to_string(),
                        "International representation".to_string(),
                    ],
                    residence: "Sándor Palace, Budapest".to_string(),
                },
                prime_minister: PrimeMinister {
                    name: "Viktor Mihály Orbán".to_string(),
                    party: "Fidesz - Magyar Polgári Szövetség".to_string(),
                    appointment_date: "2010-05-29".to_string(),
                    terms_served: 4,
                    government_program: vec![
                        "National sovereignty protection".to_string(),
                        "Christian democratic governance".to_string(),
                        "Economic competitiveness enhancement".to_string(),
                        "Traditional family values protection".to_string(),
                        "Migration control and border security".to_string(),
                    ],
                    coalition_partners: vec!["KDNP - Kereszténydemokrata Néppárt".to_string()],
                    reforms_agenda: vec![
                        "Judicial system reform".to_string(),
                        "Media law reform".to_string(),
                        "Higher education restructuring".to_string(),
                        "Civil society regulation".to_string(),
                    ],
                },
                government_cabinet: GovernmentCabinet {
                    formation_date: "2022-05-24".to_string(),
                    total_ministers: 12,
                    ministries: vec![
                        Ministry {
                            name: "Ministry of Interior".to_string(),
                            minister_name: "Sándor Pintér".to_string(),
                            party: "Fidesz".to_string(),
                            portfolio_areas: vec!["Public administration".to_string(), "Law enforcement".to_string(), "Emergency management".to_string()],
                            budget_2024: 1_850_000_000_000.0,
                            staff_count: 55_000,
                            main_headquarters: "József Attila utca 2-4, Budapest".to_string(),
                        },
                        Ministry {
                            name: "Ministry of Finance".to_string(),
                            minister_name: "Mihály Varga".to_string(),
                            party: "Fidesz".to_string(),
                            portfolio_areas: vec!["Public finances".to_string(), "Tax policy".to_string(), "Budget management".to_string()],
                            budget_2024: 2_200_000_000_000.0,
                            staff_count: 32_000,
                            main_headquarters: "József nádor tér 2-4, Budapest".to_string(),
                        },
                        Ministry {
                            name: "Ministry of Defense".to_string(),
                            minister_name: "Kristóf Szalay-Bobrovniczky".to_string(),
                            party: "Fidesz".to_string(),
                            portfolio_areas: vec!["Armed forces".to_string(), "Defense policy".to_string(), "NATO cooperation".to_string()],
                            budget_2024: 1_450_000_000_000.0,
                            staff_count: 45_000,
                            main_headquarters: "Balaton utca 7-11, Budapest".to_string(),
                        },
                    ],
                    state_secretaries: vec![
                        StateSecretary {
                            name: "Bence Rétvári".to_string(),
                            ministry: "Ministry of Interior".to_string(),
                            specific_area: "Parliamentary and Strategic State Secretary".to_string(),
                            appointment_date: "2022-05-24".to_string(),
                        },
                    ],
                    government_spokesperson: "Alexandra Szentkirályi".to_string(),
                },
                national_assembly: NationalAssembly {
                    seats_total: 199,
                    current_composition: {
                        let mut composition = HashMap::new();
                        composition.insert("Fidesz-KDNP".to_string(), 135);
                        composition.insert("Egységben Magyarországért".to_string(), 27);
                        composition.insert("Jobbik".to_string(), 16);
                        composition.insert("LMP".to_string(), 10);
                        composition.insert("DK".to_string(), 9);
                        composition.insert("Momentum".to_string(), 2);
                        composition
                    },
                    speaker: "László Kövér".to_string(),
                    deputy_speakers: vec![
                        "Mónika Dunai".to_string(),
                        "István Jakab".to_string(),
                        "Sándor Lezsák".to_string(),
                    ],
                    parliamentary_groups: vec![
                        ParliamentaryGroup {
                            party_name: "Fidesz - Magyar Polgári Szövetség".to_string(),
                            leader: "Máté Kocsis".to_string(),
                            seats: 117,
                            political_orientation: "National conservative".to_string(),
                            founded: "1988".to_string(),
                            european_affiliation: "European Conservatives and Reformists".to_string(),
                        },
                        ParliamentaryGroup {
                            party_name: "KDNP - Kereszténydemokrata Néppárt".to_string(),
                            leader: "Zsolt Semjén".to_string(),
                            seats: 18,
                            political_orientation: "Christian democratic".to_string(),
                            founded: "1989".to_string(),
                            european_affiliation: "European People's Party".to_string(),
                        },
                    ],
                    current_session: "IX. National Assembly (2022-2026)".to_string(),
                    electoral_system: "Mixed electoral system with compensation mechanism".to_string(),
                },
                council_of_ministers: CouncilOfMinisters {
                    formation_basis: "Prime Minister leadership with collective responsibility".to_string(),
                    decision_making: "Consensus-based decision making under PM leadership".to_string(),
                    coordination_role: "Policy coordination and implementation oversight".to_string(),
                    european_affairs: "EU policy coordination through specialized committees".to_string(),
                },
                current_legislature: "Fourth Orbán Government (2022-present)".to_string(),
                political_parties: vec![
                    PoliticalParty {
                        name: "Fidesz - Magyar Polgári Szövetség".to_string(),
                        abbreviation: "Fidesz".to_string(),
                        leader: "Viktor Orbán".to_string(),
                        founded: "1988-03-30".to_string(),
                        ideology: vec!["National conservatism".to_string(), "Christian democracy".to_string(), "Illiberalism".to_string()],
                        membership: 220_000,
                        headquarters: "Lendvay utca 28, Budapest".to_string(),
                        european_party_family: "European Conservatives and Reformists".to_string(),
                    },
                    PoliticalParty {
                        name: "Magyar Szocialista Párt".to_string(),
                        abbreviation: "MSZP".to_string(),
                        leader: "Bertalan Tóth".to_string(),
                        founded: "1989-10-07".to_string(),
                        ideology: vec!["Social democracy".to_string(), "Pro-European".to_string()],
                        membership: 15_000,
                        headquarters: "Jókai utca 6, Budapest".to_string(),
                        european_party_family: "Party of European Socialists".to_string(),
                    },
                ],
            },
            territorial_organization: HungarianTerritorialOrganization {
                administrative_division: AdministrativeDivision {
                    total_area_km2: 93_028.0,
                    population: 9_689_000,
                    counties_count: 19,
                    districts_count: 175,
                    municipalities_count: 3_155,
                    cities_count: 346,
                },
                counties: vec![
                    County {
                        name: "Pest megye".to_string(),
                        capital: "Budapest (administrative functions)".to_string(),
                        area_km2: 6_393.0,
                        population: 1_296_029,
                        districts_count: 17,
                        county_assembly: CountyAssembly {
                            members_count: 20,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("Fidesz-KDNP".to_string(), 14);
                                composition.insert("Opposition".to_string(), 6);
                                composition
                            },
                            current_term: "2019-2024".to_string(),
                            president: "Gergely Karácsony".to_string(),
                        },
                        competencies: vec!["Regional development".to_string(), "Public services coordination".to_string()],
                        budget_2024: 85_000_000_000.0,
                    },
                    County {
                        name: "Borsod-Abaúj-Zemplén megye".to_string(),
                        capital: "Miskolc".to_string(),
                        area_km2: 7_247.0,
                        population: 645_985,
                        districts_count: 13,
                        county_assembly: CountyAssembly {
                            members_count: 20,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("Fidesz-KDNP".to_string(), 15);
                                composition.insert("Opposition".to_string(), 5);
                                composition
                            },
                            current_term: "2019-2024".to_string(),
                            president: "Béla Radics".to_string(),
                        },
                        competencies: vec!["Regional development".to_string(), "Industrial policy coordination".to_string()],
                        budget_2024: 45_000_000_000.0,
                    },
                ],
                districts: vec![
                    District {
                        name: "Budakeszi járás".to_string(),
                        county: "Pest megye".to_string(),
                        area_km2: 154.0,
                        population: 78_456,
                        administrative_center: "Budakeszi".to_string(),
                        municipalities_count: 8,
                    },
                ],
                municipalities: vec![
                    Municipality {
                        name: "Budapest".to_string(),
                        district: "Special status".to_string(),
                        area_km2: 525.0,
                        population: 1_706_851,
                        mayor: "Gergely Karácsony".to_string(),
                        mayor_party: "Independent (Párbeszéd support)".to_string(),
                        municipal_assembly_seats: 33,
                        municipal_status: "Capital city with county rights".to_string(),
                        budget_2024: 980_000_000_000.0,
                    },
                    Municipality {
                        name: "Debrecen".to_string(),
                        district: "Debreceni járás".to_string(),
                        area_km2: 461.0,
                        population: 201_432,
                        mayor: "László Papp".to_string(),
                        mayor_party: "Fidesz".to_string(),
                        municipal_assembly_seats: 21,
                        municipal_status: "City with county rights".to_string(),
                        budget_2024: 125_000_000_000.0,
                    },
                ],
                budapest_special_status: BudapestSpecialStatus {
                    constitutional_basis: "Article 31 of Fundamental Law - special status as capital".to_string(),
                    special_competencies: vec![
                        "Combined municipal and county functions".to_string(),
                        "Metropolitan governance".to_string(),
                        "Special state representation functions".to_string(),
                    ],
                    districts_count: 23,
                    lord_mayor: "Gergely Karácsony".to_string(),
                    budapest_assembly: BudapestAssembly {
                        members_count: 33,
                        political_composition: {
                            let mut composition = HashMap::new();
                            composition.insert("Fidesz-KDNP".to_string(), 12);
                            composition.insert("Opposition coalition".to_string(), 21);
                            composition
                        },
                        president: "Ervin Démuth".to_string(),
                    },
                    budget_2024: 980_000_000_000.0,
                },
                territorial_statistics: TerritorialStatistics {
                    total_area_km2: 93_028.0,
                    total_population: 9_689_000,
                    population_density: 104.1,
                    urban_population_percentage: 71.4,
                    largest_cities: vec!["Budapest".to_string(), "Debrecen".to_string(), "Miskolc".to_string(), "Szeged".to_string()],
                },
            },
            judicial_system: HungarianJudicialSystem {
                curia: Curia {
                    president: "András Zs. Varga".to_string(),
                    vice_president: "Csaba Vasvári".to_string(),
                    justices_count: 67,
                    chambers: vec!["Civil Chamber".to_string(), "Criminal Chamber".to_string(), "Administrative-Labor Chamber".to_string()],
                    competencies: vec![
                        "Final appeals in civil and criminal matters".to_string(),
                        "Extraordinary remedies".to_string(),
                        "Legal uniformity decisions".to_string(),
                        "Electoral disputes".to_string(),
                    ],
                    landmark_decisions: vec![
                        LandmarkDecision {
                            case_number: "Kfv.II.37.086/2019/7.".to_string(),
                            date: "2019-11-26".to_string(),
                            court: "Curia".to_string(),
                            subject: "Digital privacy rights in employment".to_string(),
                            legal_principle: "Balance between employer oversight and worker privacy".to_string(),
                            impact: "Established framework for workplace digital rights".to_string(),
                        },
                    ],
                    headquarters: "Markó utca 16, Budapest".to_string(),
                },
                constitutional_court: ConstitutionalCourt {
                    president: "Tamás Sulyok".to_string(),
                    vice_president: "Ágnes Czine".to_string(),
                    justices_count: 15,
                    appointment_method: "Parliamentary election with two-thirds majority".to_string(),
                    term_years: 12,
                    competencies: vec![
                        "Constitutional review of laws".to_string(),
                        "Constitutional complaints".to_string(),
                        "Conflicts between state organs".to_string(),
                        "Review of international treaty constitutionality".to_string(),
                        "Constitutional interpretation".to_string(),
                    ],
                    recent_decisions: vec![
                        ConstitutionalDecision {
                            case_number: "22/2016. (XII. 5.) AB határozat".to_string(),
                            date: "2016-12-05".to_string(),
                            subject: "EU migrant quota decision constitutionality".to_string(),
                            constitutional_articles: vec!["Article B".to_string(), "Article E".to_string()],
                            outcome: "Constitutional identity limits EU law supremacy".to_string(),
                            dissenting_opinions: vec!["Judge Pokol - broader constitutional identity concept".to_string()],
                        },
                    ],
                    headquarters: "Donáti utca 35-45, Budapest".to_string(),
                },
                supreme_administrative_court: SupremeAdministrativeCourt {
                    president: "András Patyi".to_string(),
                    vice_president: "Tünde Handó".to_string(),
                    justices_count: 42,
                    competencies: vec![
                        "Administrative disputes final instance".to_string(),
                        "Public procurement disputes".to_string(),
                        "Civil service disputes".to_string(),
                        "Tax and customs disputes".to_string(),
                    ],
                    recent_decisions: vec![
                        AdministrativeDecision {
                            case_number: "Kf.700.141/2023/5.".to_string(),
                            date: "2023-09-15".to_string(),
                            subject: "Digital administration and data protection compliance".to_string(),
                            legal_reasoning: "Administrative procedures must ensure GDPR compliance".to_string(),
                            administrative_impact: "Enhanced data protection in public administration".to_string(),
                        },
                    ],
                    headquarters: "Fő utca 70-78, Budapest".to_string(),
                },
                appellate_courts: vec![
                    AppellateCourt {
                        name: "Budapest Court of Appeal".to_string(),
                        location: "Budapest".to_string(),
                        jurisdiction: "Central Hungary".to_string(),
                        judges_count: 156,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string(), "Administrative".to_string()],
                    },
                    AppellateCourt {
                        name: "Debrecen Court of Appeal".to_string(),
                        location: "Debrecen".to_string(),
                        jurisdiction: "Eastern Hungary".to_string(),
                        judges_count: 89,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string()],
                    },
                ],
                regional_courts: vec![
                    RegionalCourt {
                        name: "Budapest-Capital Regional Court".to_string(),
                        location: "Budapest".to_string(),
                        jurisdiction: "Budapest and Pest County".to_string(),
                        judges_count: 287,
                        annual_cases: 145_630,
                    },
                ],
                district_courts: vec![
                    DistrictCourt {
                        name: "Pest Central District Court".to_string(),
                        location: "Budapest".to_string(),
                        jurisdiction: "Central Budapest districts".to_string(),
                        judges_count: 168,
                        annual_cases: 89_450,
                    },
                ],
                prosecution_service: ProsecutionService {
                    prosecutor_general: "Péter Polt".to_string(),
                    chief_prosecutors: vec!["Chief Prosecutor Budapest".to_string(), "Chief Prosecutor Debrecen".to_string()],
                    regional_prosecutors: vec!["Regional prosecutors in all counties".to_string()],
                    structure: vec![
                        "Hierarchical organization".to_string(),
                        "Prosecutor General leadership".to_string(),
                        "Specialized departments".to_string(),
                    ],
                    competencies: vec![
                        "Criminal prosecution".to_string(),
                        "Legal supervision".to_string(),
                        "Public interest protection".to_string(),
                    ],
                },
                judicial_administration: JudicialAdministration {
                    national_judicial_office: NationalJudicialOffice {
                        president: "György Senyei".to_string(),
                        competencies: vec!["Court administration".to_string(), "Judicial budget management".to_string(), "Judge appointments".to_string()],
                        administrative_role: "Central administration of court system".to_string(),
                        budget_management: "Judicial budget allocation and oversight".to_string(),
                    },
                    judicial_council: JudicialCouncil {
                        composition: "15 members - judges and legal experts".to_string(),
                        competencies: vec!["Judicial independence protection".to_string(), "Judicial ethics oversight".to_string()],
                        role_in_appointments: "Advisory role in high court appointments".to_string(),
                    },
                    court_administration: vec!["Centralized administration".to_string(), "Regional court management".to_string()],
                    judicial_independence_guarantees: vec!["Constitutional independence".to_string(), "Budget autonomy".to_string(), "Appointment security".to_string()],
                },
            },
            legal_codes: HungarianLegalCodes {
                civil_code: CivilCode {
                    official_name: "2013. évi V. törvény a Polgári Törvénykönyvről".to_string(),
                    approval_date: "2013-02-11".to_string(),
                    effective_date: "2014-03-15".to_string(),
                    total_provisions: 8_042,
                    books: vec![
                        CodeBook {
                            number: 1,
                            title: "General Provisions".to_string(),
                            chapters: vec!["Basic provisions".to_string(), "Natural persons".to_string(), "Legal persons".to_string()],
                            articles_range: "Sections 1:1 - 2:52".to_string(),
                        },
                        CodeBook {
                            number: 2,
                            title: "Family Law".to_string(),
                            chapters: vec!["Marriage".to_string(), "Registered partnership".to_string(), "Parent-child relations".to_string()],
                            articles_range: "Sections 4:1 - 4:175".to_string(),
                        },
                        CodeBook {
                            number: 3,
                            title: "Property Law".to_string(),
                            chapters: vec!["Possession".to_string(), "Ownership".to_string(), "Limited real rights".to_string()],
                            articles_range: "Sections 5:1 - 5:200".to_string(),
                        },
                        CodeBook {
                            number: 4,
                            title: "Law of Obligations".to_string(),
                            chapters: vec!["General provisions".to_string(), "Contracts".to_string(), "Non-contractual obligations".to_string()],
                            articles_range: "Sections 6:1 - 6:648".to_string(),
                        },
                    ],
                    key_principles: vec![
                        "Good faith and fair dealing".to_string(),
                        "Prohibition of abuse of rights".to_string(),
                        "Contractual freedom".to_string(),
                        "Protection of weaker party".to_string(),
                    ],
                    recodification_context: "Complete recodification replacing socialist-era civil code from 1959".to_string(),
                },
                criminal_code: CriminalCode {
                    official_name: "2012. évi C. törvény a Büntető Törvénykönyvről".to_string(),
                    approval_date: "2012-06-25".to_string(),
                    last_amendment: "2023-12-20".to_string(),
                    total_provisions: 459,
                    general_part: CodePart {
                        name: "General Part".to_string(),
                        chapters: vec!["Basic provisions".to_string(), "Criminal liability".to_string(), "Penalties and measures".to_string()],
                        main_subjects: vec!["Criminal law principles".to_string(), "Sanctions system".to_string()],
                    },
                    special_part: CodePart {
                        name: "Special Part".to_string(),
                        chapters: vec!["Crimes against life".to_string(), "Economic crimes".to_string(), "Public order crimes".to_string()],
                        main_subjects: vec!["Specific criminal offenses".to_string(), "Penalty frameworks".to_string()],
                    },
                    penalty_system: PenaltySystem {
                        prison_sentences: vec!["Up to 20 years".to_string(), "Life imprisonment".to_string(), "Death penalty abolished".to_string()],
                        fines_system: "Day-fine system based on defendant's income".to_string(),
                        alternative_sanctions: vec!["Community service".to_string(), "House arrest".to_string(), "Confiscation".to_string()],
                        rehabilitation_programs: vec!["Probation supervision".to_string(), "Drug treatment".to_string(), "Social reintegration".to_string()],
                    },
                    recent_reforms: vec![
                        "2018 - Stop Soros laws criminalizing migration assistance".to_string(),
                        "2020 - Enhanced penalties for crimes during state of emergency".to_string(),
                        "2021 - Child protection criminal provisions".to_string(),
                    ],
                },
                civil_procedure_code: CivilProcedureCode {
                    official_name: "2016. évi CXXX. törvény a polgári perrendtartásról".to_string(),
                    approval_date: "2016-12-19".to_string(),
                    procedural_principles: vec![
                        "Adversarial principle".to_string(),
                        "Oral proceedings".to_string(),
                        "Concentration principle".to_string(),
                        "Judicial case management".to_string(),
                    ],
                    types_of_proceedings: vec![
                        "General proceedings".to_string(),
                        "Simplified proceedings".to_string(),
                        "Small claims procedure".to_string(),
                        "Enforcement proceedings".to_string(),
                    ],
                    appeal_system: vec![
                        "Appeal to regional courts".to_string(),
                        "Review to appellate courts".to_string(),
                        "Extraordinary remedies to Curia".to_string(),
                    ],
                    electronic_procedures: vec![
                        "Electronic filing system (EPER)".to_string(),
                        "Video conference hearings".to_string(),
                        "Digital case management".to_string(),
                    ],
                },
                criminal_procedure_code: CriminalProcedureCode {
                    official_name: "2017. évi XC. törvény a büntetőeljárásról".to_string(),
                    approval_date: "2017-05-30".to_string(),
                    investigation_phase: "Police investigation under prosecutor supervision and control".to_string(),
                    trial_phase: "Public oral trial with adversarial proceedings and presumption of innocence".to_string(),
                    appeal_phase: "Right to appeal with full review of facts and law".to_string(),
                    special_procedures: vec![
                        "Accelerated procedure for flagrant crimes".to_string(),
                        "Plea bargaining procedure".to_string(),
                        "Mediation in criminal matters".to_string(),
                    ],
                },
                administrative_procedure_code: AdministrativeProcedureCode {
                    official_name: "2016. évi CL. törvény az általános közigazgatási rendtartásról".to_string(),
                    approval_date: "2016-12-19".to_string(),
                    general_principles: vec![
                        "Legality".to_string(),
                        "Proportionality".to_string(),
                        "Cooperation".to_string(),
                        "Efficiency".to_string(),
                        "Transparency".to_string(),
                    ],
                    administrative_acts: "Administrative decisions must be reasoned and reviewable".to_string(),
                    procedural_guarantees: vec![
                        "Right to be heard".to_string(),
                        "Right to legal representation".to_string(),
                        "Right to appeal".to_string(),
                        "Right to judicial review".to_string(),
                    ],
                    digital_administration: vec![
                        "Digital by default principle".to_string(),
                        "Electronic communication".to_string(),
                        "Online services portal".to_string(),
                    ],
                },
                labor_code: LaborCode {
                    official_name: "2012. évi I. törvény a munka törvénykönyvéről".to_string(),
                    approval_date: "2012-03-05".to_string(),
                    individual_labor_relations: "Employment contracts, working time, wages, termination procedures".to_string(),
                    collective_labor_relations: "Trade unions, collective bargaining, strikes, workplace representation".to_string(),
                    labor_inspection: "State labor inspection with enforcement powers".to_string(),
                    social_security_integration: "Coordination with social insurance and pension systems".to_string(),
                },
                tax_codes: vec![
                    TaxCode {
                        name: "Personal Income Tax Act".to_string(),
                        tax_type: "Personal Income Tax".to_string(),
                        approval_date: "1995-12-26".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "General income tax rate".to_string(),
                                rate_percentage: 15.0,
                                threshold_amount: 0.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Family tax allowance".to_string(), "Minimum wage exemption".to_string()],
                        collection_procedures: "Monthly withholding with annual declaration".to_string(),
                    },
                    TaxCode {
                        name: "Corporate Tax Act".to_string(),
                        tax_type: "Corporate Income Tax".to_string(),
                        approval_date: "1996-12-17".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Standard corporate rate".to_string(),
                                rate_percentage: 9.0,
                                threshold_amount: 0.0,
                                applicable_from: "2017-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Small enterprise exemption".to_string(), "Development tax allowances".to_string()],
                        collection_procedures: "Annual declaration with quarterly advance payments".to_string(),
                    },
                ],
                family_code: FamilyCode {
                    legal_framework: "Family law integrated in Civil Code Book Four".to_string(),
                    marriage_regulation: "Marriage between man and woman - constitutional definition".to_string(),
                    registered_partnership: "Registered partnership for same-sex couples since 2009".to_string(),
                    parental_authority: "Joint parental authority with child's best interest principle".to_string(),
                    adoption_procedures: "Full adoption with court approval and child welfare assessment".to_string(),
                },
                company_law: CompanyLaw {
                    main_act: "2006. évi IV. törvény a gazdasági társaságokról".to_string(),
                    company_types: vec![
                        "Kft. - Limited Liability Company".to_string(),
                        "Zrt. - Public Limited Company".to_string(),
                        "Bt. - Limited Partnership".to_string(),
                        "Kkt. - General Partnership".to_string(),
                    ],
                    commercial_registration: "Company Court registration with electronic procedures".to_string(),
                    corporate_governance: "Management board and supervisory board system for larger companies".to_string(),
                },
            },
            european_integration: HungarianEuropeanIntegration {
                eu_membership: EUMembership {
                    accession_date: "2004-05-01".to_string(),
                    accession_treaty: "Treaty of Accession 2003".to_string(),
                    referendum_date: "2003-04-12".to_string(),
                    referendum_result: "83.76% in favor".to_string(),
                    membership_benefits: vec![
                        "Single market access".to_string(),
                        "Structural and cohesion funds".to_string(),
                        "Freedom of movement".to_string(),
                        "Agricultural support".to_string(),
                    ],
                    membership_obligations: vec![
                        "EU law implementation".to_string(),
                        "Budget contributions".to_string(),
                        "Policy coordination".to_string(),
                        "Rule of law compliance".to_string(),
                    ],
                },
                eurozone_status: EurozoneStatus {
                    current_currency: "Hungarian forint (HUF)".to_string(),
                    euro_adoption_timeline: "No official timeline - indefinite postponement".to_string(),
                    convergence_criteria: vec![
                        "Inflation criterion not met".to_string(),
                        "Budget deficit criterion challenging".to_string(),
                        "Not in ERM II".to_string(),
                    ],
                    political_position: "Government opposes euro adoption".to_string(),
                },
                schengen_area: SchengenParticipation {
                    implementation_date: "2007-12-21".to_string(),
                    border_controls: "Free movement within Schengen with enhanced external border security".to_string(),
                    police_cooperation: "Enhanced cooperation with focus on migration control".to_string(),
                    information_systems: vec!["SIS II".to_string(), "VIS".to_string(), "EURODAC".to_string()],
                },
                eu_representation: EURepresentation {
                    european_parliament_meps: 21,
                    council_voting_weight: 12,
                    european_commission_commissioner: "Olivér Várhelyi (Neighbourhood and Enlargement)".to_string(),
                    permanent_representation: "Ambassador Bálint Ódor to EU".to_string(),
                },
                eu_legislation_transposition: EULegislationTransposition {
                    transposition_rate: 95.8,
                    infringement_procedures: 35,
                    recent_transpositions: vec![
                        "Whistleblower Protection Directive".to_string(),
                        "Copyright Directive".to_string(),
                    ],
                    pending_transpositions: vec![
                        "AI Act implementation".to_string(),
                        "Digital Services Act compliance".to_string(),
                    ],
                },
                article_7_procedure: Article7Procedure {
                    procedure_status: "Article 7(1) procedure triggered in 2018".to_string(),
                    issues_raised: vec![
                        "Judicial independence concerns".to_string(),
                        "Media freedom restrictions".to_string(),
                        "Academic freedom limitations".to_string(),
                        "NGO restrictions".to_string(),
                        "LGBTI rights concerns".to_string(),
                    ],
                    hungarian_response: vec![
                        "National sovereignty defense".to_string(),
                        "Constitutional identity protection".to_string(),
                        "Christian democratic values preservation".to_string(),
                    ],
                    current_developments: vec![
                        "Rule of Law Mechanism activated".to_string(),
                        "EU funds conditionality applied".to_string(),
                        "European Court of Justice proceedings".to_string(),
                    ],
                },
            },
            regional_administration: HungarianRegionalAdministration {
                county_system: CountySystem {
                    legal_framework: "2011. évi CLXXXIX. törvény Magyarország helyi önkormányzatairól".to_string(),
                    county_competencies: vec![
                        "Regional development coordination".to_string(),
                        "Minority rights protection".to_string(),
                        "Environmental protection".to_string(),
                        "Cultural heritage preservation".to_string(),
                    ],
                    financial_framework: "State support system with limited own revenues".to_string(),
                    coordination_mechanisms: vec![
                        "Regional development councils".to_string(),
                        "Inter-county cooperation agreements".to_string(),
                        "EU funds coordination".to_string(),
                    ],
                },
                regional_development: RegionalDevelopment {
                    strategic_documents: vec![
                        "National Development 2030 Strategy".to_string(),
                        "Territorial Development Concept".to_string(),
                        "Smart Specialization Strategy".to_string(),
                    ],
                    development_priorities: vec![
                        "Economic competitiveness".to_string(),
                        "Infrastructure modernization".to_string(),
                        "Digital transformation".to_string(),
                        "Green transition".to_string(),
                    ],
                    innovation_support: vec![
                        "Innovation clusters".to_string(),
                        "Technology parks".to_string(),
                        "Research and development centers".to_string(),
                    ],
                    infrastructure_projects: vec![
                        "M4 highway completion".to_string(),
                        "5G network development".to_string(),
                        "Renewable energy projects".to_string(),
                    ],
                },
                nuts_regions: vec![
                    NutsRegion {
                        name: "Central Hungary".to_string(),
                        nuts_code: "HU10".to_string(),
                        counties_included: vec!["Budapest".to_string(), "Pest".to_string()],
                        gdp_per_capita: 28_500.0,
                        development_status: "More developed region".to_string(),
                    },
                    NutsRegion {
                        name: "Transdanubia".to_string(),
                        nuts_code: "HU20".to_string(),
                        counties_included: vec!["Fejér".to_string(), "Komárom-Esztergom".to_string(), "Veszprém".to_string()],
                        gdp_per_capita: 18_200.0,
                        development_status: "Transition region".to_string(),
                    },
                    NutsRegion {
                        name: "Great Plain and North".to_string(),
                        nuts_code: "HU30".to_string(),
                        counties_included: vec!["Bács-Kiskun".to_string(), "Békés".to_string(), "Csongrád-Csanád".to_string()],
                        gdp_per_capita: 15_800.0,
                        development_status: "Less developed region".to_string(),
                    },
                ],
                development_programs: vec![
                    DevelopmentProgram {
                        name: "Economic Development and Innovation Programme".to_string(),
                        funding_source: "EU Structural Funds and national co-financing".to_string(),
                        budget: 8_500_000_000.0,
                        objectives: vec!["Innovation enhancement".to_string(), "SME development".to_string()],
                        implementation_period: "2021-2027".to_string(),
                    },
                ],
            },
            local_government: HungarianLocalGovernment {
                municipal_system: MunicipalSystem {
                    legal_framework: "Act CLXXXIX of 2011 on Local Governments".to_string(),
                    municipal_assembly: "Representative body elected by direct vote for 5 years".to_string(),
                    mayor_powers: vec![
                        "Executive authority".to_string(),
                        "Municipal administration leadership".to_string(),
                        "External representation".to_string(),
                        "Emergency powers during crisis".to_string(),
                    ],
                    competencies: vec![
                        "Local public services".to_string(),
                        "Primary and secondary education".to_string(),
                        "Healthcare provision".to_string(),
                        "Social services".to_string(),
                        "Local economic development".to_string(),
                    ],
                    types_of_settlements: vec![
                        "Községek (villages)".to_string(),
                        "Városok (towns)".to_string(),
                        "Megyei jogú városok (cities with county rights)".to_string(),
                        "Főváros (capital city)".to_string(),
                    ],
                },
                local_associations: vec![
                    LocalAssociation {
                        name: "Association of Hungarian Local Authorities".to_string(),
                        legal_form: "Interest representation association".to_string(),
                        purposes: vec!["Local government interests representation".to_string(), "Professional cooperation".to_string()],
                        membership: vec!["Over 3,000 municipalities".to_string()],
                    },
                ],
                inter_municipal_cooperation: InterMunicipalCooperation {
                    cooperation_forms: vec!["Associations".to_string(), "Joint offices".to_string(), "Public service companies".to_string()],
                    joint_offices: vec!["Building authority offices".to_string(), "Social service offices".to_string()],
                    service_associations: vec!["Waste management".to_string(), "Water utilities".to_string()],
                    development_councils: vec!["Micro-regional development councils".to_string(), "Tourism associations".to_string()],
                },
                local_finance: LocalFinance {
                    revenue_sources: vec![
                        "Local business tax".to_string(),
                        "Local property taxes".to_string(),
                        "State normative grants".to_string(),
                        "EU development funds".to_string(),
                    ],
                    normative_financing: "State grants based on mandatory tasks and settlement size".to_string(),
                    grants_and_subsidies: vec!["Task-specific grants".to_string(), "Investment subsidies".to_string(), "EU funds".to_string()],
                    borrowing_rules: "Government authorization required for borrowing above threshold".to_string(),
                },
                citizen_participation: CitizenParticipation {
                    participatory_mechanisms: vec![
                        "Municipal assemblies open sessions".to_string(),
                        "Public consultations".to_string(),
                        "Citizens' forums".to_string(),
                        "Participatory budgeting initiatives".to_string(),
                    ],
                    local_referendums: "Local referendums binding with qualified participation".to_string(),
                    public_consultations: vec!["Strategic planning".to_string(), "Major investment decisions".to_string(), "Urban development".to_string()],
                    civic_initiatives: vec!["Local NGOs".to_string(), "Neighborhood associations".to_string(), "Cultural organizations".to_string()],
                },
            },
            fundamental_law_system: FundamentalLawSystem {
                adoption_process: AdoptionProcess {
                    constitutional_assembly: "Two-thirds majority in single-chamber Parliament".to_string(),
                    adoption_date: "2011-04-25".to_string(),
                    public_consultation: "National Consultation on Constitution preceding adoption".to_string(),
                    political_context: "Adopted by Fidesz supermajority after 2010 electoral victory".to_string(),
                },
                constitutional_identity_doctrine: ConstitutionalIdentityDoctrine {
                    core_elements: vec![
                        "Christian culture as foundation".to_string(),
                        "National sovereignty protection".to_string(),
                        "Traditional family definition".to_string(),
                        "Hungarian nation unity across borders".to_string(),
                    ],
                    protection_mechanisms: vec![
                        "Constitutional identity review".to_string(),
                        "National consultation procedures".to_string(),
                        "Parliamentary supermajority requirements".to_string(),
                    ],
                    eu_law_relations: "Constitutional identity limits EU law supremacy in core areas".to_string(),
                    constitutional_court_jurisprudence: vec![
                        "22/2016. (XII. 5.) AB határozat - EU quota decision".to_string(),
                        "2/2019. (III. 5.) AB határozat - Constitutional identity elements".to_string(),
                    ],
                },
                amendments: vec![
                    ConstitutionalAmendment {
                        amendment_number: "Fourth Amendment".to_string(),
                        date: "2013-03-25".to_string(),
                        subject_matter: "Judicial system reform and constitutional court limitations".to_string(),
                        political_controversy: Some("Opposition criticized as constitutional coup".to_string()),
                    },
                    ConstitutionalAmendment {
                        amendment_number: "Seventh Amendment".to_string(),
                        date: "2018-06-20".to_string(),
                        subject_matter: "Stop Soros provisions and migration restrictions".to_string(),
                        political_controversy: Some("EU criticism for restricting asylum rights".to_string()),
                    },
                    ConstitutionalAmendment {
                        amendment_number: "Ninth Amendment".to_string(),
                        date: "2020-12-15".to_string(),
                        subject_matter: "Child protection and traditional family values".to_string(),
                        political_controversy: Some("LGBTI rights organizations criticized restrictions".to_string()),
                    },
                ],
                interpretation_principles: vec![
                    "Living constitution doctrine".to_string(),
                    "Historical constitution continuity".to_string(),
                    "National constitutional identity".to_string(),
                    "Teleological interpretation".to_string(),
                ],
            },
        }
    }
}