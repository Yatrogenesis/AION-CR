use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZambiaLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub presidential_multiparty_system: PresidentialMultipartySystem,
    pub copper_economy_governance: CopperEconomyGovernance,
    pub democratic_consolidation: DemocraticConsolidation,
    pub government_structure: GovernmentStructure,
    pub judicial_system: JudicialSystem,
    pub provincial_administration: ProvincialAdministration,
    pub traditional_governance: TraditionalGovernance,
    pub economic_diversification: EconomicDiversification,
    pub natural_resources_framework: NaturalResourcesFramework,
    pub regional_integration: RegionalIntegration,
    pub language_policy: LanguagePolicy,
    pub vision_2030: Vision2030,
    pub decentralization_program: DecentralizationProgram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    pub constitution_2016: Constitution2016,
    pub constitutional_referendum: ConstitutionalReferendum,
    pub fundamental_rights: FundamentalRights,
    pub separation_of_powers: SeparationOfPowers,
    pub constitutional_court: ConstitutionalCourt,
    pub amendment_procedures: AmendmentProcedures,
    pub emergency_powers: EmergencyPowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution2016 {
    pub promulgation_date: String,
    pub key_principles: Vec<String>,
    pub democratic_governance: DemocraticGovernance,
    pub rule_of_law: RuleOfLaw,
    pub national_unity: NationalUnity,
    pub good_governance: GoodGovernance,
    pub christian_nation_declaration: ChristianNationDeclaration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidentialMultipartySystem {
    pub executive_presidency: ExecutivePresidency,
    pub multiparty_democracy: MultipartyDemocracy,
    pub electoral_system: ElectoralSystem,
    pub political_competition: PoliticalCompetition,
    pub democratic_transitions: DemocraticTransitions,
    pub opposition_politics: OppositionPolitics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopperEconomyGovernance {
    pub mining_sector: MiningSector,
    pub copper_production: CopperProduction,
    pub mining_taxation: MiningTaxation,
    pub resource_management: ResourceManagement,
    pub economic_dependency: EconomicDependency,
    pub mining_communities: MiningCommunities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticConsolidation {
    pub democratic_institutions: DemocraticInstitutions,
    pub civil_society: CivilSociety,
    pub media_freedom: MediaFreedom,
    pub human_rights_protection: HumanRightsProtection,
    pub electoral_integrity: ElectoralIntegrity,
    pub governance_improvements: GovernanceImprovements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentStructure {
    pub executive_branch: ExecutiveBranch,
    pub legislative_branch: LegislativeBranch,
    pub cabinet_system: CabinetSystem,
    pub provincial_coordination: ProvincialCoordination,
    pub public_administration: PublicAdministration,
    pub civil_service: CivilService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveBranch {
    pub president: President,
    pub vice_president: VicePresident,
    pub ministers: Ministers,
    pub presidential_powers: PresidentialPowers,
    pub executive_coordination: ExecutiveCoordination,
    pub succession_procedures: SuccessionProcedures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialSystem {
    pub supreme_court: SupremeCourt,
    pub constitutional_court: ConstitutionalCourt,
    pub court_of_appeal: CourtOfAppeal,
    pub high_court: HighCourt,
    pub subordinate_courts: SubordinateCourts,
    pub local_courts: LocalCourts,
    pub traditional_courts: TraditionalCourts,
    pub judicial_independence: JudicialIndependence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvincialAdministration {
    pub provinces: Vec<Province>,
    pub districts: Vec<District>,
    pub constituencies: Vec<Constituency>,
    pub wards: Vec<Ward>,
    pub chiefdoms: Vec<Chiefdom>,
    pub local_government: LocalGovernment,
    pub rural_urban_administration: RuralUrbanAdministration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub name: String,
    pub capital: String,
    pub population: u64,
    pub area_km2: f64,
    pub provincial_minister: ProvincialMinister,
    pub permanent_secretary: PermanentSecretary,
    pub economic_activities: Vec<String>,
    pub development_priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalGovernance {
    pub traditional_rulers: TraditionalRulers,
    pub chiefs: Chiefs,
    pub headmen: Headmen,
    pub village_headmen: VillageHeadmen,
    pub customary_law: CustomaryLaw,
    pub house_of_chiefs: HouseOfChiefs,
    pub modern_integration: ModernIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicDiversification {
    pub diversification_strategy: DiversificationStrategy,
    pub agriculture_development: AgricultureDevelopment,
    pub manufacturing_sector: ManufacturingSector,
    pub tourism_industry: TourismIndustry,
    pub energy_sector: EnergySector,
    pub private_sector_development: PrivateSectorDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalResourcesFramework {
    pub mineral_resources: MineralResources,
    pub water_resources: WaterResources,
    pub forest_resources: ForestResources,
    pub wildlife_resources: WildlifeResources,
    pub agricultural_land: AgriculturalLand,
    pub environmental_protection: EnvironmentalProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalIntegration {
    pub sadc_membership: SADCMembership,
    pub comesa_participation: COMESAParticipation,
    pub african_union: AfricanUnion,
    pub bilateral_relations: BilateralRelations,
    pub regional_trade: RegionalTrade,
    pub infrastructure_cooperation: InfrastructureCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePolicy {
    pub english_official: EnglishOfficial,
    pub local_languages: LocalLanguages,
    pub bemba_predominance: BembaPredominance,
    pub nyanja_lingua_franca: NyanjaLinguaFranca,
    pub multilingual_education: MultilingualEducation,
    pub language_development: LanguageDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vision2030 {
    pub development_vision: DevelopmentVision,
    pub prosperity_goals: ProsperityGoals,
    pub transformation_agenda: TransformationAgenda,
    pub strategic_pillars: StrategicPillars,
    pub implementation_framework: ImplementationFramework,
    pub monitoring_evaluation: MonitoringEvaluation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizationProgram {
    pub decentralization_policy: DecentralizationPolicy,
    pub devolution_framework: DevolutionFramework,
    pub local_governance_strengthening: LocalGovernanceStrengthening,
    pub capacity_building: CapacityBuilding,
    pub fiscal_decentralization: FiscalDecentralization,
    pub citizen_participation: CitizenParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalRights {
    pub civil_rights: Vec<String>,
    pub political_rights: Vec<String>,
    pub economic_rights: Vec<String>,
    pub social_rights: Vec<String>,
    pub cultural_rights: Vec<String>,
    pub environmental_rights: Vec<String>,
    pub group_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparationOfPowers {
    pub executive_powers: Vec<String>,
    pub legislative_powers: Vec<String>,
    pub judicial_powers: Vec<String>,
    pub checks_and_balances: Vec<String>,
    pub institutional_independence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalReferendum {
    pub referendum_date: String,
    pub voter_turnout: f64,
    pub approval_rate: f64,
    pub regional_variations: HashMap<String, f64>,
    pub campaign_dynamics: CampaignDynamics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDynamics {
    pub yes_campaign: YesCampaign,
    pub no_campaign: NoCampaign,
    pub civil_society_position: CivilSocietyPosition,
    pub media_coverage: MediaCoverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YesCampaign {
    pub campaign_messages: Vec<String>,
    pub supporting_organizations: Vec<String>,
    pub key_arguments: Vec<String>,
    pub mobilization_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoCampaign {
    pub campaign_messages: Vec<String>,
    pub supporting_organizations: Vec<String>,
    pub key_arguments: Vec<String>,
    pub mobilization_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilSocietyPosition {
    pub supporting_organizations: Vec<String>,
    pub opposing_organizations: Vec<String>,
    pub neutral_organizations: Vec<String>,
    pub advocacy_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCoverage {
    pub media_balance: f64,
    pub coverage_quality: Vec<String>,
    pub information_access: Vec<String>,
    pub media_plurality: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticGovernance {
    pub democratic_principles: Vec<String>,
    pub electoral_democracy: ElectoralDemocracy,
    pub participatory_democracy: ParticipatoryDemocracy,
    pub constitutional_democracy: ConstitutionalDemocracy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralDemocracy {
    pub free_fair_elections: Vec<String>,
    pub electoral_management: ElectoralManagement,
    pub political_parties: PoliticalParties,
    pub voter_education: VoterEducation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralManagement {
    pub electoral_commission: ElectoralCommission,
    pub voter_registration: VoterRegistration,
    pub election_administration: ElectionAdministration,
    pub dispute_resolution: ElectoralDisputeResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralCommission {
    pub composition: String,
    pub independence: Vec<String>,
    pub powers_functions: Vec<String>,
    pub accountability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRegistration {
    pub registration_procedures: Vec<String>,
    pub voter_roll_maintenance: Vec<String>,
    pub voter_education: Vec<String>,
    pub accessibility_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionAdministration {
    pub polling_procedures: Vec<String>,
    pub vote_counting: Vec<String>,
    pub result_transmission: Vec<String>,
    pub election_security: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralDisputeResolution {
    pub dispute_mechanisms: Vec<String>,
    pub electoral_tribunals: Vec<String>,
    pub appeal_procedures: Vec<String>,
    pub enforcement_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParties {
    pub ruling_party: RulingParty,
    pub opposition_parties: Vec<OppositionParty>,
    pub party_system: PartySystem,
    pub party_financing: PartyFinancing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulingParty {
    pub party_name: String,
    pub leadership: Vec<String>,
    pub ideology: Vec<String>,
    pub electoral_performance: Vec<String>,
    pub governance_record: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OppositionParty {
    pub party_name: String,
    pub leadership: Vec<String>,
    pub ideology: Vec<String>,
    pub electoral_performance: Vec<String>,
    pub opposition_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartySystem {
    pub competitive_system: Vec<String>,
    pub party_competition: Vec<String>,
    pub electoral_alternation: Vec<String>,
    pub coalition_dynamics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyFinancing {
    pub funding_sources: Vec<String>,
    pub public_funding: Vec<String>,
    pub private_funding: Vec<String>,
    pub transparency_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterEducation {
    pub civic_education: Vec<String>,
    pub voter_awareness: Vec<String>,
    pub election_information: Vec<String>,
    pub media_role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipatoryDemocracy {
    pub citizen_participation: Vec<String>,
    pub public_consultation: Vec<String>,
    pub community_involvement: Vec<String>,
    pub grassroots_democracy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalDemocracy {
    pub constitutional_supremacy: Vec<String>,
    pub constitutional_review: Vec<String>,
    pub rights_protection: Vec<String>,
    pub institutional_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOfLaw {
    pub legal_supremacy: Vec<String>,
    pub equal_justice: Vec<String>,
    pub due_process: Vec<String>,
    pub access_to_justice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalUnity {
    pub unity_principles: Vec<String>,
    pub ethnic_harmony: Vec<String>,
    pub cultural_diversity: Vec<String>,
    pub social_cohesion: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodGovernance {
    pub governance_principles: Vec<String>,
    pub transparency_accountability: Vec<String>,
    pub public_service_delivery: Vec<String>,
    pub anti_corruption: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChristianNationDeclaration {
    pub constitutional_provision: String,
    pub religious_freedom: Vec<String>,
    pub secular_governance: Vec<String>,
    pub interfaith_relations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutivePresidency {
    pub presidential_powers: Vec<String>,
    pub executive_functions: Vec<String>,
    pub presidential_accountability: Vec<String>,
    pub term_limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartyDemocracy {
    pub multiparty_system: Vec<String>,
    pub political_pluralism: Vec<String>,
    pub democratic_competition: Vec<String>,
    pub opposition_rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystem {
    pub presidential_elections: PresidentialElections,
    pub parliamentary_elections: ParliamentaryElections,
    pub local_elections: LocalElections,
    pub electoral_formula: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidentialElections {
    pub election_system: String,
    pub candidacy_requirements: Vec<String>,
    pub campaign_regulations: Vec<String>,
    pub runoff_provisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentaryElections {
    pub constituency_system: String,
    pub seat_allocation: String,
    pub candidate_requirements: Vec<String>,
    pub proportional_representation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalElections {
    pub council_elections: Vec<String>,
    pub mayoral_elections: Vec<String>,
    pub ward_elections: Vec<String>,
    pub traditional_leadership: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalCompetition {
    pub competitive_elections: Vec<String>,
    pub electoral_contestation: Vec<String>,
    pub campaign_dynamics: Vec<String>,
    pub voter_choice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticTransitions {
    pub transition_history: Vec<String>,
    pub peaceful_transitions: Vec<String>,
    pub electoral_alternation: Vec<String>,
    pub democratic_consolidation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OppositionPolitics {
    pub opposition_role: Vec<String>,
    pub parliamentary_opposition: Vec<String>,
    pub opposition_rights: Vec<String>,
    pub political_space: Vec<String>,
}

impl Default for ZambiaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ConstitutionalFramework::default(),
            presidential_multiparty_system: PresidentialMultipartySystem::default(),
            copper_economy_governance: CopperEconomyGovernance::default(),
            democratic_consolidation: DemocraticConsolidation::default(),
            government_structure: GovernmentStructure::default(),
            judicial_system: JudicialSystem::default(),
            provincial_administration: ProvincialAdministration::default(),
            traditional_governance: TraditionalGovernance::default(),
            economic_diversification: EconomicDiversification::default(),
            natural_resources_framework: NaturalResourcesFramework::default(),
            regional_integration: RegionalIntegration::default(),
            language_policy: LanguagePolicy::default(),
            vision_2030: Vision2030::default(),
            decentralization_program: DecentralizationProgram::default(),
        }
    }
}

macro_rules! impl_default_zambia {
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

impl_default_zambia!(
    ConstitutionalFramework, Constitution2016, PresidentialMultipartySystem, CopperEconomyGovernance,
    DemocraticConsolidation, GovernmentStructure, ExecutiveBranch, JudicialSystem,
    ProvincialAdministration, Province, TraditionalGovernance, EconomicDiversification,
    NaturalResourcesFramework, RegionalIntegration, LanguagePolicy, Vision2030,
    DecentralizationProgram, FundamentalRights, SeparationOfPowers, ConstitutionalReferendum,
    CampaignDynamics, YesCampaign, NoCampaign, CivilSocietyPosition, MediaCoverage,
    DemocraticGovernance, ElectoralDemocracy, ElectoralManagement, ElectoralCommission,
    VoterRegistration, ElectionAdministration, ElectoralDisputeResolution, PoliticalParties,
    RulingParty, OppositionParty, PartySystem, PartyFinancing, VoterEducation,
    ParticipatoryDemocracy, ConstitutionalDemocracy, RuleOfLaw, NationalUnity, GoodGovernance,
    ChristianNationDeclaration, ExecutivePresidency, MultipartyDemocracy, ElectoralSystem,
    PresidentialElections, ParliamentaryElections, LocalElections, PoliticalCompetition,
    DemocraticTransitions, OppositionPolitics
);

pub fn create_zambia_legal_system() -> ZambiaLegalSystem {
    ZambiaLegalSystem::default()
}