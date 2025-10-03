use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewZealandLegalSystem {
    pub constitutional_framework: NZConstitutionalFramework,
    pub parliamentary_democracy: NZParliamentaryDemocracy,
    pub constitutional_monarchy: NZConstitutionalMonarchy,
    pub unitary_system: NZUnitarySystem,
    pub government_structure: NZGovernmentStructure,
    pub judicial_system: NZJudicialSystem,
    pub maori_rights: MaoriRights,
    pub treaty_waitangi: TreatyOfWaitangi,
    pub local_government: NZLocalGovernment,
    pub economic_framework: NZEconomicFramework,
    pub regional_integration: NZRegionalIntegration,
    pub immigration_citizenship: NZImmigrationCitizenship,
    pub environmental_law: NZEnvironmentalLaw,
    pub social_welfare: NZSocialWelfare,
    pub defense_security: NZDefenseSecurity,
    pub common_law_system: NZCommonLawSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZConstitutionalFramework {
    pub constitution_act_1986: ConstitutionAct1986,
    pub unwritten_constitution: UnwrittenConstitution,
    pub bill_of_rights: NZBillOfRights,
    pub electoral_act: ElectoralAct,
    pub human_rights_act: HumanRightsAct,
    pub ombudsmen_act: OmbudsmenAct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionAct1986 {
    pub sovereign_role: String,
    pub governor_general: String,
    pub executive_authority: String,
    pub legislative_authority: String,
    pub judicial_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZParliamentaryDemocracy {
    pub westminster_system: NZWestminsterSystem,
    pub unicameral_parliament: UnicameralParliament,
    pub mmp_electoral_system: MMPElectoralSystem,
    pub responsible_government: NZResponsibleGovernment,
    pub political_parties: NZPoliticalParties,
    pub select_committees: SelectCommittees,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicameralParliament {
    pub house_of_representatives: NZHouseOfRepresentatives,
    pub speaker_role: SpeakerRole,
    pub parliamentary_procedure: ParliamentaryProcedure,
    pub legislative_process: LegislativeProcess,
    pub question_time: QuestionTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMPElectoralSystem {
    pub electorate_seats: String,
    pub party_list_seats: String,
    pub threshold_requirement: String,
    pub proportional_representation: String,
    pub electoral_commission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZConstitutionalMonarchy {
    pub sovereign_role: SovereignRole,
    pub governor_general_powers: NZGovernorGeneralPowers,
    pub reserve_powers: NZReservePowers,
    pub royal_assent: RoyalAssent,
    pub succession_laws: NZSuccessionLaws,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZUnitarySystem {
    pub central_government: CentralGovernment,
    pub local_government_structure: LocalGovernmentStructure,
    pub regional_councils: RegionalCouncils,
    pub territorial_authorities: TerritorialAuthorities,
    pub crown_entities: CrownEntities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZGovernmentStructure {
    pub executive_council: NZExecutiveCouncil,
    pub cabinet_system: NZCabinetSystem,
    pub prime_minister: NZPrimeMinister,
    pub ministers: NZMinisters,
    pub public_service: NZPublicService,
    pub state_sector: StateSector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZJudicialSystem {
    pub supreme_court: SupremeCourt,
    pub court_of_appeal: CourtOfAppeal,
    pub high_court: NZHighCourt,
    pub district_courts: DistrictCourts,
    pub specialist_courts: SpecialistCourts,
    pub tribunals: NZTribunals,
    pub judicial_independence: NZJudicialIndependence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCourt {
    pub establishment_2003: String,
    pub final_appellate_court: String,
    pub constitutional_jurisdiction: String,
    pub chief_justice: String,
    pub justices_appointment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaoriRights {
    pub treaty_principles: TreatyPrinciples,
    pub partnership_model: PartnershipModel,
    pub protection_participation: ProtectionParticipation,
    pub active_protection: ActiveProtection,
    pub redress_settlements: RedressSettlements,
    pub maori_development: MaoriDevelopment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyOfWaitangi {
    pub treaty_1840: Treaty1840,
    pub three_articles: ThreeArticles,
    pub two_versions: TwoVersions,
    pub waitangi_tribunal: WaitangiTribunal,
    pub treaty_settlements: TreatySettlements,
    pub ongoing_relationship: OngoingRelationship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treaty1840 {
    pub kawanatanga: String,
    pub tino_rangatiratanga: String,
    pub oritetanga: String,
    pub partnership_principle: String,
    pub protection_principle: String,
    pub participation_principle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZLocalGovernment {
    pub local_government_act: LocalGovernmentAct,
    pub regional_councils: NZRegionalCouncils,
    pub city_district_councils: CityDistrictCouncils,
    pub community_boards: CommunityBoards,
    pub rates_funding: RatesFunding,
    pub bylaws_regulations: BylawsRegulations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZEconomicFramework {
    pub reserve_bank_act: ReserveBankAct,
    pub commerce_act: CommerceAct,
    pub fair_trading_act: FairTradingAct,
    pub companies_act: CompaniesAct,
    pub taxation_system: NZTaxationSystem,
    pub employment_relations: EmploymentRelations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZRegionalIntegration {
    pub anzus_treaty: NZANZUSSecurityTreaty,
    pub cptpp_participation: NZCPTPPParticipation,
    pub pacific_partnerships: NZPacificPartnerships,
    pub asean_dialogue: ASEANDialogue,
    pub apec_membership: NZAPECMembership,
    pub commonwealth_membership: CommonwealthMembership,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZImmigrationCitizenship {
    pub immigration_act: ImmigrationAct,
    pub citizenship_act: NZCitizenshipAct,
    pub refugee_protection: NZRefugeeProtection,
    pub skilled_migrant_category: SkilledMigrantCategory,
    pub family_reunification: FamilyReunification,
    pub humanitarian_policy: HumanitarianPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZEnvironmentalLaw {
    pub resource_management_act: ResourceManagementAct,
    pub climate_change_response: ClimateChangeResponse,
    pub conservation_act: ConservationAct,
    pub biosecurity_act: BiosecurityAct,
    pub exclusive_economic_zone: ExclusiveEconomicZone,
    pub zero_carbon_act: ZeroCarbonAct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZSocialWelfare {
    pub social_security_act: SocialSecurityAct,
    pub acc_system: ACCSystem,
    pub health_disability: HealthDisability,
    pub education_act: EducationAct,
    pub children_families: ChildrenFamilies,
    pub housing_assistance: HousingAssistance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZDefenseSecurity {
    pub defence_act: NZDefenceAct,
    pub gcsb_act: GCSBAct,
    pub sis_act: SISAct,
    pub national_security: NZNationalSecurity,
    pub cyber_security: NZCyberSecurity,
    pub intelligence_oversight: IntelligenceOversight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NZCommonLawSystem {
    pub precedent_system: NZPrecedentSystem,
    pub equity_principles: NZEquityPrinciples,
    pub statutory_interpretation: NZStatutoryInterpretation,
    pub bill_of_rights_application: BillOfRightsApplication,
    pub tort_law: NZTortLaw,
    pub contract_law: NZContractLaw,
}

macro_rules! impl_nz_defaults {
    ($($name:ident),*) => {
        $(impl Default for $name { fn default() -> Self { Self {} } })*
    };
}

impl_nz_defaults!(
    ConstitutionAct1986, UnwrittenConstitution, NZBillOfRights, ElectoralAct,
    HumanRightsAct, OmbudsmenAct, NZWestminsterSystem, NZHouseOfRepresentatives,
    SpeakerRole, ParliamentaryProcedure, LegislativeProcess, QuestionTime,
    MMPElectoralSystem, NZResponsibleGovernment, NZPoliticalParties, SelectCommittees,
    SovereignRole, NZGovernorGeneralPowers, NZReservePowers, RoyalAssent,
    NZSuccessionLaws, CentralGovernment, LocalGovernmentStructure, RegionalCouncils,
    TerritorialAuthorities, CrownEntities, NZExecutiveCouncil, NZCabinetSystem,
    NZPrimeMinister, NZMinisters, NZPublicService, StateSector, SupremeCourt,
    CourtOfAppeal, NZHighCourt, DistrictCourts, SpecialistCourts, NZTribunals,
    NZJudicialIndependence, TreatyPrinciples, PartnershipModel, ProtectionParticipation,
    ActiveProtection, RedressSettlements, MaoriDevelopment, Treaty1840,
    ThreeArticles, TwoVersions, WaitangiTribunal, TreatySettlements, OngoingRelationship,
    LocalGovernmentAct, NZRegionalCouncils, CityDistrictCouncils, CommunityBoards,
    RatesFunding, BylawsRegulations, ReserveBankAct, CommerceAct, FairTradingAct,
    CompaniesAct, NZTaxationSystem, EmploymentRelations, NZANZUSSecurityTreaty,
    NZCPTPPParticipation, NZPacificPartnerships, ASEANDialogue, NZAPECMembership,
    CommonwealthMembership, ImmigrationAct, NZCitizenshipAct, NZRefugeeProtection,
    SkilledMigrantCategory, FamilyReunification, HumanitarianPolicy, ResourceManagementAct,
    ClimateChangeResponse, ConservationAct, BiosecurityAct, ExclusiveEconomicZone,
    ZeroCarbonAct, SocialSecurityAct, ACCSystem, HealthDisability, EducationAct,
    ChildrenFamilies, HousingAssistance, NZDefenceAct, GCSBAct, SISAct,
    NZNationalSecurity, NZCyberSecurity, IntelligenceOversight, NZPrecedentSystem,
    NZEquityPrinciples, NZStatutoryInterpretation, BillOfRightsApplication,
    NZTortLaw, NZContractLaw
);

impl Default for NZConstitutionalFramework {
    fn default() -> Self {
        Self {
            constitution_act_1986: ConstitutionAct1986::default(),
            unwritten_constitution: UnwrittenConstitution::default(),
            bill_of_rights: NZBillOfRights::default(),
            electoral_act: ElectoralAct::default(),
            human_rights_act: HumanRightsAct::default(),
            ombudsmen_act: OmbudsmenAct::default(),
        }
    }
}

impl Default for NZParliamentaryDemocracy {
    fn default() -> Self {
        Self {
            westminster_system: NZWestminsterSystem::default(),
            unicameral_parliament: UnicameralParliament::default(),
            mmp_electoral_system: MMPElectoralSystem::default(),
            responsible_government: NZResponsibleGovernment::default(),
            political_parties: NZPoliticalParties::default(),
            select_committees: SelectCommittees::default(),
        }
    }
}

impl Default for UnicameralParliament {
    fn default() -> Self {
        Self {
            house_of_representatives: NZHouseOfRepresentatives::default(),
            speaker_role: SpeakerRole::default(),
            parliamentary_procedure: ParliamentaryProcedure::default(),
            legislative_process: LegislativeProcess::default(),
            question_time: QuestionTime::default(),
        }
    }
}

impl Default for NZConstitutionalMonarchy {
    fn default() -> Self {
        Self {
            sovereign_role: SovereignRole::default(),
            governor_general_powers: NZGovernorGeneralPowers::default(),
            reserve_powers: NZReservePowers::default(),
            royal_assent: RoyalAssent::default(),
            succession_laws: NZSuccessionLaws::default(),
        }
    }
}

impl Default for NZUnitarySystem {
    fn default() -> Self {
        Self {
            central_government: CentralGovernment::default(),
            local_government_structure: LocalGovernmentStructure::default(),
            regional_councils: RegionalCouncils::default(),
            territorial_authorities: TerritorialAuthorities::default(),
            crown_entities: CrownEntities::default(),
        }
    }
}

impl Default for NZGovernmentStructure {
    fn default() -> Self {
        Self {
            executive_council: NZExecutiveCouncil::default(),
            cabinet_system: NZCabinetSystem::default(),
            prime_minister: NZPrimeMinister::default(),
            ministers: NZMinisters::default(),
            public_service: NZPublicService::default(),
            state_sector: StateSector::default(),
        }
    }
}

impl Default for NZJudicialSystem {
    fn default() -> Self {
        Self {
            supreme_court: SupremeCourt::default(),
            court_of_appeal: CourtOfAppeal::default(),
            high_court: NZHighCourt::default(),
            district_courts: DistrictCourts::default(),
            specialist_courts: SpecialistCourts::default(),
            tribunals: NZTribunals::default(),
            judicial_independence: NZJudicialIndependence::default(),
        }
    }
}

impl Default for MaoriRights {
    fn default() -> Self {
        Self {
            treaty_principles: TreatyPrinciples::default(),
            partnership_model: PartnershipModel::default(),
            protection_participation: ProtectionParticipation::default(),
            active_protection: ActiveProtection::default(),
            redress_settlements: RedressSettlements::default(),
            maori_development: MaoriDevelopment::default(),
        }
    }
}

impl Default for TreatyOfWaitangi {
    fn default() -> Self {
        Self {
            treaty_1840: Treaty1840::default(),
            three_articles: ThreeArticles::default(),
            two_versions: TwoVersions::default(),
            waitangi_tribunal: WaitangiTribunal::default(),
            treaty_settlements: TreatySettlements::default(),
            ongoing_relationship: OngoingRelationship::default(),
        }
    }
}

impl Default for NZLocalGovernment {
    fn default() -> Self {
        Self {
            local_government_act: LocalGovernmentAct::default(),
            regional_councils: NZRegionalCouncils::default(),
            city_district_councils: CityDistrictCouncils::default(),
            community_boards: CommunityBoards::default(),
            rates_funding: RatesFunding::default(),
            bylaws_regulations: BylawsRegulations::default(),
        }
    }
}

impl Default for NZEconomicFramework {
    fn default() -> Self {
        Self {
            reserve_bank_act: ReserveBankAct::default(),
            commerce_act: CommerceAct::default(),
            fair_trading_act: FairTradingAct::default(),
            companies_act: CompaniesAct::default(),
            taxation_system: NZTaxationSystem::default(),
            employment_relations: EmploymentRelations::default(),
        }
    }
}

impl Default for NZRegionalIntegration {
    fn default() -> Self {
        Self {
            anzus_treaty: NZANZUSSecurityTreaty::default(),
            cptpp_participation: NZCPTPPParticipation::default(),
            pacific_partnerships: NZPacificPartnerships::default(),
            asean_dialogue: ASEANDialogue::default(),
            apec_membership: NZAPECMembership::default(),
            commonwealth_membership: CommonwealthMembership::default(),
        }
    }
}

impl Default for NZImmigrationCitizenship {
    fn default() -> Self {
        Self {
            immigration_act: ImmigrationAct::default(),
            citizenship_act: NZCitizenshipAct::default(),
            refugee_protection: NZRefugeeProtection::default(),
            skilled_migrant_category: SkilledMigrantCategory::default(),
            family_reunification: FamilyReunification::default(),
            humanitarian_policy: HumanitarianPolicy::default(),
        }
    }
}

impl Default for NZEnvironmentalLaw {
    fn default() -> Self {
        Self {
            resource_management_act: ResourceManagementAct::default(),
            climate_change_response: ClimateChangeResponse::default(),
            conservation_act: ConservationAct::default(),
            biosecurity_act: BiosecurityAct::default(),
            exclusive_economic_zone: ExclusiveEconomicZone::default(),
            zero_carbon_act: ZeroCarbonAct::default(),
        }
    }
}

impl Default for NZSocialWelfare {
    fn default() -> Self {
        Self {
            social_security_act: SocialSecurityAct::default(),
            acc_system: ACCSystem::default(),
            health_disability: HealthDisability::default(),
            education_act: EducationAct::default(),
            children_families: ChildrenFamilies::default(),
            housing_assistance: HousingAssistance::default(),
        }
    }
}

impl Default for NZDefenseSecurity {
    fn default() -> Self {
        Self {
            defence_act: NZDefenceAct::default(),
            gcsb_act: GCSBAct::default(),
            sis_act: SISAct::default(),
            national_security: NZNationalSecurity::default(),
            cyber_security: NZCyberSecurity::default(),
            intelligence_oversight: IntelligenceOversight::default(),
        }
    }
}

impl Default for NZCommonLawSystem {
    fn default() -> Self {
        Self {
            precedent_system: NZPrecedentSystem::default(),
            equity_principles: NZEquityPrinciples::default(),
            statutory_interpretation: NZStatutoryInterpretation::default(),
            bill_of_rights_application: BillOfRightsApplication::default(),
            tort_law: NZTortLaw::default(),
            contract_law: NZContractLaw::default(),
        }
    }
}

impl Default for NewZealandLegalSystem {
    fn default() -> Self {
        Self {
            constitutional_framework: NZConstitutionalFramework::default(),
            parliamentary_democracy: NZParliamentaryDemocracy::default(),
            constitutional_monarchy: NZConstitutionalMonarchy::default(),
            unitary_system: NZUnitarySystem::default(),
            government_structure: NZGovernmentStructure::default(),
            judicial_system: NZJudicialSystem::default(),
            maori_rights: MaoriRights::default(),
            treaty_waitangi: TreatyOfWaitangi::default(),
            local_government: NZLocalGovernment::default(),
            economic_framework: NZEconomicFramework::default(),
            regional_integration: NZRegionalIntegration::default(),
            immigration_citizenship: NZImmigrationCitizenship::default(),
            environmental_law: NZEnvironmentalLaw::default(),
            social_welfare: NZSocialWelfare::default(),
            defense_security: NZDefenseSecurity::default(),
            common_law_system: NZCommonLawSystem::default(),
        }
    }
}

pub fn create_new_zealand_legal_system() -> NewZealandLegalSystem {
    NewZealandLegalSystem::default()
}