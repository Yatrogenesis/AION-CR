use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete Greek Legal System Implementation
/// Ελληνική Δημοκρατία (Hellenic Republic)
/// Current President: Katerina Sakellaropoulou (2020-2025)
/// Current Prime Minister: Kyriakos Mitsotakis (ND - New Democracy)
/// Current Government: Cabinet of Kyriakos Mitsotakis (Second Term)
/// EU Member since 1981, Eurozone since 2001, Schengen since 2000

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreeceLegalSystem {
    pub constitutional_framework: GreekConstitution,
    pub government_structure: GreekGovernment,
    pub territorial_organization: GreekTerritorialOrganization,
    pub judicial_system: GreekJudicialSystem,
    pub legal_codes: GreekLegalCodes,
    pub european_integration: GreekEuropeanIntegration,
    pub regional_administration: GreekRegionalAdministration,
    pub local_government: GreekLocalGovernment,
    pub orthodox_church_status: GreekOrthodoxChurchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekConstitution {
    pub name: String,
    pub adoption_date: String,
    pub last_revision: String,
    pub total_articles: u32,
    pub fundamental_principles: Vec<ConstitutionalPrinciple>,
    pub rights_and_duties: Vec<ConstitutionalRight>,
    pub economic_system: EconomicConstitutionalFramework,
    pub key_articles: HashMap<String, String>,
    pub constitutional_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalPrinciple {
    pub article: String,
    pub title: String,
    pub content_greek: String,
    pub content_english: String,
    pub interpretation: String,
    pub historical_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstitutionalRight {
    pub article: String,
    pub category: String,
    pub right_name: String,
    pub content_greek: String,
    pub limitations: Vec<String>,
    pub jurisprudence: Vec<String>,
    pub european_charter_alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EconomicConstitutionalFramework {
    pub economic_system_type: String,
    pub property_rights: String,
    pub social_market_principles: Vec<String>,
    pub state_intervention_limits: Vec<String>,
    pub public_sector_definition: String,
    pub economic_development_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekGovernment {
    pub president: PresidentOfRepublic,
    pub prime_minister: PrimeMinister,
    pub council_of_ministers: CouncilOfMinisters,
    pub hellenic_parliament: HellenicParliament,
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
    pub previous_positions: Vec<String>,
    pub government_program: Vec<String>,
    pub coalition_agreements: Vec<String>,
    pub reforms_agenda: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilOfMinisters {
    pub formation_date: String,
    pub total_ministers: u32,
    pub ministries: Vec<Ministry>,
    pub deputy_ministers: Vec<DeputyMinister>,
    pub government_spokesperson: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ministry {
    pub name: String,
    pub minister_name: String,
    pub portfolio_areas: Vec<String>,
    pub budget_2024: f64,
    pub staff_count: u32,
    pub main_headquarters: String,
    pub regional_offices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeputyMinister {
    pub name: String,
    pub ministry: String,
    pub specific_area: String,
    pub appointment_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HellenicParliament {
    pub seats_total: u32,
    pub current_composition: HashMap<String, u32>,
    pub speaker: String,
    pub deputy_speakers: Vec<String>,
    pub parliamentary_groups: Vec<ParliamentaryGroup>,
    pub current_session: String,
    pub electoral_system: String,
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
pub struct GreekTerritorialOrganization {
    pub mainland_greece: MainlandGreece,
    pub islands: Vec<GreekIsland>,
    pub regions: Vec<Region>,
    pub regional_units: Vec<RegionalUnit>,
    pub municipalities: Vec<Municipality>,
    pub territorial_statistics: TerritorialStatistics,
    pub administrative_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MainlandGreece {
    pub total_area_km2: f64,
    pub population: u64,
    pub geographic_regions: Vec<GeographicRegion>,
    pub major_cities: Vec<MajorCity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeographicRegion {
    pub name: String,
    pub area_km2: f64,
    pub population: u64,
    pub main_cities: Vec<String>,
    pub economic_characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MajorCity {
    pub name: String,
    pub population: u64,
    pub region: String,
    pub economic_role: String,
    pub historical_significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekIsland {
    pub name: String,
    pub island_group: String,
    pub area_km2: f64,
    pub population: u64,
    pub main_town: String,
    pub economic_activities: Vec<String>,
    pub special_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Region {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
    pub regional_units_count: u32,
    pub regional_governor: String,
    pub regional_council: RegionalCouncil,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalCouncil {
    pub total_members: u32,
    pub political_composition: HashMap<String, u32>,
    pub current_term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalUnit {
    pub name: String,
    pub capital: String,
    pub region: String,
    pub area_km2: f64,
    pub population: u64,
    pub municipalities_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Municipality {
    pub name: String,
    pub regional_unit: String,
    pub area_km2: f64,
    pub population: u64,
    pub mayor: String,
    pub mayor_party: String,
    pub municipal_council_seats: u32,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerritorialStatistics {
    pub total_area_km2: f64,
    pub total_population: u64,
    pub population_density: f64,
    pub regions_total: u32,
    pub regional_units_total: u32,
    pub municipalities_total: u32,
    pub islands_inhabited: u32,
    pub coastline_km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekJudicialSystem {
    pub supreme_courts: SupremeCourts,
    pub council_of_state: CouncilOfState,
    pub court_of_auditors: CourtOfAuditors,
    pub special_supreme_court: SpecialSupremeCourt,
    pub courts_of_appeal: Vec<CourtOfAppeal>,
    pub first_instance_courts: Vec<FirstInstanceCourt>,
    pub prosecution_service: ProsecutionService,
    pub judicial_independence: JudicialIndependence,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupremeCourts {
    pub areios_pagos: AreioPagos,
    pub civil_chamber: String,
    pub criminal_chamber: String,
    pub landmark_decisions: Vec<LandmarkDecision>,
    pub headquarters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AreioPagos {
    pub president: String,
    pub deputy_president: String,
    pub justices_count: u32,
    pub sections: Vec<String>,
    pub competencies: Vec<String>,
    pub historical_significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilOfState {
    pub president: String,
    pub vice_president: String,
    pub councillors_count: u32,
    pub sections: Vec<String>,
    pub competencies: Vec<String>,
    pub recent_decisions: Vec<AdministrativeDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourtOfAuditors {
    pub president: String,
    pub councillors_count: u32,
    pub audit_competencies: Vec<String>,
    pub annual_reports: Vec<String>,
    pub public_finance_oversight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpecialSupremeCourt {
    pub president: String,
    pub composition: String,
    pub competencies: Vec<String>,
    pub election_disputes: String,
    pub constitutional_interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourtOfAppeal {
    pub name: String,
    pub jurisdiction: String,
    pub location: String,
    pub judges_count: u32,
    pub annual_cases: u32,
    pub specializations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirstInstanceCourt {
    pub name: String,
    pub jurisdiction: String,
    pub location: String,
    pub judges_count: u32,
    pub court_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProsecutionService {
    pub prosecutor_supreme_court: String,
    pub prosecutors_appeal: Vec<String>,
    pub prosecutors_first_instance: Vec<String>,
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
    pub decision_number: String,
    pub date: String,
    pub subject: String,
    pub legal_reasoning: String,
    pub administrative_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekLegalCodes {
    pub civil_code: CivilCode,
    pub criminal_code: CriminalCode,
    pub civil_procedure_code: CivilProcedureCode,
    pub criminal_procedure_code: CriminalProcedureCode,
    pub administrative_procedure_code: AdministrativeProcedureCode,
    pub labor_legislation: LaborLegislation,
    pub tax_codes: Vec<TaxCode>,
    pub commercial_law: CommercialLaw,
    pub family_law: FamilyLaw,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CivilCode {
    pub official_name: String,
    pub approval_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub books: Vec<CodeBook>,
    pub key_principles: Vec<String>,
    pub german_influence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CriminalCode {
    pub official_name: String,
    pub approval_date: String,
    pub last_amendment: String,
    pub total_articles: u32,
    pub parts: Vec<CodePart>,
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
    pub digitalization_reforms: Vec<String>,
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
pub struct LaborLegislation {
    pub main_law: String,
    pub approval_date: String,
    pub individual_labor_relations: String,
    pub collective_labor_relations: String,
    pub labor_courts: String,
    pub social_security_integration: String,
    pub recent_reforms: Vec<String>,
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
pub struct CommercialLaw {
    pub company_law: String,
    pub company_types: Vec<String>,
    pub commercial_transactions: String,
    pub insolvency_law: String,
    pub capital_markets_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilyLaw {
    pub marriage_law: String,
    pub civil_partnership: String,
    pub parental_authority: String,
    pub adoption_procedures: String,
    pub inheritance_law: String,
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
pub struct GreekEuropeanIntegration {
    pub eu_membership: EUMembership,
    pub eurozone_participation: EurozoneParticipation,
    pub schengen_area: SchengenParticipation,
    pub eu_representation: EURepresentation,
    pub eu_legislation_transposition: EULegislationTransposition,
    pub economic_adjustment_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EUMembership {
    pub accession_date: String,
    pub accession_treaty: String,
    pub membership_benefits: Vec<String>,
    pub membership_obligations: Vec<String>,
    pub structural_funds_received: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EurozoneParticipation {
    pub euro_adoption_date: String,
    pub drachma_exchange_rate: String,
    pub ecb_representation: String,
    pub fiscal_rules_compliance: Vec<String>,
    pub economic_surveillance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchengenParticipation {
    pub implementation_date: String,
    pub border_controls: String,
    pub migration_challenges: Vec<String>,
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
pub struct GreekRegionalAdministration {
    pub administrative_regions: Vec<AdministrativeRegion>,
    pub decentralized_administrations: Vec<DecentralizedAdministration>,
    pub regional_development: RegionalDevelopment,
    pub kallikratis_plan: KallikratisPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdministrativeRegion {
    pub name: String,
    pub capital: String,
    pub area_km2: f64,
    pub population: u64,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecentralizedAdministration {
    pub name: String,
    pub regions_covered: Vec<String>,
    pub secretary_general: String,
    pub headquarters: String,
    pub competencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionalDevelopment {
    pub development_strategies: Vec<String>,
    pub eu_funding_programs: Vec<String>,
    pub infrastructure_projects: Vec<String>,
    pub innovation_ecosystems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KallikratisPlan {
    pub implementation_date: String,
    pub objectives: Vec<String>,
    pub territorial_reforms: Vec<String>,
    pub administrative_simplification: Vec<String>,
    pub financial_efficiency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekLocalGovernment {
    pub municipal_system: MunicipalSystem,
    pub local_communities: Vec<LocalCommunity>,
    pub metropolitan_organizations: Vec<MetropolitanOrganization>,
    pub local_finance: LocalFinance,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MunicipalSystem {
    pub legal_framework: String,
    pub municipal_council: String,
    pub mayor_powers: Vec<String>,
    pub competencies: Vec<String>,
    pub financial_autonomy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalCommunity {
    pub name: String,
    pub municipality: String,
    pub population: u64,
    pub president: String,
    pub local_council: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetropolitanOrganization {
    pub name: String,
    pub member_municipalities: Vec<String>,
    pub governance_structure: String,
    pub competencies: Vec<String>,
    pub budget_2024: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalFinance {
    pub revenue_sources: Vec<String>,
    pub transfer_system: String,
    pub borrowing_rules: String,
    pub fiscal_supervision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CitizenParticipation {
    pub participatory_mechanisms: Vec<String>,
    pub digital_democracy: Vec<String>,
    pub local_referendums: String,
    pub citizen_assemblies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GreekOrthodoxChurchStatus {
    pub constitutional_status: String,
    pub church_state_relations: ChurchStateRelations,
    pub ecclesiastical_hierarchy: EcclesiasticalHierarchy,
    pub legal_framework: String,
    pub property_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChurchStateRelations {
    pub constitutional_article: String,
    pub prevailing_religion: String,
    pub state_support: Vec<String>,
    pub religious_education: String,
    pub clergy_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcclesiasticalHierarchy {
    pub archbishop_athens: String,
    pub holy_synod: String,
    pub metropolitans: Vec<String>,
    pub dioceses_count: u32,
    pub monastic_communities: Vec<String>,
}

impl Default for GreeceLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: GreekConstitution {
                name: "Σύνταγμα της Ελλάδας (Constitution of Greece)".to_string(),
                adoption_date: "1975-06-11".to_string(),
                last_revision: "2019-11-25".to_string(),
                total_articles: 120,
                fundamental_principles: vec![
                    ConstitutionalPrinciple {
                        article: "Άρθρο 1".to_string(),
                        title: "Μορφή του πολιτεύματος".to_string(),
                        content_greek: "Το πολίτευμα της Ελλάδας είναι Προεδρευόμενη Κοινοβουλευτική Δημοκρατία. Η λαϊκή κυριαρχία είναι το θεμέλιο του πολιτεύματος. Πάσα εξουσία πηγάζει από το λαό και υπάρχει για αυτόν και το έθνος· ασκείται όπως ορίζει το Σύνταγμα.".to_string(),
                        content_english: "The form of government of Greece is that of a parliamentary republic. Popular sovereignty is the foundation of government. All powers derive from the People and exist for the People and the Nation; they are exercised as specified by the Constitution.".to_string(),
                        interpretation: "Establishes Greece as a parliamentary republic based on popular sovereignty".to_string(),
                        historical_context: "Adopted after the fall of the military junta in 1974".to_string(),
                    },
                    ConstitutionalPrinciple {
                        article: "Άρθρο 2".to_string(),
                        title: "Σεβασμός του ανθρώπου".to_string(),
                        content_greek: "Ο σεβασμός και η προστασία της αξίας του ανθρώπου αποτελούν την πρωταρχική υποχρέωση του Κράτους. Η Ελλάδα, τηρώντας τους κανόνες του διεθνούς δικαίου, επιδιώκει την ενίσχυση της ειρήνης και της δικαιοσύνης και την ανάπτυξη φιλικών σχέσεων μεταξύ των λαών και των κρατών.".to_string(),
                        content_english: "Respect and protection of the value of the human being constitute the primary obligation of the State. Greece, observing the rules of international law, pursues the strengthening of peace and justice and the development of friendly relations between peoples and states.".to_string(),
                        interpretation: "Fundamental principle of human dignity and international cooperation".to_string(),
                        historical_context: "Reflects post-dictatorship commitment to human rights".to_string(),
                    },
                    ConstitutionalPrinciple {
                        article: "Άρθρο 3".to_string(),
                        title: "Θρησκευτικές σχέσεις".to_string(),
                        content_greek: "Επικρατούσα θρησκεία στην Ελλάδα είναι η θρησκεία της Ανατολικής Ορθόδοξης Εκκλησίας του Χριστού. Η Ορθόδοξη Εκκλησία της Ελλάδας, που γνωρίζει κεφαλή της τον Κύριο ημών Ιησού Χριστό, υπάρχει αδιαλείπτως ενωμένη δογματικά με τη Μεγάλη Εκκλησία της Κωνσταντινουπόλεως και με πάσαν άλλην ομόδοξον Εκκλησίαν του Χριστού, τηρεί απαρασαλεύτως, όπως εκείναι, τους ιερούς αποστολικούς και συνοδικούς κανόνες και τις ιερές παραδόσεις.".to_string(),
                        content_english: "The prevailing religion in Greece is that of the Eastern Orthodox Church of Christ. The Orthodox Church of Greece, acknowledging our Lord Jesus Christ as its head, is inseparably united in doctrine with the Great Church of Christ in Constantinople and with every other Orthodox Church of Christ, observes unwaveringly, as they do, the holy apostolic and synodal canons and the holy traditions.".to_string(),
                        interpretation: "Establishes Orthodox Christianity as the prevailing religion while maintaining religious freedom".to_string(),
                        historical_context: "Reflects the historical role of Orthodox Church in Greek national identity".to_string(),
                    },
                ],
                rights_and_duties: vec![
                    ConstitutionalRight {
                        article: "Άρθρο 4".to_string(),
                        category: "Ισότητα ενώπιον του νόμου".to_string(),
                        right_name: "Equality before the law".to_string(),
                        content_greek: "Όλοι οι Έλληνες είναι ίσοι ενώπιον του νόμου. Οι Έλληνες άνδρες και γυναίκες έχουν ίσα δικαιώματα και ίσες υποχρεώσεις.".to_string(),
                        limitations: vec!["No limitations on fundamental equality principle".to_string()],
                        jurisprudence: vec!["Council of State decisions on gender equality".to_string()],
                        european_charter_alignment: "Fully aligned with EU Charter of Fundamental Rights Article 20".to_string(),
                    },
                    ConstitutionalRight {
                        article: "Άρθρο 5".to_string(),
                        category: "Προσωπική ελευθερία".to_string(),
                        right_name: "Personal freedom".to_string(),
                        content_greek: "Καθένας έχει δικαίωμα στην ελεύθερη ανάπτυξη της προσωπικότητάς του και στη συμμετοχή στην κοινωνική, οικονομική και πολιτική ζωή της χώρας, εφόσον δεν παραβλάπτει τα δικαιώματα των άλλων και δεν παραβιάζει το Σύνταγμα και τα χρηστά ήθη.".to_string(),
                        limitations: vec!["Must not harm others' rights".to_string(), "Must respect Constitution and moral standards".to_string()],
                        jurisprudence: vec!["Supreme Court decisions on personal autonomy".to_string()],
                        european_charter_alignment: "Consistent with EU fundamental rights framework".to_string(),
                    },
                ],
                economic_system: EconomicConstitutionalFramework {
                    economic_system_type: "Social market economy with state intervention capabilities".to_string(),
                    property_rights: "Private property protected with social function limitations".to_string(),
                    social_market_principles: vec![
                        "Free market with social corrections".to_string(),
                        "State regulation of economic activity".to_string(),
                        "Social protection guarantees".to_string(),
                    ],
                    state_intervention_limits: vec![
                        "Public interest justification required".to_string(),
                        "Proportionality principle".to_string(),
                        "Compensation for expropriation".to_string(),
                    ],
                    public_sector_definition: "State and public entities economic activities".to_string(),
                    economic_development_goals: vec![
                        "Sustainable development".to_string(),
                        "Social cohesion".to_string(),
                        "Environmental protection".to_string(),
                    ],
                },
                key_articles: {
                    let mut articles = HashMap::new();
                    articles.insert("Article_1".to_string(), "Το πολίτευμα της Ελλάδας είναι Προεδρευόμενη Κοινοβουλευτική Δημοκρατία".to_string());
                    articles.insert("Article_2".to_string(), "Σεβασμός και προστασία της αξίας του ανθρώπου".to_string());
                    articles.insert("Article_3".to_string(), "Επικρατούσα θρησκεία η Ανατολική Ορθόδοξη Εκκλησία".to_string());
                    articles.insert("Article_25".to_string(), "Δικαιώματα του ανθρώπου ως θεμέλιο του πολιτεύματος".to_string());
                    articles
                },
                constitutional_history: vec![
                    "Constitution of 1822 - First Greek Constitution".to_string(),
                    "Constitution of 1844 - Constitutional Monarchy".to_string(),
                    "Constitution of 1864 - Democratic reforms".to_string(),
                    "Constitution of 1911 - Venizelos reforms".to_string(),
                    "Constitution of 1927 - Republican Constitution".to_string(),
                    "Constitution of 1952 - Post-war reconstruction".to_string(),
                    "Constitution of 1975 - Post-junta democratic restoration".to_string(),
                ],
            },
            government_structure: GreekGovernment {
                president: PresidentOfRepublic {
                    name: "Katerina Sakellaropoulou".to_string(),
                    term_start: "2020-03-13".to_string(),
                    term_end: "2025-03-13".to_string(),
                    election_date: "2020-01-22".to_string(),
                    parliamentary_votes: 261,
                    previous_occupation: "President of the Council of State, Environmental lawyer".to_string(),
                    constitutional_powers: vec![
                        "Appoint Prime Minister".to_string(),
                        "Dissolve Parliament".to_string(),
                        "Promulgate laws".to_string(),
                        "Supreme commander of Armed Forces".to_string(),
                        "International representation".to_string(),
                    ],
                    residence: "Presidential Mansion, Athens".to_string(),
                },
                prime_minister: PrimeMinister {
                    name: "Kyriakos Mitsotakis".to_string(),
                    party: "ND - Nea Dimokratia (New Democracy)".to_string(),
                    appointment_date: "2019-07-08".to_string(),
                    previous_positions: vec![
                        "Leader of New Democracy (2016-present)".to_string(),
                        "Minister of Administrative Reform (2013-2015)".to_string(),
                        "Member of Parliament for Chania (2004-present)".to_string(),
                        "McKinsey & Company consultant".to_string(),
                    ],
                    government_program: vec![
                        "Digital transformation and modernization".to_string(),
                        "Economic growth and competitiveness".to_string(),
                        "Climate change action".to_string(),
                        "Migration management".to_string(),
                        "Health system strengthening".to_string(),
                    ],
                    coalition_agreements: vec![
                        "Single-party government since 2019".to_string(),
                        "Parliamentary majority with 158 seats".to_string(),
                    ],
                    reforms_agenda: vec![
                        "Digital governance reform".to_string(),
                        "Tax system simplification".to_string(),
                        "Labor market flexibility".to_string(),
                        "Judicial system modernization".to_string(),
                    ],
                },
                council_of_ministers: CouncilOfMinisters {
                    formation_date: "2023-06-25".to_string(),
                    total_ministers: 20,
                    ministries: vec![
                        Ministry {
                            name: "Ministry of Interior".to_string(),
                            minister_name: "Makis Voridis".to_string(),
                            portfolio_areas: vec!["Local government".to_string(), "Public administration".to_string(), "Elections".to_string()],
                            budget_2024: 2_800_000_000.0,
                            staff_count: 45_000,
                            main_headquarters: "Stadiou Street, Athens".to_string(),
                            regional_offices: vec!["Thessaloniki".to_string(), "Patras".to_string(), "Heraklion".to_string()],
                        },
                        Ministry {
                            name: "Ministry of Finance".to_string(),
                            minister_name: "Kostis Hatzidakis".to_string(),
                            portfolio_areas: vec!["Public finances".to_string(), "Tax policy".to_string(), "Budget".to_string()],
                            budget_2024: 4_200_000_000.0,
                            staff_count: 28_000,
                            main_headquarters: "Karageorgi Servias Street, Athens".to_string(),
                            regional_offices: vec!["All major cities".to_string()],
                        },
                        Ministry {
                            name: "Ministry of National Defense".to_string(),
                            minister_name: "Nikos Dendias".to_string(),
                            portfolio_areas: vec!["Armed Forces".to_string(), "Defense policy".to_string(), "Defense industry".to_string()],
                            budget_2024: 7_500_000_000.0,
                            staff_count: 140_000,
                            main_headquarters: "Holargos, Athens".to_string(),
                            regional_offices: vec!["Military commands nationwide".to_string()],
                        },
                    ],
                    deputy_ministers: vec![
                        DeputyMinister {
                            name: "Thanos Plevris".to_string(),
                            ministry: "Ministry of Health".to_string(),
                            specific_area: "Mental Health".to_string(),
                            appointment_date: "2023-06-25".to_string(),
                        },
                    ],
                    government_spokesperson: "Pavlos Marinakis".to_string(),
                },
                hellenic_parliament: HellenicParliament {
                    seats_total: 300,
                    current_composition: {
                        let mut composition = HashMap::new();
                        composition.insert("ND".to_string(), 158);
                        composition.insert("SYRIZA".to_string(), 47);
                        composition.insert("PASOK".to_string(), 32);
                        composition.insert("KKE".to_string(), 20);
                        composition.insert("Elliniki_Lysi".to_string(), 16);
                        composition.insert("Niki".to_string(), 12);
                        composition.insert("Plefsi_Eleftherias".to_string(), 10);
                        composition.insert("MeRA25".to_string(), 5);
                        composition
                    },
                    speaker: "Konstantinos Tasoulas".to_string(),
                    deputy_speakers: vec![
                        "Thanasis Bouras".to_string(),
                        "Charalambos Athanasiou".to_string(),
                        "Anna-Michelle Asimakopoulou".to_string(),
                    ],
                    parliamentary_groups: vec![
                        ParliamentaryGroup {
                            party_name: "Nea Dimokratia".to_string(),
                            leader: "Kyriakos Mitsotakis".to_string(),
                            seats: 158,
                            political_orientation: "Liberal Conservative".to_string(),
                            founded: "1974".to_string(),
                            european_affiliation: "European People's Party".to_string(),
                        },
                        ParliamentaryGroup {
                            party_name: "SYRIZA - Progressi Symmachia".to_string(),
                            leader: "Stefanos Kasselakis".to_string(),
                            seats: 47,
                            political_orientation: "Democratic Socialist".to_string(),
                            founded: "2004".to_string(),
                            european_affiliation: "The Left in the European Parliament".to_string(),
                        },
                    ],
                    current_session: "XVIII Parliamentary Period (2023-2027)".to_string(),
                    electoral_system: "Reinforced proportionality system with 50-seat bonus".to_string(),
                },
                current_legislature: "2nd Mitsotakis Government".to_string(),
                government_formation: "New Democracy single-party government with parliamentary majority".to_string(),
                political_parties: vec![
                    PoliticalParty {
                        name: "Nea Dimokratia".to_string(),
                        abbreviation: "ND".to_string(),
                        leader: "Kyriakos Mitsotakis".to_string(),
                        founded: "1974-10-04".to_string(),
                        ideology: vec!["Liberal conservatism".to_string(), "Economic liberalism".to_string(), "Pro-European".to_string()],
                        membership: 180_000,
                        headquarters: "Rigillis Street, Athens".to_string(),
                        european_party_family: "European People's Party".to_string(),
                    },
                    PoliticalParty {
                        name: "SYRIZA - Progressi Symmachia".to_string(),
                        abbreviation: "SYRIZA".to_string(),
                        leader: "Stefanos Kasselakis".to_string(),
                        founded: "2004-01-15".to_string(),
                        ideology: vec!["Democratic socialism".to_string(), "Left-wing populism".to_string(), "Anti-austerity".to_string()],
                        membership: 110_000,
                        headquarters: "Koumoundourou Square, Athens".to_string(),
                        european_party_family: "Party of the European Left".to_string(),
                    },
                ],
            },
            territorial_organization: GreekTerritorialOrganization {
                mainland_greece: MainlandGreece {
                    total_area_km2: 107_340.0,
                    population: 8_718_156,
                    geographic_regions: vec![
                        GeographicRegion {
                            name: "Macedonia".to_string(),
                            area_km2: 34_177.0,
                            population: 2_406_393,
                            main_cities: vec!["Thessaloniki".to_string(), "Kavala".to_string(), "Serres".to_string()],
                            economic_characteristics: vec!["Industry".to_string(), "Agriculture".to_string(), "Trade".to_string()],
                        },
                        GeographicRegion {
                            name: "Attica".to_string(),
                            area_km2: 3_808.0,
                            population: 3_812_330,
                            main_cities: vec!["Athens".to_string(), "Piraeus".to_string(), "Marousi".to_string()],
                            economic_characteristics: vec!["Services".to_string(), "Tourism".to_string(), "Finance".to_string()],
                        },
                    ],
                    major_cities: vec![
                        MajorCity {
                            name: "Athens".to_string(),
                            population: 3_168_846,
                            region: "Attica".to_string(),
                            economic_role: "Capital and main economic center".to_string(),
                            historical_significance: "Birthplace of democracy, ancient Greek civilization".to_string(),
                        },
                        MajorCity {
                            name: "Thessaloniki".to_string(),
                            population: 824_676,
                            region: "Central Macedonia".to_string(),
                            economic_role: "Northern Greece economic hub".to_string(),
                            historical_significance: "Byzantine heritage, commercial center".to_string(),
                        },
                    ],
                },
                islands: vec![
                    GreekIsland {
                        name: "Crete".to_string(),
                        island_group: "Crete".to_string(),
                        area_km2: 8_336.0,
                        population: 623_065,
                        main_town: "Heraklion".to_string(),
                        economic_activities: vec!["Tourism".to_string(), "Agriculture".to_string(), "Olive oil".to_string()],
                        special_status: Some("Largest Greek island".to_string()),
                    },
                    GreekIsland {
                        name: "Euboea".to_string(),
                        island_group: "Sporades".to_string(),
                        area_km2: 3_670.0,
                        population: 191_206,
                        main_town: "Chalcis".to_string(),
                        economic_activities: vec!["Industry".to_string(), "Agriculture".to_string(), "Tourism".to_string()],
                        special_status: Some("Second largest Greek island".to_string()),
                    },
                ],
                regions: vec![
                    Region {
                        name: "Attica".to_string(),
                        capital: "Athens".to_string(),
                        area_km2: 3_808.0,
                        population: 3_812_330,
                        regional_units_count: 8,
                        regional_governor: "Nikos Hardalias".to_string(),
                        regional_council: RegionalCouncil {
                            total_members: 51,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("ND".to_string(), 28);
                                composition.insert("SYRIZA".to_string(), 12);
                                composition.insert("PASOK".to_string(), 8);
                                composition.insert("Others".to_string(), 3);
                                composition
                            },
                            current_term: "2023-2028".to_string(),
                        },
                        budget_2024: 2_100_000_000.0,
                    },
                    Region {
                        name: "Central Macedonia".to_string(),
                        capital: "Thessaloniki".to_string(),
                        area_km2: 18_811.0,
                        population: 1_882_108,
                        regional_units_count: 7,
                        regional_governor: "Apostolos Tzitzikostas".to_string(),
                        regional_council: RegionalCouncil {
                            total_members: 51,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("ND".to_string(), 32);
                                composition.insert("SYRIZA".to_string(), 11);
                                composition.insert("PASOK".to_string(), 6);
                                composition.insert("Others".to_string(), 2);
                                composition
                            },
                            current_term: "2023-2028".to_string(),
                        },
                        budget_2024: 1_800_000_000.0,
                    },
                ],
                regional_units: vec![
                    RegionalUnit {
                        name: "Central Athens".to_string(),
                        capital: "Athens".to_string(),
                        region: "Attica".to_string(),
                        area_km2: 361.0,
                        population: 2_664_776,
                        municipalities_count: 7,
                    },
                    RegionalUnit {
                        name: "Thessaloniki".to_string(),
                        capital: "Thessaloniki".to_string(),
                        region: "Central Macedonia".to_string(),
                        area_km2: 3_683.0,
                        population: 1_110_551,
                        municipalities_count: 14,
                    },
                ],
                municipalities: vec![
                    Municipality {
                        name: "Athens".to_string(),
                        regional_unit: "Central Athens".to_string(),
                        area_km2: 38.96,
                        population: 664_046,
                        mayor: "Kostas Bakoyannis".to_string(),
                        mayor_party: "ND".to_string(),
                        municipal_council_seats: 69,
                        budget_2024: 580_000_000.0,
                    },
                    Municipality {
                        name: "Thessaloniki".to_string(),
                        regional_unit: "Thessaloniki".to_string(),
                        area_km2: 19.31,
                        population: 315_196,
                        mayor: "Konstantinos Zervas".to_string(),
                        mayor_party: "Independent".to_string(),
                        municipal_council_seats: 45,
                        budget_2024: 320_000_000.0,
                    },
                ],
                territorial_statistics: TerritorialStatistics {
                    total_area_km2: 131_957.0,
                    total_population: 10_413_982,
                    population_density: 78.9,
                    regions_total: 13,
                    regional_units_total: 74,
                    municipalities_total: 332,
                    islands_inhabited: 227,
                    coastline_km: 13_676.0,
                },
                administrative_reforms: vec![
                    "Kapodistrias Plan (1997) - Municipal consolidation".to_string(),
                    "Kallikratis Plan (2010) - Regional and local government reform".to_string(),
                    "Kleisthenes Plan (2019) - Further administrative modernization".to_string(),
                ],
            },
            judicial_system: GreekJudicialSystem {
                supreme_courts: SupremeCourts {
                    areios_pagos: AreioPagos {
                        president: "Vassiliki Thanou".to_string(),
                        deputy_president: "Dimitrios Kaiafa-Gbandi".to_string(),
                        justices_count: 73,
                        sections: vec!["Civil".to_string(), "Criminal".to_string()],
                        competencies: vec![
                            "Final appeals in civil matters".to_string(),
                            "Criminal cassation appeals".to_string(),
                            "Jurisdictional conflicts resolution".to_string(),
                        ],
                        historical_significance: "Named after ancient Athenian court, established 1834".to_string(),
                    },
                    civil_chamber: "Handles civil and commercial law appeals".to_string(),
                    criminal_chamber: "Handles criminal law appeals and cassation".to_string(),
                    landmark_decisions: vec![
                        LandmarkDecision {
                            case_number: "AP 1256/2019".to_string(),
                            date: "2019-05-15".to_string(),
                            court: "Areios Pagos".to_string(),
                            subject: "Constitutional interpretation of property rights in crisis".to_string(),
                            legal_principle: "Property rights must be balanced with public interest during economic emergency".to_string(),
                            impact: "Shaped understanding of constitutional rights during economic crisis".to_string(),
                        },
                    ],
                    headquarters: "Alexandras Avenue, Athens".to_string(),
                },
                council_of_state: CouncilOfState {
                    president: "Aikaterini Sakellaropoulou (former, now President of Republic)".to_string(),
                    vice_president: "Michail Pikramenos".to_string(),
                    councillors_count: 60,
                    sections: vec![
                        "Administrative Law".to_string(),
                        "Constitutional Review".to_string(),
                        "Legislative Review".to_string(),
                    ],
                    competencies: vec![
                        "Administrative disputes".to_string(),
                        "Constitutional review of laws".to_string(),
                        "Electoral disputes".to_string(),
                        "Advisory opinions on legislation".to_string(),
                    ],
                    recent_decisions: vec![
                        AdministrativeDecision {
                            decision_number: "STE 2045/2023".to_string(),
                            date: "2023-04-20".to_string(),
                            subject: "Digital governance and data protection compliance".to_string(),
                            legal_reasoning: "Administrative acts must comply with GDPR and constitutional privacy rights".to_string(),
                            administrative_impact: "Shaped digital transformation of public administration".to_string(),
                        },
                    ],
                },
                court_of_auditors: CourtOfAuditors {
                    president: "Georgios Zournatzis".to_string(),
                    councillors_count: 45,
                    audit_competencies: vec![
                        "State budget execution audit".to_string(),
                        "Public entities financial control".to_string(),
                        "EU funds management oversight".to_string(),
                        "Public procurement compliance".to_string(),
                    ],
                    annual_reports: vec![
                        "Annual Report on Budget Execution 2023".to_string(),
                        "EU Funds Management Report 2023".to_string(),
                    ],
                    public_finance_oversight: "Constitutional mandate for public finance oversight and accountability".to_string(),
                },
                special_supreme_court: SpecialSupremeCourt {
                    president: "Rotation between highest courts".to_string(),
                    composition: "11 members from highest courts and university professors".to_string(),
                    competencies: vec![
                        "Conflicts between highest courts".to_string(),
                        "Election disputes final resolution".to_string(),
                        "Constitutional interpretation conflicts".to_string(),
                    ],
                    election_disputes: "Final authority on election validity and parliamentary seat disputes".to_string(),
                    constitutional_interpretation: "Resolves conflicts between different constitutional interpretations".to_string(),
                },
                courts_of_appeal: vec![
                    CourtOfAppeal {
                        name: "Athens Court of Appeal".to_string(),
                        jurisdiction: "Attica Region".to_string(),
                        location: "Athens".to_string(),
                        judges_count: 180,
                        annual_cases: 25_640,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string(), "Commercial".to_string()],
                    },
                    CourtOfAppeal {
                        name: "Thessaloniki Court of Appeal".to_string(),
                        jurisdiction: "Northern Greece".to_string(),
                        location: "Thessaloniki".to_string(),
                        judges_count: 95,
                        annual_cases: 14_230,
                        specializations: vec!["Civil".to_string(), "Criminal".to_string()],
                    },
                ],
                first_instance_courts: vec![
                    FirstInstanceCourt {
                        name: "Athens First Instance Court".to_string(),
                        jurisdiction: "Athens metropolitan area".to_string(),
                        location: "Athens".to_string(),
                        judges_count: 320,
                        court_types: vec!["General jurisdiction".to_string(), "Specialized chambers".to_string()],
                    },
                ],
                prosecution_service: ProsecutionService {
                    prosecutor_supreme_court: "Isidoros Ntogias".to_string(),
                    prosecutors_appeal: vec!["Regional prosecutors in major cities".to_string()],
                    prosecutors_first_instance: vec!["District prosecutors nationwide".to_string()],
                    structure: vec![
                        "Hierarchical organization".to_string(),
                        "Functional independence".to_string(),
                        "Specialized units".to_string(),
                    ],
                    competencies: vec![
                        "Criminal prosecution".to_string(),
                        "Public interest protection".to_string(),
                        "Anti-corruption investigations".to_string(),
                    ],
                },
                judicial_independence: JudicialIndependence {
                    constitutional_guarantees: vec![
                        "Life tenure until retirement".to_string(),
                        "Salary protection".to_string(),
                        "Immunity from prosecution for judicial acts".to_string(),
                    ],
                    judicial_council: "Supreme Judicial Council oversees career and discipline".to_string(),
                    appointment_procedures: "Competitive examinations and merit-based promotion".to_string(),
                    disciplinary_system: "Judicial council disciplinary procedures with appeal rights".to_string(),
                },
            },
            legal_codes: GreekLegalCodes {
                civil_code: CivilCode {
                    official_name: "Αστικός Κώδικας (Civil Code)".to_string(),
                    approval_date: "1946-02-23".to_string(),
                    last_amendment: "2023-12-15".to_string(),
                    total_articles: 2_035,
                    books: vec![
                        CodeBook {
                            number: 1,
                            title: "General Principles".to_string(),
                            chapters: vec!["Legal persons".to_string(), "Legal acts".to_string(), "Prescription".to_string()],
                            articles_range: "Articles 1-380".to_string(),
                        },
                        CodeBook {
                            number: 2,
                            title: "Law of Obligations".to_string(),
                            chapters: vec!["General provisions".to_string(), "Contracts".to_string(), "Torts".to_string()],
                            articles_range: "Articles 381-946".to_string(),
                        },
                        CodeBook {
                            number: 3,
                            title: "Property Law".to_string(),
                            chapters: vec!["Possession".to_string(), "Ownership".to_string(), "Limited real rights".to_string()],
                            articles_range: "Articles 947-1345".to_string(),
                        },
                        CodeBook {
                            number: 4,
                            title: "Family Law".to_string(),
                            chapters: vec!["Marriage".to_string(), "Parent-child relations".to_string(), "Guardianship".to_string()],
                            articles_range: "Articles 1346-1680".to_string(),
                        },
                        CodeBook {
                            number: 5,
                            title: "Inheritance Law".to_string(),
                            chapters: vec!["Succession".to_string(), "Wills".to_string(), "Inheritance contracts".to_string()],
                            articles_range: "Articles 1681-2035".to_string(),
                        },
                    ],
                    key_principles: vec![
                        "Good faith and fair dealing".to_string(),
                        "Abuse of rights prohibition".to_string(),
                        "Legal certainty".to_string(),
                    ],
                    german_influence: "Based on German BGB model with adaptations to Greek legal tradition".to_string(),
                },
                criminal_code: CriminalCode {
                    official_name: "Ποινικός Κώδικας (Criminal Code)".to_string(),
                    approval_date: "1950-08-17".to_string(),
                    last_amendment: "2024-01-10".to_string(),
                    total_articles: 486,
                    parts: vec![
                        CodePart {
                            name: "General Part".to_string(),
                            chapters: vec!["Criminal law principles".to_string(), "Criminal liability".to_string(), "Penalties".to_string()],
                            main_subjects: vec!["Crime elements".to_string(), "Punishment system".to_string(), "Circumstances".to_string()],
                        },
                        CodePart {
                            name: "Special Part".to_string(),
                            chapters: vec!["Crimes against life".to_string(), "Property crimes".to_string(), "Public order crimes".to_string()],
                            main_subjects: vec!["Specific criminal offenses".to_string(), "Aggravating factors".to_string()],
                        },
                    ],
                    penalty_system: PenaltySystem {
                        prison_sentences: vec!["Up to life imprisonment".to_string(), "Death penalty abolished in 1993".to_string()],
                        fines_system: "Day-fine system based on offender's economic capacity".to_string(),
                        alternative_measures: vec!["Community service".to_string(), "Electronic monitoring".to_string(), "Conditional suspension".to_string()],
                        rehabilitation_programs: vec!["Drug rehabilitation".to_string(), "Professional training".to_string(), "Social reintegration".to_string()],
                    },
                    recent_reforms: vec![
                        "2019 reform - Digitalization of criminal procedures".to_string(),
                        "2021 reform - Domestic violence enhanced penalties".to_string(),
                        "2023 reform - Cybercrime legislation update".to_string(),
                    ],
                },
                civil_procedure_code: CivilProcedureCode {
                    official_name: "Κώδικας Πολιτικής Δικονομίας".to_string(),
                    approval_date: "1968-09-26".to_string(),
                    procedural_principles: vec![
                        "Adversarial principle".to_string(),
                        "Oral proceedings".to_string(),
                        "Concentration principle".to_string(),
                        "Judge's active role".to_string(),
                    ],
                    types_of_proceedings: vec![
                        "Ordinary proceedings".to_string(),
                        "Summary proceedings".to_string(),
                        "Urgent proceedings".to_string(),
                        "Special proceedings".to_string(),
                    ],
                    appeal_system: vec![
                        "Appeal to Courts of Appeal".to_string(),
                        "Cassation to Areios Pagos".to_string(),
                        "Constitutional review to Council of State".to_string(),
                    ],
                    digitalization_reforms: vec![
                        "Electronic filing system".to_string(),
                        "Video conference hearings".to_string(),
                        "Digital case management".to_string(),
                    ],
                },
                criminal_procedure_code: CriminalProcedureCode {
                    official_name: "Κώδικας Ποινικής Δικονομίας".to_string(),
                    approval_date: "1950-09-18".to_string(),
                    investigation_phase: "Preliminary investigation by prosecutor with judicial police support".to_string(),
                    trial_phase: "Public oral trial with right to defense and presumption of innocence".to_string(),
                    appeal_phase: "Right to appeal to higher courts with full review".to_string(),
                    special_procedures: vec![
                        "Fast-track procedures for minor offenses".to_string(),
                        "Plea bargaining in specific cases".to_string(),
                        "Victim-offender mediation".to_string(),
                    ],
                },
                administrative_procedure_code: AdministrativeProcedureCode {
                    official_name: "Κώδικας Διοικητικής Διαδικασίας".to_string(),
                    approval_date: "1999-07-02".to_string(),
                    general_principles: vec![
                        "Legality".to_string(),
                        "Equal treatment".to_string(),
                        "Proportionality".to_string(),
                        "Transparency".to_string(),
                        "Efficiency".to_string(),
                    ],
                    administrative_acts: "Administrative decisions must be reasoned and subject to judicial review".to_string(),
                    procedural_guarantees: vec![
                        "Right to be heard".to_string(),
                        "Right to legal representation".to_string(),
                        "Right to access administrative files".to_string(),
                        "Right to appeal".to_string(),
                    ],
                    digital_administration: vec![
                        "gov.gr digital platform".to_string(),
                        "Electronic document management".to_string(),
                        "Digital signatures".to_string(),
                    ],
                },
                labor_legislation: LaborLegislation {
                    main_law: "Law 4808/2021 - Individual and Collective Labor Relations".to_string(),
                    approval_date: "2021-06-17".to_string(),
                    individual_labor_relations: "Employment contracts, working time, wages, termination procedures".to_string(),
                    collective_labor_relations: "Trade unions, collective bargaining, strikes, lockouts".to_string(),
                    labor_courts: "Specialized labor courts for employment disputes with accelerated procedures".to_string(),
                    social_security_integration: "Coordination with EFKA (Unified Social Security Fund)".to_string(),
                    recent_reforms: vec![
                        "2019 reform - Labor market flexibility measures".to_string(),
                        "2021 reform - Individual and collective labor relations modernization".to_string(),
                        "2022 reform - Remote work regulation".to_string(),
                    ],
                },
                tax_codes: vec![
                    TaxCode {
                        name: "Income Tax Code".to_string(),
                        tax_type: "Personal and Corporate Income Tax".to_string(),
                        approval_date: "2013-12-23".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Personal Income Tax - Progressive rates".to_string(),
                                rate_percentage: 44.0,
                                threshold_amount: 40_000.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                            TaxRate {
                                category: "Corporate Income Tax".to_string(),
                                rate_percentage: 22.0,
                                threshold_amount: 0.0,
                                applicable_from: "2024-01-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Tax-free threshold for low incomes".to_string(), "Family allowances".to_string()],
                        collection_procedures: "Monthly withholding for employees, quarterly payments for self-employed".to_string(),
                    },
                    TaxCode {
                        name: "VAT Code".to_string(),
                        tax_type: "Value Added Tax".to_string(),
                        approval_date: "2006-12-27".to_string(),
                        tax_rates: vec![
                            TaxRate {
                                category: "Standard VAT rate".to_string(),
                                rate_percentage: 24.0,
                                threshold_amount: 0.0,
                                applicable_from: "2016-06-01".to_string(),
                            },
                            TaxRate {
                                category: "Reduced VAT rate".to_string(),
                                rate_percentage: 13.0,
                                threshold_amount: 0.0,
                                applicable_from: "2016-06-01".to_string(),
                            },
                        ],
                        exemptions: vec!["Essential goods".to_string(), "Medical services".to_string()],
                        collection_procedures: "Monthly or quarterly VAT returns depending on turnover".to_string(),
                    },
                ],
                commercial_law: CommercialLaw {
                    company_law: "Law 4548/2018 - Sociétés Anonymes and Law 3190/1955 - Limited Liability Companies".to_string(),
                    company_types: vec![
                        "SA - Société Anonyme (Public Limited Company)".to_string(),
                        "EPE - Limited Liability Company".to_string(),
                        "IKE - Private Company".to_string(),
                        "OE - General Partnership".to_string(),
                    ],
                    commercial_transactions: "Commercial Code provisions on commercial acts and business transactions".to_string(),
                    insolvency_law: "Law 4738/2020 - Insolvency Code with preventive restructuring measures".to_string(),
                    capital_markets_law: "Law 4514/2018 - Capital Market Commission regulation".to_string(),
                },
                family_law: FamilyLaw {
                    marriage_law: "Civil and religious marriage recognition with civil effects".to_string(),
                    civil_partnership: "Cohabitation Agreement Law 4356/2015 for same-sex and different-sex couples".to_string(),
                    parental_authority: "Joint parental authority principle with child's best interest priority".to_string(),
                    adoption_procedures: "Full and simple adoption with court approval and social services evaluation".to_string(),
                    inheritance_law: "Forced heirship system with testamentary freedom limitations".to_string(),
                },
            },
            european_integration: GreekEuropeanIntegration {
                eu_membership: EUMembership {
                    accession_date: "1981-01-01".to_string(),
                    accession_treaty: "Treaty of Accession 1979".to_string(),
                    membership_benefits: vec![
                        "Single market access".to_string(),
                        "Structural and cohesion funds".to_string(),
                        "Common Agricultural Policy support".to_string(),
                        "Freedom of movement".to_string(),
                    ],
                    membership_obligations: vec![
                        "EU law supremacy and direct effect".to_string(),
                        "Budget contributions".to_string(),
                        "Policy coordination in EMU".to_string(),
                        "Solidarity mechanisms participation".to_string(),
                    ],
                    structural_funds_received: 75_000_000_000.0,
                },
                eurozone_participation: EurozoneParticipation {
                    euro_adoption_date: "2001-01-01".to_string(),
                    drachma_exchange_rate: "340.75 drachmas = 1 euro".to_string(),
                    ecb_representation: "Yannis Stournaras (Governor, Bank of Greece)".to_string(),
                    fiscal_rules_compliance: vec![
                        "Enhanced Surveillance (post-bailout)".to_string(),
                        "Stability and Growth Pact".to_string(),
                        "European Semester".to_string(),
                    ],
                    economic_surveillance: "Post-program enhanced surveillance until debt sustainability achieved".to_string(),
                },
                schengen_area: SchengenParticipation {
                    implementation_date: "2000-03-26".to_string(),
                    border_controls: "Free movement within Schengen with external border controls".to_string(),
                    migration_challenges: vec![
                        "Frontex cooperation for external borders".to_string(),
                        "Migration hotspots management".to_string(),
                        "Asylum system pressures".to_string(),
                    ],
                    information_systems: vec!["SIS II".to_string(), "VIS".to_string(), "EURODAC".to_string()],
                },
                eu_representation: EURepresentation {
                    european_parliament_meps: 21,
                    council_voting_weight: 12,
                    european_commission_commissioner: "Margaritis Schinas (Vice-President for Promoting our European Way of Life)".to_string(),
                    permanent_representation: "Ambassador Ioannis Vrailas to EU".to_string(),
                },
                eu_legislation_transposition: EULegislationTransposition {
                    transposition_rate: 97.8,
                    infringement_procedures: 28,
                    recent_transpositions: vec![
                        "Digital Services Act".to_string(),
                        "Digital Markets Act".to_string(),
                        "European Green Deal legislation".to_string(),
                    ],
                    pending_transpositions: vec![
                        "AI Act implementation".to_string(),
                        "Data Act transposition".to_string(),
                    ],
                },
                economic_adjustment_programs: vec![
                    "First Economic Adjustment Programme (2010-2012)".to_string(),
                    "Second Economic Adjustment Programme (2012-2014)".to_string(),
                    "Third Economic Adjustment Programme (2015-2018)".to_string(),
                    "Enhanced Surveillance (2018-present)".to_string(),
                ],
            },
            regional_administration: GreekRegionalAdministration {
                administrative_regions: vec![
                    AdministrativeRegion {
                        name: "Attica".to_string(),
                        capital: "Athens".to_string(),
                        area_km2: 3_808.0,
                        population: 3_812_330,
                        regional_governor: "Nikos Hardalias".to_string(),
                        regional_assembly: RegionalAssembly {
                            members_count: 51,
                            political_composition: {
                                let mut composition = HashMap::new();
                                composition.insert("ND".to_string(), 28);
                                composition.insert("SYRIZA".to_string(), 12);
                                composition.insert("PASOK".to_string(), 8);
                                composition.insert("Others".to_string(), 3);
                                composition
                            },
                            current_term: "2023-2028".to_string(),
                        },
                        competencies: vec![
                            "Regional development".to_string(),
                            "Tourism promotion".to_string(),
                            "Environmental protection".to_string(),
                            "Cultural heritage".to_string(),
                        ],
                        budget_2024: 2_100_000_000.0,
                    },
                ],
                decentralized_administrations: vec![
                    DecentralizedAdministration {
                        name: "Attica Decentralized Administration".to_string(),
                        regions_covered: vec!["Attica".to_string()],
                        secretary_general: "Maria Kollia-Tsaroucha".to_string(),
                        headquarters: "Athens".to_string(),
                        competencies: vec![
                            "State services coordination".to_string(),
                            "Administrative oversight".to_string(),
                            "Regional development support".to_string(),
                        ],
                    },
                ],
                regional_development: RegionalDevelopment {
                    development_strategies: vec![
                        "Smart Specialization Strategies".to_string(),
                        "Regional Innovation Systems".to_string(),
                        "Sustainable development goals".to_string(),
                    ],
                    eu_funding_programs: vec![
                        "Partnership Agreement 2021-2027".to_string(),
                        "Regional Operational Programs".to_string(),
                        "INTERREG programs".to_string(),
                    ],
                    infrastructure_projects: vec![
                        "Digital infrastructure development".to_string(),
                        "Transport connectivity improvement".to_string(),
                        "Green energy transition".to_string(),
                    ],
                    innovation_ecosystems: vec![
                        "Research and innovation hubs".to_string(),
                        "Technology parks".to_string(),
                        "Startup support programs".to_string(),
                    ],
                },
                kallikratis_plan: KallikratisPlan {
                    implementation_date: "2010-01-01".to_string(),
                    objectives: vec![
                        "Administrative reorganization".to_string(),
                        "Cost reduction".to_string(),
                        "Service quality improvement".to_string(),
                        "Democratic participation enhancement".to_string(),
                    ],
                    territorial_reforms: vec![
                        "Municipality consolidation from 1034 to 325".to_string(),
                        "Regional units creation".to_string(),
                        "Administrative regions strengthening".to_string(),
                    ],
                    administrative_simplification: vec![
                        "One-stop-shop services".to_string(),
                        "Digital transformation".to_string(),
                        "Bureaucracy reduction".to_string(),
                    ],
                    financial_efficiency: "Economies of scale through consolidation and shared services".to_string(),
                },
            },
            local_government: GreekLocalGovernment {
                municipal_system: MunicipalSystem {
                    legal_framework: "Law 3852/2010 (Kallikratis) and subsequent amendments".to_string(),
                    municipal_council: "Elected body with legislative and oversight functions".to_string(),
                    mayor_powers: vec![
                        "Executive authority".to_string(),
                        "Municipal administration".to_string(),
                        "Local development planning".to_string(),
                        "Service delivery oversight".to_string(),
                    ],
                    competencies: vec![
                        "Urban planning and building permits".to_string(),
                        "Local infrastructure maintenance".to_string(),
                        "Social services provision".to_string(),
                        "Environmental protection".to_string(),
                        "Cultural and sports activities".to_string(),
                    ],
                    financial_autonomy: "Own revenues supplemented by state transfers and EU funding".to_string(),
                },
                local_communities: vec![
                    LocalCommunity {
                        name: "Agia Paraskevi".to_string(),
                        municipality: "Athens".to_string(),
                        population: 56_836,
                        president: "Local community president".to_string(),
                        local_council: "Community council with local representation".to_string(),
                    },
                ],
                metropolitan_organizations: vec![
                    MetropolitanOrganization {
                        name: "Athens Urban Area Organization".to_string(),
                        member_municipalities: vec!["Athens".to_string(), "Piraeus".to_string(), "Peristeri".to_string()],
                        governance_structure: "Inter-municipal cooperation body".to_string(),
                        competencies: vec![
                            "Metropolitan transport planning".to_string(),
                            "Waste management coordination".to_string(),
                            "Strategic planning".to_string(),
                        ],
                        budget_2024: 450_000_000.0,
                    },
                ],
                local_finance: LocalFinance {
                    revenue_sources: vec![
                        "Property taxes".to_string(),
                        "Municipal fees and charges".to_string(),
                        "State transfers".to_string(),
                        "EU funding".to_string(),
                    ],
                    transfer_system: "Regular Transfer Amounts (TAK) based on objective criteria".to_string(),
                    borrowing_rules: "Limited borrowing capacity with central government authorization".to_string(),
                    fiscal_supervision: "Court of Auditors oversight and Ministry of Interior monitoring".to_string(),
                },
                citizen_participation: CitizenParticipation {
                    participatory_mechanisms: vec![
                        "Municipal assemblies".to_string(),
                        "Public consultations".to_string(),
                        "Citizens' committees".to_string(),
                    ],
                    digital_democracy: vec![
                        "Online consultation platforms".to_string(),
                        "E-participation tools".to_string(),
                        "Digital voting systems".to_string(),
                    ],
                    local_referendums: "Municipal referendums allowed on local issues with qualified majority".to_string(),
                    citizen_assemblies: vec![
                        "Neighborhood councils".to_string(),
                        "Thematic citizen panels".to_string(),
                    ],
                },
            },
            orthodox_church_status: GreekOrthodoxChurchStatus {
                constitutional_status: "Prevailing religion according to Article 3 of Constitution".to_string(),
                church_state_relations: ChurchStateRelations {
                    constitutional_article: "Article 3 - Prevailing religion status".to_string(),
                    prevailing_religion: "Eastern Orthodox Church of Christ".to_string(),
                    state_support: vec![
                        "Clergy salaries paid by state".to_string(),
                        "Religious education in public schools".to_string(),
                        "Tax exemptions for church property".to_string(),
                    ],
                    religious_education: "Mandatory Orthodox religious education with opt-out provisions".to_string(),
                    clergy_status: "Priests are civil servants with state salaries".to_string(),
                },
                ecclesiastical_hierarchy: EcclesiasticalHierarchy {
                    archbishop_athens: "Ieronymos II, Archbishop of Athens and All Greece".to_string(),
                    holy_synod: "Governing body of Orthodox Church of Greece".to_string(),
                    metropolitans: vec![
                        "Metropolitan of Thessaloniki".to_string(),
                        "Metropolitan of Patras".to_string(),
                        "Metropolitan of Larissa".to_string(),
                    ],
                    dioceses_count: 81,
                    monastic_communities: vec![
                        "Mount Athos Autonomous Monastic State".to_string(),
                        "Various monasteries throughout Greece".to_string(),
                    ],
                },
                legal_framework: "Law 590/1977 on Church-State relations and subsequent modifications".to_string(),
                property_status: "Church property protected with special legal status".to_string(),
            },
        }
    }
}