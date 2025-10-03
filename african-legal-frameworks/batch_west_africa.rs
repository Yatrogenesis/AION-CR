use serde::{Deserialize, Serialize};

// GUINEA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuineaLegalSystem {
    pub constitutional_framework: GuineaConstitutionalFramework,
    pub presidential_system: GuineaPresidentialSystem,
    pub mining_governance: GuineaMiningGovernance,
    pub traditional_governance: GuineaTraditionalGovernance,
    pub francophone_identity: GuineaFrancophoneIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuineaConstitutionalFramework {
    pub constitution_2020: String,
    pub fundamental_rights: Vec<String>,
}

// SIERRA LEONE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SierraLeoneLegalSystem {
    pub constitutional_framework: SLConstitutionalFramework,
    pub presidential_system: SLPresidentialSystem,
    pub post_civil_war: SLPostCivilWar,
    pub diamond_governance: SLDiamondGovernance,
    pub traditional_governance: SLTraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLConstitutionalFramework {
    pub constitution_1991: String,
    pub fundamental_rights: Vec<String>,
}

// LIBERIA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiberiaLegalSystem {
    pub constitutional_framework: LiberiaConstitutionalFramework,
    pub presidential_system: LiberiaPresidentialSystem,
    pub post_civil_war: LiberiaPostCivilWar,
    pub dual_legal_system: LiberiaDualLegalSystem,
    pub economic_development: LiberiaEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiberiaConstitutionalFramework {
    pub constitution_1986: String,
    pub fundamental_rights: Vec<String>,
}

// CÔTE D'IVOIRE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoteDIvoireLegalSystem {
    pub constitutional_framework: CIConstitutionalFramework,
    pub presidential_system: CIPresidentialSystem,
    pub post_crisis_reconciliation: CIPostCrisisReconciliation,
    pub cocoa_economy: CICocoaEconomy,
    pub traditional_governance: CITraditionalGovernance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIConstitutionalFramework {
    pub constitution_2016: String,
    pub fundamental_rights: Vec<String>,
}

// BENIN LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeninLegalSystem {
    pub constitutional_framework: BeninConstitutionalFramework,
    pub presidential_system: BeninPresidentialSystem,
    pub democratic_renewal: BeninDemocraticRenewal,
    pub traditional_governance: BeninTraditionalGovernance,
    pub economic_development: BeninEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeninConstitutionalFramework {
    pub constitution_1990: String,
    pub fundamental_rights: Vec<String>,
}

// TOGO LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TogoLegalSystem {
    pub constitutional_framework: TogoConstitutionalFramework,
    pub presidential_system: TogoPresidentialSystem,
    pub political_transition: TogoPoliticalTransition,
    pub traditional_governance: TogoTraditionalGovernance,
    pub economic_development: TogoEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TogoConstitutionalFramework {
    pub constitution_1992: String,
    pub fundamental_rights: Vec<String>,
}

// IMPLEMENTATION MACROS
macro_rules! impl_system_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_system_defaults!(
    GuineaConstitutionalFramework, GuineaPresidentialSystem, GuineaMiningGovernance,
    GuineaTraditionalGovernance, GuineaFrancophoneIdentity,
    SLConstitutionalFramework, SLPresidentialSystem, SLPostCivilWar, SLDiamondGovernance,
    SLTraditionalGovernance,
    LiberiaConstitutionalFramework, LiberiaPresidentialSystem, LiberiaPostCivilWar,
    LiberiaDualLegalSystem, LiberiaEconomicDevelopment,
    CIConstitutionalFramework, CIPresidentialSystem, CIPostCrisisReconciliation,
    CICocoaEconomy, CITraditionalGovernance,
    BeninConstitutionalFramework, BeninPresidentialSystem, BeninDemocraticRenewal,
    BeninTraditionalGovernance, BeninEconomicDevelopment,
    TogoConstitutionalFramework, TogoPresidentialSystem, TogoPoliticalTransition,
    TogoTraditionalGovernance, TogoEconomicDevelopment
);

impl Default for GuineaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: GuineaConstitutionalFramework::default(),
            presidential_system: GuineaPresidentialSystem::default(),
            mining_governance: GuineaMiningGovernance::default(),
            traditional_governance: GuineaTraditionalGovernance::default(),
            francophone_identity: GuineaFrancophoneIdentity::default(),
        }
    }
}

impl Default for SierraLeoneLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SLConstitutionalFramework::default(),
            presidential_system: SLPresidentialSystem::default(),
            post_civil_war: SLPostCivilWar::default(),
            diamond_governance: SLDiamondGovernance::default(),
            traditional_governance: SLTraditionalGovernance::default(),
        }
    }
}

impl Default for LiberiaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: LiberiaConstitutionalFramework::default(),
            presidential_system: LiberiaPresidentialSystem::default(),
            post_civil_war: LiberiaPostCivilWar::default(),
            dual_legal_system: LiberiaDualLegalSystem::default(),
            economic_development: LiberiaEconomicDevelopment::default(),
        }
    }
}

impl Default for CoteDIvoireLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: CIConstitutionalFramework::default(),
            presidential_system: CIPresidentialSystem::default(),
            post_crisis_reconciliation: CIPostCrisisReconciliation::default(),
            cocoa_economy: CICocoaEconomy::default(),
            traditional_governance: CITraditionalGovernance::default(),
        }
    }
}

impl Default for BeninLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: BeninConstitutionalFramework::default(),
            presidential_system: BeninPresidentialSystem::default(),
            democratic_renewal: BeninDemocraticRenewal::default(),
            traditional_governance: BeninTraditionalGovernance::default(),
            economic_development: BeninEconomicDevelopment::default(),
        }
    }
}

impl Default for TogoLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: TogoConstitutionalFramework::default(),
            presidential_system: TogoPresidentialSystem::default(),
            political_transition: TogoPoliticalTransition::default(),
            traditional_governance: TogoTraditionalGovernance::default(),
            economic_development: TogoEconomicDevelopment::default(),
        }
    }
}

pub fn create_west_africa_systems() -> (GuineaLegalSystem, SierraLeoneLegalSystem, LiberiaLegalSystem, CoteDIvoireLegalSystem, BeninLegalSystem, TogoLegalSystem) {
    (
        GuineaLegalSystem::default(),
        SierraLeoneLegalSystem::default(),
        LiberiaLegalSystem::default(),
        CoteDIvoireLegalSystem::default(),
        BeninLegalSystem::default(),
        TogoLegalSystem::default(),
    )
}