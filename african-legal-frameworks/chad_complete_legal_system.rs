use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChadLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub presidential_system: PresidentialSystem,
    pub oil_governance: OilGovernance,
    pub sahel_security: SahelSecurity,
    pub government_structure: GovernmentStructure,
    pub judicial_system: JudicialSystem,
    pub territorial_administration: TerritorialAdministration,
    pub traditional_governance: TraditionalGovernance,
    pub economic_development: EconomicDevelopment,
    pub natural_resources: NaturalResources,
    pub regional_integration: RegionalIntegration,
    pub francophone_arabic_identity: FrancophoneArabicIdentity,
    pub vision_2030: Vision2030,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    pub constitution_2018: Constitution2018,
    pub fundamental_rights: FundamentalRights,
    pub separation_of_powers: SeparationOfPowers,
    pub constitutional_council: ConstitutionalCouncil,
    pub emergency_powers: EmergencyPowers,
}

impl Default for ChadLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ConstitutionalFramework::default(),
            presidential_system: PresidentialSystem::default(),
            oil_governance: OilGovernance::default(),
            sahel_security: SahelSecurity::default(),
            government_structure: GovernmentStructure::default(),
            judicial_system: JudicialSystem::default(),
            territorial_administration: TerritorialAdministration::default(),
            traditional_governance: TraditionalGovernance::default(),
            economic_development: EconomicDevelopment::default(),
            natural_resources: NaturalResources::default(),
            regional_integration: RegionalIntegration::default(),
            francophone_arabic_identity: FrancophoneArabicIdentity::default(),
            vision_2030: Vision2030::default(),
        }
    }
}

macro_rules! impl_defaults_chad {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self { } } })*
    };
}

impl_defaults_chad!(
    ConstitutionalFramework, Constitution2018, PresidentialSystem, OilGovernance,
    SahelSecurity, GovernmentStructure, JudicialSystem, TerritorialAdministration,
    TraditionalGovernance, EconomicDevelopment, NaturalResources, RegionalIntegration,
    FrancophoneArabicIdentity, Vision2030, FundamentalRights, SeparationOfPowers,
    ConstitutionalCouncil, EmergencyPowers
);

pub fn create_chad_legal_system() -> ChadLegalSystem { ChadLegalSystem::default() }