use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CARLegalSystem {
    pub constitutional_framework: ConstitutionalFramework,
    pub post_conflict_reconstruction: PostConflictReconstruction,
    pub presidential_system: PresidentialSystem,
    pub mineral_wealth_governance: MineralWealthGovernance,
    pub government_structure: GovernmentStructure,
    pub judicial_system: JudicialSystem,
    pub territorial_administration: TerritorialAdministration,
    pub traditional_governance: TraditionalGovernance,
    pub peace_reconciliation: PeaceReconciliation,
    pub economic_recovery: EconomicRecovery,
    pub natural_resources: NaturalResources,
    pub regional_integration: RegionalIntegration,
    pub francophone_identity: FrancophoneIdentity,
    pub plan_relance: PlanRelance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    pub constitution_2016: Constitution2016,
    pub fundamental_rights: FundamentalRights,
    pub separation_of_powers: SeparationOfPowers,
    pub constitutional_court: ConstitutionalCourt,
    pub emergency_powers: EmergencyPowers,
}

impl Default for CARLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: ConstitutionalFramework::default(),
            post_conflict_reconstruction: PostConflictReconstruction::default(),
            presidential_system: PresidentialSystem::default(),
            mineral_wealth_governance: MineralWealthGovernance::default(),
            government_structure: GovernmentStructure::default(),
            judicial_system: JudicialSystem::default(),
            territorial_administration: TerritorialAdministration::default(),
            traditional_governance: TraditionalGovernance::default(),
            peace_reconciliation: PeaceReconciliation::default(),
            economic_recovery: EconomicRecovery::default(),
            natural_resources: NaturalResources::default(),
            regional_integration: RegionalIntegration::default(),
            francophone_identity: FrancophoneIdentity::default(),
            plan_relance: PlanRelance::default(),
        }
    }
}

macro_rules! impl_defaults_car {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self { } } })*
    };
}

impl_defaults_car!(
    ConstitutionalFramework, Constitution2016, PostConflictReconstruction, PresidentialSystem,
    MineralWealthGovernance, GovernmentStructure, JudicialSystem, TerritorialAdministration,
    TraditionalGovernance, PeaceReconciliation, EconomicRecovery, NaturalResources,
    RegionalIntegration, FrancophoneIdentity, PlanRelance, FundamentalRights,
    SeparationOfPowers, ConstitutionalCourt, EmergencyPowers
);

pub fn create_car_legal_system() -> CARLegalSystem { CARLegalSystem::default() }