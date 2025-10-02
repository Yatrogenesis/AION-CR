use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthAfricaLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub provinces: Vec<Province>,
    pub national_government: NationalGovernment,
    pub judicial_system: JudicialSystem,
    pub electoral_system: ElectoralSystem,
    pub legal_codes: LegalCodes,
    pub municipalities: Vec<Municipality>,
    pub traditional_authorities: Vec<TraditionalAuthority>,
    pub language_legal_framework: LanguageLegalFramework,
    pub ubuntu_constitutional_values: UbuntuConstitutionalValues,
    pub transitional_justice: TransitionalJustice,
    pub mineral_rights_framework: MineralRightsFramework,
    pub land_reform_framework: LandReformFramework,
    pub api_integrations: ApiIntegrations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    pub constitution_date: String,
    pub constitutional_principles: ConstitutionalPrinciples,
    pub bill_of_rights: BillOfRights,
    pub separation_of_powers: SeparationOfPowers,
    pub cooperative_governance: CooperativeGovernance,
    pub constitutional_amendments: Vec<ConstitutionalAmendment>,
    pub constitutional_interpretation: ConstitutionalInterpretation,
    pub constitutional_court: ConstitutionalCourt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalPrinciples {
    pub founding_provisions: Vec<String>,
    pub constitutional_supremacy: Vec<String>,
    pub rule_of_law: Vec<String>,
    pub democracy: Vec<String>,
    pub human_dignity: Vec<String>,
    pub equality: Vec<String>,
    pub freedom: Vec<String>,
    pub non_racialism: Vec<String>,
    pub non_sexism: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfRights {
    pub fundamental_rights: FundamentalRights,
    pub civil_political_rights: CivilPoliticalRights,
    pub socio_economic_rights: SocioEconomicRights,
    pub cultural_rights: CulturalRights,
    pub environmental_rights: EnvironmentalRights,
    pub property_rights: PropertyRights,
    pub enforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRights {
    pub human_dignity: Vec<String>,
    pub life: Vec<String>,
    pub freedom_security: Vec<String>,
    pub privacy: Vec<String>,
    pub freedom_religion: Vec<String>,
    pub freedom_expression: Vec<String>,
    pub assembly_demonstration: Vec<String>,
    pub political_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilPoliticalRights {
    pub citizenship: Vec<String>,
    pub freedom_movement: Vec<String>,
    pub freedom_trade: Vec<String>,
    pub labour_relations: Vec<String>,
    pub access_to_information: Vec<String>,
    pub just_administrative_action: Vec<String>,
    pub access_to_courts: Vec<String>,
    pub arrested_detained_accused: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocioEconomicRights {
    pub healthcare_food_water: Vec<String>,
    pub social_security: Vec<String>,
    pub education: Vec<String>,
    pub children: Vec<String>,
    pub language_culture: Vec<String>,
    pub housing: Vec<String>,
    pub progressive_realization: Vec<String>,
    pub state_obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalRights {
    pub language_rights: Vec<String>,
    pub cultural_rights: Vec<String>,
    pub religious_rights: Vec<String>,
    pub traditional_leadership: Vec<String>,
    pub customary_law: Vec<String>,
    pub cultural_institutions: Vec<String>,
    pub heritage_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRights {
    pub environmental_protection: Vec<String>,
    pub ecologically_sustainable_development: Vec<String>,
    pub pollution_prevention: Vec<String>,
    pub conservation: Vec<String>,
    pub future_generations: Vec<String>,
    pub environmental_governance: Vec<String>,
    pub climate_change: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyRights {
    pub property_protection: Vec<String>,
    pub expropriation: Vec<String>,
    pub land_reform: Vec<String>,
    pub restitution: Vec<String>,
    pub redistribution: Vec<String>,
    pub tenure_reform: Vec<String>,
    pub just_equitable_compensation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparationOfPowers {
    pub executive_authority: ExecutiveAuthority,
    pub legislative_authority: LegislativeAuthority,
    pub judicial_authority: JudicialAuthority,
    pub checks_and_balances: ChecksAndBalances,
    pub institutions_supporting_democracy: InstitutionsSupportingDemocracy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveAuthority {
    pub president: President,
    pub deputy_president: DeputyPresident,
    pub cabinet: Cabinet,
    pub national_executive: NationalExecutive,
    pub provincial_executive: ProvincialExecutive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct President {
    pub election_process: String,
    pub term_office: String,
    pub powers_functions: Vec<String>,
    pub oath_affirmation: String,
    pub removal_procedures: Vec<String>,
    pub executive_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeputyPresident {
    pub appointment_process: String,
    pub functions: Vec<String>,
    pub succession_provisions: Vec<String>,
    pub delegation_powers: Vec<String>,
    pub coordination_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cabinet {
    pub composition: Vec<String>,
    pub collective_responsibility: String,
    pub decision_making: Vec<String>,
    pub ministerial_accountability: Vec<String>,
    pub portfolio_allocation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalExecutive {
    pub executive_structure: Vec<String>,
    pub implementation_authority: Vec<String>,
    pub policy_execution: Vec<String>,
    pub administrative_oversight: Vec<String>,
    pub intergovernmental_coordination: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialExecutive {
    pub premier: Premier,
    pub provincial_cabinet: ProvincialCabinet,
    pub executive_councils: Vec<ExecutiveCouncil>,
    pub provincial_administration: ProvincialAdministration,
    pub intergovernmental_relations: IntergovernmentalRelations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Premier {
    pub election_process: String,
    pub term_office: String,
    pub powers_functions: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
    pub provincial_leadership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialCabinet {
    pub composition: Vec<String>,
    pub ministerial_appointments: Vec<String>,
    pub collective_responsibility: String,
    pub provincial_policy: Vec<String>,
    pub implementation_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveCouncil {
    pub council_composition: Vec<String>,
    pub decision_making_processes: Vec<String>,
    pub administrative_coordination: Vec<String>,
    pub policy_implementation: Vec<String>,
    pub performance_monitoring: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialAdministration {
    pub administrative_structure: Vec<String>,
    pub service_delivery: Vec<String>,
    pub human_resource_management: Vec<String>,
    pub financial_management: Vec<String>,
    pub performance_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntergovernmentalRelations {
    pub national_provincial: Vec<String>,
    pub provincial_local: Vec<String>,
    pub cooperative_governance: Vec<String>,
    pub dispute_resolution: Vec<String>,
    pub fiscal_relations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeAuthority {
    pub parliament: Parliament,
    pub provincial_legislatures: ProvincialLegislatures,
    pub legislative_process: LegislativeProcess,
    pub public_participation: PublicParticipation,
    pub legislative_oversight: LegislativeOversight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parliament {
    pub national_assembly: NationalAssembly,
    pub national_council_provinces: NationalCouncilProvinces,
    pub joint_sittings: Vec<String>,
    pub parliamentary_procedures: ParliamentaryProcedures,
    pub parliamentary_committees: ParliamentaryCommittees,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalAssembly {
    pub composition: String,
    pub election_system: String,
    pub term_office: String,
    pub powers_functions: Vec<String>,
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalCouncilProvinces {
    pub composition: String,
    pub provincial_representation: Vec<String>,
    pub powers_functions: Vec<String>,
    pub voting_procedures: Vec<String>,
    pub provincial_interests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryProcedures {
    pub rules_procedures: Vec<String>,
    pub debate_procedures: Vec<String>,
    pub voting_procedures: Vec<String>,
    pub committee_procedures: Vec<String>,
    pub disciplinary_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryCommittees {
    pub portfolio_committees: Vec<PortfolioCommittee>,
    pub select_committees: Vec<SelectCommittee>,
    pub joint_committees: Vec<JointCommittee>,
    pub ad_hoc_committees: Vec<AdHocCommittee>,
    pub committee_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioCommittee {
    pub committee_name: String,
    pub mandate: String,
    pub oversight_responsibilities: Vec<String>,
    pub public_participation: Vec<String>,
    pub reporting_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCommittee {
    pub committee_name: String,
    pub provincial_representation: Vec<String>,
    pub functions: Vec<String>,
    pub coordination_role: Vec<String>,
    pub intergovernmental_focus: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointCommittee {
    pub committee_name: String,
    pub joint_mandate: String,
    pub bicameral_coordination: Vec<String>,
    pub shared_oversight: Vec<String>,
    pub joint_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdHocCommittee {
    pub committee_purpose: String,
    pub specific_mandate: String,
    pub duration: String,
    pub investigative_powers: Vec<String>,
    pub reporting_deadline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialLegislatures {
    pub legislature_structure: Vec<String>,
    pub legislative_powers: Vec<String>,
    pub provincial_competencies: Vec<String>,
    pub legislative_procedures: Vec<String>,
    pub oversight_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeProcess {
    pub bill_types: Vec<BillType>,
    pub legislative_procedures: Vec<String>,
    pub public_participation_process: Vec<String>,
    pub presidential_assent: Vec<String>,
    pub constitutional_review: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillType {
    pub bill_category: String,
    pub introduction_requirements: Vec<String>,
    pub procedural_requirements: Vec<String>,
    pub approval_requirements: Vec<String>,
    pub constitutional_implications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicParticipation {
    pub constitutional_mandate: Vec<String>,
    pub participation_mechanisms: Vec<String>,
    pub citizen_engagement: Vec<String>,
    pub civil_society_involvement: Vec<String>,
    pub transparency_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeOversight {
    pub oversight_mechanisms: Vec<String>,
    pub executive_accountability: Vec<String>,
    pub performance_monitoring: Vec<String>,
    pub public_finance_oversight: Vec<String>,
    pub institutional_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialAuthority {
    pub judicial_independence: JudicialIndependence,
    pub court_structure: CourtStructure,
    pub judicial_service_commission: JudicialServiceCommission,
    pub judicial_conduct: JudicialConduct,
    pub access_to_justice: AccessToJustice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialIndependence {
    pub constitutional_protection: Vec<String>,
    pub tenure_security: Vec<String>,
    pub financial_independence: Vec<String>,
    pub administrative_independence: Vec<String>,
    pub judicial_immunity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtStructure {
    pub constitutional_court: CourtLevel,
    pub supreme_court_appeal: CourtLevel,
    pub high_courts: Vec<CourtLevel>,
    pub magistrates_courts: Vec<CourtLevel>,
    pub specialized_courts: Vec<CourtLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtLevel {
    pub court_name: String,
    pub jurisdiction: Vec<String>,
    pub composition: Vec<String>,
    pub appointment_procedures: Vec<String>,
    pub powers_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialServiceCommission {
    pub composition: Vec<String>,
    pub appointment_functions: Vec<String>,
    pub disciplinary_functions: Vec<String>,
    pub advisory_functions: Vec<String>,
    pub independence_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialConduct {
    pub ethical_standards: Vec<String>,
    pub conduct_rules: Vec<String>,
    pub disciplinary_procedures: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
    pub judicial_education: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToJustice {
    pub constitutional_guarantee: Vec<String>,
    pub legal_aid: Vec<String>,
    pub court_accessibility: Vec<String>,
    pub alternative_dispute_resolution: Vec<String>,
    pub language_accessibility: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksAndBalances {
    pub executive_legislative: Vec<String>,
    pub legislative_judicial: Vec<String>,
    pub judicial_executive: Vec<String>,
    pub institutional_safeguards: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsSupportingDemocracy {
    pub public_protector: PublicProtector,
    pub south_african_human_rights_commission: SouthAfricanHumanRightsCommission,
    pub commission_gender_equality: CommissionGenderEquality,
    pub auditor_general: AuditorGeneral,
    pub electoral_commission: ElectoralCommission,
    pub independent_communications_authority: IndependentCommunicationsAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProtector {
    pub mandate: Vec<String>,
    pub appointment_process: String,
    pub independence_guarantees: Vec<String>,
    pub investigative_powers: Vec<String>,
    pub reporting_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthAfricanHumanRightsCommission {
    pub mandate: Vec<String>,
    pub composition: Vec<String>,
    pub functions: Vec<String>,
    pub monitoring_role: Vec<String>,
    pub enforcement_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionGenderEquality {
    pub mandate: Vec<String>,
    pub gender_equality_promotion: Vec<String>,
    pub monitoring_functions: Vec<String>,
    pub advisory_role: Vec<String>,
    pub reporting_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditorGeneral {
    pub mandate: Vec<String>,
    pub independence_guarantees: Vec<String>,
    pub audit_functions: Vec<String>,
    pub reporting_requirements: Vec<String>,
    pub accountability_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralCommission {
    pub mandate: Vec<String>,
    pub independence_measures: Vec<String>,
    pub electoral_administration: Vec<String>,
    pub oversight_functions: Vec<String>,
    pub dispute_resolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependentCommunicationsAuthority {
    pub mandate: Vec<String>,
    pub regulatory_functions: Vec<String>,
    pub broadcasting_oversight: Vec<String>,
    pub telecommunications_regulation: Vec<String>,
    pub media_freedom_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeGovernance {
    pub constitutional_principles: Vec<String>,
    pub intergovernmental_relations: IntergovernmentalRelationsFramework,
    pub fiscal_federalism: FiscalFederalism,
    pub dispute_resolution_mechanisms: DisputeResolutionMechanisms,
    pub concurrent_powers: ConcurrentPowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntergovernmentalRelationsFramework {
    pub three_spheres_government: Vec<String>,
    pub coordination_mechanisms: Vec<String>,
    pub cooperation_principles: Vec<String>,
    pub conflict_resolution: Vec<String>,
    pub information_sharing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalFederalism {
    pub revenue_sharing: Vec<String>,
    pub fiscal_transfers: Vec<String>,
    pub provincial_taxation: Vec<String>,
    pub municipal_financing: Vec<String>,
    pub fiscal_coordination: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolutionMechanisms {
    pub intergovernmental_disputes: Vec<String>,
    pub mediation_processes: Vec<String>,
    pub arbitration_procedures: Vec<String>,
    pub constitutional_court_role: Vec<String>,
    pub political_resolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentPowers {
    pub concurrent_competencies: Vec<String>,
    pub shared_responsibilities: Vec<String>,
    pub coordination_requirements: Vec<String>,
    pub conflict_resolution_concurrent: Vec<String>,
    pub implementation_coordination: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalAmendment {
    pub amendment_number: String,
    pub year: String,
    pub title: String,
    pub provisions_amended: Vec<String>,
    pub amendment_procedure: String,
    pub constitutional_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalInterpretation {
    pub interpretive_principles: Vec<String>,
    pub constitutional_values: Vec<String>,
    pub international_law: Vec<String>,
    pub foreign_law: Vec<String>,
    pub ubuntu_philosophy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCourt {
    pub composition: Vec<String>,
    pub appointment_procedures: Vec<String>,
    pub jurisdiction: Vec<String>,
    pub constitutional_review: Vec<String>,
    pub fundamental_rights_enforcement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub province_name: String,
    pub province_code: String,
    pub capital_city: String,
    pub population: u32,
    pub area_km2: f64,
    pub provincial_government: ProvincialGovernment,
    pub economic_profile: ProvinceEconomicProfile,
    pub cultural_profile: ProvinceCulturalProfile,
    pub development_indicators: ProvinceDevelopmentIndicators,
    pub municipalities: Vec<ProvincemunicipulaData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialGovernment {
    pub legislative_structure: ProvincialLegislativeStructure,
    pub executive_structure: ProvincialExecutiveStructure,
    pub administration: ProvincialAdministrationStructure,
    pub service_delivery: ProvincialServiceDelivery,
    pub development_planning: ProvincialDevelopmentPlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialLegislativeStructure {
    pub legislature_composition: Vec<String>,
    pub legislative_powers: Vec<String>,
    pub committee_system: Vec<String>,
    pub legislative_procedures: Vec<String>,
    pub public_participation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialExecutiveStructure {
    pub premier_office: Vec<String>,
    pub provincial_cabinet: Vec<String>,
    pub executive_functions: Vec<String>,
    pub policy_implementation: Vec<String>,
    pub intergovernmental_coordination: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialAdministrationStructure {
    pub administrative_departments: Vec<String>,
    pub public_service: Vec<String>,
    pub human_resource_management: Vec<String>,
    pub financial_management: Vec<String>,
    pub performance_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialServiceDelivery {
    pub health_services: Vec<String>,
    pub education_services: Vec<String>,
    pub social_development: Vec<String>,
    pub infrastructure_services: Vec<String>,
    pub economic_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialDevelopmentPlanning {
    pub development_strategies: Vec<String>,
    pub planning_processes: Vec<String>,
    pub implementation_frameworks: Vec<String>,
    pub monitoring_evaluation: Vec<String>,
    pub stakeholder_engagement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceEconomicProfile {
    pub economic_sectors: Vec<String>,
    pub key_industries: Vec<String>,
    pub natural_resources: Vec<String>,
    pub economic_indicators: Vec<String>,
    pub development_potential: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceCulturalProfile {
    pub cultural_heritage: Vec<String>,
    pub languages_spoken: Vec<String>,
    pub traditional_authorities: Vec<String>,
    pub cultural_institutions: Vec<String>,
    pub cultural_practices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceDevelopmentIndicators {
    pub human_development_index: String,
    pub poverty_levels: String,
    pub employment_rates: String,
    pub education_levels: String,
    pub health_indicators: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincemunicipulaData {
    pub municipality_name: String,
    pub municipality_type: String,
    pub population: u32,
    pub governance_structure: MunicipalGovernanceStructure,
    pub service_delivery_municipal: MunicipalServiceDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunicipalGovernanceStructure {
    pub council_composition: Vec<String>,
    pub mayoral_system: Vec<String>,
    pub administrative_structure: Vec<String>,
    pub decision_making_processes: Vec<String>,
    pub public_participation_municipal: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunicipalServiceDelivery {
    pub basic_services: Vec<String>,
    pub infrastructure_development: Vec<String>,
    pub local_economic_development: Vec<String>,
    pub social_services_municipal: Vec<String>,
    pub environmental_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalGovernment {
    pub executive_branch: ExecutiveBranch,
    pub legislative_branch: LegislativeBranch,
    pub judicial_branch: JudicialBranch,
    pub public_administration: PublicAdministration,
    pub state_owned_enterprises: Vec<StateOwnedEnterprise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveBranch {
    pub presidency: Presidency,
    pub deputy_presidency: DeputyPresidency,
    pub national_cabinet: NationalCabinet,
    pub national_departments: Vec<NationalDepartment>,
    pub government_coordination: GovernmentCoordination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presidency {
    pub presidential_functions: Vec<String>,
    pub executive_powers: Vec<String>,
    pub ceremonial_functions: Vec<String>,
    pub international_relations: Vec<String>,
    pub national_leadership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeputyPresidency {
    pub deputy_presidential_functions: Vec<String>,
    pub coordination_role: Vec<String>,
    pub succession_provisions: Vec<String>,
    pub special_assignments: Vec<String>,
    pub institutional_representation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalCabinet {
    pub cabinet_composition: Vec<String>,
    pub collective_responsibility: Vec<String>,
    pub decision_making_cabinet: Vec<String>,
    pub policy_coordination: Vec<String>,
    pub implementation_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalDepartment {
    pub department_name: String,
    pub minister: String,
    pub mandate: Vec<String>,
    pub organizational_structure: Vec<String>,
    pub service_delivery_mandate: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentCoordination {
    pub cabinet_committees: Vec<String>,
    pub intergovernmental_forums: Vec<String>,
    pub policy_coordination_mechanisms: Vec<String>,
    pub implementation_coordination: Vec<String>,
    pub performance_monitoring_coordination: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeBranch {
    pub parliament_detailed: ParliamentDetailed,
    pub provincial_legislatures_detailed: ProvincialLegislaturesDetailed,
    pub legislative_support_services: LegislativeSupportServices,
    pub parliamentary_oversight_detailed: ParliamentaryOversightDetailed,
    pub legislative_drafting: LegislativeDrafting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentDetailed {
    pub national_assembly_detailed: NationalAssemblyDetailed,
    pub ncop_detailed: NcopDetailed,
    pub joint_procedures: Vec<String>,
    pub parliamentary_administration: ParliamentaryAdministration,
    pub parliamentary_budget: ParliamentaryBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalAssemblyDetailed {
    pub electoral_system_detailed: Vec<String>,
    pub party_representation: Vec<String>,
    pub assembly_procedures: Vec<String>,
    pub committee_system_assembly: Vec<String>,
    pub oversight_mechanisms_assembly: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcopDetailed {
    pub provincial_delegation_system: Vec<String>,
    pub voting_procedures_ncop: Vec<String>,
    pub committee_system_ncop: Vec<String>,
    pub provincial_coordination: Vec<String>,
    pub intergovernmental_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryAdministration {
    pub administrative_structure: Vec<String>,
    pub support_services: Vec<String>,
    pub resource_management: Vec<String>,
    pub facilities_management: Vec<String>,
    pub parliamentary_staff: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryBudget {
    pub budget_allocation: Vec<String>,
    pub financial_management: Vec<String>,
    pub audit_oversight: Vec<String>,
    pub resource_allocation: Vec<String>,
    pub performance_budgeting: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialLegislaturesDetailed {
    pub legislature_structures: Vec<String>,
    pub provincial_competencies_detailed: Vec<String>,
    pub legislative_procedures_provincial: Vec<String>,
    pub oversight_functions_provincial: Vec<String>,
    pub intergovernmental_role_provincial: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeSupportServices {
    pub research_services: Vec<String>,
    pub legal_services: Vec<String>,
    pub library_services: Vec<String>,
    pub information_services: Vec<String>,
    pub translation_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryOversightDetailed {
    pub oversight_mandate: Vec<String>,
    pub oversight_mechanisms_detailed: Vec<String>,
    pub committee_oversight: Vec<String>,
    pub public_participation_oversight: Vec<String>,
    pub accountability_enforcement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeDrafting {
    pub drafting_procedures: Vec<String>,
    pub legislative_review: Vec<String>,
    pub constitutional_compliance: Vec<String>,
    pub drafting_standards: Vec<String>,
    pub stakeholder_consultation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialBranch {
    pub court_system_detailed: CourtSystemDetailed,
    pub judicial_administration_detailed: JudicialAdministrationDetailed,
    pub legal_profession: LegalProfession,
    pub access_justice_detailed: AccessJusticeDetailed,
    pub alternative_dispute_resolution: AlternativeDisputeResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtSystemDetailed {
    pub constitutional_court_detailed: ConstitutionalCourtDetailed,
    pub supreme_court_appeal_detailed: SupremeCourtAppealDetailed,
    pub high_courts_detailed: Vec<HighCourtDetailed>,
    pub magistrates_courts_detailed: Vec<MagistratesCourtDetailed>,
    pub specialized_courts_detailed: Vec<SpecializedCourtDetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCourtDetailed {
    pub constitutional_jurisdiction: Vec<String>,
    pub appointment_process: Vec<String>,
    pub composition_detailed: Vec<String>,
    pub procedures: Vec<String>,
    pub constitutional_interpretation_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourtAppealDetailed {
    pub appellate_jurisdiction: Vec<String>,
    pub appeal_procedures: Vec<String>,
    pub composition_sca: Vec<String>,
    pub case_management: Vec<String>,
    pub precedent_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighCourtDetailed {
    pub court_name: String,
    pub jurisdiction_area: String,
    pub civil_jurisdiction: Vec<String>,
    pub criminal_jurisdiction: Vec<String>,
    pub administrative_jurisdiction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagistratesCourtDetailed {
    pub court_location: String,
    pub jurisdiction_magistrates: Vec<String>,
    pub civil_procedures: Vec<String>,
    pub criminal_procedures: Vec<String>,
    pub small_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedCourtDetailed {
    pub court_specialization: String,
    pub specialized_jurisdiction_detailed: Vec<String>,
    pub specialized_procedures_detailed: Vec<String>,
    pub expertise_requirements_detailed: Vec<String>,
    pub integration_general_system: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialAdministrationDetailed {
    pub office_chief_justice: Vec<String>,
    pub judicial_service_commission_detailed: Vec<String>,
    pub court_management: Vec<String>,
    pub case_flow_management: Vec<String>,
    pub judicial_training_detailed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProfession {
    pub legal_practice_council: Vec<String>,
    pub attorneys_profession: Vec<String>,
    pub advocates_profession: Vec<String>,
    pub legal_education: Vec<String>,
    pub professional_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessJusticeDetailed {
    pub legal_aid_south_africa: Vec<String>,
    pub public_interest_litigation: Vec<String>,
    pub community_advice_offices: Vec<String>,
    pub small_claims_courts: Vec<String>,
    pub traditional_courts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeDisputeResolution {
    pub mediation_services: Vec<String>,
    pub arbitration_services: Vec<String>,
    pub traditional_dispute_resolution: Vec<String>,
    pub community_courts: Vec<String>,
    pub family_mediation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAdministration {
    pub public_service: PublicService,
    pub public_service_commission: PublicServiceCommission,
    pub government_departments: GovernmentDepartments,
    pub performance_management_government: PerformanceManagementGovernment,
    pub e_government: EGovernment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicService {
    pub public_service_principles: Vec<String>,
    pub human_resource_management: Vec<String>,
    pub career_development: Vec<String>,
    pub performance_management: Vec<String>,
    pub ethics_integrity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicServiceCommission {
    pub mandate_psc: Vec<String>,
    pub composition_psc: Vec<String>,
    pub functions_psc: Vec<String>,
    pub oversight_role_psc: Vec<String>,
    pub reporting_mechanisms_psc: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentDepartments {
    pub departmental_structure: Vec<String>,
    pub ministerial_oversight: Vec<String>,
    pub administrative_management: Vec<String>,
    pub service_delivery_departments: Vec<String>,
    pub intergovernmental_coordination_departments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceManagementGovernment {
    pub performance_monitoring: Vec<String>,
    pub outcomes_based_approach: Vec<String>,
    pub service_delivery_indicators: Vec<String>,
    pub public_participation_performance: Vec<String>,
    pub accountability_performance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGovernment {
    pub digital_government_strategy: Vec<String>,
    pub e_services: Vec<String>,
    pub digital_infrastructure: Vec<String>,
    pub cybersecurity: Vec<String>,
    pub digital_inclusion: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateOwnedEnterprise {
    pub enterprise_name: String,
    pub sector: String,
    pub mandate: Vec<String>,
    pub governance_structure: Vec<String>,
    pub performance_monitoring_soe: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialSystem {
    pub court_hierarchy_comprehensive: CourtHierarchyComprehensive,
    pub judicial_independence_detailed: JudicialIndependenceDetailed,
    pub constitutional_jurisdiction_detailed: ConstitutionalJurisdictionDetailed,
    pub judicial_review_detailed: JudicialReviewDetailed,
    pub court_administration_comprehensive: CourtAdministrationComprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtHierarchyComprehensive {
    pub apex_courts: Vec<ApexCourt>,
    pub superior_courts: Vec<SuperiorCourt>,
    pub lower_courts: Vec<LowerCourt>,
    pub specialized_jurisdictions_comprehensive: Vec<SpecializedJurisdictionComprehensive>,
    pub traditional_courts_system: TraditionalCourtsSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApexCourt {
    pub court_name: String,
    pub constitutional_role: Vec<String>,
    pub jurisdiction_apex: Vec<String>,
    pub composition_apex: Vec<String>,
    pub appointment_procedures_apex: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperiorCourt {
    pub court_name: String,
    pub jurisdiction_superior: Vec<String>,
    pub geographical_jurisdiction: String,
    pub specialized_divisions: Vec<String>,
    pub appeal_procedures_superior: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowerCourt {
    pub court_type: String,
    pub jurisdiction_lower: Vec<String>,
    pub procedural_rules: Vec<String>,
    pub accessibility_measures: Vec<String>,
    pub community_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedJurisdictionComprehensive {
    pub jurisdiction_area: String,
    pub specialized_expertise: Vec<String>,
    pub procedural_innovations: Vec<String>,
    pub integration_mechanisms: Vec<String>,
    pub performance_indicators_specialized: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalCourtsSystem {
    pub traditional_authorities_judicial: Vec<String>,
    pub customary_law_application: Vec<String>,
    pub integration_formal_system: Vec<String>,
    pub dispute_resolution_traditional: Vec<String>,
    pub constitutional_compliance_traditional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialIndependenceDetailed {
    pub independence_framework: Vec<String>,
    pub appointment_protection: Vec<String>,
    pub tenure_protection: Vec<String>,
    pub financial_protection: Vec<String>,
    pub administrative_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalJurisdictionDetailed {
    pub constitutional_review_detailed: Vec<String>,
    pub rights_enforcement: Vec<String>,
    pub institutional_disputes: Vec<String>,
    pub intergovernmental_disputes: Vec<String>,
    pub constitutional_interpretation_detailed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialReviewDetailed {
    pub administrative_action_review: Vec<String>,
    pub constitutional_review_detailed: Vec<String>,
    pub procedural_review: Vec<String>,
    pub substantive_review: Vec<String>,
    pub remedial_powers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtAdministrationComprehensive {
    pub administration_structure: Vec<String>,
    pub case_management_comprehensive: Vec<String>,
    pub court_technology: Vec<String>,
    pub access_facilitation: Vec<String>,
    pub performance_management_courts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystem {
    pub electoral_framework: ElectoralFramework,
    pub political_parties: Vec<PoliticalParty>,
    pub electoral_administration: ElectoralAdministration,
    pub electoral_disputes: ElectoralDisputes,
    pub electoral_reforms: ElectoralReforms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralFramework {
    pub electoral_laws: Vec<ElectoralLaw>,
    pub electoral_system_design: ElectoralSystemDesign,
    pub voter_registration: VoterRegistration,
    pub electoral_procedures: ElectoralProcedures,
    pub electoral_integrity: ElectoralIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralLaw {
    pub law_title: String,
    pub provisions: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
    pub constitutional_basis: Vec<String>,
    pub amendment_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystemDesign {
    pub proportional_representation: Vec<String>,
    pub electoral_formula: Vec<String>,
    pub constituency_delimitation: Vec<String>,
    pub electoral_threshold: Vec<String>,
    pub ballot_design: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRegistration {
    pub registration_system: Vec<String>,
    pub voter_eligibility: Vec<String>,
    pub registration_procedures: Vec<String>,
    pub voter_education: Vec<String>,
    pub accessibility_registration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralProcedures {
    pub candidate_nomination: Vec<String>,
    pub campaign_procedures: Vec<String>,
    pub voting_procedures: Vec<String>,
    pub counting_procedures: Vec<String>,
    pub result_declaration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralIntegrity {
    pub integrity_measures: Vec<String>,
    pub transparency_mechanisms: Vec<String>,
    pub oversight_bodies: Vec<String>,
    pub international_observation: Vec<String>,
    pub electoral_offenses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub party_name: String,
    pub political_ideology: String,
    pub party_registration: Vec<String>,
    pub organizational_structure: Vec<String>,
    pub funding_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralAdministration {
    pub iec_structure: Vec<String>,
    pub electoral_management: Vec<String>,
    pub election_operations: Vec<String>,
    pub voter_services: Vec<String>,
    pub technology_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralDisputes {
    pub dispute_resolution_mechanisms: Vec<String>,
    pub electoral_court: Vec<String>,
    pub complaint_procedures: Vec<String>,
    pub appeal_processes: Vec<String>,
    pub enforcement_remedies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralReforms {
    pub reform_initiatives: Vec<String>,
    pub stakeholder_consultation: Vec<String>,
    pub implementation_strategies: Vec<String>,
    pub monitoring_evaluation: Vec<String>,
    pub international_assistance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalCodes {
    pub constitutional_law: ConstitutionalLaw,
    pub civil_law: CivilLaw,
    pub criminal_law: CriminalLaw,
    pub administrative_law: AdministrativeLaw,
    pub labour_law: LabourLaw,
    pub commercial_law: CommercialLaw,
    pub environmental_law: EnvironmentalLaw,
    pub customary_law: CustomaryLaw,
    pub procedural_law: ProceduralLaw,
    pub specialized_legislation: SpecializedLegislation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalLaw {
    pub constitutional_text: Vec<String>,
    pub constitutional_jurisprudence: Vec<String>,
    pub constitutional_principles_law: Vec<String>,
    pub fundamental_rights_law: Vec<String>,
    pub constitutional_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilLaw {
    pub law_of_persons: Vec<String>,
    pub family_law: Vec<String>,
    pub law_of_property: Vec<String>,
    pub law_of_succession: Vec<String>,
    pub law_of_contract: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalLaw {
    pub criminal_code: Vec<String>,
    pub common_law_crimes: Vec<String>,
    pub statutory_offenses: Vec<String>,
    pub criminal_procedure: Vec<String>,
    pub sentencing_law: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrativeLaw {
    pub promotion_administrative_justice_act: Vec<String>,
    pub public_administration_law: Vec<String>,
    pub administrative_review: Vec<String>,
    pub just_administrative_action: Vec<String>,
    pub administrative_appeals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabourLaw {
    pub labour_relations_act: Vec<String>,
    pub basic_conditions_employment_act: Vec<String>,
    pub employment_equity_act: Vec<String>,
    pub skills_development_act: Vec<String>,
    pub occupational_health_safety_act: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialLaw {
    pub companies_act: Vec<String>,
    pub competition_act: Vec<String>,
    pub consumer_protection_act: Vec<String>,
    pub insolvency_law: Vec<String>,
    pub intellectual_property_law: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalLaw {
    pub national_environmental_management_act: Vec<String>,
    pub environmental_impact_assessment: Vec<String>,
    pub biodiversity_act: Vec<String>,
    pub water_act: Vec<String>,
    pub mineral_petroleum_resources_development_act: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomaryLaw {
    pub indigenous_law: Vec<String>,
    pub traditional_leadership: Vec<String>,
    pub customary_marriages: Vec<String>,
    pub customary_succession: Vec<String>,
    pub constitutional_recognition: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLaw {
    pub civil_procedure: Vec<String>,
    pub criminal_procedure_detailed: Vec<String>,
    pub administrative_procedure: Vec<String>,
    pub constitutional_procedure: Vec<String>,
    pub evidence_law: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedLegislation {
    pub financial_services_laws: Vec<String>,
    pub telecommunications_laws: Vec<String>,
    pub transport_laws: Vec<String>,
    pub health_laws: Vec<String>,
    pub education_laws: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Municipality {
    pub municipality_name: String,
    pub municipality_type: String,
    pub province: String,
    pub population: u32,
    pub municipal_governance: MunicipalGovernance,
    pub service_delivery: ServiceDelivery,
    pub local_economic_development: LocalEconomicDevelopment,
    pub community_engagement: CommunityEngagement,
    pub financial_management: FinancialManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunicipalGovernance {
    pub council_structure: Vec<String>,
    pub mayoral_system: Vec<String>,
    pub administrative_structure: Vec<String>,
    pub decision_making: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDelivery {
    pub basic_services: Vec<String>,
    pub infrastructure_development: Vec<String>,
    pub social_services: Vec<String>,
    pub environmental_services: Vec<String>,
    pub emergency_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalEconomicDevelopment {
    pub economic_development_strategies: Vec<String>,
    pub business_support: Vec<String>,
    pub tourism_development: Vec<String>,
    pub skills_development: Vec<String>,
    pub infrastructure_investment: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityEngagement {
    pub public_participation: Vec<String>,
    pub ward_committees: Vec<String>,
    pub community_development_workers: Vec<String>,
    pub civic_education: Vec<String>,
    pub feedback_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialManagement {
    pub budget_processes: Vec<String>,
    pub revenue_collection: Vec<String>,
    pub expenditure_management: Vec<String>,
    pub financial_reporting: Vec<String>,
    pub audit_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalAuthority {
    pub traditional_council_name: String,
    pub geographical_area: String,
    pub traditional_leader: String,
    pub recognition_status: String,
    pub customary_law_jurisdiction: CustomaryLawJurisdiction,
    pub community_governance: CommunityGovernance,
    pub land_administration: LandAdministration,
    pub dispute_resolution: DisputeResolution,
    pub cultural_preservation: CulturalPreservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomaryLawJurisdiction {
    pub customary_law_areas: Vec<String>,
    pub traditional_courts: Vec<String>,
    pub customary_practices: Vec<String>,
    pub constitutional_alignment: Vec<String>,
    pub legal_pluralism: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityGovernance {
    pub traditional_governance_structures: Vec<String>,
    pub community_participation_traditional: Vec<String>,
    pub decision_making_traditional: Vec<String>,
    pub accountability_traditional: Vec<String>,
    pub integration_formal_governance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandAdministration {
    pub traditional_land_tenure: Vec<String>,
    pub land_allocation: Vec<String>,
    pub land_use_planning: Vec<String>,
    pub land_disputes_traditional: Vec<String>,
    pub land_reform_integration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolution {
    pub traditional_dispute_resolution: Vec<String>,
    pub mediation_mechanisms: Vec<String>,
    pub restorative_justice_traditional: Vec<String>,
    pub community_sanctions: Vec<String>,
    pub integration_formal_justice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalPreservation {
    pub cultural_heritage_protection: Vec<String>,
    pub traditional_practices_preservation: Vec<String>,
    pub language_preservation: Vec<String>,
    pub oral_traditions: Vec<String>,
    pub cultural_institutions_traditional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageLegalFramework {
    pub official_languages: Vec<String>,
    pub language_rights: LanguageRights,
    pub language_policy: LanguagePolicy,
    pub multilingual_legal_system: MultilingualLegalSystem,
    pub language_planning: LanguagePlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageRights {
    pub constitutional_language_rights: Vec<String>,
    pub language_use_courts: Vec<String>,
    pub language_education_rights: Vec<String>,
    pub language_access_services: Vec<String>,
    pub language_protection_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePolicy {
    pub national_language_policy: Vec<String>,
    pub provincial_language_policies: Vec<String>,
    pub municipal_language_policies: Vec<String>,
    pub institutional_language_policies: Vec<String>,
    pub language_implementation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualLegalSystem {
    pub multilingual_legislation: Vec<String>,
    pub court_interpretation: Vec<String>,
    pub legal_translation: Vec<String>,
    pub multilingual_legal_education: Vec<String>,
    pub language_accessibility_justice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePlanning {
    pub language_development: Vec<String>,
    pub terminology_development: Vec<String>,
    pub language_standardization: Vec<String>,
    pub corpus_planning: Vec<String>,
    pub status_planning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbuntuConstitutionalValues {
    pub ubuntu_philosophy: UbuntuPhilosophy,
    pub restorative_justice: RestorativeJustice,
    pub community_values: CommunityValues,
    pub human_interconnectedness: HumanInterconnectedness,
    pub ubuntu_jurisprudence: UbuntuJurisprudence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbuntuPhilosophy {
    pub ubuntu_principles: Vec<String>,
    pub constitutional_recognition: Vec<String>,
    pub judicial_interpretation: Vec<String>,
    pub community_orientation: Vec<String>,
    pub human_dignity_ubuntu: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorativeJustice {
    pub restorative_principles: Vec<String>,
    pub community_healing: Vec<String>,
    pub victim_offender_mediation: Vec<String>,
    pub community_service: Vec<String>,
    pub reintegration_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityValues {
    pub collective_responsibility: Vec<String>,
    pub social_cohesion: Vec<String>,
    pub mutual_support: Vec<String>,
    pub consensus_decision_making: Vec<String>,
    pub community_solidarity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanInterconnectedness {
    pub interdependence_recognition: Vec<String>,
    pub social_responsibility: Vec<String>,
    pub collective_well_being: Vec<String>,
    pub ubuntu_ethics: Vec<String>,
    pub human_relationships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbuntuJurisprudence {
    pub ubuntu_constitutional_interpretation: Vec<String>,
    pub ubuntu_sentencing: Vec<String>,
    pub ubuntu_dispute_resolution: Vec<String>,
    pub ubuntu_human_rights: Vec<String>,
    pub ubuntu_administrative_justice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionalJustice {
    pub truth_and_reconciliation_commission: TruthAndReconciliationCommission,
    pub transitional_mechanisms: TransitionalMechanisms,
    pub transformation_agenda: TransformationAgenda,
    pub historical_redress: HistoricalRedress,
    pub institutional_reform: InstitutionalReform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthAndReconciliationCommission {
    pub trc_mandate: Vec<String>,
    pub truth_seeking: Vec<String>,
    pub amnesty_provisions: Vec<String>,
    pub reparations_recommendations: Vec<String>,
    pub reconciliation_processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionalMechanisms {
    pub transitional_arrangements: Vec<String>,
    pub interim_constitution: Vec<String>,
    pub constitutional_assembly: Vec<String>,
    pub negotiated_settlement: Vec<String>,
    pub democratic_transition: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationAgenda {
    pub social_transformation: Vec<String>,
    pub economic_transformation: Vec<String>,
    pub institutional_transformation: Vec<String>,
    pub cultural_transformation: Vec<String>,
    pub legal_transformation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalRedress {
    pub apartheid_legacy: Vec<String>,
    pub redress_mechanisms: Vec<String>,
    pub affirmative_action: Vec<String>,
    pub land_restitution: Vec<String>,
    pub reparations_programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalReform {
    pub security_sector_reform: Vec<String>,
    pub judicial_transformation: Vec<String>,
    pub public_service_transformation: Vec<String>,
    pub legislative_reform: Vec<String>,
    pub constitutional_reforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralRightsFramework {
    pub mineral_and_petroleum_resources_development_act: MineralAndPetroleumResourcesDevelopmentAct,
    pub mining_rights: MiningRights,
    pub environmental_mining_regulations: EnvironmentalMiningRegulations,
    pub community_participation_mining: CommunityParticipationMining,
    pub mining_transformation: MiningTransformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralAndPetroleumResourcesDevelopmentAct {
    pub state_custodianship: Vec<String>,
    pub licensing_regime: Vec<String>,
    pub mining_rights_system: Vec<String>,
    pub beneficiation_requirements: Vec<String>,
    pub transformation_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningRights {
    pub prospecting_rights: Vec<String>,
    pub mining_rights_detailed: Vec<String>,
    pub mining_permits: Vec<String>,
    pub exploration_rights: Vec<String>,
    pub production_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalMiningRegulations {
    pub environmental_impact_assessments: Vec<String>,
    pub environmental_management_plans: Vec<String>,
    pub mine_closure_planning: Vec<String>,
    pub rehabilitation_requirements: Vec<String>,
    pub environmental_monitoring: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityParticipationMining {
    pub community_consultation: Vec<String>,
    pub social_labour_plans: Vec<String>,
    pub community_development: Vec<String>,
    pub local_procurement: Vec<String>,
    pub skills_development_mining: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningTransformation {
    pub black_economic_empowerment: Vec<String>,
    pub mining_charter: Vec<String>,
    pub ownership_transformation: Vec<String>,
    pub management_transformation: Vec<String>,
    pub procurement_transformation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandReformFramework {
    pub land_reform_program: LandReformProgram,
    pub land_restitution: LandRestitution,
    pub land_redistribution: LandRedistribution,
    pub tenure_reform: TenureReform,
    pub agricultural_development: AgriculturalDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandReformProgram {
    pub land_reform_objectives: Vec<String>,
    pub three_pillar_approach: Vec<String>,
    pub implementation_mechanisms: Vec<String>,
    pub beneficiary_selection: Vec<String>,
    pub support_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandRestitution {
    pub restitution_process: Vec<String>,
    pub land_claims_court: Vec<String>,
    pub compensation_mechanisms: Vec<String>,
    pub settlement_options: Vec<String>,
    pub post_settlement_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandRedistribution {
    pub redistribution_mechanisms: Vec<String>,
    pub willing_buyer_willing_seller: Vec<String>,
    pub land_acquisition: Vec<String>,
    pub beneficiary_support: Vec<String>,
    pub productive_use_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenureReform {
    pub tenure_security: Vec<String>,
    pub informal_settlements: Vec<String>,
    pub farm_dwellers: Vec<String>,
    pub traditional_tenure: Vec<String>,
    pub urban_land_tenure: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalDevelopment {
    pub agricultural_support: Vec<String>,
    pub farmer_development: Vec<String>,
    pub extension_services: Vec<String>,
    pub market_access: Vec<String>,
    pub agricultural_finance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiIntegrations {
    pub government_apis: Vec<GovernmentApi>,
    pub judicial_apis: Vec<JudicialApi>,
    pub legislative_apis: Vec<LegislativeApi>,
    pub electoral_apis: Vec<ElectoralApi>,
    pub municipal_apis: Vec<MunicipalApi>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentApi {
    pub api_name: String,
    pub department: String,
    pub endpoint_url: String,
    pub data_format: String,
    pub authentication_method: String,
    pub update_frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialApi {
    pub api_name: String,
    pub court_level: String,
    pub endpoint_url: String,
    pub case_data: Vec<String>,
    pub public_access_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeApi {
    pub api_name: String,
    pub legislative_body: String,
    pub endpoint_url: String,
    pub legislative_data: Vec<String>,
    pub historical_coverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralApi {
    pub api_name: String,
    pub electoral_data_type: String,
    pub endpoint_url: String,
    pub real_time_updates: String,
    pub coverage_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunicipalApi {
    pub api_name: String,
    pub municipality: String,
    pub endpoint_url: String,
    pub service_data: Vec<String>,
    pub citizen_access: String,
}

impl SouthAfricaLegalSystem {
    pub fn new() -> Self {
        Self {
            constitutional_framework: ConstitutionalFramework::new(),
            provinces: Self::initialize_provinces(),
            national_government: NationalGovernment::new(),
            judicial_system: JudicialSystem::new(),
            electoral_system: ElectoralSystem::new(),
            legal_codes: LegalCodes::new(),
            municipalities: Self::initialize_municipalities(),
            traditional_authorities: Self::initialize_traditional_authorities(),
            language_legal_framework: LanguageLegalFramework::new(),
            ubuntu_constitutional_values: UbuntuConstitutionalValues::new(),
            transitional_justice: TransitionalJustice::new(),
            mineral_rights_framework: MineralRightsFramework::new(),
            land_reform_framework: LandReformFramework::new(),
            api_integrations: ApiIntegrations::new(),
        }
    }

    fn initialize_provinces() -> Vec<Province> {
        vec![
            Province::new_eastern_cape(),
            Province::new_free_state(),
            Province::new_gauteng(),
            Province::new_kwazulu_natal(),
            Province::new_limpopo(),
            Province::new_mpumalanga(),
            Province::new_northern_cape(),
            Province::new_north_west(),
            Province::new_western_cape(),
        ]
    }

    fn initialize_municipalities() -> Vec<Municipality> {
        vec![
            Municipality::new_city_of_cape_town(),
            Municipality::new_city_of_johannesburg(),
            Municipality::new_ethekwini(),
            Municipality::new_city_of_tshwane(),
            Municipality::new_nelson_mandela_bay(),
        ]
    }

    fn initialize_traditional_authorities() -> Vec<TraditionalAuthority> {
        vec![
            TraditionalAuthority::new_zulu_traditional_authority(),
            TraditionalAuthority::new_xhosa_traditional_authority(),
            TraditionalAuthority::new_sotho_traditional_authority(),
            TraditionalAuthority::new_tswana_traditional_authority(),
            TraditionalAuthority::new_venda_traditional_authority(),
        ]
    }

    pub fn get_complete_legal_framework(&self) -> String {
        format!(
            "SOUTH AFRICA COMPLETE LEGAL SYSTEM\n\
            Constitution: 1996 Constitution, Democratic Republic\n\
            Government: Parliamentary Republic with President as Head of State and Government\n\
            Parliament: Bicameral - National Assembly (400 seats) and National Council of Provinces (90 seats)\n\
            Judiciary: Constitutional Court (apex), Supreme Court of Appeal, High Courts, Magistrates Courts\n\
            Provinces: 9 provinces with significant autonomy\n\
            Municipalities: 3 spheres of government (national, provincial, local)\n\
            Legal Tradition: Mixed system (common law, civil law, customary law)\n\
            Languages: 11 official languages\n\
            Special Features: Ubuntu philosophy, transitional justice, land reform, mineral rights, traditional authorities"
        )
    }
}

impl Default for SouthAfricaLegalSystem {
    fn default() -> Self {
        Self::new()
    }
}