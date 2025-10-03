use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticTerritorialRegulations {
    pub arctic_council_framework: ArcticCouncilFramework,
    pub unclos_arctic_application: UNCLOSArcticApplication,
    pub svalbard_treaty_regime: SvalbardTreatyRegime,
    pub greenland_self_governance: GreenlandSelfGovernance,
    pub faroe_islands_autonomy: FaroeIslandsAutonomy,
    pub alaska_state_framework: AlaskaStateFramework,
    pub northwest_territories: NorthwestTerritories,
    pub yukon_territory: YukonTerritory,
    pub nunavut_territory: NunavutTerritory,
    pub arctic_marine_areas: ArcticMarineAreas,
    pub indigenous_rights_arctic: IndigenousRightsArctic,
    pub environmental_protection: ArcticEnvironmentalProtection,
    pub resource_governance: ArcticResourceGovernance,
    pub shipping_navigation: ArcticShippingNavigation,
    pub scientific_cooperation: ArcticScientificCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticCouncilFramework {
    pub ottawa_declaration_1996: OttawaDeclaration1996,
    pub member_states: ArcticCouncilMembers,
    pub permanent_participants: PermanentParticipants,
    pub working_groups: ArcticWorkingGroups,
    pub ministerial_meetings: MinisterialMeetings,
    pub arctic_strategies: ArcticStrategies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OttawaDeclaration1996 {
    pub sustainable_development: String,
    pub environmental_protection: String,
    pub indigenous_peoples: String,
    pub common_arctic_issues: String,
    pub cooperation_coordination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticCouncilMembers {
    pub canada: CanadaArctic,
    pub denmark: DenmarkArctic,
    pub finland: FinlandArctic,
    pub iceland: IcelandArctic,
    pub norway: NorwayArctic,
    pub russia: RussiaArctic,
    pub sweden: SwedenArctic,
    pub united_states: USArctic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentParticipants {
    pub aleut_international_association: AleutInternationalAssociation,
    pub arctic_athabaskan_council: ArcticAthabaskanCouncil,
    pub gwich_in_council_international: GwichInCouncilInternational,
    pub inuit_circumpolar_council: InuitCircumpolarCouncil,
    pub russian_arctic_indigenous_peoples: RussianArcticIndigenousPeoples,
    pub saami_council: SaamiCouncil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UNCLOSArcticApplication {
    pub territorial_sea: ArcticTerritorialSea,
    pub exclusive_economic_zones: ArcticEEZ,
    pub continental_shelf: ArcticContinentalShelf,
    pub high_seas: ArcticHighSeas,
    pub deep_seabed: ArcticDeepSeabed,
    pub commission_limits_continental_shelf: CLCS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticContinentalShelf {
    pub article_76_claims: Article76Claims,
    pub lomonosov_ridge: LomonosovRidge,
    pub mendeleev_ridge: MendeleevRidge,
    pub alpha_ridge: AlphaRidge,
    pub submissions_clcs: SubmissionsCLCS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvalbardTreatyRegime {
    pub treaty_1920: SvalbardTreaty1920,
    pub norwegian_sovereignty: NorwegianSovereignty,
    pub demilitarization: Demilitarization,
    pub equal_access: EqualAccess,
    pub environmental_protection: SvalbardEnvironmental,
    pub mining_activities: SvalbardMining,
    pub scientific_research: SvalbardScientific,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvalbardTreaty1920 {
    pub signatory_states: Vec<String>,
    pub territorial_scope: String,
    pub sovereignty_norway: String,
    pub equal_liberty_access: String,
    pub demilitarized_zone: String,
    pub taxation_limitations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenlandSelfGovernance {
    pub self_government_act_2009: SelfGovernmentAct2009,
    pub greenlandic_language: GreenlandicLanguage,
    pub natural_resources: GreenlandNaturalResources,
    pub judicial_affairs: GreenlandJudicialAffairs,
    pub foreign_affairs_cooperation: ForeignAffairsCooperation,
    pub independence_pathway: IndependencePathway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfGovernmentAct2009 {
    pub expanded_self_rule: String,
    pub subsurface_resources: String,
    pub revenue_sharing: String,
    pub danish_state_subsidy: String,
    pub international_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaroeIslandsAutonomy {
    pub home_rule_act_1948: HomeRuleAct1948,
    pub self_government_act_2005: FaroeSelfGovernmentAct2005,
    pub faroese_language: FaroeseLanguage,
    pub fisheries_management: FaroeFisheries,
    pub international_agreements: FaroeInternationalAgreements,
    pub constitutional_status: FaroeConstitutionalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlaskaStateFramework {
    pub statehood_1959: AlaskaStatehood1959,
    pub alaska_native_claims_settlement_act: ANCSA,
    pub oil_revenue_sharing: AlaskaOilRevenue,
    pub permanent_fund: AlaskaPermanentFund,
    pub federal_land_management: FederalLandManagement,
    pub subsistence_rights: SubsistenceRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ANCSA {
    pub settlement_1971: String,
    pub land_conveyance: String,
    pub monetary_compensation: String,
    pub native_corporations: String,
    pub cultural_preservation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorthwestTerritories {
    pub territorial_governance: NWTGovernance,
    pub consensus_government: ConsensusGovernment,
    pub indigenous_rights: NWTIndigenousRights,
    pub devolution_agreement: DevolutionAgreement,
    pub resource_management: NWTResourceManagement,
    pub official_languages: NWTOfficialLanguages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YukonTerritory {
    pub territorial_governance: YukonGovernance,
    pub first_nations_agreements: FirstNationsAgreements,
    pub devolution_powers: YukonDevolution,
    pub resource_management: YukonResourceManagement,
    pub environmental_assessment: YukonEnvironmentalAssessment,
    pub official_languages: YukonOfficialLanguages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NunavutTerritory {
    pub creation_1999: NunavutCreation1999,
    pub inuit_self_governance: InuitSelfGovernance,
    pub inuktitut_language: InuktitutLanguage,
    pub land_claims_agreement: LandClaimsAgreement,
    pub territorial_governance: NunavutGovernance,
    pub traditional_knowledge: TraditionalKnowledge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NunavutCreation1999 {
    pub territorial_division: String,
    pub inuit_majority: String,
    pub cultural_governance: String,
    pub traditional_values: String,
    pub consensus_democracy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticMarineAreas {
    pub northwest_passage: NorthwestPassage,
    pub northern_sea_route: NorthernSeaRoute,
    pub transpolar_sea_route: TranspolarSeaRoute,
    pub central_arctic_ocean: CentralArcticOcean,
    pub beaufort_sea: BeaufortSea,
    pub chukchi_sea: ChukchiSea,
    pub laptev_sea: LaptevSea,
    pub kara_sea: KaraSea,
    pub barents_sea: BarentsSea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndigenousRightsArctic {
    pub undrip_application: UNDRIPApplication,
    pub traditional_territories: TraditionalTerritories,
    pub hunting_fishing_rights: HuntingFishingRights,
    pub cultural_preservation: CulturalPreservation,
    pub self_determination: SelfDetermination,
    pub free_prior_informed_consent: FPIC,
    pub traditional_governance: ArcticTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticEnvironmentalProtection {
    pub arctic_environmental_protection_strategy: AEPS,
    pub polar_bear_agreement: PolarBearAgreement,
    pub migratory_species_protection: MigratorySpeciesProtection,
    pub marine_protected_areas: ArcticMPA,
    pub climate_change_adaptation: ClimateChangeAdaptation,
    pub pollution_prevention: PollutionPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticResourceGovernance {
    pub oil_gas_exploration: OilGasExploration,
    pub mining_regulations: ArcticMining,
    pub fisheries_management: ArcticFisheries,
    pub renewable_energy: ArcticRenewableEnergy,
    pub environmental_impact_assessment: ArcticEIA,
    pub benefit_sharing_agreements: BenefitSharingAgreements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticShippingNavigation {
    pub international_maritime_organization: IMOArctic,
    pub polar_code: PolarCode,
    pub search_rescue_agreement: SARAAgreement,
    pub oil_pollution_preparedness: OilPollutionPreparedness,
    pub vessel_traffic_services: VesselTrafficServices,
    pub icebreaker_cooperation: IcebreakerCooperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcticScientificCooperation {
    pub international_polar_year: InternationalPolarYear,
    pub arctic_science_cooperation_agreement: ASCAgreement,
    pub data_sharing_protocols: DataSharingProtocols,
    pub research_stations: ArcticResearchStations,
    pub climate_monitoring: ClimateMonitoring,
    pub biodiversity_research: BiodiversityResearch,
}

macro_rules! impl_arctic_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_arctic_defaults!(
    OttawaDeclaration1996, ArcticCouncilMembers, PermanentParticipants, ArcticWorkingGroups,
    MinisterialMeetings, ArcticStrategies, CanadaArctic, DenmarkArctic, FinlandArctic,
    IcelandArctic, NorwayArctic, RussiaArctic, SwedenArctic, USArctic,
    AleutInternationalAssociation, ArcticAthabaskanCouncil, GwichInCouncilInternational,
    InuitCircumpolarCouncil, RussianArcticIndigenousPeoples, SaamiCouncil,
    ArcticTerritorialSea, ArcticEEZ, ArcticHighSeas, ArcticDeepSeabed, CLCS,
    Article76Claims, LomonosovRidge, MendeleevRidge, AlphaRidge, SubmissionsCLCS,
    SvalbardTreaty1920, NorwegianSovereignty, Demilitarization, EqualAccess,
    SvalbardEnvironmental, SvalbardMining, SvalbardScientific, SelfGovernmentAct2009,
    GreenlandicLanguage, GreenlandNaturalResources, GreenlandJudicialAffairs,
    ForeignAffairsCooperation, IndependencePathway, HomeRuleAct1948,
    FaroeSelfGovernmentAct2005, FaroeseLanguage, FaroeFisheries, FaroeInternationalAgreements,
    FaroeConstitutionalStatus, AlaskaStatehood1959, ANCSA, AlaskaOilRevenue,
    AlaskaPermanentFund, FederalLandManagement, SubsistenceRights, NWTGovernance,
    ConsensusGovernment, NWTIndigenousRights, DevolutionAgreement, NWTResourceManagement,
    NWTOfficialLanguages, YukonGovernance, FirstNationsAgreements, YukonDevolution,
    YukonResourceManagement, YukonEnvironmentalAssessment, YukonOfficialLanguages,
    NunavutCreation1999, InuitSelfGovernance, InuktitutLanguage, LandClaimsAgreement,
    NunavutGovernance, TraditionalKnowledge, NorthwestPassage, NorthernSeaRoute,
    TranspolarSeaRoute, CentralArcticOcean, BeaufortSea, ChukchiSea, LaptevSea,
    KaraSea, BarentsSea, UNDRIPApplication, TraditionalTerritories, HuntingFishingRights,
    CulturalPreservation, SelfDetermination, FPIC, ArcticTraditionalGovernance,
    AEPS, PolarBearAgreement, MigratorySpeciesProtection, ArcticMPA,
    ClimateChangeAdaptation, PollutionPrevention, OilGasExploration, ArcticMining,
    ArcticFisheries, ArcticRenewableEnergy, ArcticEIA, BenefitSharingAgreements,
    IMOArctic, PolarCode, SARAAgreement, OilPollutionPreparedness, VesselTrafficServices,
    IcebreakerCooperation, InternationalPolarYear, ASCAgreement, DataSharingProtocols,
    ArcticResearchStations, ClimateMonitoring, BiodiversityResearch
);

impl Default for ArcticCouncilFramework {
    fn default() -> Self {
        Self {
            ottawa_declaration_1996: OttawaDeclaration1996::default(),
            member_states: ArcticCouncilMembers::default(),
            permanent_participants: PermanentParticipants::default(),
            working_groups: ArcticWorkingGroups::default(),
            ministerial_meetings: MinisterialMeetings::default(),
            arctic_strategies: ArcticStrategies::default(),
        }
    }
}

impl Default for UNCLOSArcticApplication {
    fn default() -> Self {
        Self {
            territorial_sea: ArcticTerritorialSea::default(),
            exclusive_economic_zones: ArcticEEZ::default(),
            continental_shelf: ArcticContinentalShelf::default(),
            high_seas: ArcticHighSeas::default(),
            deep_seabed: ArcticDeepSeabed::default(),
            commission_limits_continental_shelf: CLCS::default(),
        }
    }
}

impl Default for ArcticContinentalShelf {
    fn default() -> Self {
        Self {
            article_76_claims: Article76Claims::default(),
            lomonosov_ridge: LomonosovRidge::default(),
            mendeleev_ridge: MendeleevRidge::default(),
            alpha_ridge: AlphaRidge::default(),
            submissions_clcs: SubmissionsCLCS::default(),
        }
    }
}

impl Default for SvalbardTreatyRegime {
    fn default() -> Self {
        Self {
            treaty_1920: SvalbardTreaty1920::default(),
            norwegian_sovereignty: NorwegianSovereignty::default(),
            demilitarization: Demilitarization::default(),
            equal_access: EqualAccess::default(),
            environmental_protection: SvalbardEnvironmental::default(),
            mining_activities: SvalbardMining::default(),
            scientific_research: SvalbardScientific::default(),
        }
    }
}

impl Default for GreenlandSelfGovernance {
    fn default() -> Self {
        Self {
            self_government_act_2009: SelfGovernmentAct2009::default(),
            greenlandic_language: GreenlandicLanguage::default(),
            natural_resources: GreenlandNaturalResources::default(),
            judicial_affairs: GreenlandJudicialAffairs::default(),
            foreign_affairs_cooperation: ForeignAffairsCooperation::default(),
            independence_pathway: IndependencePathway::default(),
        }
    }
}

impl Default for FaroeIslandsAutonomy {
    fn default() -> Self {
        Self {
            home_rule_act_1948: HomeRuleAct1948::default(),
            self_government_act_2005: FaroeSelfGovernmentAct2005::default(),
            faroese_language: FaroeseLanguage::default(),
            fisheries_management: FaroeFisheries::default(),
            international_agreements: FaroeInternationalAgreements::default(),
            constitutional_status: FaroeConstitutionalStatus::default(),
        }
    }
}

impl Default for AlaskaStateFramework {
    fn default() -> Self {
        Self {
            statehood_1959: AlaskaStatehood1959::default(),
            alaska_native_claims_settlement_act: ANCSA::default(),
            oil_revenue_sharing: AlaskaOilRevenue::default(),
            permanent_fund: AlaskaPermanentFund::default(),
            federal_land_management: FederalLandManagement::default(),
            subsistence_rights: SubsistenceRights::default(),
        }
    }
}

impl Default for NorthwestTerritories {
    fn default() -> Self {
        Self {
            territorial_governance: NWTGovernance::default(),
            consensus_government: ConsensusGovernment::default(),
            indigenous_rights: NWTIndigenousRights::default(),
            devolution_agreement: DevolutionAgreement::default(),
            resource_management: NWTResourceManagement::default(),
            official_languages: NWTOfficialLanguages::default(),
        }
    }
}

impl Default for YukonTerritory {
    fn default() -> Self {
        Self {
            territorial_governance: YukonGovernance::default(),
            first_nations_agreements: FirstNationsAgreements::default(),
            devolution_powers: YukonDevolution::default(),
            resource_management: YukonResourceManagement::default(),
            environmental_assessment: YukonEnvironmentalAssessment::default(),
            official_languages: YukonOfficialLanguages::default(),
        }
    }
}

impl Default for NunavutTerritory {
    fn default() -> Self {
        Self {
            creation_1999: NunavutCreation1999::default(),
            inuit_self_governance: InuitSelfGovernance::default(),
            inuktitut_language: InuktitutLanguage::default(),
            land_claims_agreement: LandClaimsAgreement::default(),
            territorial_governance: NunavutGovernance::default(),
            traditional_knowledge: TraditionalKnowledge::default(),
        }
    }
}

impl Default for ArcticMarineAreas {
    fn default() -> Self {
        Self {
            northwest_passage: NorthwestPassage::default(),
            northern_sea_route: NorthernSeaRoute::default(),
            transpolar_sea_route: TranspolarSeaRoute::default(),
            central_arctic_ocean: CentralArcticOcean::default(),
            beaufort_sea: BeaufortSea::default(),
            chukchi_sea: ChukchiSea::default(),
            laptev_sea: LaptevSea::default(),
            kara_sea: KaraSea::default(),
            barents_sea: BarentsSea::default(),
        }
    }
}

impl Default for IndigenousRightsArctic {
    fn default() -> Self {
        Self {
            undrip_application: UNDRIPApplication::default(),
            traditional_territories: TraditionalTerritories::default(),
            hunting_fishing_rights: HuntingFishingRights::default(),
            cultural_preservation: CulturalPreservation::default(),
            self_determination: SelfDetermination::default(),
            free_prior_informed_consent: FPIC::default(),
            traditional_governance: ArcticTraditionalGovernance::default(),
        }
    }
}

impl Default for ArcticEnvironmentalProtection {
    fn default() -> Self {
        Self {
            arctic_environmental_protection_strategy: AEPS::default(),
            polar_bear_agreement: PolarBearAgreement::default(),
            migratory_species_protection: MigratorySpeciesProtection::default(),
            marine_protected_areas: ArcticMPA::default(),
            climate_change_adaptation: ClimateChangeAdaptation::default(),
            pollution_prevention: PollutionPrevention::default(),
        }
    }
}

impl Default for ArcticResourceGovernance {
    fn default() -> Self {
        Self {
            oil_gas_exploration: OilGasExploration::default(),
            mining_regulations: ArcticMining::default(),
            fisheries_management: ArcticFisheries::default(),
            renewable_energy: ArcticRenewableEnergy::default(),
            environmental_impact_assessment: ArcticEIA::default(),
            benefit_sharing_agreements: BenefitSharingAgreements::default(),
        }
    }
}

impl Default for ArcticShippingNavigation {
    fn default() -> Self {
        Self {
            international_maritime_organization: IMOArctic::default(),
            polar_code: PolarCode::default(),
            search_rescue_agreement: SARAAgreement::default(),
            oil_pollution_preparedness: OilPollutionPreparedness::default(),
            vessel_traffic_services: VesselTrafficServices::default(),
            icebreaker_cooperation: IcebreakerCooperation::default(),
        }
    }
}

impl Default for ArcticScientificCooperation {
    fn default() -> Self {
        Self {
            international_polar_year: InternationalPolarYear::default(),
            arctic_science_cooperation_agreement: ASCAgreement::default(),
            data_sharing_protocols: DataSharingProtocols::default(),
            research_stations: ArcticResearchStations::default(),
            climate_monitoring: ClimateMonitoring::default(),
            biodiversity_research: BiodiversityResearch::default(),
        }
    }
}

impl Default for ArcticTerritorialRegulations {
    fn default() -> Self {
        Self {
            arctic_council_framework: ArcticCouncilFramework::default(),
            unclos_arctic_application: UNCLOSArcticApplication::default(),
            svalbard_treaty_regime: SvalbardTreatyRegime::default(),
            greenland_self_governance: GreenlandSelfGovernance::default(),
            faroe_islands_autonomy: FaroeIslandsAutonomy::default(),
            alaska_state_framework: AlaskaStateFramework::default(),
            northwest_territories: NorthwestTerritories::default(),
            yukon_territory: YukonTerritory::default(),
            nunavut_territory: NunavutTerritory::default(),
            arctic_marine_areas: ArcticMarineAreas::default(),
            indigenous_rights_arctic: IndigenousRightsArctic::default(),
            environmental_protection: ArcticEnvironmentalProtection::default(),
            resource_governance: ArcticResourceGovernance::default(),
            shipping_navigation: ArcticShippingNavigation::default(),
            scientific_cooperation: ArcticScientificCooperation::default(),
        }
    }
}

pub fn create_arctic_territorial_regulations() -> ArcticTerritorialRegulations {
    ArcticTerritorialRegulations::default()
}