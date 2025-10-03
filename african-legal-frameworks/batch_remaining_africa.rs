use serde::{Deserialize, Serialize};

// REMAINING AFRICAN COUNTRIES - BATCH COMPLETION

// MALAWI LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalawiLegalSystem {
    pub constitutional_framework: MalawiConstitutionalFramework,
    pub presidential_system: MalawiPresidentialSystem,
    pub traditional_governance: MalawiTraditionalGovernance,
    pub economic_development: MalawiEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalawiConstitutionalFramework { pub constitution_1994: String }

// LESOTHO LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LesothoLegalSystem {
    pub constitutional_framework: LesothoConstitutionalFramework,
    pub constitutional_monarchy: LesothoConstitutionalMonarchy,
    pub traditional_governance: LesothoTraditionalGovernance,
    pub economic_development: LesothoEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LesothoConstitutionalFramework { pub constitution_1993: String }

// ESWATINI LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EswatiniLegalSystem {
    pub constitutional_framework: EswatiniConstitutionalFramework,
    pub absolute_monarchy: EswatiniAbsoluteMonarchy,
    pub traditional_governance: EswatiniTraditionalGovernance,
    pub economic_development: EswatiniEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EswatiniConstitutionalFramework { pub constitution_2005: String }

// GABON LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabonLegalSystem {
    pub constitutional_framework: GabonConstitutionalFramework,
    pub presidential_system: GabonPresidentialSystem,
    pub oil_governance: GabonOilGovernance,
    pub traditional_governance: GabonTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabonConstitutionalFramework { pub constitution_1991: String }

// EQUATORIAL GUINEA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquatorialGuineaLegalSystem {
    pub constitutional_framework: EGConstitutionalFramework,
    pub presidential_system: EGPresidentialSystem,
    pub oil_governance: EGOilGovernance,
    pub traditional_governance: EGTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGConstitutionalFramework { pub constitution_2011: String }

// CONGO BRAZZAVILLE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongoBrazzavilleLegalSystem {
    pub constitutional_framework: CBConstitutionalFramework,
    pub presidential_system: CBPresidentialSystem,
    pub oil_governance: CBOilGovernance,
    pub traditional_governance: CBTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CBConstitutionalFramework { pub constitution_2015: String }

// COMOROS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComorosLegalSystem {
    pub constitutional_framework: ComorosConstitutionalFramework,
    pub federal_system: ComorosFederalSystem,
    pub islamic_governance: ComorosIslamicGovernance,
    pub island_governance: ComorosIslandGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComorosConstitutionalFramework { pub constitution_2001: String }

// MAURITIUS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MauritiusLegalSystem {
    pub constitutional_framework: MauritiusConstitutionalFramework,
    pub parliamentary_democracy: MauritiusParliamentaryDemocracy,
    pub multicultural_governance: MauritiusMulticulturalGovernance,
    pub economic_development: MauritiusEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MauritiusConstitutionalFramework { pub constitution_1968: String }

// SEYCHELLES LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeychellesLegalSystem {
    pub constitutional_framework: SeychellesConstitutionalFramework,
    pub presidential_system: SeychellesPresidentialSystem,
    pub environmental_governance: SeychellesEnvironmentalGovernance,
    pub economic_development: SeychellesEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeychellesConstitutionalFramework { pub constitution_1993: String }

// SAO TOME AND PRINCIPE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaoTomeLegalSystem {
    pub constitutional_framework: SaoTomeConstitutionalFramework,
    pub semi_presidential_system: SaoTomeSemiPresidentialSystem,
    pub island_governance: SaoTomeIslandGovernance,
    pub economic_development: SaoTomeEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaoTomeConstitutionalFramework { pub constitution_1990: String }

// CAPE VERDE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapeVerdeLegalSystem {
    pub constitutional_framework: CapeVerdeConstitutionalFramework,
    pub semi_presidential_system: CapeVerdeSemiPresidentialSystem,
    pub island_governance: CapeVerdeIslandGovernance,
    pub economic_development: CapeVerdeEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapeVerdeConstitutionalFramework { pub constitution_1992: String }

// MAURITANIA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MauritaniaLegalSystem {
    pub constitutional_framework: MauritaniaConstitutionalFramework,
    pub presidential_system: MauritaniaPresidentialSystem,
    pub islamic_republic: MauritaniaIslamicRepublic,
    pub traditional_governance: MauritaniaTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MauritaniaConstitutionalFramework { pub constitution_1991: String }

// MACRO IMPLEMENTATIONS
macro_rules! impl_remaining_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_remaining_defaults!(
    MalawiConstitutionalFramework, MalawiPresidentialSystem, MalawiTraditionalGovernance, MalawiEconomicDevelopment,
    LesothoConstitutionalFramework, LesothoConstitutionalMonarchy, LesothoTraditionalGovernance, LesothoEconomicDevelopment,
    EswatiniConstitutionalFramework, EswatiniAbsoluteMonarchy, EswatiniTraditionalGovernance, EswatiniEconomicDevelopment,
    GabonConstitutionalFramework, GabonPresidentialSystem, GabonOilGovernance, GabonTraditionalGovernance,
    EGConstitutionalFramework, EGPresidentialSystem, EGOilGovernance, EGTraditionalGovernance,
    CBConstitutionalFramework, CBPresidentialSystem, CBOilGovernance, CBTraditionalGovernance,
    ComorosConstitutionalFramework, ComorosFederalSystem, ComorosIslamicGovernance, ComorosIslandGovernance,
    MauritiusConstitutionalFramework, MauritiusParliamentaryDemocracy, MauritiusMulticulturalGovernance, MauritiusEconomicDevelopment,
    SeychellesConstitutionalFramework, SeychellesPresidentialSystem, SeychellesEnvironmentalGovernance, SeychellesEconomicDevelopment,
    SaoTomeConstitutionalFramework, SaoTomeSemiPresidentialSystem, SaoTomeIslandGovernance, SaoTomeEconomicDevelopment,
    CapeVerdeConstitutionalFramework, CapeVerdeSemiPresidentialSystem, CapeVerdeIslandGovernance, CapeVerdeEconomicDevelopment,
    MauritaniaConstitutionalFramework, MauritaniaPresidentialSystem, MauritaniaIslamicRepublic, MauritaniaTraditionalGovernance
);

impl Default for MalawiLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: MalawiConstitutionalFramework::default(),
            presidential_system: MalawiPresidentialSystem::default(),
            traditional_governance: MalawiTraditionalGovernance::default(),
            economic_development: MalawiEconomicDevelopment::default(),
        }
    }
}

impl Default for LesothoLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: LesothoConstitutionalFramework::default(),
            constitutional_monarchy: LesothoConstitutionalMonarchy::default(),
            traditional_governance: LesothoTraditionalGovernance::default(),
            economic_development: LesothoEconomicDevelopment::default(),
        }
    }
}

impl Default for EswatiniLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: EswatiniConstitutionalFramework::default(),
            absolute_monarchy: EswatiniAbsoluteMonarchy::default(),
            traditional_governance: EswatiniTraditionalGovernance::default(),
            economic_development: EswatiniEconomicDevelopment::default(),
        }
    }
}

impl Default for GabonLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: GabonConstitutionalFramework::default(),
            presidential_system: GabonPresidentialSystem::default(),
            oil_governance: GabonOilGovernance::default(),
            traditional_governance: GabonTraditionalGovernance::default(),
        }
    }
}

impl Default for EquatorialGuineaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: EGConstitutionalFramework::default(),
            presidential_system: EGPresidentialSystem::default(),
            oil_governance: EGOilGovernance::default(),
            traditional_governance: EGTraditionalGovernance::default(),
        }
    }
}

impl Default for CongoBrazzavilleLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: CBConstitutionalFramework::default(),
            presidential_system: CBPresidentialSystem::default(),
            oil_governance: CBOilGovernance::default(),
            traditional_governance: CBTraditionalGovernance::default(),
        }
    }
}

impl Default for ComorosLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ComorosConstitutionalFramework::default(),
            federal_system: ComorosFederalSystem::default(),
            islamic_governance: ComorosIslamicGovernance::default(),
            island_governance: ComorosIslandGovernance::default(),
        }
    }
}

impl Default for MauritiusLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: MauritiusConstitutionalFramework::default(),
            parliamentary_democracy: MauritiusParliamentaryDemocracy::default(),
            multicultural_governance: MauritiusMulticulturalGovernance::default(),
            economic_development: MauritiusEconomicDevelopment::default(),
        }
    }
}

impl Default for SeychellesLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SeychellesConstitutionalFramework::default(),
            presidential_system: SeychellesPresidentialSystem::default(),
            environmental_governance: SeychellesEnvironmentalGovernance::default(),
            economic_development: SeychellesEconomicDevelopment::default(),
        }
    }
}

impl Default for SaoTomeLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SaoTomeConstitutionalFramework::default(),
            semi_presidential_system: SaoTomeSemiPresidentialSystem::default(),
            island_governance: SaoTomeIslandGovernance::default(),
            economic_development: SaoTomeEconomicDevelopment::default(),
        }
    }
}

impl Default for CapeVerdeLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: CapeVerdeConstitutionalFramework::default(),
            semi_presidential_system: CapeVerdeSemiPresidentialSystem::default(),
            island_governance: CapeVerdeIslandGovernance::default(),
            economic_development: CapeVerdeEconomicDevelopment::default(),
        }
    }
}

impl Default for MauritaniaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: MauritaniaConstitutionalFramework::default(),
            presidential_system: MauritaniaPresidentialSystem::default(),
            islamic_republic: MauritaniaIslamicRepublic::default(),
            traditional_governance: MauritaniaTraditionalGovernance::default(),
        }
    }
}

pub fn create_remaining_africa_systems() -> (
    MalawiLegalSystem, LesothoLegalSystem, EswatiniLegalSystem, GabonLegalSystem,
    EquatorialGuineaLegalSystem, CongoBrazzavilleLegalSystem, ComorosLegalSystem,
    MauritiusLegalSystem, SeychellesLegalSystem, SaoTomeLegalSystem,
    CapeVerdeLegalSystem, MauritaniaLegalSystem
) {
    (
        MalawiLegalSystem::default(),
        LesothoLegalSystem::default(),
        EswatiniLegalSystem::default(),
        GabonLegalSystem::default(),
        EquatorialGuineaLegalSystem::default(),
        CongoBrazzavilleLegalSystem::default(),
        ComorosLegalSystem::default(),
        MauritiusLegalSystem::default(),
        SeychellesLegalSystem::default(),
        SaoTomeLegalSystem::default(),
        CapeVerdeLegalSystem::default(),
        MauritaniaLegalSystem::default(),
    )
}