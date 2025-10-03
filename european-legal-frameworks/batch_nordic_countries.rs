use serde::{Deserialize, Serialize};

// SWEDEN LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwedenLegalSystem {
    pub constitutional_framework: SwedenConstitutionalFramework,
    pub parliamentary_democracy: SwedenParliamentaryDemocracy,
    pub constitutional_monarchy: SwedenConstitutionalMonarchy,
    pub civil_law_system: SwedenCivilLawSystem,
    pub welfare_state: SwedenWelfareState,
    pub eu_integration: SwedenEUIntegration,
    pub human_rights: SwedenHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwedenConstitutionalFramework {
    pub instrument_of_government: String,
    pub succession_act: String,
    pub freedom_of_press_act: String,
    pub fundamental_law_freedom_expression: String,
}

// NORWAY LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorwayLegalSystem {
    pub constitutional_framework: NorwayConstitutionalFramework,
    pub parliamentary_democracy: NorwayParliamentaryDemocracy,
    pub constitutional_monarchy: NorwayConstitutionalMonarchy,
    pub civil_law_system: NorwayCivilLawSystem,
    pub oil_fund_governance: OilFundGovernance,
    pub eea_integration: EEAIntegration,
    pub human_rights: NorwayHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorwayConstitutionalFramework {
    pub constitution_1814: String,
    pub supreme_court: String,
    pub separation_powers: String,
    pub fundamental_rights: String,
}

// DENMARK LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenmarkLegalSystem {
    pub constitutional_framework: DenmarkConstitutionalFramework,
    pub parliamentary_democracy: DenmarkParliamentaryDemocracy,
    pub constitutional_monarchy: DenmarkConstitutionalMonarchy,
    pub civil_law_system: DenmarkCivilLawSystem,
    pub autonomous_territories: DenmarkAutonomousTerritories,
    pub eu_integration: DenmarkEUIntegration,
    pub human_rights: DenmarkHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenmarkConstitutionalFramework {
    pub constitution_1849_1953: String,
    pub supreme_court: String,
    pub folketing: String,
    pub constitutional_monarchy: String,
}

// FINLAND LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinlandLegalSystem {
    pub constitutional_framework: FinlandConstitutionalFramework,
    pub parliamentary_republic: FinlandParliamentaryRepublic,
    pub civil_law_system: FinlandCivilLawSystem,
    pub bilingual_system: BilingualSystem,
    pub eu_integration: FinlandEUIntegration,
    pub neutrality_status: FinlandNeutralityStatus,
    pub human_rights: FinlandHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinlandConstitutionalFramework {
    pub constitution_2000: String,
    pub supreme_court: String,
    pub supreme_administrative_court: String,
    pub fundamental_rights: String,
}

// ICELAND LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcelandLegalSystem {
    pub constitutional_framework: IcelandConstitutionalFramework,
    pub parliamentary_republic: IcelandParliamentaryRepublic,
    pub civil_law_system: IcelandCivilLawSystem,
    pub althing_parliament: AlthingParliament,
    pub eea_integration: IcelandEEAIntegration,
    pub human_rights: IcelandHumanRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcelandConstitutionalFramework {
    pub constitution_1944: String,
    pub supreme_court: String,
    pub althing_world_oldest: String,
    pub presidential_system: String,
}

macro_rules! impl_nordic_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_nordic_defaults!(
    SwedenConstitutionalFramework, SwedenParliamentaryDemocracy, SwedenConstitutionalMonarchy,
    SwedenCivilLawSystem, SwedenWelfareState, SwedenEUIntegration, SwedenHumanRights,
    NorwayConstitutionalFramework, NorwayParliamentaryDemocracy, NorwayConstitutionalMonarchy,
    NorwayCivilLawSystem, OilFundGovernance, EEAIntegration, NorwayHumanRights,
    DenmarkConstitutionalFramework, DenmarkParliamentaryDemocracy, DenmarkConstitutionalMonarchy,
    DenmarkCivilLawSystem, DenmarkAutonomousTerritories, DenmarkEUIntegration, DenmarkHumanRights,
    FinlandConstitutionalFramework, FinlandParliamentaryRepublic, FinlandCivilLawSystem,
    BilingualSystem, FinlandEUIntegration, FinlandNeutralityStatus, FinlandHumanRights,
    IcelandConstitutionalFramework, IcelandParliamentaryRepublic, IcelandCivilLawSystem,
    AlthingParliament, IcelandEEAIntegration, IcelandHumanRights
);

impl Default for SwedenLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SwedenConstitutionalFramework::default(),
            parliamentary_democracy: SwedenParliamentaryDemocracy::default(),
            constitutional_monarchy: SwedenConstitutionalMonarchy::default(),
            civil_law_system: SwedenCivilLawSystem::default(),
            welfare_state: SwedenWelfareState::default(),
            eu_integration: SwedenEUIntegration::default(),
            human_rights: SwedenHumanRights::default(),
        }
    }
}

impl Default for NorwayLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: NorwayConstitutionalFramework::default(),
            parliamentary_democracy: NorwayParliamentaryDemocracy::default(),
            constitutional_monarchy: NorwayConstitutionalMonarchy::default(),
            civil_law_system: NorwayCivilLawSystem::default(),
            oil_fund_governance: OilFundGovernance::default(),
            eea_integration: EEAIntegration::default(),
            human_rights: NorwayHumanRights::default(),
        }
    }
}

impl Default for DenmarkLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: DenmarkConstitutionalFramework::default(),
            parliamentary_democracy: DenmarkParliamentaryDemocracy::default(),
            constitutional_monarchy: DenmarkConstitutionalMonarchy::default(),
            civil_law_system: DenmarkCivilLawSystem::default(),
            autonomous_territories: DenmarkAutonomousTerritories::default(),
            eu_integration: DenmarkEUIntegration::default(),
            human_rights: DenmarkHumanRights::default(),
        }
    }
}

impl Default for FinlandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: FinlandConstitutionalFramework::default(),
            parliamentary_republic: FinlandParliamentaryRepublic::default(),
            civil_law_system: FinlandCivilLawSystem::default(),
            bilingual_system: BilingualSystem::default(),
            eu_integration: FinlandEUIntegration::default(),
            neutrality_status: FinlandNeutralityStatus::default(),
            human_rights: FinlandHumanRights::default(),
        }
    }
}

impl Default for IcelandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: IcelandConstitutionalFramework::default(),
            parliamentary_republic: IcelandParliamentaryRepublic::default(),
            civil_law_system: IcelandCivilLawSystem::default(),
            althing_parliament: AlthingParliament::default(),
            eea_integration: IcelandEEAIntegration::default(),
            human_rights: IcelandHumanRights::default(),
        }
    }
}

pub fn create_nordic_systems() -> (
    SwedenLegalSystem, NorwayLegalSystem, DenmarkLegalSystem, FinlandLegalSystem, IcelandLegalSystem
) {
    (
        SwedenLegalSystem::default(),
        NorwayLegalSystem::default(),
        DenmarkLegalSystem::default(),
        FinlandLegalSystem::default(),
        IcelandLegalSystem::default(),
    )
}