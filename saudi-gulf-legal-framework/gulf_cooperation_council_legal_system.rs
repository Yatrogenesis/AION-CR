use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCCLegalFramework {
    pub gcc_charter: GCCCharter,
    pub member_states: Vec<GCCMemberState>,
    pub gcc_institutions: Vec<GCCInstitution>,
    pub unified_legislation: Vec<UnifiedLaw>,
    pub economic_integration: EconomicIntegration,
    pub security_cooperation: SecurityCooperation,
    pub dispute_resolution: DisputeResolution,
    pub external_relations: ExternalRelations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCCCharter {
    pub charter_name: String,
    pub signing_date: String,
    pub founding_principles: Vec<String>,
    pub objectives: Vec<String>,
    pub membership_criteria: Vec<String>,
    pub institutional_framework: String,
    pub amendment_procedures: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCCMemberState {
    pub country_name: String,
    pub ruler_title: String,
    pub current_ruler: String,
    pub capital_city: String,
    pub population: u64,
    pub area_km2: u64,
    pub gdp_usd: u64,
    pub constitutional_system: ConstitutionalSystem,
    pub legal_system: NationalLegalSystem,
    pub economic_profile: EconomicProfile,
    pub governance_structure: GovernanceStructure,
    pub gcc_integration_level: String,
    pub special_characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalSystem {
    pub constitution_name: String,
    pub promulgation_date: String,
    pub constitutional_type: String, // Absolute Monarchy, Constitutional Monarchy, etc.
    pub amendment_mechanism: String,
    pub fundamental_principles: Vec<String>,
    pub rights_framework: Vec<String>,
    pub institutional_design: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalLegalSystem {
    pub legal_tradition: String, // Islamic Law, Mixed System, etc.
    pub court_structure: CourtStructure,
    pub legislation_hierarchy: Vec<String>,
    pub law_sources: Vec<String>,
    pub international_law_integration: String,
    pub modern_codification: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtStructure {
    pub supreme_court: String,
    pub appellate_courts: Vec<String>,
    pub specialized_courts: Vec<String>,
    pub judicial_administration: String,
    pub judge_appointment: String,
    pub judicial_independence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicProfile {
    pub primary_industries: Vec<String>,
    pub economic_diversification: String,
    pub sovereign_wealth_funds: Vec<String>,
    pub trade_patterns: Vec<String>,
    pub development_strategies: Vec<String>,
    pub fiscal_framework: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStructure {
    pub executive_branch: ExecutiveStructure,
    pub legislative_branch: LegislativeStructure,
    pub judicial_branch: JudicialStructure,
    pub advisory_councils: Vec<String>,
    pub federal_structure: Option<FederalStructure>,
    pub local_government: LocalGovernment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveStructure {
    pub head_of_state: String,
    pub head_of_government: String,
    pub cabinet_structure: String,
    pub appointment_mechanism: String,
    pub executive_powers: Vec<String>,
    pub succession_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeStructure {
    pub parliament_name: String,
    pub chamber_structure: String, // Unicameral, Bicameral
    pub composition: String,
    pub legislative_powers: Vec<String>,
    pub election_appointment: String,
    pub legislative_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialStructure {
    pub court_system: String,
    pub judicial_review: String,
    pub independence_level: String,
    pub appointment_process: String,
    pub specialization: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederalStructure {
    pub federal_entities: Vec<String>,
    pub competence_distribution: String,
    pub federal_coordination: String,
    pub local_autonomy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGovernment {
    pub municipal_system: String,
    pub local_councils: String,
    pub service_delivery: Vec<String>,
    pub citizen_participation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCCInstitution {
    pub institution_name: String,
    pub establishment_date: String,
    pub headquarters: String,
    pub mandate: Vec<String>,
    pub membership: String,
    pub decision_making: String,
    pub secretariat: String,
    pub budget_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedLaw {
    pub law_title: String,
    pub adoption_date: String,
    pub legal_area: String,
    pub harmonization_level: String,
    pub implementation_status: HashMap<String, String>, // Country -> Status
    pub compliance_monitoring: String,
    pub dispute_resolution_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicIntegration {
    pub customs_union: CustomsUnion,
    pub common_market: CommonMarket,
    pub monetary_cooperation: MonetaryCooperation,
    pub infrastructure_integration: InfrastructureIntegration,
    pub investment_framework: InvestmentFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsUnion {
    pub establishment_date: String,
    pub common_external_tariff: String,
    pub trade_facilitation: Vec<String>,
    pub customs_procedures: String,
    pub implementation_challenges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMarket {
    pub free_movement_goods: String,
    pub free_movement_capital: String,
    pub free_movement_services: String,
    pub free_movement_persons: String,
    pub regulatory_harmonization: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryCooperation {
    pub monetary_union_plans: String,
    pub central_bank_cooperation: String,
    pub currency_coordination: String,
    pub financial_integration: String,
    pub banking_supervision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureIntegration {
    pub transportation_networks: Vec<String>,
    pub energy_integration: String,
    pub telecommunications: String,
    pub water_resources: String,
    pub cross_border_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentFramework {
    pub investment_agreements: Vec<String>,
    pub dispute_resolution: String,
    pub investment_protection: String,
    pub regulatory_cooperation: String,
    pub development_funds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCooperation {
    pub defense_cooperation: DefenseCooperation,
    pub internal_security: InternalSecurity,
    pub cybersecurity: Cybersecurity,
    pub counter_terrorism: CounterTerrorism,
    pub maritime_security: MaritimeSecurity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseCooperation {
    pub joint_defense_agreements: Vec<String>,
    pub military_exercises: Vec<String>,
    pub defense_procurement: String,
    pub intelligence_sharing: String,
    pub defense_industrial_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalSecurity {
    pub police_cooperation: String,
    pub border_security: String,
    pub information_sharing: String,
    pub joint_operations: Vec<String>,
    pub capacity_building: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cybersecurity {
    pub cyber_defense_framework: String,
    pub information_sharing: String,
    pub incident_response: String,
    pub capacity_building: String,
    pub regulatory_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterTerrorism {
    pub legal_framework: String,
    pub intelligence_cooperation: String,
    pub capacity_building: String,
    pub preventive_measures: String,
    pub international_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaritimeSecurity {
    pub maritime_domain_awareness: String,
    pub naval_cooperation: String,
    pub port_security: String,
    pub anti_piracy_operations: String,
    pub environmental_protection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolution {
    pub gcc_court: GCCCourt,
    pub arbitration_center: ArbitrationCenter,
    pub mediation_mechanisms: Vec<String>,
    pub diplomatic_procedures: String,
    pub enforcement_mechanisms: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCCCourt {
    pub establishment_date: String,
    pub jurisdiction: Vec<String>,
    pub composition: String,
    pub procedures: String,
    pub case_types: Vec<String>,
    pub enforcement_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationCenter {
    pub center_name: String,
    pub location: String,
    pub arbitration_rules: String,
    pub arbitrator_panel: String,
    pub case_management: String,
    pub enforcement_cooperation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalRelations {
    pub strategic_partnerships: Vec<String>,
    pub trade_agreements: Vec<String>,
    pub diplomatic_coordination: String,
    pub international_organizations: Vec<String>,
    pub collective_positions: Vec<String>,
}

impl GCCLegalFramework {
    pub fn new() -> Self {
        let gcc_charter = GCCCharter {
            charter_name: "Charter of the Cooperation Council for the Arab States of the Gulf".to_string(),
            signing_date: "1981-05-25".to_string(),
            founding_principles: vec![
                "Islamic faith and Arab identity".to_string(),
                "Common destiny and shared interests".to_string(),
                "Similar political systems".to_string(),
                "Economic cooperation and integration".to_string(),
                "Coordination in foreign policy".to_string()
            ],
            objectives: vec![
                "Achieve coordination, integration and inter-connection".to_string(),
                "Deepen and strengthen relations, links and areas of cooperation".to_string(),
                "Unify regulations in various fields".to_string(),
                "Stimulate scientific and technological progress".to_string(),
                "Establish scientific research centers".to_string()
            ],
            membership_criteria: vec![
                "Arab state in the Gulf region".to_string(),
                "Similar political and social systems".to_string(),
                "Islamic faith".to_string(),
                "Commitment to GCC principles".to_string()
            ],
            institutional_framework: "Supreme Council, Ministerial Council, Secretariat General".to_string(),
            amendment_procedures: "Unanimous consent of all member states".to_string(),
        };

        let member_states = vec![
            GCCMemberState {
                country_name: "Saudi Arabia".to_string(),
                ruler_title: "King".to_string(),
                current_ruler: "King Salman bin Abdulaziz Al Saud".to_string(),
                capital_city: "Riyadh".to_string(),
                population: 35000000,
                area_km2: 2149690,
                gdp_usd: 833000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Basic Law of Governance".to_string(),
                    promulgation_date: "1992-03-01".to_string(),
                    constitutional_type: "Absolute Monarchy".to_string(),
                    amendment_mechanism: "Royal decree with scholarly consultation".to_string(),
                    fundamental_principles: vec!["Islamic governance".to_string(), "Sharia law supremacy".to_string()],
                    rights_framework: vec!["Islamic rights framework".to_string(), "Traditional protections".to_string()],
                    institutional_design: "Monarchy with advisory councils".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Islamic Law (Sharia)".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Supreme Court".to_string(),
                        appellate_courts: vec!["Courts of Appeal".to_string()],
                        specialized_courts: vec!["Commercial Courts".to_string(), "Labor Courts".to_string()],
                        judicial_administration: "Supreme Judicial Council".to_string(),
                        judge_appointment: "Royal appointment".to_string(),
                        judicial_independence: "Limited independence within Islamic framework".to_string(),
                    },
                    legislation_hierarchy: vec!["Basic Law".to_string(), "Royal Decrees".to_string(), "Ministerial Regulations".to_string()],
                    law_sources: vec!["Quran".to_string(), "Sunnah".to_string(), "Scholarly consensus".to_string()],
                    international_law_integration: "Selective integration within Islamic framework".to_string(),
                    modern_codification: vec!["Commercial Law".to_string(), "Labor Law".to_string(), "Capital Markets Law".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Oil and gas".to_string(), "Petrochemicals".to_string(), "Mining".to_string()],
                    economic_diversification: "Vision 2030 diversification program".to_string(),
                    sovereign_wealth_funds: vec!["Public Investment Fund".to_string()],
                    trade_patterns: vec!["Oil exports".to_string(), "Manufacturing imports".to_string()],
                    development_strategies: vec!["NEOM".to_string(), "Red Sea Project".to_string(), "Qiddiya".to_string()],
                    fiscal_framework: "Oil-dependent with diversification efforts".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "King".to_string(),
                        head_of_government: "King (Prime Minister)".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Royal appointment".to_string(),
                        executive_powers: vec!["Executive authority".to_string(), "Legislative initiative".to_string()],
                        succession_system: "Agnatic succession with Allegiance Council".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "Shura Council".to_string(),
                        chamber_structure: "Unicameral".to_string(),
                        composition: "150 appointed members".to_string(),
                        legislative_powers: vec!["Advisory role".to_string(), "Budget review".to_string()],
                        election_appointment: "Royal appointment".to_string(),
                        legislative_process: "Consultation and advice to King".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Islamic court system".to_string(),
                        judicial_review: "Limited constitutional review".to_string(),
                        independence_level: "Administrative independence".to_string(),
                        appointment_process: "Royal appointment with scholarly input".to_string(),
                        specialization: vec!["Sharia courts".to_string(), "Commercial courts".to_string()],
                    },
                    advisory_councils: vec!["Shura Council".to_string(), "Senior Scholars Council".to_string()],
                    federal_structure: None,
                    local_government: LocalGovernment {
                        municipal_system: "Provincial and municipal councils".to_string(),
                        local_councils: "Appointed with some elected elements".to_string(),
                        service_delivery: vec!["Municipal services".to_string(), "Local development".to_string()],
                        citizen_participation: "Limited but increasing".to_string(),
                    },
                },
                gcc_integration_level: "High - founding member and largest economy".to_string(),
                special_characteristics: vec!["Custodian of Two Holy Mosques".to_string(), "Largest GCC economy".to_string(), "Regional leader".to_string()],
            },
            GCCMemberState {
                country_name: "United Arab Emirates".to_string(),
                ruler_title: "President".to_string(),
                current_ruler: "Sheikh Mohamed bin Zayed Al Nahyan".to_string(),
                capital_city: "Abu Dhabi".to_string(),
                population: 9900000,
                area_km2: 83600,
                gdp_usd: 507000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Constitution of the United Arab Emirates".to_string(),
                    promulgation_date: "1971-12-02".to_string(),
                    constitutional_type: "Federal Constitutional Monarchy".to_string(),
                    amendment_mechanism: "Federal Supreme Council unanimous consent".to_string(),
                    fundamental_principles: vec!["Federal unity".to_string(), "Islamic identity".to_string(), "Arab heritage".to_string()],
                    rights_framework: vec!["Constitutional rights".to_string(), "Islamic principles".to_string()],
                    institutional_design: "Federal system with emirate autonomy".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Mixed Islamic and Civil Law".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Federal Supreme Court".to_string(),
                        appellate_courts: vec!["Federal Courts of Appeal".to_string(), "Local Courts of Appeal".to_string()],
                        specialized_courts: vec!["ADGM Courts".to_string(), "DIFC Courts".to_string(), "Commercial Courts".to_string()],
                        judicial_administration: "Federal Judicial Authority".to_string(),
                        judge_appointment: "Federal and local appointment".to_string(),
                        judicial_independence: "Constitutional independence".to_string(),
                    },
                    legislation_hierarchy: vec!["Constitution".to_string(), "Federal Laws".to_string(), "Local Laws".to_string(), "Regulations".to_string()],
                    law_sources: vec!["Constitution".to_string(), "Federal legislation".to_string(), "Islamic law".to_string(), "Custom".to_string()],
                    international_law_integration: "Active integration through specialized zones".to_string(),
                    modern_codification: vec!["Civil Code".to_string(), "Commercial Code".to_string(), "Penal Code".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Oil and gas".to_string(), "Trade and logistics".to_string(), "Tourism".to_string(), "Financial services".to_string()],
                    economic_diversification: "UAE Vision 2071 - most diversified GCC economy".to_string(),
                    sovereign_wealth_funds: vec!["Abu Dhabi Investment Authority".to_string(), "Mubadala".to_string()],
                    trade_patterns: vec!["Re-export hub".to_string(), "Services export".to_string()],
                    development_strategies: vec!["Dubai 2040".to_string(), "Abu Dhabi 2030".to_string(), "UAE Centennial 2071".to_string()],
                    fiscal_framework: "Federal and emirate level taxation".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "President".to_string(),
                        head_of_government: "Prime Minister".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Federal Supreme Council election".to_string(),
                        executive_powers: vec!["Federal executive authority".to_string(), "Foreign policy".to_string()],
                        succession_system: "Federal Supreme Council election".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "Federal National Council".to_string(),
                        chamber_structure: "Unicameral".to_string(),
                        composition: "40 members - half elected, half appointed".to_string(),
                        legislative_powers: vec!["Federal law review".to_string(), "Budget scrutiny".to_string()],
                        election_appointment: "Mixed election and appointment".to_string(),
                        legislative_process: "Consultative with limited legislative power".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Federal and local court systems".to_string(),
                        judicial_review: "Constitutional review by Federal Supreme Court".to_string(),
                        independence_level: "Constitutional independence".to_string(),
                        appointment_process: "Federal Supreme Council appointment".to_string(),
                        specialization: vec!["Federal courts".to_string(), "Local courts".to_string(), "Financial free zones".to_string()],
                    },
                    advisory_councils: vec!["Federal National Council".to_string(), "Emirates councils".to_string()],
                    federal_structure: Some(FederalStructure {
                        federal_entities: vec!["Abu Dhabi".to_string(), "Dubai".to_string(), "Sharjah".to_string(), "Ajman".to_string(), "Umm Al Quwain".to_string(), "Ras Al Khaimah".to_string(), "Fujairah".to_string()],
                        competence_distribution: "Federal powers vs. emirate reserved powers".to_string(),
                        federal_coordination: "Federal Supreme Council coordination".to_string(),
                        local_autonomy: "Significant emirate autonomy".to_string(),
                    }),
                    local_government: LocalGovernment {
                        municipal_system: "Emirate-level municipal systems".to_string(),
                        local_councils: "Emirate and municipal councils".to_string(),
                        service_delivery: vec!["Local services".to_string(), "Infrastructure".to_string()],
                        citizen_participation: "Increasing participation mechanisms".to_string(),
                    },
                },
                gcc_integration_level: "High - founding member with strong regional ties".to_string(),
                special_characteristics: vec!["Federal system".to_string(), "Economic diversification leader".to_string(), "International business hub".to_string()],
            },
            GCCMemberState {
                country_name: "Kuwait".to_string(),
                ruler_title: "Emir".to_string(),
                current_ruler: "Sheikh Mishal Al-Ahmad Al-Jaber Al-Sabah".to_string(),
                capital_city: "Kuwait City".to_string(),
                population: 4270000,
                area_km2: 17818,
                gdp_usd: 184000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Constitution of Kuwait".to_string(),
                    promulgation_date: "1962-11-11".to_string(),
                    constitutional_type: "Constitutional Monarchy".to_string(),
                    amendment_mechanism: "Two-thirds parliamentary majority and Emiri approval".to_string(),
                    fundamental_principles: vec!["Democratic governance".to_string(), "Islamic identity".to_string(), "Constitutional monarchy".to_string()],
                    rights_framework: vec!["Constitutional bill of rights".to_string(), "Democratic freedoms".to_string()],
                    institutional_design: "Parliamentary system with Emiri powers".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Mixed Islamic and Civil Law".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Court of Cassation".to_string(),
                        appellate_courts: vec!["Court of Appeal".to_string()],
                        specialized_courts: vec!["Constitutional Court".to_string(), "Administrative Court".to_string()],
                        judicial_administration: "Supreme Judicial Council".to_string(),
                        judge_appointment: "Higher Judicial Council".to_string(),
                        judicial_independence: "Constitutional independence".to_string(),
                    },
                    legislation_hierarchy: vec!["Constitution".to_string(), "Laws".to_string(), "Emiri Decrees".to_string(), "Regulations".to_string()],
                    law_sources: vec!["Constitution".to_string(), "Parliamentary legislation".to_string(), "Islamic law".to_string()],
                    international_law_integration: "Constitutional integration of international law".to_string(),
                    modern_codification: vec!["Civil Code".to_string(), "Commercial Code".to_string(), "Penal Code".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Oil and gas".to_string(), "Petrochemicals".to_string(), "Financial services".to_string()],
                    economic_diversification: "Kuwait Vision 2035 diversification".to_string(),
                    sovereign_wealth_funds: vec!["Kuwait Investment Authority".to_string()],
                    trade_patterns: vec!["Oil exports".to_string(), "Consumer goods imports".to_string()],
                    development_strategies: vec!["New Kuwait Vision".to_string(), "Silk City project".to_string()],
                    fiscal_framework: "Oil-dependent with sovereign wealth management".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "Emir".to_string(),
                        head_of_government: "Prime Minister".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Emiri appointment with parliamentary confidence".to_string(),
                        executive_powers: vec!["Executive authority".to_string(), "Foreign policy".to_string()],
                        succession_system: "Al-Sabah family succession".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "National Assembly (Majlis al-Ummah)".to_string(),
                        chamber_structure: "Unicameral".to_string(),
                        composition: "50 elected members plus ministers".to_string(),
                        legislative_powers: vec!["Legislative authority".to_string(), "Budget approval".to_string(), "Government oversight".to_string()],
                        election_appointment: "Direct election".to_string(),
                        legislative_process: "Full legislative powers with Emiri veto".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Independent judicial system".to_string(),
                        judicial_review: "Constitutional Court judicial review".to_string(),
                        independence_level: "Constitutional independence".to_string(),
                        appointment_process: "Higher Judicial Council".to_string(),
                        specialization: vec!["Civil courts".to_string(), "Criminal courts".to_string(), "Administrative courts".to_string()],
                    },
                    advisory_councils: vec!["State Council".to_string()],
                    federal_structure: None,
                    local_government: LocalGovernment {
                        municipal_system: "Municipal Council".to_string(),
                        local_councils: "Elected Municipal Council".to_string(),
                        service_delivery: vec!["Municipal services".to_string(), "Urban planning".to_string()],
                        citizen_participation: "Direct election of municipal council".to_string(),
                    },
                },
                gcc_integration_level: "High - founding member with strong parliamentary tradition".to_string(),
                special_characteristics: vec!["Parliamentary democracy".to_string(), "Strong legislature".to_string(), "Constitutional tradition".to_string()],
            },
            GCCMemberState {
                country_name: "Qatar".to_string(),
                ruler_title: "Emir".to_string(),
                current_ruler: "Sheikh Tamim bin Hamad Al Thani".to_string(),
                capital_city: "Doha".to_string(),
                population: 2881000,
                area_km2: 11586,
                gdp_usd: 236000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Permanent Constitution of Qatar".to_string(),
                    promulgation_date: "2003-06-08".to_string(),
                    constitutional_type: "Constitutional Monarchy".to_string(),
                    amendment_mechanism: "Two-thirds Shura Council majority and Emiri approval".to_string(),
                    fundamental_principles: vec!["Islamic Sharia".to_string(), "Democratic principles".to_string(), "Social justice".to_string()],
                    rights_framework: vec!["Constitutional rights".to_string(), "Islamic framework".to_string()],
                    institutional_design: "Monarchy with consultative council".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Mixed Islamic and Civil Law".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Court of Cassation".to_string(),
                        appellate_courts: vec!["Court of Appeal".to_string()],
                        specialized_courts: vec!["Constitutional Court".to_string(), "Administrative Court".to_string(), "QFC Courts".to_string()],
                        judicial_administration: "Supreme Judiciary Council".to_string(),
                        judge_appointment: "Emiri appointment on judicial council recommendation".to_string(),
                        judicial_independence: "Constitutional independence".to_string(),
                    },
                    legislation_hierarchy: vec!["Constitution".to_string(), "Laws".to_string(), "Emiri Decrees".to_string()],
                    law_sources: vec!["Constitution".to_string(), "Islamic Sharia".to_string(), "Legislation".to_string()],
                    international_law_integration: "Financial center common law integration".to_string(),
                    modern_codification: vec!["Civil Code".to_string(), "Commercial Code".to_string(), "QFC Law".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Natural gas (LNG)".to_string(), "Oil".to_string(), "Petrochemicals".to_string()],
                    economic_diversification: "Qatar National Vision 2030".to_string(),
                    sovereign_wealth_funds: vec!["Qatar Investment Authority".to_string()],
                    trade_patterns: vec!["LNG exports".to_string(), "Financial services".to_string()],
                    development_strategies: vec!["Qatar National Vision 2030".to_string(), "World Cup legacy".to_string()],
                    fiscal_framework: "Hydrocarbon revenues with diversification".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "Emir".to_string(),
                        head_of_government: "Prime Minister".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Emiri appointment".to_string(),
                        executive_powers: vec!["Executive authority".to_string(), "Policy direction".to_string()],
                        succession_system: "Al Thani family succession".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "Shura Council".to_string(),
                        chamber_structure: "Unicameral".to_string(),
                        composition: "45 members - 30 elected, 15 appointed".to_string(),
                        legislative_powers: vec!["Legislative approval".to_string(), "Budget review".to_string()],
                        election_appointment: "Mixed election and appointment".to_string(),
                        legislative_process: "Legislative role with Emiri powers".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Independent judiciary".to_string(),
                        judicial_review: "Constitutional review authority".to_string(),
                        independence_level: "Constitutional independence".to_string(),
                        appointment_process: "Supreme Judiciary Council recommendation".to_string(),
                        specialization: vec!["Civil courts".to_string(), "Sharia courts".to_string(), "Financial center courts".to_string()],
                    },
                    advisory_councils: vec!["Shura Council".to_string()],
                    federal_structure: None,
                    local_government: LocalGovernment {
                        municipal_system: "Central Municipal Council".to_string(),
                        local_councils: "Elected Municipal Council".to_string(),
                        service_delivery: vec!["Municipal services".to_string(), "Infrastructure development".to_string()],
                        citizen_participation: "Municipal elections".to_string(),
                    },
                },
                gcc_integration_level: "Medium - founding member with independent foreign policy".to_string(),
                special_characteristics: vec!["World's largest LNG exporter".to_string(), "2022 FIFA World Cup host".to_string(), "Global diplomatic hub".to_string()],
            },
            GCCMemberState {
                country_name: "Bahrain".to_string(),
                ruler_title: "King".to_string(),
                current_ruler: "King Hamad bin Isa Al Khalifa".to_string(),
                capital_city: "Manama".to_string(),
                population: 1701000,
                area_km2: 786,
                gdp_usd: 44000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Constitution of the Kingdom of Bahrain".to_string(),
                    promulgation_date: "2002-02-14".to_string(),
                    constitutional_type: "Constitutional Monarchy".to_string(),
                    amendment_mechanism: "Two-thirds parliamentary majority and royal approval".to_string(),
                    fundamental_principles: vec!["Constitutional monarchy".to_string(), "Islamic Sharia".to_string(), "Democratic participation".to_string()],
                    rights_framework: vec!["Bill of rights".to_string(), "Democratic freedoms".to_string()],
                    institutional_design: "Bicameral parliament with royal powers".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Mixed Islamic and Common Law".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Court of Cassation".to_string(),
                        appellate_courts: vec!["High Court of Appeal".to_string()],
                        specialized_courts: vec!["Constitutional Court".to_string(), "High Administrative Court".to_string()],
                        judicial_administration: "Supreme Judicial Council".to_string(),
                        judge_appointment: "Higher Judicial Council".to_string(),
                        judicial_independence: "Constitutional independence".to_string(),
                    },
                    legislation_hierarchy: vec!["Constitution".to_string(), "Laws".to_string(), "Royal Decrees".to_string()],
                    law_sources: vec!["Constitution".to_string(), "Islamic Sharia".to_string(), "Parliamentary laws".to_string()],
                    international_law_integration: "Financial center English law integration".to_string(),
                    modern_codification: vec!["Civil Code".to_string(), "Commercial Code".to_string(), "Financial services law".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Financial services".to_string(), "Oil refining".to_string(), "Aluminum".to_string()],
                    economic_diversification: "Most diversified GCC economy after UAE".to_string(),
                    sovereign_wealth_funds: vec!["Mumtalakat Holding Company".to_string()],
                    trade_patterns: vec!["Financial services export".to_string(), "Manufacturing".to_string()],
                    development_strategies: vec!["Bahrain Vision 2030".to_string(), "FinTech development".to_string()],
                    fiscal_framework: "Diversified revenue base".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "King".to_string(),
                        head_of_government: "Prime Minister".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Royal appointment".to_string(),
                        executive_powers: vec!["Executive authority".to_string(), "Constitutional powers".to_string()],
                        succession_system: "Al Khalifa family succession".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "National Assembly".to_string(),
                        chamber_structure: "Bicameral".to_string(),
                        composition: "Council of Representatives (40 elected) + Shura Council (40 appointed)".to_string(),
                        legislative_powers: vec!["Legislative authority".to_string(), "Budget approval".to_string()],
                        election_appointment: "Lower house elected, upper house appointed".to_string(),
                        legislative_process: "Bicameral legislative process".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Independent judiciary".to_string(),
                        judicial_review: "Constitutional Court review".to_string(),
                        independence_level: "Constitutional independence".to_string(),
                        appointment_process: "Higher Judicial Council".to_string(),
                        specialization: vec!["Civil courts".to_string(), "Sharia courts".to_string(), "Commercial courts".to_string()],
                    },
                    advisory_councils: vec!["Shura Council".to_string()],
                    federal_structure: None,
                    local_government: LocalGovernment {
                        municipal_system: "Northern and Southern Municipal Councils".to_string(),
                        local_councils: "Elected municipal councils".to_string(),
                        service_delivery: vec!["Municipal services".to_string(), "Local development".to_string()],
                        citizen_participation: "Municipal elections".to_string(),
                    },
                },
                gcc_integration_level: "High - founding member and regional financial center".to_string(),
                special_characteristics: vec!["Regional financial center".to_string(), "Bicameral parliament".to_string(), "Sectarian diversity".to_string()],
            },
            GCCMemberState {
                country_name: "Oman".to_string(),
                ruler_title: "Sultan".to_string(),
                current_ruler: "Sultan Haitham bin Tariq Al Said".to_string(),
                capital_city: "Muscat".to_string(),
                population: 5100000,
                area_km2: 309500,
                gdp_usd: 115000000000,
                constitutional_system: ConstitutionalSystem {
                    constitution_name: "Basic Statute of the State".to_string(),
                    promulgation_date: "1996-11-06".to_string(),
                    constitutional_type: "Absolute Monarchy".to_string(),
                    amendment_mechanism: "Sultanic decree".to_string(),
                    fundamental_principles: vec!["Sultanic authority".to_string(), "Islamic principles".to_string(), "National unity".to_string()],
                    rights_framework: vec!["Traditional protections".to_string(), "Islamic framework".to_string()],
                    institutional_design: "Consultative councils with Sultanic authority".to_string(),
                },
                legal_system: NationalLegalSystem {
                    legal_tradition: "Mixed Islamic and Civil Law".to_string(),
                    court_structure: CourtStructure {
                        supreme_court: "Supreme Court".to_string(),
                        appellate_courts: vec!["Courts of Appeal".to_string()],
                        specialized_courts: vec!["Administrative Court".to_string(), "Commercial Court".to_string()],
                        judicial_administration: "Supreme Judicial Council".to_string(),
                        judge_appointment: "Sultanic appointment".to_string(),
                        judicial_independence: "Administrative independence".to_string(),
                    },
                    legislation_hierarchy: vec!["Basic Statute".to_string(), "Royal Decrees".to_string(), "Ministerial Decisions".to_string()],
                    law_sources: vec!["Basic Statute".to_string(), "Islamic law".to_string(), "Royal decrees".to_string()],
                    international_law_integration: "Selective integration".to_string(),
                    modern_codification: vec!["Civil Code".to_string(), "Commercial Code".to_string(), "Penal Code".to_string()],
                },
                economic_profile: EconomicProfile {
                    primary_industries: vec!["Oil and gas".to_string(), "Mining".to_string(), "Fishing".to_string(), "Agriculture".to_string()],
                    economic_diversification: "Oman Vision 2040 diversification".to_string(),
                    sovereign_wealth_funds: vec!["Oman Investment Authority".to_string()],
                    trade_patterns: vec!["Oil exports".to_string(), "Re-export trade".to_string()],
                    development_strategies: vec!["Oman Vision 2040".to_string(), "Tourism development".to_string(), "Logistics hub".to_string()],
                    fiscal_framework: "Oil-dependent with fiscal reforms".to_string(),
                },
                governance_structure: GovernanceStructure {
                    executive_branch: ExecutiveStructure {
                        head_of_state: "Sultan".to_string(),
                        head_of_government: "Sultan".to_string(),
                        cabinet_structure: "Council of Ministers".to_string(),
                        appointment_mechanism: "Sultanic appointment".to_string(),
                        executive_powers: vec!["Absolute executive authority".to_string(), "Legislative initiative".to_string()],
                        succession_system: "Al Said family with Royal Family Council".to_string(),
                    },
                    legislative_branch: LegislativeStructure {
                        parliament_name: "Council of Oman".to_string(),
                        chamber_structure: "Bicameral".to_string(),
                        composition: "Majlis A'Shura (86 elected) + State Council (85 appointed)".to_string(),
                        legislative_powers: vec!["Consultative role".to_string(), "Policy review".to_string()],
                        election_appointment: "Lower house elected, upper house appointed".to_string(),
                        legislative_process: "Consultative with Sultanic authority".to_string(),
                    },
                    judicial_branch: JudicialStructure {
                        court_system: "Independent court system".to_string(),
                        judicial_review: "Limited constitutional review".to_string(),
                        independence_level: "Administrative independence".to_string(),
                        appointment_process: "Supreme Judicial Council recommendation".to_string(),
                        specialization: vec!["Civil courts".to_string(), "Sharia courts".to_string(), "Military courts".to_string()],
                    },
                    advisory_councils: vec!["Council of Oman".to_string(), "State Council".to_string()],
                    federal_structure: None,
                    local_government: LocalGovernment {
                        municipal_system: "Governorate system".to_string(),
                        local_councils: "Appointed governors and councils".to_string(),
                        service_delivery: vec!["Governorate services".to_string(), "Local administration".to_string()],
                        citizen_participation: "Traditional consultation mechanisms".to_string(),
                    },
                },
                gcc_integration_level: "Medium - founding member with independent foreign policy".to_string(),
                special_characteristics: vec!["Neutral foreign policy".to_string(), "Ibadi Islamic tradition".to_string(), "Balanced regional relations".to_string()],
            },
        ];

        GCCLegalFramework {
            gcc_charter,
            member_states,
            gcc_institutions: vec![
                GCCInstitution {
                    institution_name: "Supreme Council".to_string(),
                    establishment_date: "1981-05-25".to_string(),
                    headquarters: "Riyadh, Saudi Arabia".to_string(),
                    mandate: vec!["Highest GCC authority".to_string(), "Policy formulation".to_string(), "Strategic decisions".to_string()],
                    membership: "Heads of State of member countries".to_string(),
                    decision_making: "Consensus or unanimity".to_string(),
                    secretariat: "Secretary-General".to_string(),
                    budget_source: "Member state contributions".to_string(),
                },
                GCCInstitution {
                    institution_name: "Ministerial Council".to_string(),
                    establishment_date: "1981-05-25".to_string(),
                    headquarters: "Riyadh, Saudi Arabia".to_string(),
                    mandate: vec!["Policy preparation".to_string(), "Implementation oversight".to_string(), "Coordination".to_string()],
                    membership: "Foreign Ministers of member countries".to_string(),
                    decision_making: "Majority voting".to_string(),
                    secretariat: "Secretariat General support".to_string(),
                    budget_source: "Member state contributions".to_string(),
                }
            ],
            unified_legislation: vec![
                UnifiedLaw {
                    law_title: "Unified Economic Agreement".to_string(),
                    adoption_date: "1981-11-11".to_string(),
                    legal_area: "Economic cooperation and integration".to_string(),
                    harmonization_level: "Framework agreement with national implementation".to_string(),
                    implementation_status: {
                        let mut status = HashMap::new();
                        status.insert("Saudi Arabia".to_string(), "Implemented".to_string());
                        status.insert("UAE".to_string(), "Implemented".to_string());
                        status.insert("Kuwait".to_string(), "Implemented".to_string());
                        status.insert("Qatar".to_string(), "Implemented".to_string());
                        status.insert("Bahrain".to_string(), "Implemented".to_string());
                        status.insert("Oman".to_string(), "Implemented".to_string());
                        status
                    },
                    compliance_monitoring: "Secretariat General monitoring".to_string(),
                    dispute_resolution_mechanism: "GCC dispute resolution procedures".to_string(),
                }
            ],
            economic_integration: EconomicIntegration {
                customs_union: CustomsUnion {
                    establishment_date: "2003-01-01".to_string(),
                    common_external_tariff: "5% average tariff".to_string(),
                    trade_facilitation: vec!["Single window".to_string(), "Electronic processing".to_string()],
                    customs_procedures: "Harmonized customs procedures".to_string(),
                    implementation_challenges: vec!["Revenue sharing".to_string(), "Non-tariff barriers".to_string()],
                },
                common_market: CommonMarket {
                    free_movement_goods: "Achieved within customs union".to_string(),
                    free_movement_capital: "Partial implementation".to_string(),
                    free_movement_services: "Limited implementation".to_string(),
                    free_movement_persons: "GCC nationals free movement".to_string(),
                    regulatory_harmonization: vec!["Financial services".to_string(), "Telecommunications".to_string()],
                },
                monetary_cooperation: MonetaryCooperation {
                    monetary_union_plans: "Delayed indefinitely".to_string(),
                    central_bank_cooperation: "GCC Central Bank governors cooperation".to_string(),
                    currency_coordination: "Some countries pegged to USD".to_string(),
                    financial_integration: "Cross-border banking and investments".to_string(),
                    banking_supervision: "National supervision with coordination".to_string(),
                },
                infrastructure_integration: InfrastructureIntegration {
                    transportation_networks: vec!["GCC Railway".to_string(), "Road networks".to_string()],
                    energy_integration: "GCC Interconnection Authority (GCCIA)".to_string(),
                    telecommunications: "Integrated telecommunications infrastructure".to_string(),
                    water_resources: "Water security cooperation".to_string(),
                    cross_border_projects: vec!["Qatar-Bahrain Causeway".to_string(), "UAE-Oman pipeline".to_string()],
                },
                investment_framework: InvestmentFramework {
                    investment_agreements: vec!["Unified Investment Law".to_string(), "Investment promotion agreements".to_string()],
                    dispute_resolution: "GCC Commercial Arbitration Centre".to_string(),
                    investment_protection: "National treatment for GCC investors".to_string(),
                    regulatory_cooperation: "Investment facilitation measures".to_string(),
                    development_funds: vec!["GCC Development Fund".to_string(), "Arab Monetary Fund".to_string()],
                },
            },
            security_cooperation: SecurityCooperation {
                defense_cooperation: DefenseCooperation {
                    joint_defense_agreements: vec!["Peninsula Shield Force".to_string(), "Defense cooperation agreements".to_string()],
                    military_exercises: vec!["Peninsula Shield exercises".to_string(), "Naval cooperation".to_string()],
                    defense_procurement: "Coordinated defense procurement".to_string(),
                    intelligence_sharing: "Intelligence cooperation framework".to_string(),
                    defense_industrial_cooperation: "Joint defense manufacturing projects".to_string(),
                },
                internal_security: InternalSecurity {
                    police_cooperation: "GCC Police cooperation".to_string(),
                    border_security: "Integrated border management".to_string(),
                    information_sharing: "Criminal intelligence sharing".to_string(),
                    joint_operations: vec!["Counter-terrorism operations".to_string(), "Drug enforcement".to_string()],
                    capacity_building: "Joint training programs".to_string(),
                },
                cybersecurity: Cybersecurity {
                    cyber_defense_framework: "GCC Cybersecurity Framework".to_string(),
                    information_sharing: "Cyber threat intelligence sharing".to_string(),
                    incident_response: "Joint incident response capabilities".to_string(),
                    capacity_building: "Cybersecurity training and education".to_string(),
                    regulatory_cooperation: "Harmonized cybersecurity regulations".to_string(),
                },
                counter_terrorism: CounterTerrorism {
                    legal_framework: "Unified counter-terrorism legislation".to_string(),
                    intelligence_cooperation: "Counter-terrorism intelligence sharing".to_string(),
                    capacity_building: "Joint counter-terrorism training".to_string(),
                    preventive_measures: "Counter-radicalization programs".to_string(),
                    international_cooperation: "Cooperation with international partners".to_string(),
                },
                maritime_security: MaritimeSecurity {
                    maritime_domain_awareness: "Regional maritime security center".to_string(),
                    naval_cooperation: "Joint naval patrols and exercises".to_string(),
                    port_security: "Port security cooperation".to_string(),
                    anti_piracy_operations: "Joint anti-piracy operations".to_string(),
                    environmental_protection: "Marine environmental protection".to_string(),
                },
            },
            dispute_resolution: DisputeResolution {
                gcc_court: GCCCourt {
                    establishment_date: "Planned but not established".to_string(),
                    jurisdiction: vec!["Inter-state disputes".to_string(), "Economic disputes".to_string()],
                    composition: "To be determined".to_string(),
                    procedures: "To be developed".to_string(),
                    case_types: vec!["Treaty interpretation".to_string(), "Commercial disputes".to_string()],
                    enforcement_authority: "Member state enforcement".to_string(),
                },
                arbitration_center: ArbitrationCenter {
                    center_name: "GCC Commercial Arbitration Centre".to_string(),
                    location: "Bahrain".to_string(),
                    arbitration_rules: "UNCITRAL-based rules".to_string(),
                    arbitrator_panel: "Regional and international arbitrators".to_string(),
                    case_management: "Professional case management".to_string(),
                    enforcement_cooperation: "New York Convention enforcement".to_string(),
                },
                mediation_mechanisms: vec!["Diplomatic mediation".to_string(), "Good offices".to_string()],
                diplomatic_procedures: "Traditional Gulf diplomatic consultation".to_string(),
                enforcement_mechanisms: "Member state enforcement of decisions".to_string(),
            },
            external_relations: ExternalRelations {
                strategic_partnerships: vec!["United States".to_string(), "European Union".to_string(), "China".to_string(), "India".to_string()],
                trade_agreements: vec!["GCC-EU Cooperation Agreement".to_string(), "Various bilateral FTAs".to_string()],
                diplomatic_coordination: "Coordinated foreign policy positions".to_string(),
                international_organizations: vec!["Arab League".to_string(), "OIC".to_string(), "UN".to_string()],
                collective_positions: vec!["Energy policy".to_string(), "Regional security".to_string(), "Economic cooperation".to_string()],
            },
        }
    }

    pub fn get_member_state(&self, name: &str) -> Option<&GCCMemberState> {
        self.member_states.iter().find(|state| state.country_name == name)
    }

    pub fn get_gcc_institution(&self, name: &str) -> Option<&GCCInstitution> {
        self.gcc_institutions.iter().find(|inst| inst.institution_name == name)
    }

    pub fn analyze_integration_level(&self) -> String {
        let economic_integration = "Customs union achieved, common market partial";
        let political_integration = "Intergovernmental cooperation with limited supranational elements";
        let security_integration = "Strong defense and security cooperation";

        format!(
            "GCC Integration Analysis: {} | {} | {}. \
            The GCC represents one of the most successful regional integration efforts \
            in the developing world, with significant economic and security cooperation.",
            economic_integration, political_integration, security_integration
        )
    }

    pub fn get_unified_legislation(&self) -> &Vec<UnifiedLaw> {
        &self.unified_legislation
    }

    pub fn assess_dispute_resolution_capacity(&self) -> String {
        "GCC dispute resolution relies primarily on diplomatic consultation and mediation, \
        with commercial arbitration through the GCC Commercial Arbitration Centre. \
        Plans for a GCC Court remain unrealized, reflecting political sensitivities.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcc_framework_creation() {
        let gcc = GCCLegalFramework::new();
        assert_eq!(gcc.member_states.len(), 6);
        assert!(gcc.gcc_charter.founding_principles.len() > 0);
    }

    #[test]
    fn test_member_state_lookup() {
        let gcc = GCCLegalFramework::new();
        let saudi = gcc.get_member_state("Saudi Arabia");
        assert!(saudi.is_some());
        assert_eq!(saudi.unwrap().ruler_title, "King");
    }

    #[test]
    fn test_institution_lookup() {
        let gcc = GCCLegalFramework::new();
        let supreme_council = gcc.get_gcc_institution("Supreme Council");
        assert!(supreme_council.is_some());
    }

    #[test]
    fn test_integration_analysis() {
        let gcc = GCCLegalFramework::new();
        let analysis = gcc.analyze_integration_level();
        assert!(analysis.contains("Customs union achieved"));
    }
}