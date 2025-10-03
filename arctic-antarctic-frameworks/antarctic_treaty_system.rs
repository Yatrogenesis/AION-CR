use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntarcticTreatySystem {
    pub antarctic_treaty_1959: AntarcticTreaty1959,
    pub environmental_protocol: EnvironmentalProtocol,
    pub conservation_antarctic_seals: CCAS,
    pub conservation_antarctic_marine_living_resources: CCAMLR,
    pub territorial_claims_system: TerritorialClaimsSystem,
    pub antarctic_protected_areas: AntarcticProtectedAreas,
    pub scientific_cooperation: AntarcticScientificCooperation,
    pub logistics_operations: LogisticsOperations,
    pub tourism_regulation: TourismRegulation,
    pub environmental_monitoring: EnvironmentalMonitoring,
    pub climate_research: ClimateResearch,
    pub mineral_resources_prohibition: MineralResourcesProhibition,
    pub consultative_parties: ConsultativeParties,
    pub non_consultative_parties: NonConsultativeParties,
    pub international_cooperation: InternationalCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntarcticTreaty1959 {
    pub peaceful_purposes: PeacefulPurposes,
    pub scientific_cooperation: TreatyScientificCooperation,
    pub territorial_claims_provisions: TerritorialClaimsProvisions,
    pub nuclear_prohibition: NuclearProhibition,
    pub freedom_scientific_investigation: FreedomScientificInvestigation,
    pub international_cooperation_science: InternationalCooperationScience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeacefulPurposes {
    pub demilitarization: String,
    pub prohibition_military_activities: String,
    pub prohibition_weapons_testing: String,
    pub prohibition_nuclear_explosions: String,
    pub inspection_provisions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalProtocol {
    pub protocol_1991: Protocol1991,
    pub comprehensive_protection: ComprehensiveProtection,
    pub environmental_impact_assessment: AntarcticEIA,
    pub waste_disposal_management: WasteDisposalManagement,
    pub marine_pollution_prevention: MarinePollutionPrevention,
    pub area_protection_management: AreaProtectionManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol1991 {
    pub environmental_protection: String,
    pub designation_natural_reserve: String,
    pub prohibition_mineral_activities: String,
    pub environmental_impact_assessment: String,
    pub waste_disposal: String,
    pub marine_pollution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCAS {
    pub convention_1972: Convention1972,
    pub seal_conservation: SealConservation,
    pub protection_measures: ProtectionMeasures,
    pub scientific_research: SealScientificResearch,
    pub prohibited_activities: ProhibitedActivities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCAMLR {
    pub convention_1980: CCAMLR1980,
    pub ecosystem_approach: EcosystemApproach,
    pub conservation_measures: ConservationMeasures,
    pub scientific_committee: ScientificCommittee,
    pub fisheries_management: AntarcticFisheriesManagement,
    pub marine_protected_areas: AntarcticMPA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCAMLR1980 {
    pub conservation_objective: String,
    pub ecosystem_protection: String,
    pub rational_use: String,
    pub precautionary_principle: String,
    pub scientific_basis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritorialClaimsSystem {
    pub article_iv_provisions: ArticleIVProvisions,
    pub claimant_states: ClaimantStates,
    pub non_claimant_states: NonClaimantStates,
    pub sovereignty_suspension: SovereigntySuspension,
    pub territorial_boundaries: TerritorialBoundaries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimantStates {
    pub argentina_claim: ArgentinaClaim,
    pub australia_claim: AustraliaClaim,
    pub chile_claim: ChileClaim,
    pub france_claim: FranceClaim,
    pub new_zealand_claim: NewZealandClaim,
    pub norway_claim: NorwayClaim,
    pub united_kingdom_claim: UnitedKingdomClaim,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntarcticProtectedAreas {
    pub antarctic_specially_protected_areas: ASPA,
    pub antarctic_specially_managed_areas: ASMA,
    pub historic_sites_monuments: HistoricSitesMonuments,
    pub sites_special_scientific_interest: SSSI,
    pub protection_designation: ProtectionDesignation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntarcticScientificCooperation {
    pub scientific_committee_antarctic_research: SCAR,
    pub international_polar_year: AntarcticIPY,
    pub data_sharing_agreements: DataSharingAgreements,
    pub research_coordination: ResearchCoordination,
    pub international_research_stations: InternationalResearchStations,
    pub collaborative_programs: CollaborativePrograms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCAR {
    pub establishment_1958: String,
    pub scientific_advice: String,
    pub research_coordination: String,
    pub data_management: String,
    pub international_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticsOperations {
    pub transportation_systems: TransportationSystems,
    pub research_stations: AntarcticResearchStations,
    pub supply_operations: SupplyOperations,
    pub emergency_response: EmergencyResponse,
    pub communications_systems: CommunicationsSystems,
    pub infrastructure_development: InfrastructureDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TourismRegulation {
    pub iaato_guidelines: IAATOGuidelines,
    pub visitor_management: VisitorManagement,
    pub environmental_guidelines: EnvironmentalGuidelines,
    pub safety_requirements: SafetyRequirements,
    pub permit_system: PermitSystem,
    pub liability_insurance: LiabilityInsurance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAATOGuidelines {
    pub international_association: String,
    pub self_regulation: String,
    pub environmental_protection: String,
    pub visitor_guidelines: String,
    pub site_guidelines: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalMonitoring {
    pub climate_monitoring: AntarcticClimateMonitoring,
    pub ecosystem_monitoring: EcosystemMonitoring,
    pub pollution_monitoring: PollutionMonitoring,
    pub biodiversity_monitoring: BiodiversityMonitoring,
    pub ice_sheet_monitoring: IceSheetMonitoring,
    pub atmospheric_monitoring: AtmosphericMonitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateResearch {
    pub ice_core_research: IceCoreResearch,
    pub climate_modeling: ClimateModeling,
    pub sea_level_research: SeaLevelResearch,
    pub atmospheric_research: AtmosphericResearch,
    pub oceanographic_research: OceanographicResearch,
    pub paleoclimate_research: PaleoclimateResearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralResourcesProhibition {
    pub protocol_prohibition: ProtocolProhibition,
    pub convention_regulation_mineral_resources: CRAMRA,
    pub environmental_protection: MineralEnvironmentalProtection,
    pub scientific_research_exception: ScientificResearchException,
    pub future_review_provisions: FutureReviewProvisions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultativeParties {
    pub original_signatories: OriginalSignatories,
    pub acceding_parties: AccedingParties,
    pub voting_rights: VotingRights,
    pub decision_making: DecisionMaking,
    pub consensus_requirement: ConsensusRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalSignatories {
    pub argentina: String,
    pub australia: String,
    pub belgium: String,
    pub chile: String,
    pub france: String,
    pub japan: String,
    pub new_zealand: String,
    pub norway: String,
    pub south_africa: String,
    pub soviet_union_russia: String,
    pub united_kingdom: String,
    pub united_states: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalCooperation {
    pub united_nations_cooperation: UNCooperation,
    pub specialized_agencies: SpecializedAgencies,
    pub environmental_organizations: EnvironmentalOrganizations,
    pub scientific_organizations: ScientificOrganizations,
    pub observer_status: ObserverStatus,
}

macro_rules! impl_antarctic_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_antarctic_defaults!(
    PeacefulPurposes, TreatyScientificCooperation, TerritorialClaimsProvisions,
    NuclearProhibition, FreedomScientificInvestigation, InternationalCooperationScience,
    Protocol1991, ComprehensiveProtection, AntarcticEIA, WasteDisposalManagement,
    MarinePollutionPrevention, AreaProtectionManagement, Convention1972, SealConservation,
    ProtectionMeasures, SealScientificResearch, ProhibitedActivities, CCAMLR1980,
    EcosystemApproach, ConservationMeasures, ScientificCommittee, AntarcticFisheriesManagement,
    AntarcticMPA, ArticleIVProvisions, ClaimantStates, NonClaimantStates,
    SovereigntySuspension, TerritorialBoundaries, ArgentinaClaim, AustraliaClaim,
    ChileClaim, FranceClaim, NewZealandClaim, NorwayClaim, UnitedKingdomClaim,
    ASPA, ASMA, HistoricSitesMonuments, SSSI, ProtectionDesignation, SCAR,
    AntarcticIPY, DataSharingAgreements, ResearchCoordination, InternationalResearchStations,
    CollaborativePrograms, TransportationSystems, AntarcticResearchStations,
    SupplyOperations, EmergencyResponse, CommunicationsSystems, InfrastructureDevelopment,
    IAATOGuidelines, VisitorManagement, EnvironmentalGuidelines, SafetyRequirements,
    PermitSystem, LiabilityInsurance, AntarcticClimateMonitoring, EcosystemMonitoring,
    PollutionMonitoring, BiodiversityMonitoring, IceSheetMonitoring, AtmosphericMonitoring,
    IceCoreResearch, ClimateModeling, SeaLevelResearch, AtmosphericResearch,
    OceanographicResearch, PaleoclimateResearch, ProtocolProhibition, CRAMRA,
    MineralEnvironmentalProtection, ScientificResearchException, FutureReviewProvisions,
    OriginalSignatories, AccedingParties, VotingRights, DecisionMaking,
    ConsensusRequirement, NonConsultativeParties, UNCooperation, SpecializedAgencies,
    EnvironmentalOrganizations, ScientificOrganizations, ObserverStatus
);

impl Default for AntarcticTreaty1959 {
    fn default() -> Self {
        Self {
            peaceful_purposes: PeacefulPurposes::default(),
            scientific_cooperation: TreatyScientificCooperation::default(),
            territorial_claims_provisions: TerritorialClaimsProvisions::default(),
            nuclear_prohibition: NuclearProhibition::default(),
            freedom_scientific_investigation: FreedomScientificInvestigation::default(),
            international_cooperation_science: InternationalCooperationScience::default(),
        }
    }
}

impl Default for EnvironmentalProtocol {
    fn default() -> Self {
        Self {
            protocol_1991: Protocol1991::default(),
            comprehensive_protection: ComprehensiveProtection::default(),
            environmental_impact_assessment: AntarcticEIA::default(),
            waste_disposal_management: WasteDisposalManagement::default(),
            marine_pollution_prevention: MarinePollutionPrevention::default(),
            area_protection_management: AreaProtectionManagement::default(),
        }
    }
}

impl Default for CCAS {
    fn default() -> Self {
        Self {
            convention_1972: Convention1972::default(),
            seal_conservation: SealConservation::default(),
            protection_measures: ProtectionMeasures::default(),
            scientific_research: SealScientificResearch::default(),
            prohibited_activities: ProhibitedActivities::default(),
        }
    }
}

impl Default for CCAMLR {
    fn default() -> Self {
        Self {
            convention_1980: CCAMLR1980::default(),
            ecosystem_approach: EcosystemApproach::default(),
            conservation_measures: ConservationMeasures::default(),
            scientific_committee: ScientificCommittee::default(),
            fisheries_management: AntarcticFisheriesManagement::default(),
            marine_protected_areas: AntarcticMPA::default(),
        }
    }
}

impl Default for TerritorialClaimsSystem {
    fn default() -> Self {
        Self {
            article_iv_provisions: ArticleIVProvisions::default(),
            claimant_states: ClaimantStates::default(),
            non_claimant_states: NonClaimantStates::default(),
            sovereignty_suspension: SovereigntySuspension::default(),
            territorial_boundaries: TerritorialBoundaries::default(),
        }
    }
}

impl Default for AntarcticProtectedAreas {
    fn default() -> Self {
        Self {
            antarctic_specially_protected_areas: ASPA::default(),
            antarctic_specially_managed_areas: ASMA::default(),
            historic_sites_monuments: HistoricSitesMonuments::default(),
            sites_special_scientific_interest: SSSI::default(),
            protection_designation: ProtectionDesignation::default(),
        }
    }
}

impl Default for AntarcticScientificCooperation {
    fn default() -> Self {
        Self {
            scientific_committee_antarctic_research: SCAR::default(),
            international_polar_year: AntarcticIPY::default(),
            data_sharing_agreements: DataSharingAgreements::default(),
            research_coordination: ResearchCoordination::default(),
            international_research_stations: InternationalResearchStations::default(),
            collaborative_programs: CollaborativePrograms::default(),
        }
    }
}

impl Default for LogisticsOperations {
    fn default() -> Self {
        Self {
            transportation_systems: TransportationSystems::default(),
            research_stations: AntarcticResearchStations::default(),
            supply_operations: SupplyOperations::default(),
            emergency_response: EmergencyResponse::default(),
            communications_systems: CommunicationsSystems::default(),
            infrastructure_development: InfrastructureDevelopment::default(),
        }
    }
}

impl Default for TourismRegulation {
    fn default() -> Self {
        Self {
            iaato_guidelines: IAATOGuidelines::default(),
            visitor_management: VisitorManagement::default(),
            environmental_guidelines: EnvironmentalGuidelines::default(),
            safety_requirements: SafetyRequirements::default(),
            permit_system: PermitSystem::default(),
            liability_insurance: LiabilityInsurance::default(),
        }
    }
}

impl Default for EnvironmentalMonitoring {
    fn default() -> Self {
        Self {
            climate_monitoring: AntarcticClimateMonitoring::default(),
            ecosystem_monitoring: EcosystemMonitoring::default(),
            pollution_monitoring: PollutionMonitoring::default(),
            biodiversity_monitoring: BiodiversityMonitoring::default(),
            ice_sheet_monitoring: IceSheetMonitoring::default(),
            atmospheric_monitoring: AtmosphericMonitoring::default(),
        }
    }
}

impl Default for ClimateResearch {
    fn default() -> Self {
        Self {
            ice_core_research: IceCoreResearch::default(),
            climate_modeling: ClimateModeling::default(),
            sea_level_research: SeaLevelResearch::default(),
            atmospheric_research: AtmosphericResearch::default(),
            oceanographic_research: OceanographicResearch::default(),
            paleoclimate_research: PaleoclimateResearch::default(),
        }
    }
}

impl Default for MineralResourcesProhibition {
    fn default() -> Self {
        Self {
            protocol_prohibition: ProtocolProhibition::default(),
            convention_regulation_mineral_resources: CRAMRA::default(),
            environmental_protection: MineralEnvironmentalProtection::default(),
            scientific_research_exception: ScientificResearchException::default(),
            future_review_provisions: FutureReviewProvisions::default(),
        }
    }
}

impl Default for ConsultativeParties {
    fn default() -> Self {
        Self {
            original_signatories: OriginalSignatories::default(),
            acceding_parties: AccedingParties::default(),
            voting_rights: VotingRights::default(),
            decision_making: DecisionMaking::default(),
            consensus_requirement: ConsensusRequirement::default(),
        }
    }
}

impl Default for InternationalCooperation {
    fn default() -> Self {
        Self {
            united_nations_cooperation: UNCooperation::default(),
            specialized_agencies: SpecializedAgencies::default(),
            environmental_organizations: EnvironmentalOrganizations::default(),
            scientific_organizations: ScientificOrganizations::default(),
            observer_status: ObserverStatus::default(),
        }
    }
}

impl Default for AntarcticTreatySystem {
    fn default() -> Self {
        Self {
            antarctic_treaty_1959: AntarcticTreaty1959::default(),
            environmental_protocol: EnvironmentalProtocol::default(),
            conservation_antarctic_seals: CCAS::default(),
            conservation_antarctic_marine_living_resources: CCAMLR::default(),
            territorial_claims_system: TerritorialClaimsSystem::default(),
            antarctic_protected_areas: AntarcticProtectedAreas::default(),
            scientific_cooperation: AntarcticScientificCooperation::default(),
            logistics_operations: LogisticsOperations::default(),
            tourism_regulation: TourismRegulation::default(),
            environmental_monitoring: EnvironmentalMonitoring::default(),
            climate_research: ClimateResearch::default(),
            mineral_resources_prohibition: MineralResourcesProhibition::default(),
            consultative_parties: ConsultativeParties::default(),
            non_consultative_parties: NonConsultativeParties::default(),
            international_cooperation: InternationalCooperation::default(),
        }
    }
}

pub fn create_antarctic_treaty_system() -> AntarcticTreatySystem {
    AntarcticTreatySystem::default()
}