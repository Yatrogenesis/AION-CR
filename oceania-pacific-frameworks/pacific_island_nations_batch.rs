use serde::{Deserialize, Serialize};

// FIJI LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FijiLegalSystem {
    pub constitutional_framework: FijiConstitutionalFramework,
    pub parliamentary_democracy: FijiParliamentaryDemocracy,
    pub traditional_governance: FijiTraditionalGovernance,
    pub economic_development: FijiEconomicDevelopment,
    pub post_coup_reconstruction: PostCoupReconstruction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FijiConstitutionalFramework {
    pub constitution_2013: String,
    pub fundamental_rights: Vec<String>,
}

// PAPUA NEW GUINEA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PapuaNewGuineaLegalSystem {
    pub constitutional_framework: PNGConstitutionalFramework,
    pub parliamentary_democracy: PNGParliamentaryDemocracy,
    pub traditional_governance: PNGTraditionalGovernance,
    pub resource_governance: PNGResourceGovernance,
    pub cultural_diversity: PNGCulturalDiversity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PNGConstitutionalFramework {
    pub constitution_1975: String,
    pub fundamental_rights: Vec<String>,
}

// SOLOMON ISLANDS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolomonIslandsLegalSystem {
    pub constitutional_framework: SIConstitutionalFramework,
    pub parliamentary_democracy: SIParliamentaryDemocracy,
    pub traditional_governance: SITraditionalGovernance,
    pub economic_development: SIEconomicDevelopment,
    pub post_conflict_recovery: SIPostConflictRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIConstitutionalFramework {
    pub constitution_1978: String,
    pub fundamental_rights: Vec<String>,
}

// VANUATU LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VanuatuLegalSystem {
    pub constitutional_framework: VanuatuConstitutionalFramework,
    pub parliamentary_democracy: VanuatuParliamentaryDemocracy,
    pub traditional_governance: VanuatuTraditionalGovernance,
    pub economic_development: VanuatuEconomicDevelopment,
    pub dual_legal_system: VanuatuDualLegalSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VanuatuConstitutionalFramework {
    pub constitution_1980: String,
    pub fundamental_rights: Vec<String>,
}

// SAMOA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamoaLegalSystem {
    pub constitutional_framework: SamoaConstitutionalFramework,
    pub parliamentary_democracy: SamoaParliamentaryDemocracy,
    pub traditional_governance: SamoaTraditionalGovernance,
    pub matai_system: MataiSystem,
    pub economic_development: SamoaEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamoaConstitutionalFramework {
    pub constitution_1960: String,
    pub fundamental_rights: Vec<String>,
}

// TONGA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TongaLegalSystem {
    pub constitutional_framework: TongaConstitutionalFramework,
    pub constitutional_monarchy: TongaConstitutionalMonarchy,
    pub traditional_governance: TongaTraditionalGovernance,
    pub noble_system: NobleSystem,
    pub economic_development: TongaEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TongaConstitutionalFramework {
    pub constitution_1875: String,
    pub fundamental_rights: Vec<String>,
}

// KIRIBATI LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiribatiLegalSystem {
    pub constitutional_framework: KiribatiConstitutionalFramework,
    pub presidential_system: KiribatiPresidentialSystem,
    pub traditional_governance: KiribatiTraditionalGovernance,
    pub climate_adaptation: KiribatiClimateAdaptation,
    pub economic_development: KiribatiEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiribatiConstitutionalFramework {
    pub constitution_1979: String,
    pub fundamental_rights: Vec<String>,
}

// TUVALU LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuvaluLegalSystem {
    pub constitutional_framework: TuvaluConstitutionalFramework,
    pub parliamentary_democracy: TuvaluParliamentaryDemocracy,
    pub traditional_governance: TuvaluTraditionalGovernance,
    pub climate_vulnerability: TuvaluClimateVulnerability,
    pub economic_development: TuvaluEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuvaluConstitutionalFramework {
    pub constitution_1978: String,
    pub fundamental_rights: Vec<String>,
}

// NAURU LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NauruLegalSystem {
    pub constitutional_framework: NauruConstitutionalFramework,
    pub parliamentary_democracy: NauruParliamentaryDemocracy,
    pub traditional_governance: NauruTraditionalGovernance,
    pub phosphate_governance: PhosphateGovernance,
    pub economic_transition: NauruEconomicTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NauruConstitutionalFramework {
    pub constitution_1968: String,
    pub fundamental_rights: Vec<String>,
}

// PALAU LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalauLegalSystem {
    pub constitutional_framework: PalauConstitutionalFramework,
    pub presidential_system: PalauPresidentialSystem,
    pub traditional_governance: PalauTraditionalGovernance,
    pub environmental_protection: PalauEnvironmentalProtection,
    pub economic_development: PalauEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalauConstitutionalFramework {
    pub constitution_1981: String,
    pub fundamental_rights: Vec<String>,
}

// MARSHALL ISLANDS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarshallIslandsLegalSystem {
    pub constitutional_framework: MIConstitutionalFramework,
    pub parliamentary_democracy: MIParliamentaryDemocracy,
    pub traditional_governance: MITraditionalGovernance,
    pub compact_of_free_association: CompactOfFreeAssociation,
    pub climate_adaptation: MIClimateAdaptation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIConstitutionalFramework {
    pub constitution_1979: String,
    pub fundamental_rights: Vec<String>,
}

// FEDERATED STATES OF MICRONESIA LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicronesiaLegalSystem {
    pub constitutional_framework: FSMConstitutionalFramework,
    pub federal_system: FSMFederalSystem,
    pub traditional_governance: FSMTraditionalGovernance,
    pub compact_of_free_association: FSMCompactOfFreeAssociation,
    pub economic_development: FSMEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSMConstitutionalFramework {
    pub constitution_1979: String,
    pub fundamental_rights: Vec<String>,
}

// COOK ISLANDS LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookIslandsLegalSystem {
    pub constitutional_framework: CIConstitutionalFramework,
    pub self_governance: CISelfGovernance,
    pub traditional_governance: CITraditionalGovernance,
    pub free_association_nz: FreeAssociationNZ,
    pub economic_development: CIEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIConstitutionalFramework {
    pub constitution_1965: String,
    pub fundamental_rights: Vec<String>,
}

// NIUE LEGAL SYSTEM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NiueLegalSystem {
    pub constitutional_framework: NiueConstitutionalFramework,
    pub self_governance: NiueSelfGovernance,
    pub traditional_governance: NiueTraditionalGovernance,
    pub free_association_nz: NiueFreeAssociationNZ,
    pub economic_development: NiueEconomicDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NiueConstitutionalFramework {
    pub constitution_1974: String,
    pub fundamental_rights: Vec<String>,
}

macro_rules! impl_pacific_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_pacific_defaults!(
    FijiConstitutionalFramework, FijiParliamentaryDemocracy, FijiTraditionalGovernance,
    FijiEconomicDevelopment, PostCoupReconstruction,
    PNGConstitutionalFramework, PNGParliamentaryDemocracy, PNGTraditionalGovernance,
    PNGResourceGovernance, PNGCulturalDiversity,
    SIConstitutionalFramework, SIParliamentaryDemocracy, SITraditionalGovernance,
    SIEconomicDevelopment, SIPostConflictRecovery,
    VanuatuConstitutionalFramework, VanuatuParliamentaryDemocracy, VanuatuTraditionalGovernance,
    VanuatuEconomicDevelopment, VanuatuDualLegalSystem,
    SamoaConstitutionalFramework, SamoaParliamentaryDemocracy, SamoaTraditionalGovernance,
    MataiSystem, SamoaEconomicDevelopment,
    TongaConstitutionalFramework, TongaConstitutionalMonarchy, TongaTraditionalGovernance,
    NobleSystem, TongaEconomicDevelopment,
    KiribatiConstitutionalFramework, KiribatiPresidentialSystem, KiribatiTraditionalGovernance,
    KiribatiClimateAdaptation, KiribatiEconomicDevelopment,
    TuvaluConstitutionalFramework, TuvaluParliamentaryDemocracy, TuvaluTraditionalGovernance,
    TuvaluClimateVulnerability, TuvaluEconomicDevelopment,
    NauruConstitutionalFramework, NauruParliamentaryDemocracy, NauruTraditionalGovernance,
    PhosphateGovernance, NauruEconomicTransition,
    PalauConstitutionalFramework, PalauPresidentialSystem, PalauTraditionalGovernance,
    PalauEnvironmentalProtection, PalauEconomicDevelopment,
    MIConstitutionalFramework, MIParliamentaryDemocracy, MITraditionalGovernance,
    CompactOfFreeAssociation, MIClimateAdaptation,
    FSMConstitutionalFramework, FSMFederalSystem, FSMTraditionalGovernance,
    FSMCompactOfFreeAssociation, FSMEconomicDevelopment,
    CIConstitutionalFramework, CISelfGovernance, CITraditionalGovernance,
    FreeAssociationNZ, CIEconomicDevelopment,
    NiueConstitutionalFramework, NiueSelfGovernance, NiueTraditionalGovernance,
    NiueFreeAssociationNZ, NiueEconomicDevelopment
);

impl Default for FijiLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: FijiConstitutionalFramework::default(),
            parliamentary_democracy: FijiParliamentaryDemocracy::default(),
            traditional_governance: FijiTraditionalGovernance::default(),
            economic_development: FijiEconomicDevelopment::default(),
            post_coup_reconstruction: PostCoupReconstruction::default(),
        }
    }
}

impl Default for PapuaNewGuineaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: PNGConstitutionalFramework::default(),
            parliamentary_democracy: PNGParliamentaryDemocracy::default(),
            traditional_governance: PNGTraditionalGovernance::default(),
            resource_governance: PNGResourceGovernance::default(),
            cultural_diversity: PNGCulturalDiversity::default(),
        }
    }
}

impl Default for SolomonIslandsLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SIConstitutionalFramework::default(),
            parliamentary_democracy: SIParliamentaryDemocracy::default(),
            traditional_governance: SITraditionalGovernance::default(),
            economic_development: SIEconomicDevelopment::default(),
            post_conflict_recovery: SIPostConflictRecovery::default(),
        }
    }
}

impl Default for VanuatuLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: VanuatuConstitutionalFramework::default(),
            parliamentary_democracy: VanuatuParliamentaryDemocracy::default(),
            traditional_governance: VanuatuTraditionalGovernance::default(),
            economic_development: VanuatuEconomicDevelopment::default(),
            dual_legal_system: VanuatuDualLegalSystem::default(),
        }
    }
}

impl Default for SamoaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: SamoaConstitutionalFramework::default(),
            parliamentary_democracy: SamoaParliamentaryDemocracy::default(),
            traditional_governance: SamoaTraditionalGovernance::default(),
            matai_system: MataiSystem::default(),
            economic_development: SamoaEconomicDevelopment::default(),
        }
    }
}

impl Default for TongaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: TongaConstitutionalFramework::default(),
            constitutional_monarchy: TongaConstitutionalMonarchy::default(),
            traditional_governance: TongaTraditionalGovernance::default(),
            noble_system: NobleSystem::default(),
            economic_development: TongaEconomicDevelopment::default(),
        }
    }
}

impl Default for KiribatiLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: KiribatiConstitutionalFramework::default(),
            presidential_system: KiribatiPresidentialSystem::default(),
            traditional_governance: KiribatiTraditionalGovernance::default(),
            climate_adaptation: KiribatiClimateAdaptation::default(),
            economic_development: KiribatiEconomicDevelopment::default(),
        }
    }
}

impl Default for TuvaluLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: TuvaluConstitutionalFramework::default(),
            parliamentary_democracy: TuvaluParliamentaryDemocracy::default(),
            traditional_governance: TuvaluTraditionalGovernance::default(),
            climate_vulnerability: TuvaluClimateVulnerability::default(),
            economic_development: TuvaluEconomicDevelopment::default(),
        }
    }
}

impl Default for NauruLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: NauruConstitutionalFramework::default(),
            parliamentary_democracy: NauruParliamentaryDemocracy::default(),
            traditional_governance: NauruTraditionalGovernance::default(),
            phosphate_governance: PhosphateGovernance::default(),
            economic_transition: NauruEconomicTransition::default(),
        }
    }
}

impl Default for PalauLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: PalauConstitutionalFramework::default(),
            presidential_system: PalauPresidentialSystem::default(),
            traditional_governance: PalauTraditionalGovernance::default(),
            environmental_protection: PalauEnvironmentalProtection::default(),
            economic_development: PalauEconomicDevelopment::default(),
        }
    }
}

impl Default for MarshallIslandsLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: MIConstitutionalFramework::default(),
            parliamentary_democracy: MIParliamentaryDemocracy::default(),
            traditional_governance: MITraditionalGovernance::default(),
            compact_of_free_association: CompactOfFreeAssociation::default(),
            climate_adaptation: MIClimateAdaptation::default(),
        }
    }
}

impl Default for MicronesiaLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: FSMConstitutionalFramework::default(),
            federal_system: FSMFederalSystem::default(),
            traditional_governance: FSMTraditionalGovernance::default(),
            compact_of_free_association: FSMCompactOfFreeAssociation::default(),
            economic_development: FSMEconomicDevelopment::default(),
        }
    }
}

impl Default for CookIslandsLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: CIConstitutionalFramework::default(),
            self_governance: CISelfGovernance::default(),
            traditional_governance: CITraditionalGovernance::default(),
            free_association_nz: FreeAssociationNZ::default(),
            economic_development: CIEconomicDevelopment::default(),
        }
    }
}

impl Default for NiueLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: NiueConstitutionalFramework::default(),
            self_governance: NiueSelfGovernance::default(),
            traditional_governance: NiueTraditionalGovernance::default(),
            free_association_nz: NiueFreeAssociationNZ::default(),
            economic_development: NiueEconomicDevelopment::default(),
        }
    }
}

pub fn create_pacific_island_systems() -> (
    FijiLegalSystem, PapuaNewGuineaLegalSystem, SolomonIslandsLegalSystem, VanuatuLegalSystem,
    SamoaLegalSystem, TongaLegalSystem, KiribatiLegalSystem, TuvaluLegalSystem,
    NauruLegalSystem, PalauLegalSystem, MarshallIslandsLegalSystem, MicronesiaLegalSystem,
    CookIslandsLegalSystem, NiueLegalSystem
) {
    (
        FijiLegalSystem::default(),
        PapuaNewGuineaLegalSystem::default(),
        SolomonIslandsLegalSystem::default(),
        VanuatuLegalSystem::default(),
        SamoaLegalSystem::default(),
        TongaLegalSystem::default(),
        KiribatiLegalSystem::default(),
        TuvaluLegalSystem::default(),
        NauruLegalSystem::default(),
        PalauLegalSystem::default(),
        MarshallIslandsLegalSystem::default(),
        MicronesiaLegalSystem::default(),
        CookIslandsLegalSystem::default(),
        NiueLegalSystem::default(),
    )
}