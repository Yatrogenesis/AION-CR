use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurundiLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub post_civil_war_governance: PostCivilWarGovernance,
    pub ethnic_power_sharing: EthnicPowerSharing,
    pub arusha_peace_accord: ArushaPeaceAccord,
    pub government_structure: GovernmentStructure,
    pub judicial_system: JudicialSystem,
    pub provincial_administration: ProvincialAdministration,
    pub truth_reconciliation_commission: TruthReconciliationCommission,
    pub regional_integration: RegionalIntegration,
    pub economic_development: EconomicDevelopment,
    pub agricultural_framework: AgriculturalFramework,
    pub great_lakes_dynamics: GreatLakesDynamics,
    pub language_policy: LanguagePolicy,
    pub vision_2025: Vision2025,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    pub constitution_2018: Constitution2018,
    pub fundamental_rights: FundamentalRights,
    pub separation_of_powers: SeparationOfPowers,
    pub constitutional_court: ConstitutionalCourt,
    pub amendment_procedures: AmendmentProcedures,
    pub emergency_powers: EmergencyPowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution2018 {
    pub promulgation_date: String,
    pub key_principles: Vec<String>,
    pub national_unity: NationalUnity,
    pub democratic_governance: DemocraticGovernance,
    pub rule_of_law: RuleOfLaw,
    pub constitutional_supremacy: ConstitutionalSupremacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCivilWarGovernance {
    pub transition_period: TransitionPeriod,
    pub peace_building: PeaceBuilding,
    pub democratic_transition: DemocraticTransition,
    pub security_sector_reform: SecuritySectorReform,
    pub ex_combatant_reintegration: ExCombatantReintegration,
    pub refugee_return: RefugeeReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthnicPowerSharing {
    pub hutu_tutsi_balance: HutuTutsiBalance,
    pub governmental_quotas: GovernmentalQuotas,
    pub military_integration: MilitaryIntegration,
    pub judicial_representation: JudicialRepresentation,
    pub civil_service_quotas: CivilServiceQuotas,
    pub political_party_regulations: PoliticalPartyRegulations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArushaPeaceAccord {
    pub signing_date: String,
    pub key_provisions: Vec<String>,
    pub implementation_timeline: ImplementationTimeline,
    pub international_monitoring: InternationalMonitoring,
    pub power_sharing_arrangements: PowerSharingArrangements,
    pub ceasefire_agreements: CeasefireAgreements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentStructure {
    pub executive_branch: ExecutiveBranch,
    pub legislative_branch: LegislativeBranch,
    pub senate: Senate,
    pub national_assembly: NationalAssembly,
    pub local_government: LocalGovernment,
    pub civil_service: CivilService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveBranch {
    pub president: President,
    pub vice_presidents: VicePresidents,
    pub council_of_ministers: CouncilOfMinisters,
    pub presidential_powers: PresidentialPowers,
    pub term_limits: TermLimits,
    pub succession_procedures: SuccessionProcedures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialSystem {
    pub supreme_court: SupremeCourt,
    pub constitutional_court: ConstitutionalCourt,
    pub court_of_appeals: CourtOfAppeals,
    pub high_court: HighCourt,
    pub magistrate_courts: MagistrateCourts,
    pub traditional_courts: TraditionalCourts,
    pub gacaca_inspired_courts: GacacaInspiredCourts,
    pub judicial_independence: JudicialIndependence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialAdministration {
    pub provinces: Vec<Province>,
    pub communes: Vec<Commune>,
    pub collines: Vec<Colline>,
    pub decentralization: Decentralization,
    pub local_development: LocalDevelopment,
    pub rural_governance: RuralGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub area_km2: f64,
    pub governor: Governor,
    pub economic_activities: Vec<String>,
    pub development_priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthReconciliationCommission {
    pub establishment: Establishment,
    pub mandate: Mandate,
    pub investigation_powers: InvestigationPowers,
    pub victim_support: VictimSupport,
    pub reparations_program: ReparationsProgram,
    pub national_healing: NationalHealing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalIntegration {
    pub east_african_community: EastAfricanCommunity,
    pub cepgl_revival: CEPGLRevival,
    pub great_lakes_region: GreatLakesRegion,
    pub african_union: AfricanUnion,
    pub regional_trade: RegionalTrade,
    pub cross_border_cooperation: CrossBorderCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicDevelopment {
    pub national_development_plan: NationalDevelopmentPlan,
    pub poverty_reduction: PovertyReduction,
    pub agricultural_transformation: AgriculturalTransformation,
    pub infrastructure_development: InfrastructureDevelopment,
    pub private_sector_development: PrivateSectorDevelopment,
    pub financial_sector: FinancialSector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalFramework {
    pub coffee_sector: CoffeeSector,
    pub tea_production: TeaProduction,
    pub food_security: FoodSecurity,
    pub livestock_development: LivestockDevelopment,
    pub land_reform: LandReform,
    pub cooperative_movement: CooperativeMovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreatLakesDynamics {
    pub regional_security: RegionalSecurity,
    pub refugee_dynamics: RefugeeDynamics,
    pub cross_border_trade: CrossBorderTrade,
    pub environmental_cooperation: EnvironmentalCooperation,
    pub lake_tanganyika: LakeTanganyika,
    pub natural_resources: NaturalResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePolicy {
    pub kirundi_national: KirundiNational,
    pub french_official: FrenchOfficial,
    pub english_introduction: EnglishIntroduction,
    pub swahili_regional: SwahiliRegional,
    pub multilingual_education: MultilingualEducation,
    pub cultural_preservation: CulturalPreservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vision2025 {
    pub development_goals: DevelopmentGoals,
    pub transformation_pillars: TransformationPillars,
    pub human_development: HumanDevelopment,
    pub economic_growth: EconomicGrowth,
    pub governance_improvements: GovernanceImprovements,
    pub regional_positioning: RegionalPositioning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRights {
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub economic_rights: Vec<String>,
    pub social_rights: Vec<String>,
    pub cultural_rights: Vec<String>,
    pub environmental_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparationOfPowers {
    pub executive_powers: Vec<String>,
    pub legislative_powers: Vec<String>,
    pub judicial_powers: Vec<String>,
    pub checks_and_balances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalUnity {
    pub unity_principles: Vec<String>,
    pub ethnic_harmony: EthnicHarmony,
    pub national_identity: NationalIdentity,
    pub social_cohesion: SocialCohesion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticGovernance {
    pub democratic_principles: Vec<String>,
    pub electoral_system: ElectoralSystem,
    pub political_pluralism: PoliticalPluralism,
    pub participatory_democracy: ParticipatoryDemocracy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOfLaw {
    pub legal_supremacy: LegalSupremacy,
    pub equal_justice: EqualJustice,
    pub due_process: DueProcess,
    pub legal_accountability: LegalAccountability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalSupremacy {
    pub constitutional_primacy: ConstitutionalPrimacy,
    pub constitutional_review: ConstitutionalReview,
    pub constitutional_interpretation: ConstitutionalInterpretation,
    pub constitutional_enforcement: ConstitutionalEnforcement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionPeriod {
    pub timeline: String,
    pub key_milestones: Vec<String>,
    pub transitional_institutions: Vec<String>,
    pub international_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeaceBuilding {
    pub reconciliation_programs: Vec<String>,
    pub community_dialogue: CommunityDialogue,
    pub peace_education: PeaceEducation,
    pub conflict_prevention: ConflictPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticTransition {
    pub electoral_reforms: ElectoralReforms,
    pub institutional_building: InstitutionalBuilding,
    pub civil_society_development: CivilSocietyDevelopment,
    pub media_freedom: MediaFreedom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySectorReform {
    pub military_integration: MilitaryIntegration,
    pub police_reform: PoliceReform,
    pub intelligence_services: IntelligenceServices,
    pub security_governance: SecurityGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExCombatantReintegration {
    pub demobilization: Demobilization,
    pub reintegration_programs: ReintegrationPrograms,
    pub vocational_training: VocationalTraining,
    pub economic_opportunities: EconomicOpportunities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefugeeReturn {
    pub return_programs: ReturnPrograms,
    pub property_restoration: PropertyRestoration,
    pub resettlement_support: ResettlementSupport,
    pub integration_assistance: IntegrationAssistance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HutuTutsiBalance {
    pub constitutional_requirements: Vec<String>,
    pub implementation_mechanisms: Vec<String>,
    pub monitoring_systems: Vec<String>,
    pub dispute_resolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentalQuotas {
    pub cabinet_composition: CabinetComposition,
    pub parliamentary_representation: ParliamentaryRepresentation,
    pub provincial_administration: ProvincialAdministration,
    pub public_enterprises: PublicEnterprises,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CabinetComposition {
    pub hutu_percentage: f64,
    pub tutsi_percentage: f64,
    pub gender_requirements: f64,
    pub appointment_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryRepresentation {
    pub assembly_quotas: AssemblyQuotas,
    pub senate_composition: SenateComposition,
    pub committee_balance: CommitteeBalance,
    pub leadership_rotation: LeadershipRotation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyQuotas {
    pub ethnic_balance: EthnicBalance,
    pub gender_quotas: GenderQuotas,
    pub regional_representation: RegionalRepresentation,
    pub youth_inclusion: YouthInclusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateComposition {
    pub ex_presidents: ExPresidents,
    pub ethnic_representatives: EthnicRepresentatives,
    pub regional_delegates: RegionalDelegates,
    pub appointed_members: AppointedMembers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilServiceQuotas {
    pub recruitment_procedures: RecruitmentProcedures,
    pub promotion_systems: PromotionSystems,
    pub training_programs: TrainingPrograms,
    pub performance_evaluation: PerformanceEvaluation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalPartyRegulations {
    pub registration_requirements: RegistrationRequirements,
    pub ethnic_inclusivity: EthnicInclusivity,
    pub financing_rules: FinancingRules,
    pub campaign_regulations: CampaignRegulations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationTimeline {
    pub phases: Vec<Phase>,
    pub milestones: Vec<Milestone>,
    pub monitoring_mechanisms: Vec<String>,
    pub evaluation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    pub name: String,
    pub duration: String,
    pub objectives: Vec<String>,
    pub activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub target_date: String,
    pub indicators: Vec<String>,
    pub responsible_parties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalMonitoring {
    pub monitoring_bodies: Vec<String>,
    pub reporting_mechanisms: Vec<String>,
    pub compliance_assessment: ComplianceAssessment,
    pub corrective_measures: CorrectiveMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSharingArrangements {
    pub executive_sharing: ExecutiveSharing,
    pub legislative_sharing: LegislativeSharing,
    pub judicial_sharing: JudicialSharing,
    pub security_sharing: SecuritySharing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeasefireAgreements {
    pub ceasefire_terms: CeasefireTerms,
    pub monitoring_mechanisms: MonitoringMechanisms,
    pub violation_procedures: ViolationProcedures,
    pub enforcement_measures: EnforcementMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct President {
    pub powers: Vec<String>,
    pub responsibilities: Vec<String>,
    pub term_duration: String,
    pub election_procedure: ElectionProcedure,
    pub impeachment_process: ImpeachmentProcess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VicePresidents {
    pub first_vice_president: FirstVicePresident,
    pub second_vice_president: SecondVicePresident,
    pub ethnic_balance_requirement: EthnicBalanceRequirement,
    pub succession_order: SuccessionOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstVicePresident {
    pub roles: Vec<String>,
    pub appointment_process: AppointmentProcess,
    pub ethnic_requirement: String,
    pub coordination_responsibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondVicePresident {
    pub roles: Vec<String>,
    pub appointment_process: AppointmentProcess,
    pub ethnic_requirement: String,
    pub coordination_responsibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilOfMinisters {
    pub composition: Composition,
    pub appointment_procedures: AppointmentProcedures,
    pub collective_responsibility: CollectiveResponsibility,
    pub ethnic_balance: EthnicBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    pub total_ministers: u32,
    pub hutu_ministers: u32,
    pub tutsi_ministers: u32,
    pub female_ministers: u32,
    pub portfolio_distribution: PortfolioDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioDistribution {
    pub key_ministries: Vec<KeyMinistry>,
    pub technical_ministries: Vec<TechnicalMinistry>,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMinistry {
    pub name: String,
    pub responsibilities: Vec<String>,
    pub ethnic_consideration: String,
    pub coordination_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalMinistry {
    pub name: String,
    pub technical_focus: Vec<String>,
    pub coordination_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeBranch {
    pub bicameral_structure: BicameralStructure,
    pub legislative_process: LegislativeProcess,
    pub oversight_functions: OversightFunctions,
    pub ethnic_representation: EthnicRepresentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicameralStructure {
    pub national_assembly: NationalAssembly,
    pub senate: Senate,
    pub joint_sessions: JointSessions,
    pub coordination_mechanisms: CoordinationMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalAssembly {
    pub composition: AssemblyComposition,
    pub powers: Vec<String>,
    pub committees: Vec<Committee>,
    pub election_system: ElectionSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyComposition {
    pub total_seats: u32,
    pub directly_elected: u32,
    pub women_seats: u32,
    pub youth_representatives: u32,
    pub ethnic_balance: EthnicBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Committee {
    pub name: String,
    pub mandate: Vec<String>,
    pub composition: CommitteeComposition,
    pub working_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeComposition {
    pub total_members: u32,
    pub chairperson_rotation: String,
    pub ethnic_representation: String,
    pub gender_balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Senate {
    pub composition: SenateComposition,
    pub powers: Vec<String>,
    pub special_functions: Vec<String>,
    pub appointment_procedures: SenateAppointmentProcedures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateAppointmentProcedures {
    pub ex_presidents: ExPresidentsAppointment,
    pub ethnic_representatives: EthnicRepresentativesAppointment,
    pub regional_representation: RegionalRepresentationAppointment,
    pub presidential_appointments: PresidentialAppointments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGovernment {
    pub provincial_level: ProvincialLevel,
    pub communal_level: CommunalLevel,
    pub colline_level: CollineLevel,
    pub decentralization_policy: DecentralizationPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialLevel {
    pub governors: Vec<Governor>,
    pub provincial_councils: Vec<ProvincialCouncil>,
    pub development_planning: DevelopmentPlanning,
    pub service_delivery: ServiceDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Governor {
    pub appointment_process: String,
    pub powers: Vec<String>,
    pub accountability_mechanisms: Vec<String>,
    pub coordination_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialCouncil {
    pub composition: String,
    pub powers: Vec<String>,
    pub relationship_with_governor: String,
    pub development_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunalLevel {
    pub administrators: Vec<Administrator>,
    pub communal_councils: Vec<CommunalCouncil>,
    pub service_provision: ServiceProvision,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Administrator {
    pub appointment_process: String,
    pub responsibilities: Vec<String>,
    pub accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunalCouncil {
    pub election_process: String,
    pub composition: String,
    pub powers: Vec<String>,
    pub oversight_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollineLevel {
    pub traditional_structure: TraditionalStructure,
    pub modern_administration: ModernAdministration,
    pub community_participation: CommunityParticipation,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourt {
    pub composition: SupremeCourtComposition,
    pub jurisdiction: Jurisdiction,
    pub appointment_process: JudicialAppointmentProcess,
    pub ethnic_balance: JudicialEthnicBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourtComposition {
    pub chief_justice: ChiefJustice,
    pub associate_justices: Vec<AssociateJustice>,
    pub ethnic_requirements: EthnicRequirements,
    pub gender_considerations: GenderConsiderations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChiefJustice {
    pub appointment_process: String,
    pub term_duration: String,
    pub powers: Vec<String>,
    pub administrative_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociateJustice {
    pub appointment_criteria: Vec<String>,
    pub specialization_areas: Vec<String>,
    pub tenure_security: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCourt {
    pub jurisdiction: ConstitutionalJurisdiction,
    pub composition: ConstitutionalCourtComposition,
    pub procedures: ConstitutionalProcedures,
    pub independence_guarantees: IndependenceGuarantees,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalJurisdiction {
    pub constitutional_review: ConstitutionalReview,
    pub electoral_disputes: ElectoralDisputes,
    pub rights_protection: RightsProtection,
    pub institutional_conflicts: InstitutionalConflicts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalCourts {
    pub bashingantahe: Bashingantahe,
    pub customary_law: CustomaryLaw,
    pub conflict_resolution: TraditionalConflictResolution,
    pub integration_mechanisms: IntegrationMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bashingantahe {
    pub traditional_role: TraditionalRole,
    pub modern_adaptation: ModernAdaptation,
    pub selection_process: SelectionProcess,
    pub community_recognition: CommunityRecognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalRole {
    pub historical_function: Vec<String>,
    pub wisdom_keepers: WisdomKeepers,
    pub mediation_role: MediationRole,
    pub cultural_preservation: CulturalPreservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernAdaptation {
    pub formal_recognition: FormalRecognition,
    pub training_programs: TrainingPrograms,
    pub coordination_mechanisms: CoordinationMechanisms,
    pub oversight_systems: OversightSystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GacacaInspiredCourts {
    pub community_justice: CommunityJustice,
    pub reconciliation_focus: ReconciliationFocus,
    pub victim_participation: VictimParticipation,
    pub truth_seeking: TruthSeeking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityJustice {
    pub participatory_approach: ParticipatoryApproach,
    pub local_ownership: LocalOwnership,
    pub accessible_procedures: AccessibleProcedures,
    pub community_healing: CommunityHealing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commune {
    pub name: String,
    pub province: String,
    pub population: u64,
    pub area_km2: f64,
    pub administrator: String,
    pub economic_profile: EconomicProfile,
    pub development_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicProfile {
    pub main_activities: Vec<String>,
    pub agricultural_production: AgriculturalProduction,
    pub market_access: MarketAccess,
    pub employment_patterns: EmploymentPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalProduction {
    pub crops: Vec<String>,
    pub livestock: Vec<String>,
    pub productivity_levels: ProductivityLevels,
    pub value_chains: ValueChains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketAccess {
    pub local_markets: Vec<String>,
    pub regional_markets: Vec<String>,
    pub transportation_infrastructure: TransportationInfrastructure,
    pub market_information_systems: MarketInformationSystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colline {
    pub name: String,
    pub commune: String,
    pub population: u64,
    pub households: u32,
    pub chief: String,
    pub development_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decentralization {
    pub policy_framework: PolicyFramework,
    pub fiscal_decentralization: FiscalDecentralization,
    pub administrative_decentralization: AdministrativeDecentralization,
    pub capacity_building: CapacityBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFramework {
    pub legal_instruments: Vec<String>,
    pub implementation_guidelines: Vec<String>,
    pub monitoring_systems: Vec<String>,
    pub evaluation_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalDecentralization {
    pub revenue_sharing: RevenueSharing,
    pub local_taxation: LocalTaxation,
    pub budget_autonomy: BudgetAutonomy,
    pub financial_management: FinancialManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrativeDecentralization {
    pub service_delivery: ServiceDelivery,
    pub human_resources: HumanResources,
    pub coordination_mechanisms: CoordinationMechanisms,
    pub accountability_systems: AccountabilitySystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDevelopment {
    pub community_driven_development: CommunityDrivenDevelopment,
    pub participatory_planning: ParticipatoryPlanning,
    pub local_economic_development: LocalEconomicDevelopment,
    pub infrastructure_development: InfrastructureDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDrivenDevelopment {
    pub project_identification: ProjectIdentification,
    pub community_participation: CommunityParticipation,
    pub implementation_modalities: ImplementationModalities,
    pub sustainability_measures: SustainabilityMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipatoryPlanning {
    pub planning_processes: Vec<String>,
    pub stakeholder_engagement: StakeholderEngagement,
    pub priority_setting: PrioritySetting,
    pub resource_allocation: ResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalEconomicDevelopment {
    pub value_chain_development: ValueChainDevelopment,
    pub sme_support: SMESupport,
    pub cooperative_development: CooperativeDevelopment,
    pub market_linkages: MarketLinkages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuralGovernance {
    pub traditional_authorities: TraditionalAuthorities,
    pub modern_institutions: ModernInstitutions,
    pub conflict_resolution_mechanisms: ConflictResolutionMechanisms,
    pub community_mobilization: CommunityMobilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalAuthorities {
    pub chiefs: Vec<Chief>,
    pub elders: Vec<Elder>,
    pub traditional_councils: Vec<TraditionalCouncil>,
    pub customary_practices: CustomaryPractices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chief {
    pub name: String,
    pub area: String,
    pub traditional_role: String,
    pub modern_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elder {
    pub name: String,
    pub area: String,
    pub expertise: Vec<String>,
    pub advisory_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalCouncil {
    pub composition: String,
    pub functions: Vec<String>,
    pub decision_making: String,
    pub dispute_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomaryPractices {
    pub land_management: LandManagement,
    pub marriage_customs: MarriageCustoms,
    pub inheritance_rules: InheritanceRules,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandManagement {
    pub customary_tenure: CustomaryTenure,
    pub allocation_mechanisms: AllocationMechanisms,
    pub dispute_resolution: LandDisputeResolution,
    pub environmental_protection: EnvironmentalProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarriageCustoms {
    pub traditional_ceremonies: TraditionalCeremonies,
    pub bride_price: BridePrice,
    pub family_obligations: FamilyObligations,
    pub modern_adaptations: ModernAdaptations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceRules {
    pub patrilineal_system: PatrilinealSystem,
    pub property_distribution: PropertyDistribution,
    pub widow_protection: WidowProtection,
    pub dispute_mechanisms: DisputeMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatrilinealSystem {
    pub principles: Vec<String>,
    pub modern_challenges: Vec<String>,
    pub adaptations: Vec<String>,
    pub gender_considerations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDistribution {
    pub land_inheritance: LandInheritance,
    pub livestock_inheritance: LivestockInheritance,
    pub household_property: HouseholdProperty,
    pub commercial_assets: CommercialAssets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandInheritance {
    pub traditional_rules: Vec<String>,
    pub modern_complications: Vec<String>,
    pub resolution_mechanisms: Vec<String>,
    pub gender_equity: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Establishment {
    pub legal_basis: LegalBasis,
    pub institutional_framework: InstitutionalFramework,
    pub mandate_definition: MandateDefinition,
    pub operational_principles: OperationalPrinciples,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalBasis {
    pub constitutional_provisions: Vec<String>,
    pub enabling_legislation: Vec<String>,
    pub international_standards: Vec<String>,
    pub transitional_justice_framework: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalFramework {
    pub organizational_structure: OrganizationalStructure,
    pub governance_mechanisms: GovernanceMechanisms,
    pub independence_guarantees: IndependenceGuarantees,
    pub accountability_measures: AccountabilityMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationalStructure {
    pub commissioners: Vec<Commissioner>,
    pub secretariat: Secretariat,
    pub regional_offices: Vec<RegionalOffice>,
    pub advisory_bodies: Vec<AdvisoryBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commissioner {
    pub name: String,
    pub background: String,
    pub expertise: Vec<String>,
    pub appointment_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secretariat {
    pub executive_secretary: ExecutiveSecretary,
    pub departments: Vec<Department>,
    pub staff_composition: StaffComposition,
    pub operational_units: Vec<OperationalUnit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSecretary {
    pub qualifications: Vec<String>,
    pub responsibilities: Vec<String>,
    pub appointment_terms: String,
    pub reporting_relationships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub name: String,
    pub mandate: Vec<String>,
    pub staff_size: u32,
    pub specialized_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffComposition {
    pub total_staff: u32,
    pub professional_staff: u32,
    pub support_staff: u32,
    pub ethnic_balance: String,
    pub gender_balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalUnit {
    pub name: String,
    pub function: Vec<String>,
    pub geographic_coverage: String,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalOffice {
    pub location: String,
    pub coverage_area: Vec<String>,
    pub staff_size: u32,
    pub local_partnerships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisoryBody {
    pub name: String,
    pub composition: String,
    pub expertise_areas: Vec<String>,
    pub advisory_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    pub truth_seeking: TruthSeeking,
    pub reconciliation_promotion: ReconciliationPromotion,
    pub reparations_recommendation: ReparationsRecommendation,
    pub institutional_reform: InstitutionalReform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthSeeking {
    pub investigation_scope: InvestigationScope,
    pub evidence_collection: EvidenceCollection,
    pub victim_testimony: VictimTestimony,
    pub perpetrator_confessions: PerpetratorConfessions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationScope {
    pub temporal_mandate: TemporalMandate,
    pub geographic_coverage: GeographicCoverage,
    pub violation_categories: ViolationCategories,
    pub institutional_focus: InstitutionalFocus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMandate {
    pub start_date: String,
    pub end_date: String,
    pub key_periods: Vec<String>,
    pub periodization_rationale: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicCoverage {
    pub national_scope: String,
    pub provincial_focus: Vec<String>,
    pub cross_border_dimensions: Vec<String>,
    pub regional_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationCategories {
    pub gross_human_rights_violations: Vec<String>,
    pub violations_international_humanitarian_law: Vec<String>,
    pub economic_crimes: Vec<String>,
    pub cultural_violations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalFocus {
    pub state_institutions: Vec<String>,
    pub armed_groups: Vec<String>,
    pub political_parties: Vec<String>,
    pub civil_society: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCollection {
    pub documentary_evidence: DocumentaryEvidence,
    pub testimonial_evidence: TestimonialEvidence,
    pub forensic_evidence: ForensicEvidence,
    pub digital_evidence: DigitalEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentaryEvidence {
    pub official_documents: Vec<String>,
    pub personal_documents: Vec<String>,
    pub media_reports: Vec<String>,
    pub organizational_records: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestimonialEvidence {
    pub victim_statements: VictimStatements,
    pub witness_testimonies: WitnessTestimonies,
    pub expert_testimonies: ExpertTestimonies,
    pub perpetrator_statements: PerpetratorStatements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimStatements {
    pub collection_procedures: Vec<String>,
    pub protection_measures: Vec<String>,
    pub verification_methods: Vec<String>,
    pub confidentiality_protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessTestimonies {
    pub recruitment_strategies: Vec<String>,
    pub interview_protocols: Vec<String>,
    pub corroboration_methods: Vec<String>,
    pub protection_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertTestimonies {
    pub expertise_areas: Vec<String>,
    pub qualification_criteria: Vec<String>,
    pub testimony_procedures: Vec<String>,
    pub independence_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetratorStatements {
    pub voluntary_confessions: VoluntaryConfessions,
    pub incentive_mechanisms: IncentiveMechanisms,
    pub verification_procedures: VerificationProcedures,
    pub protection_considerations: ProtectionConsiderations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoluntaryConfessions {
    pub confession_procedures: Vec<String>,
    pub verification_methods: Vec<String>,
    pub victim_notification: Vec<String>,
    pub reconciliation_dimension: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncentiveMechanisms {
    pub truth_telling_incentives: Vec<String>,
    pub cooperation_benefits: Vec<String>,
    pub conditional_amnesty: Vec<String>,
    pub prosecution_considerations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProcedures {
    pub cross_referencing: Vec<String>,
    pub independent_verification: Vec<String>,
    pub expert_analysis: Vec<String>,
    pub victim_consultation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionConsiderations {
    pub security_measures: Vec<String>,
    pub confidentiality_protocols: Vec<String>,
    pub witness_protection: Vec<String>,
    pub family_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicEvidence {
    pub mass_grave_investigations: MassGraveInvestigations,
    pub ballistics_analysis: BallisticsAnalysis,
    pub dna_analysis: DNAAnalysis,
    pub site_investigations: SiteInvestigations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassGraveInvestigations {
    pub identification_procedures: Vec<String>,
    pub exhumation_protocols: Vec<String>,
    pub victim_identification: Vec<String>,
    pub family_notification: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallisticsAnalysis {
    pub weapon_identification: Vec<String>,
    pub trajectory_analysis: Vec<String>,
    pub ammunition_tracing: Vec<String>,
    pub crime_scene_reconstruction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAAnalysis {
    pub sample_collection: Vec<String>,
    pub laboratory_procedures: Vec<String>,
    pub database_management: Vec<String>,
    pub family_matching: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInvestigations {
    pub crime_scene_analysis: Vec<String>,
    pub physical_evidence: Vec<String>,
    pub environmental_analysis: Vec<String>,
    pub photographic_documentation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalEvidence {
    pub electronic_communications: Vec<String>,
    pub digital_documents: Vec<String>,
    pub metadata_analysis: Vec<String>,
    pub chain_of_custody: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimTestimony {
    pub testimony_collection: TestimonyCollection,
    pub victim_protection: VictimProtection,
    pub psychological_support: PsychologicalSupport,
    pub participation_mechanisms: ParticipationMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestimonyCollection {
    pub public_hearings: PublicHearings,
    pub private_sessions: PrivateSessions,
    pub community_dialogues: CommunityDialogues,
    pub written_submissions: WrittenSubmissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicHearings {
    pub selection_criteria: Vec<String>,
    pub preparation_process: Vec<String>,
    pub media_coverage: Vec<String>,
    pub symbolic_value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateSessions {
    pub confidentiality_measures: Vec<String>,
    pub sensitive_cases: Vec<String>,
    pub protection_protocols: Vec<String>,
    pub documentation_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDialogues {
    pub dialogue_format: Vec<String>,
    pub community_preparation: Vec<String>,
    pub facilitation_methods: Vec<String>,
    pub outcome_documentation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrittenSubmissions {
    pub submission_guidelines: Vec<String>,
    pub processing_procedures: Vec<String>,
    pub verification_methods: Vec<String>,
    pub integration_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimProtection {
    pub physical_security: PhysicalSecurity,
    pub psychological_safety: PsychologicalSafety,
    pub legal_protection: LegalProtection,
    pub social_support: SocialSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSecurity {
    pub threat_assessment: Vec<String>,
    pub security_measures: Vec<String>,
    pub relocation_support: Vec<String>,
    pub coordination_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalSafety {
    pub trauma_awareness: Vec<String>,
    pub safe_spaces: Vec<String>,
    pub counseling_support: Vec<String>,
    pub peer_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProtection {
    pub legal_representation: Vec<String>,
    pub procedural_rights: Vec<String>,
    pub confidentiality_rights: Vec<String>,
    pub remedy_access: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSupport {
    pub community_support: Vec<String>,
    pub family_support: Vec<String>,
    pub economic_support: Vec<String>,
    pub social_reintegration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalSupport {
    pub trauma_counseling: TraumaCounseling,
    pub group_therapy: GroupTherapy,
    pub family_counseling: FamilyCounseling,
    pub community_healing: CommunityHealing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaCounseling {
    pub individual_sessions: Vec<String>,
    pub specialized_therapies: Vec<String>,
    pub cultural_approaches: Vec<String>,
    pub long_term_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTherapy {
    pub peer_support_groups: Vec<String>,
    pub therapeutic_communities: Vec<String>,
    pub healing_circles: Vec<String>,
    pub collective_processing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyCounseling {
    pub family_therapy: Vec<String>,
    pub intergenerational_healing: Vec<String>,
    pub family_mediation: Vec<String>,
    pub relationship_repair: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationMechanisms {
    pub victim_representation: VictimRepresentation,
    pub consultation_processes: ConsultationProcesses,
    pub feedback_mechanisms: FeedbackMechanisms,
    pub empowerment_strategies: EmpowermentStrategies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimRepresentation {
    pub victim_organizations: Vec<String>,
    pub representative_selection: Vec<String>,
    pub consultation_mechanisms: Vec<String>,
    pub decision_making_participation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationProcesses {
    pub policy_consultation: Vec<String>,
    pub procedure_consultation: Vec<String>,
    pub outcome_consultation: Vec<String>,
    pub implementation_consultation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackMechanisms {
    pub regular_updates: Vec<String>,
    pub progress_reports: Vec<String>,
    pub grievance_procedures: Vec<String>,
    pub adjustment_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpowermentStrategies {
    pub capacity_building: Vec<String>,
    pub skills_development: Vec<String>,
    pub leadership_development: Vec<String>,
    pub organizational_strengthening: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetratorConfessions {
    pub confession_procedures: ConfessionProcedures,
    pub verification_mechanisms: VerificationMechanisms,
    pub reconciliation_processes: ReconciliationProcesses,
    pub accountability_measures: AccountabilityMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfessionProcedures {
    pub voluntary_basis: Vec<String>,
    pub procedural_safeguards: Vec<String>,
    pub legal_implications: Vec<String>,
    pub victim_notification: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationPromotion {
    pub dialogue_facilitation: DialogueFacilitation,
    pub community_reconciliation: CommunityReconciliation,
    pub institutional_reconciliation: InstitutionalReconciliation,
    pub national_reconciliation: NationalReconciliation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueFacilitation {
    pub victim_perpetrator_dialogue: VictimPerpetratorDialogue,
    pub community_dialogue: CommunityDialogue,
    pub inter_group_dialogue: InterGroupDialogue,
    pub national_dialogue: NationalDialogue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimPerpetratorDialogue {
    pub dialogue_format: Vec<String>,
    pub preparation_process: Vec<String>,
    pub facilitation_methods: Vec<String>,
    pub outcome_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDialogue {
    pub community_preparation: Vec<String>,
    pub dialogue_sessions: Vec<String>,
    pub follow_up_activities: Vec<String>,
    pub reconciliation_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterGroupDialogue {
    pub group_identification: Vec<String>,
    pub dialogue_design: Vec<String>,
    pub conflict_transformation: Vec<String>,
    pub relationship_building: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalDialogue {
    pub dialogue_framework: Vec<String>,
    pub stakeholder_participation: Vec<String>,
    pub agenda_setting: Vec<String>,
    pub consensus_building: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityReconciliation {
    pub local_reconciliation: LocalReconciliation,
    pub traditional_mechanisms: TraditionalMechanisms,
    pub modern_approaches: ModernApproaches,
    pub hybrid_models: HybridModels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalReconciliation {
    pub community_initiatives: Vec<String>,
    pub local_leadership: Vec<String>,
    pub grassroots_movements: Vec<String>,
    pub bottom_up_approaches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalMechanisms {
    pub customary_practices: Vec<String>,
    pub traditional_authorities: Vec<String>,
    pub ritual_ceremonies: Vec<String>,
    pub cultural_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernApproaches {
    pub professional_mediation: Vec<String>,
    pub therapeutic_interventions: Vec<String>,
    pub educational_programs: Vec<String>,
    pub civic_engagement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridModels {
    pub traditional_modern_integration: Vec<String>,
    pub complementary_approaches: Vec<String>,
    pub contextual_adaptation: Vec<String>,
    pub synergistic_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalReconciliation {
    pub institutional_dialogue: InstitutionalDialogue,
    pub reform_processes: ReformProcesses,
    pub capacity_building: CapacityBuilding,
    pub partnership_development: PartnershipDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalDialogue {
    pub inter_institutional_dialogue: Vec<String>,
    pub institutional_accountability: Vec<String>,
    pub institutional_transformation: Vec<String>,
    pub institutional_cooperation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReformProcesses {
    pub institutional_reform: Vec<String>,
    pub policy_reform: Vec<String>,
    pub procedural_reform: Vec<String>,
    pub cultural_reform: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipDevelopment {
    pub institutional_partnerships: Vec<String>,
    pub civil_society_partnerships: Vec<String>,
    pub international_partnerships: Vec<String>,
    pub multi_stakeholder_partnerships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalReconciliation {
    pub national_narrative: NationalNarrative,
    pub shared_vision: SharedVision,
    pub collective_memory: CollectiveMemory,
    pub national_healing: NationalHealing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalNarrative {
    pub historical_narrative: Vec<String>,
    pub reconciliation_narrative: Vec<String>,
    pub future_vision: Vec<String>,
    pub identity_construction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedVision {
    pub vision_development: Vec<String>,
    pub stakeholder_consensus: Vec<String>,
    pub vision_articulation: Vec<String>,
    pub vision_implementation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMemory {
    pub memory_construction: Vec<String>,
    pub commemoration_practices: Vec<String>,
    pub historical_preservation: Vec<String>,
    pub intergenerational_transmission: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReparationsRecommendation {
    pub reparations_framework: ReparationsFramework,
    pub beneficiary_identification: BeneficiaryIdentification,
    pub reparations_measures: ReparationsMeasures,
    pub implementation_mechanisms: ImplementationMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReparationsFramework {
    pub conceptual_framework: ConceptualFramework,
    pub legal_framework: LegalFramework,
    pub policy_framework: PolicyFramework,
    pub operational_framework: OperationalFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualFramework {
    pub reparations_principles: Vec<String>,
    pub victim_centered_approach: Vec<String>,
    pub transformative_dimension: Vec<String>,
    pub holistic_approach: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryIdentification {
    pub victim_definition: VictimDefinition,
    pub eligibility_criteria: EligibilityCriteria,
    pub registration_procedures: RegistrationProcedures,
    pub verification_mechanisms: VerificationMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimDefinition {
    pub direct_victims: Vec<String>,
    pub indirect_victims: Vec<String>,
    pub collective_victims: Vec<String>,
    pub victim_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityCriteria {
    pub temporal_criteria: Vec<String>,
    pub geographic_criteria: Vec<String>,
    pub harm_criteria: Vec<String>,
    pub causation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationProcedures {
    pub registration_process: Vec<String>,
    pub documentation_requirements: Vec<String>,
    pub assistance_mechanisms: Vec<String>,
    pub appeal_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReparationsMeasures {
    pub restitution: Restitution,
    pub compensation: Compensation,
    pub rehabilitation: Rehabilitation,
    pub satisfaction: Satisfaction,
    pub guarantees_non_repetition: GuaranteesNonRepetition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restitution {
    pub property_restitution: PropertyRestitution,
    pub rights_restoration: RightsRestoration,
    pub status_restoration: StatusRestoration,
    pub relationship_restoration: RelationshipRestoration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyRestitution {
    pub land_restitution: LandRestitution,
    pub housing_restitution: HousingRestitution,
    pub business_restitution: BusinessRestitution,
    pub personal_property: PersonalProperty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsRestoration {
    pub citizenship_rights: Vec<String>,
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub social_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusRestoration {
    pub social_status: Vec<String>,
    pub professional_status: Vec<String>,
    pub family_status: Vec<String>,
    pub community_status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRestoration {
    pub family_relationships: Vec<String>,
    pub community_relationships: Vec<String>,
    pub social_relationships: Vec<String>,
    pub institutional_relationships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compensation {
    pub monetary_compensation: MonetaryCompensation,
    pub in_kind_compensation: InKindCompensation,
    pub symbolic_compensation: SymbolicCompensation,
    pub collective_compensation: CollectiveCompensation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryCompensation {
    pub individual_payments: Vec<String>,
    pub calculation_methods: Vec<String>,
    pub payment_mechanisms: Vec<String>,
    pub financial_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InKindCompensation {
    pub goods_provision: Vec<String>,
    pub services_provision: Vec<String>,
    pub infrastructure_provision: Vec<String>,
    pub livelihood_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicCompensation {
    pub memorialization: Vec<String>,
    pub public_acknowledgment: Vec<String>,
    pub commemorative_events: Vec<String>,
    pub symbolic_gestures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveCompensation {
    pub community_projects: Vec<String>,
    pub infrastructure_development: Vec<String>,
    pub social_programs: Vec<String>,
    pub institutional_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rehabilitation {
    pub medical_rehabilitation: MedicalRehabilitation,
    pub psychological_rehabilitation: PsychologicalRehabilitation,
    pub social_rehabilitation: SocialRehabilitation,
    pub economic_rehabilitation: EconomicRehabilitation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalRehabilitation {
    pub healthcare_provision: Vec<String>,
    pub specialized_treatment: Vec<String>,
    pub prosthetic_services: Vec<String>,
    pub long_term_care: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalRehabilitation {
    pub trauma_treatment: Vec<String>,
    pub mental_health_services: Vec<String>,
    pub psychosocial_support: Vec<String>,
    pub community_based_care: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRehabilitation {
    pub social_reintegration: Vec<String>,
    pub community_acceptance: Vec<String>,
    pub social_support_networks: Vec<String>,
    pub dignity_restoration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicRehabilitation {
    pub livelihood_restoration: Vec<String>,
    pub skills_development: Vec<String>,
    pub microfinance_access: Vec<String>,
    pub employment_opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satisfaction {
    pub truth_disclosure: Vec<String>,
    pub public_acknowledgment: Vec<String>,
    pub official_apologies: Vec<String>,
    pub accountability_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuaranteesNonRepetition {
    pub institutional_reform: Vec<String>,
    pub legal_reform: Vec<String>,
    pub security_sector_reform: Vec<String>,
    pub education_reform: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationPowers {
    pub subpoena_power: SubpoenaPower,
    pub search_seizure: SearchSeizure,
    pub witness_protection: WitnessProtection,
    pub international_cooperation: InternationalCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubpoenaPower {
    pub document_subpoenas: Vec<String>,
    pub witness_subpoenas: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
    pub legal_protections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSeizure {
    pub search_warrants: Vec<String>,
    pub evidence_seizure: Vec<String>,
    pub property_preservation: Vec<String>,
    pub chain_of_custody: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessProtection {
    pub protection_measures: Vec<String>,
    pub threat_assessment: Vec<String>,
    pub relocation_support: Vec<String>,
    pub identity_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimSupport {
    pub support_services: SupportServices,
    pub victim_assistance: VictimAssistance,
    pub empowerment_programs: EmpowermentPrograms,
    pub community_support: CommunitySupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportServices {
    pub counseling_services: Vec<String>,
    pub legal_assistance: Vec<String>,
    pub medical_support: Vec<String>,
    pub social_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimAssistance {
    pub immediate_assistance: Vec<String>,
    pub medium_term_support: Vec<String>,
    pub long_term_support: Vec<String>,
    pub specialized_assistance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReparationsProgram {
    pub program_design: ProgramDesign,
    pub implementation_strategy: ImplementationStrategy,
    pub monitoring_evaluation: MonitoringEvaluation,
    pub sustainability_measures: SustainabilityMeasures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDesign {
    pub needs_assessment: Vec<String>,
    pub program_components: Vec<String>,
    pub delivery_mechanisms: Vec<String>,
    pub quality_standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStrategy {
    pub phased_implementation: Vec<String>,
    pub partnership_approach: Vec<String>,
    pub capacity_building: Vec<String>,
    pub resource_mobilization: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringEvaluation {
    pub monitoring_framework: Vec<String>,
    pub evaluation_methods: Vec<String>,
    pub feedback_mechanisms: Vec<String>,
    pub learning_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EastAfricanCommunity {
    pub membership_benefits: Vec<String>,
    pub integration_challenges: Vec<String>,
    pub trade_facilitation: TradeFacilitation,
    pub labor_mobility: LaborMobility,
    pub infrastructure_integration: InfrastructureIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFacilitation {
    pub customs_union: CustomsUnion,
    pub common_market: CommonMarket,
    pub trade_barriers: TradeBarriers,
    pub cross_border_trade: CrossBorderTrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsUnion {
    pub common_external_tariff: Vec<String>,
    pub internal_trade_liberalization: Vec<String>,
    pub customs_procedures: Vec<String>,
    pub revenue_sharing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMarket {
    pub free_movement_goods: Vec<String>,
    pub free_movement_services: Vec<String>,
    pub free_movement_capital: Vec<String>,
    pub free_movement_persons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeBarriers {
    pub non_tariff_barriers: Vec<String>,
    pub technical_barriers: Vec<String>,
    pub administrative_barriers: Vec<String>,
    pub elimination_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborMobility {
    pub work_permits: Vec<String>,
    pub professional_recognition: Vec<String>,
    pub skills_mobility: Vec<String>,
    pub migration_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureIntegration {
    pub transport_corridors: Vec<String>,
    pub energy_integration: Vec<String>,
    pub telecommunications: Vec<String>,
    pub water_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CEPGLRevival {
    pub economic_community: EconomicCommunity,
    pub great_lakes_cooperation: GreatLakesCooperation,
    pub conflict_prevention: ConflictPrevention,
    pub post_conflict_reconstruction: PostConflictReconstruction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicCommunity {
    pub trade_promotion: Vec<String>,
    pub investment_facilitation: Vec<String>,
    pub monetary_cooperation: Vec<String>,
    pub development_financing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreatLakesCooperation {
    pub regional_dialogue: Vec<String>,
    pub confidence_building: Vec<String>,
    pub cross_border_initiatives: Vec<String>,
    pub shared_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostConflictReconstruction {
    pub reconstruction_programs: Vec<String>,
    pub reconciliation_initiatives: Vec<String>,
    pub institutional_building: Vec<String>,
    pub capacity_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreatLakesRegion {
    pub regional_initiatives: Vec<String>,
    pub peace_building: Vec<String>,
    pub economic_cooperation: Vec<String>,
    pub environmental_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfricanUnion {
    pub continental_integration: Vec<String>,
    pub peace_security: Vec<String>,
    pub development_agenda: Vec<String>,
    pub governance_initiatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTrade {
    pub intra_regional_trade: Vec<String>,
    pub trade_financing: Vec<String>,
    pub value_chain_development: Vec<String>,
    pub market_integration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBorderCooperation {
    pub border_management: Vec<String>,
    pub joint_projects: Vec<String>,
    pub shared_services: Vec<String>,
    pub conflict_resolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalDevelopmentPlan {
    pub development_vision: Vec<String>,
    pub strategic_objectives: Vec<String>,
    pub implementation_framework: Vec<String>,
    pub monitoring_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PovertyReduction {
    pub poverty_analysis: Vec<String>,
    pub poverty_reduction_strategies: Vec<String>,
    pub social_protection: Vec<String>,
    pub inclusive_growth: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalTransformation {
    pub modernization_programs: Vec<String>,
    pub technology_adoption: Vec<String>,
    pub value_addition: Vec<String>,
    pub market_linkages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureDevelopment {
    pub transport_infrastructure: Vec<String>,
    pub energy_infrastructure: Vec<String>,
    pub water_infrastructure: Vec<String>,
    pub ict_infrastructure: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateSectorDevelopment {
    pub business_environment: Vec<String>,
    pub sme_development: Vec<String>,
    pub investment_promotion: Vec<String>,
    pub entrepreneurship: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialSector {
    pub banking_sector: Vec<String>,
    pub microfinance: Vec<String>,
    pub capital_markets: Vec<String>,
    pub financial_inclusion: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoffeeSector {
    pub production_systems: Vec<String>,
    pub quality_improvement: Vec<String>,
    pub value_chain_development: Vec<String>,
    pub market_access: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeaProduction {
    pub plantation_development: Vec<String>,
    pub processing_facilities: Vec<String>,
    pub quality_standards: Vec<String>,
    pub export_promotion: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSecurity {
    pub food_production: Vec<String>,
    pub nutrition_security: Vec<String>,
    pub food_systems: Vec<String>,
    pub emergency_preparedness: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestockDevelopment {
    pub livestock_production: Vec<String>,
    pub animal_health: Vec<String>,
    pub value_chain_development: Vec<String>,
    pub pastoral_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandReform {
    pub land_policy: Vec<String>,
    pub land_administration: Vec<String>,
    pub tenure_security: Vec<String>,
    pub land_use_planning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeMovement {
    pub cooperative_development: Vec<String>,
    pub cooperative_governance: Vec<String>,
    pub cooperative_financing: Vec<String>,
    pub cooperative_marketing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalSecurity {
    pub security_cooperation: Vec<String>,
    pub conflict_prevention: Vec<String>,
    pub peacekeeping: Vec<String>,
    pub early_warning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefugeeDynamics {
    pub refugee_hosting: Vec<String>,
    pub refugee_integration: Vec<String>,
    pub durable_solutions: Vec<String>,
    pub burden_sharing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCooperation {
    pub environmental_management: Vec<String>,
    pub climate_adaptation: Vec<String>,
    pub biodiversity_conservation: Vec<String>,
    pub sustainable_development: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LakeTanganyika {
    pub fisheries_management: Vec<String>,
    pub water_resources: Vec<String>,
    pub transportation: Vec<String>,
    pub environmental_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalResources {
    pub mineral_resources: Vec<String>,
    pub forest_resources: Vec<String>,
    pub water_resources: Vec<String>,
    pub sustainable_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KirundiNational {
    pub official_status: Vec<String>,
    pub promotion_policies: Vec<String>,
    pub cultural_significance: Vec<String>,
    pub preservation_efforts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchOfficial {
    pub administrative_use: Vec<String>,
    pub education_medium: Vec<String>,
    pub legal_proceedings: Vec<String>,
    pub international_relations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnglishIntroduction {
    pub eac_integration: Vec<String>,
    pub education_system: Vec<String>,
    pub business_communication: Vec<String>,
    pub capacity_building: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwahiliRegional {
    pub regional_integration: Vec<String>,
    pub trade_facilitation: Vec<String>,
    pub cultural_exchange: Vec<String>,
    pub communication_enhancement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualEducation {
    pub language_policy: Vec<String>,
    pub curriculum_development: Vec<String>,
    pub teacher_training: Vec<String>,
    pub learning_materials: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentGoals {
    pub economic_transformation: Vec<String>,
    pub social_development: Vec<String>,
    pub governance_improvement: Vec<String>,
    pub environmental_sustainability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPillars {
    pub pillar_one: String,
    pub pillar_two: String,
    pub pillar_three: String,
    pub pillar_four: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanDevelopment {
    pub education_development: Vec<String>,
    pub health_improvement: Vec<String>,
    pub skills_development: Vec<String>,
    pub social_protection: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicGrowth {
    pub growth_targets: Vec<String>,
    pub structural_transformation: Vec<String>,
    pub competitiveness: Vec<String>,
    pub innovation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceImprovements {
    pub institutional_strengthening: Vec<String>,
    pub transparency_accountability: Vec<String>,
    pub service_delivery: Vec<String>,
    pub citizen_participation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalPositioning {
    pub regional_integration: Vec<String>,
    pub competitive_advantage: Vec<String>,
    pub strategic_partnerships: Vec<String>,
    pub global_engagement: Vec<String>,
}

impl Default for BurundiLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ConstitutionalFramework::default(),
            post_civil_war_governance: PostCivilWarGovernance::default(),
            ethnic_power_sharing: EthnicPowerSharing::default(),
            arusha_peace_accord: ArushaPeaceAccord::default(),
            government_structure: GovernmentStructure::default(),
            judicial_system: JudicialSystem::default(),
            provincial_administration: ProvincialAdministration::default(),
            truth_reconciliation_commission: TruthReconciliationCommission::default(),
            regional_integration: RegionalIntegration::default(),
            economic_development: EconomicDevelopment::default(),
            agricultural_framework: AgriculturalFramework::default(),
            great_lakes_dynamics: GreatLakesDynamics::default(),
            language_policy: LanguagePolicy::default(),
            vision_2025: Vision2025::default(),
        }
    }
}

macro_rules! impl_default_burundi {
    ($($struct_name:ident),*) => {
        $(
            impl Default for $struct_name {
                fn default() -> Self {
                    Self {
                        $(${ignore})*
                    }
                }
            }
        )*
    };
}

impl_default_burundi!(
    ConstitutionalFramework, Constitution2018, PostCivilWarGovernance, EthnicPowerSharing,
    ArushaPeaceAccord, GovernmentStructure, ExecutiveBranch, LegislativeBranch, JudicialSystem,
    ProvincialAdministration, Province, TruthReconciliationCommission, RegionalIntegration,
    EconomicDevelopment, AgriculturalFramework, GreatLakesDynamics, LanguagePolicy, Vision2025,
    FundamentalRights, SeparationOfPowers, NationalUnity, DemocraticGovernance, RuleOfLaw,
    ConstitutionalSupremacy, TransitionPeriod, PeaceBuilding, DemocraticTransition,
    SecuritySectorReform, ExCombatantReintegration, RefugeeReturn, HutuTutsiBalance,
    GovernmentalQuotas, MilitaryIntegration, JudicialRepresentation, CivilServiceQuotas,
    PoliticalPartyRegulations, ImplementationTimeline, InternationalMonitoring,
    PowerSharingArrangements, CeasefireAgreements, President, VicePresidents, CouncilOfMinisters,
    PresidentialPowers, TermLimits, SuccessionProcedures, FirstVicePresident, SecondVicePresident,
    EthnicBalanceRequirement, SuccessionOrder, Composition, AppointmentProcedures,
    CollectiveResponsibility, EthnicBalance, PortfolioDistribution, KeyMinistry, TechnicalMinistry,
    BicameralStructure, LegislativeProcess, OversightFunctions, EthnicRepresentation,
    NationalAssembly, Senate, JointSessions, CoordinationMechanisms, AssemblyComposition,
    Committee, ElectionSystem, CommitteeComposition, SenateComposition, SenateAppointmentProcedures,
    LocalGovernment, CivilService, ProvincialLevel, CommunalLevel, CollineLevel,
    DecentralizationPolicy, Governor, ProvincialCouncil, DevelopmentPlanning, ServiceDelivery,
    Administrator, CommunalCouncil, ServiceProvision, CitizenParticipation, TraditionalStructure,
    ModernAdministration, CommunityParticipation, ConflictResolution, SupremeCourt,
    ConstitutionalCourt, CourtOfAppeals, HighCourt, MagistrateCourts, TraditionalCourts,
    GacacaInspiredCourts, JudicialIndependence, SupremeCourtComposition, Jurisdiction,
    JudicialAppointmentProcess, JudicialEthnicBalance, ChiefJustice, AssociateJustice,
    EthnicRequirements, GenderConsiderations, ConstitutionalJurisdiction,
    ConstitutionalCourtComposition, ConstitutionalProcedures, IndependenceGuarantees,
    ConstitutionalReview, ElectoralDisputes, RightsProtection, InstitutionalConflicts,
    Bashingantahe, CustomaryLaw, TraditionalConflictResolution, IntegrationMechanisms,
    TraditionalRole, ModernAdaptation, SelectionProcess, CommunityRecognition, WisdomKeepers,
    MediationRole, CulturalPreservation, FormalRecognition, TrainingPrograms, OversightSystems,
    CommunityJustice, ReconciliationFocus, VictimParticipation, TruthSeeking,
    ParticipatoryApproach, LocalOwnership, AccessibleProcedures, CommunityHealing
);

pub fn create_burundi_legal_system() -> BurundiLegalSystem {
    BurundiLegalSystem::default()
}