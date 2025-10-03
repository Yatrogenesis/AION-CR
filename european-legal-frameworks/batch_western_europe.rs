use serde::{Deserialize, Serialize};

// UNITED KINGDOM LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitedKingdomLegalSystem {
    pub constitutional_framework: UKConstitutionalFramework,
    pub parliamentary_democracy: UKParliamentaryDemocracy,
    pub constitutional_monarchy: UKConstitutionalMonarchy,
    pub devolved_administrations: DevolvedAdministrations,
    pub common_law_system: UKCommonLawSystem,
    pub brexit_framework: BrexitFramework,
    pub human_rights: UKHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UKConstitutionalFramework {
    pub unwritten_constitution: String,
    pub magna_carta_1215: String,
    pub bill_of_rights_1689: String,
    pub act_of_union_1707: String,
    pub parliament_acts: String,
}

// FRANCE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranceLegalSystem {
    pub constitutional_framework: FranceConstitutionalFramework,
    pub semi_presidential_system: FranceSemiPresidentialSystem,
    pub civil_law_system: FranceCivilLawSystem,
    pub eu_integration: FranceEUIntegration,
    pub overseas_territories: FranceOverseasTerritories,
    pub human_rights: FranceHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranceConstitutionalFramework {
    pub constitution_1958: String,
    pub declaration_rights_1789: String,
    pub constitutional_council: String,
    pub separation_powers: String,
}

// GERMANY LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GermanyLegalSystem {
    pub constitutional_framework: GermanyConstitutionalFramework,
    pub federal_parliamentary_democracy: GermanyFederalParliamentaryDemocracy,
    pub civil_law_system: GermanyCivilLawSystem,
    pub eu_integration: GermanyEUIntegration,
    pub federal_system: GermanyFederalSystem,
    pub human_rights: GermanyHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GermanyConstitutionalFramework {
    pub basic_law_1949: String,
    pub federal_constitutional_court: String,
    pub fundamental_rights: String,
    pub federal_structure: String,
}

// ITALY LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyLegalSystem {
    pub constitutional_framework: ItalyConstitutionalFramework,
    pub parliamentary_republic: ItalyParliamentaryRepublic,
    pub civil_law_system: ItalyCivilLawSystem,
    pub eu_integration: ItalyEUIntegration,
    pub regional_autonomy: ItalyRegionalAutonomy,
    pub human_rights: ItalyHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalyConstitutionalFramework {
    pub constitution_1947: String,
    pub constitutional_court: String,
    pub fundamental_principles: String,
    pub republican_government: String,
}

// SPAIN LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainLegalSystem {
    pub constitutional_framework: SpainConstitutionalFramework,
    pub parliamentary_monarchy: SpainParliamentaryMonarchy,
    pub civil_law_system: SpainCivilLawSystem,
    pub autonomous_communities: AutonomousCommunities,
    pub eu_integration: SpainEUIntegration,
    pub human_rights: SpainHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpainConstitutionalFramework {
    pub constitution_1978: String,
    pub constitutional_court: String,
    pub regional_autonomy: String,
    pub democratic_transition: String,
}

// NETHERLANDS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetherlandsLegalSystem {
    pub constitutional_framework: NetherlandsConstitutionalFramework,
    pub constitutional_monarchy: NetherlandsConstitutionalMonarchy,
    pub civil_law_system: NetherlandsCivilLawSystem,
    pub eu_integration: NetherlandsEUIntegration,
    pub decentralized_government: DecentralizedGovernment,
    pub human_rights: NetherlandsHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetherlandsConstitutionalFramework {
    pub constitution_1815: String,
    pub fundamental_rights: String,
    pub parliamentary_system: String,
    pub rule_of_law: String,
}

// BELGIUM LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BelgiumLegalSystem {
    pub constitutional_framework: BelgiumConstitutionalFramework,
    pub federal_constitutional_monarchy: BelgiumFederalConstitutionalMonarchy,
    pub civil_law_system: BelgiumCivilLawSystem,
    pub linguistic_communities: LinguisticCommunities,
    pub eu_integration: BelgiumEUIntegration,
    pub human_rights: BelgiumHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BelgiumConstitutionalFramework {
    pub constitution_1831: String,
    pub constitutional_court: String,
    pub federal_structure: String,
    pub linguistic_parity: String,
}

// PORTUGAL LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortugalLegalSystem {
    pub constitutional_framework: PortugalConstitutionalFramework,
    pub semi_presidential_republic: PortugalSemiPresidentialRepublic,
    pub civil_law_system: PortugalCivilLawSystem,
    pub autonomous_regions: PortugalAutonomousRegions,
    pub eu_integration: PortugalEUIntegration,
    pub human_rights: PortugalHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortugalConstitutionalFramework {
    pub constitution_1976: String,
    pub constitutional_court: String,
    pub democratic_revolution: String,
    pub autonomous_regions: String,
}

// AUSTRIA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AustriaLegalSystem {
    pub constitutional_framework: AustriaConstitutionalFramework,
    pub federal_parliamentary_republic: AustriaFederalParliamentaryRepublic,
    pub civil_law_system: AustriaCivilLawSystem,
    pub federal_structure: AustriaFederalStructure,
    pub eu_integration: AustriaEUIntegration,
    pub human_rights: AustriaHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AustriaConstitutionalFramework {
    pub constitution_1920: String,
    pub constitutional_court: String,
    pub federal_structure: String,
    pub neutrality_status: String,
}

// SWITZERLAND LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitzerlandLegalSystem {
    pub constitutional_framework: SwitzerlandConstitutionalFramework,
    pub federal_directorial_republic: FederalDirectorialRepublic,
    pub civil_law_system: SwitzerlandCivilLawSystem,
    pub direct_democracy: DirectDemocracy,
    pub neutrality_status: NeutralityStatus,
    pub human_rights: SwitzerlandHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitzerlandConstitutionalFramework {
    pub constitution_1848_1999: String,
    pub federal_assembly: String,
    pub cantonal_sovereignty: String,
    pub direct_democracy: String,
}

// IRELAND LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrelandLegalSystem {
    pub constitutional_framework: IrelandConstitutionalFramework,
    pub parliamentary_republic: IrelandParliamentaryRepublic,
    pub common_law_system: IrelandCommonLawSystem,
    pub eu_integration: IrelandEUIntegration,
    pub irish_language: IrishLanguage,
    pub human_rights: IrelandHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrelandConstitutionalFramework {
    pub constitution_1937: String,
    pub supreme_court: String,
    pub fundamental_rights: String,
    pub irish_sovereignty: String,
}

macro_rules! impl_western_europe_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_western_europe_defaults!(
    UKConstitutionalFramework, UKParliamentaryDemocracy, UKConstitutionalMonarchy,
    DevolvedAdministrations, UKCommonLawSystem, BrexitFramework, UKHumanRights,
    FranceConstitutionalFramework, FranceSemiPresidentialSystem, FranceCivilLawSystem,
    FranceEUIntegration, FranceOverseasTerritories, FranceHumanRights,
    GermanyConstitutionalFramework, GermanyFederalParliamentaryDemocracy, GermanyCivilLawSystem,
    GermanyEUIntegration, GermanyFederalSystem, GermanyHumanRights,
    ItalyConstitutionalFramework, ItalyParliamentaryRepublic, ItalyCivilLawSystem,
    ItalyEUIntegration, ItalyRegionalAutonomy, ItalyHumanRights,
    SpainConstitutionalFramework, SpainParliamentaryMonarchy, SpainCivilLawSystem,
    AutonomousCommunities, SpainEUIntegration, SpainHumanRights,
    NetherlandsConstitutionalFramework, NetherlandsConstitutionalMonarchy, NetherlandsCivilLawSystem,
    NetherlandsEUIntegration, DecentralizedGovernment, NetherlandsHumanRights,
    BelgiumConstitutionalFramework, BelgiumFederalConstitutionalMonarchy, BelgiumCivilLawSystem,
    LinguisticCommunities, BelgiumEUIntegration, BelgiumHumanRights,
    PortugalConstitutionalFramework, PortugalSemiPresidentialRepublic, PortugalCivilLawSystem,
    PortugalAutonomousRegions, PortugalEUIntegration, PortugalHumanRights,
    AustriaConstitutionalFramework, AustriaFederalParliamentaryRepublic, AustriaCivilLawSystem,
    AustriaFederalStructure, AustriaEUIntegration, AustriaHumanRights,
    SwitzerlandConstitutionalFramework, FederalDirectorialRepublic, SwitzerlandCivilLawSystem,
    DirectDemocracy, NeutralityStatus, SwitzerlandHumanRights,
    IrelandConstitutionalFramework, IrelandParliamentaryRepublic, IrelandCommonLawSystem,
    IrelandEUIntegration, IrishLanguage, IrelandHumanRights
);

impl Default for UnitedKingdomLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: UKConstitutionalFramework::default(),
            parliamentary_democracy: UKParliamentaryDemocracy::default(),
            constitutional_monarchy: UKConstitutionalMonarchy::default(),
            devolved_administrations: DevolvedAdministrations::default(),
            common_law_system: UKCommonLawSystem::default(),
            brexit_framework: BrexitFramework::default(),
            human_rights: UKHumanRights::default(),
        }
    }
}

impl Default for FranceLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: FranceConstitutionalFramework::default(),
            semi_presidential_system: FranceSemiPresidentialSystem::default(),
            civil_law_system: FranceCivilLawSystem::default(),
            eu_integration: FranceEUIntegration::default(),
            overseas_territories: FranceOverseasTerritories::default(),
            human_rights: FranceHumanRights::default(),
        }
    }
}

impl Default for GermanyLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: GermanyConstitutionalFramework::default(),
            federal_parliamentary_democracy: GermanyFederalParliamentaryDemocracy::default(),
            civil_law_system: GermanyCivilLawSystem::default(),
            eu_integration: GermanyEUIntegration::default(),
            federal_system: GermanyFederalSystem::default(),
            human_rights: GermanyHumanRights::default(),
        }
    }
}

impl Default for ItalyLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ItalyConstitutionalFramework::default(),
            parliamentary_republic: ItalyParliamentaryRepublic::default(),
            civil_law_system: ItalyCivilLawSystem::default(),
            eu_integration: ItalyEUIntegration::default(),
            regional_autonomy: ItalyRegionalAutonomy::default(),
            human_rights: ItalyHumanRights::default(),
        }
    }
}

impl Default for SpainLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SpainConstitutionalFramework::default(),
            parliamentary_monarchy: SpainParliamentaryMonarchy::default(),
            civil_law_system: SpainCivilLawSystem::default(),
            autonomous_communities: AutonomousCommunities::default(),
            eu_integration: SpainEUIntegration::default(),
            human_rights: SpainHumanRights::default(),
        }
    }
}

impl Default for NetherlandsLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: NetherlandsConstitutionalFramework::default(),
            constitutional_monarchy: NetherlandsConstitutionalMonarchy::default(),
            civil_law_system: NetherlandsCivilLawSystem::default(),
            eu_integration: NetherlandsEUIntegration::default(),
            decentralized_government: DecentralizedGovernment::default(),
            human_rights: NetherlandsHumanRights::default(),
        }
    }
}

impl Default for BelgiumLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: BelgiumConstitutionalFramework::default(),
            federal_constitutional_monarchy: BelgiumFederalConstitutionalMonarchy::default(),
            civil_law_system: BelgiumCivilLawSystem::default(),
            linguistic_communities: LinguisticCommunities::default(),
            eu_integration: BelgiumEUIntegration::default(),
            human_rights: BelgiumHumanRights::default(),
        }
    }
}

impl Default for PortugalLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: PortugalConstitutionalFramework::default(),
            semi_presidential_republic: PortugalSemiPresidentialRepublic::default(),
            civil_law_system: PortugalCivilLawSystem::default(),
            autonomous_regions: PortugalAutonomousRegions::default(),
            eu_integration: PortugalEUIntegration::default(),
            human_rights: PortugalHumanRights::default(),
        }
    }
}

impl Default for AustriaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: AustriaConstitutionalFramework::default(),
            federal_parliamentary_republic: AustriaFederalParliamentaryRepublic::default(),
            civil_law_system: AustriaCivilLawSystem::default(),
            federal_structure: AustriaFederalStructure::default(),
            eu_integration: AustriaEUIntegration::default(),
            human_rights: AustriaHumanRights::default(),
        }
    }
}

impl Default for SwitzerlandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SwitzerlandConstitutionalFramework::default(),
            federal_directorial_republic: FederalDirectorialRepublic::default(),
            civil_law_system: SwitzerlandCivilLawSystem::default(),
            direct_democracy: DirectDemocracy::default(),
            neutrality_status: NeutralityStatus::default(),
            human_rights: SwitzerlandHumanRights::default(),
        }
    }
}

impl Default for IrelandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: IrelandConstitutionalFramework::default(),
            parliamentary_republic: IrelandParliamentaryRepublic::default(),
            common_law_system: IrelandCommonLawSystem::default(),
            eu_integration: IrelandEUIntegration::default(),
            irish_language: IrishLanguage::default(),
            human_rights: IrelandHumanRights::default(),
        }
    }
}

pub fn create_western_europe_systems() -> (
    UnitedKingdomLegalSystem, FranceLegalSystem, GermanyLegalSystem, ItalyLegalSystem,
    SpainLegalSystem, NetherlandsLegalSystem, BelgiumLegalSystem, PortugalLegalSystem,
    AustriaLegalSystem, SwitzerlandLegalSystem, IrelandLegalSystem
) {
    (
        UnitedKingdomLegalSystem::default(),
        FranceLegalSystem::default(),
        GermanyLegalSystem::default(),
        ItalyLegalSystem::default(),
        SpainLegalSystem::default(),
        NetherlandsLegalSystem::default(),
        BelgiumLegalSystem::default(),
        PortugalLegalSystem::default(),
        AustriaLegalSystem::default(),
        SwitzerlandLegalSystem::default(),
        IrelandLegalSystem::default(),
    )
}