use serde::{Deserialize, Serialize};

// SUDAN LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SudanLegalSystem {
    pub constitutional_framework: SudanConstitutionalFramework,
    pub transitional_government: SudanTransitionalGovernment,
    pub post_bashir_transition: SudanPostBashirTransition,
    pub federal_system: SudanFederalSystem,
    pub islamic_law_framework: SudanIslamicLawFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SudanConstitutionalFramework {
    pub constitutional_declaration_2019: String,
    pub fundamental_rights: Vec<String>,
}

// SOUTH SUDAN LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthSudanLegalSystem {
    pub constitutional_framework: SSConstitutionalFramework,
    pub presidential_system: SSPresidentialSystem,
    pub post_independence: SSPostIndependence,
    pub oil_governance: SSOilGovernance,
    pub traditional_governance: SSTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSConstitutionalFramework {
    pub transitional_constitution_2011: String,
    pub fundamental_rights: Vec<String>,
}

// ERITREA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EritreaLegalSystem {
    pub constitutional_framework: EritreaConstitutionalFramework,
    pub presidential_system: EritreaPresidentialSystem,
    pub post_independence: EritreaPostIndependence,
    pub national_service: EritreaNationalService,
    pub traditional_governance: EritreaTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EritreaConstitutionalFramework {
    pub constitution_1997: String,
    pub fundamental_rights: Vec<String>,
}

// DJIBOUTI LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjiboutiLegalSystem {
    pub constitutional_framework: DjiboutiConstitutionalFramework,
    pub presidential_system: DjiboutiPresidentialSystem,
    pub strategic_location: DjiboutiStrategicLocation,
    pub multiethnic_governance: DjiboutiMultiethnicGovernance,
    pub economic_development: DjiboutiEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjiboutiConstitutionalFramework {
    pub constitution_1992: String,
    pub fundamental_rights: Vec<String>,
}

// SOMALIA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomaliaLegalSystem {
    pub constitutional_framework: SomaliaConstitutionalFramework,
    pub federal_system: SomaliaFederalSystem,
    pub state_building: SomaliaStateBuilding,
    pub clan_governance: SomaliaClanGovernance,
    pub islamic_law: SomaliaIslamicLaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomaliaConstitutionalFramework {
    pub provisional_constitution_2012: String,
    pub fundamental_rights: Vec<String>,
}

// IMPLEMENTATION MACROS
macro_rules! impl_horn_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_horn_defaults!(
    SudanConstitutionalFramework, SudanTransitionalGovernment, SudanPostBashirTransition,
    SudanFederalSystem, SudanIslamicLawFramework,
    SSConstitutionalFramework, SSPresidentialSystem, SSPostIndependence, SSOilGovernance,
    SSTraditionalGovernance,
    EritreaConstitutionalFramework, EritreaPresidentialSystem, EritreaPostIndependence,
    EritreaNationalService, EritreaTraditionalGovernance,
    DjiboutiConstitutionalFramework, DjiboutiPresidentialSystem, DjiboutiStrategicLocation,
    DjiboutiMultiethnicGovernance, DjiboutiEconomicDevelopment,
    SomaliaConstitutionalFramework, SomaliaFederalSystem, SomaliaStateBuilding,
    SomaliaClanGovernance, SomaliaIslamicLaw
);

impl Default for SudanLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SudanConstitutionalFramework::default(),
            transitional_government: SudanTransitionalGovernment::default(),
            post_bashir_transition: SudanPostBashirTransition::default(),
            federal_system: SudanFederalSystem::default(),
            islamic_law_framework: SudanIslamicLawFramework::default(),
        }
    }
}

impl Default for SouthSudanLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SSConstitutionalFramework::default(),
            presidential_system: SSPresidentialSystem::default(),
            post_independence: SSPostIndependence::default(),
            oil_governance: SSOilGovernance::default(),
            traditional_governance: SSTraditionalGovernance::default(),
        }
    }
}

impl Default for EritreaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: EritreaConstitutionalFramework::default(),
            presidential_system: EritreaPresidentialSystem::default(),
            post_independence: EritreaPostIndependence::default(),
            national_service: EritreaNationalService::default(),
            traditional_governance: EritreaTraditionalGovernance::default(),
        }
    }
}

impl Default for DjiboutiLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: DjiboutiConstitutionalFramework::default(),
            presidential_system: DjiboutiPresidentialSystem::default(),
            strategic_location: DjiboutiStrategicLocation::default(),
            multiethnic_governance: DjiboutiMultiethnicGovernance::default(),
            economic_development: DjiboutiEconomicDevelopment::default(),
        }
    }
}

impl Default for SomaliaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SomaliaConstitutionalFramework::default(),
            federal_system: SomaliaFederalSystem::default(),
            state_building: SomaliaStateBuilding::default(),
            clan_governance: SomaliaClanGovernance::default(),
            islamic_law: SomaliaIslamicLaw::default(),
        }
    }
}

pub fn create_horn_africa_systems() -> (SudanLegalSystem, SouthSudanLegalSystem, EritreaLegalSystem, DjiboutiLegalSystem, SomaliaLegalSystem) {
    (
        SudanLegalSystem::default(),
        SouthSudanLegalSystem::default(),
        EritreaLegalSystem::default(),
        DjiboutiLegalSystem::default(),
        SomaliaLegalSystem::default(),
    )
}