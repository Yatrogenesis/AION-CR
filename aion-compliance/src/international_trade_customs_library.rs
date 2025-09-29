use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalInternationalTradeCustomsLibrary {
    pub customs_regulations: CustomsRegulations,
    pub trade_agreements: TradeAgreements,
    pub export_control_regulations: ExportControlRegulations,
    pub import_regulations: ImportRegulations,
    pub free_trade_zones: FreeTradeZones,
    pub trade_remedies: TradeRemedies,
    pub sanctions_embargoes: SanctionsEmbargoes,
    pub origin_rules: OriginRules,
    pub trade_facilitation: TradeFacilitation,
    pub customs_valuation: CustomsValuation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsRegulations {
    pub wto_customs_frameworks: WTOCustomsFrameworks,
    pub us_customs_regulations: USCustomsRegulations,
    pub eu_customs_regulations: EUCustomsRegulations,
    pub uk_customs_regulations: UKCustomsRegulations,
    pub national_customs_codes: Vec<NationalCustomsCode>,
    pub customs_procedures: CustomsProcedures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCustomsRegulations {
    pub customs_modernization_act: CustomsModernizationAct,
    pub tariff_act_1930: TariffAct1930,
    pub trade_act_1974: TradeAct1974,
    pub customs_trade_partnership_against_terrorism: CTPAT,
    pub automated_commercial_environment: ACE,
    pub customs_broker_licensing: CustomsBrokerLicensing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAgreements {
    pub multilateral_agreements: MultilateralAgreements,
    pub bilateral_agreements: Vec<BilateralAgreement>,
    pub regional_trade_agreements: Vec<RegionalTradeAgreement>,
    pub preferential_trade_agreements: Vec<PreferentialTradeAgreement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilateralAgreements {
    pub wto_agreements: WTOAgreements,
    pub gatt_1994: GATT1994,
    pub general_agreement_trade_services: GATS,
    pub agreement_trade_related_aspects_intellectual_property: TRIPS,
    pub technical_barriers_to_trade: TBTAgreement,
    pub sanitary_phytosanitary_measures: SPSAgreement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WTOAgreements {
    pub dispute_settlement_understanding: DisputeSettlementUnderstanding,
    pub trade_policy_review_mechanism: TradePolicyReviewMechanism,
    pub plurilateral_agreements: Vec<PlurilateralAgreement>,
    pub trade_facilitation_agreement: TradeFacilitationAgreement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportControlRegulations {
    pub us_export_controls: USExportControls,
    pub eu_dual_use_regulation: EUDualUseRegulation,
    pub multilateral_export_control_regimes: MultilateralExportControlRegimes,
    pub strategic_trade_controls: StrategicTradeControls,
    pub technology_transfer_controls: TechnologyTransferControls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USExportControls {
    pub export_administration_regulations: ExportAdministrationRegulations,
    pub international_traffic_arms_regulations: ITAR,
    pub office_foreign_assets_control: OFAC,
    pub foreign_direct_product_rule: ForeignDirectProductRule,
    pub deemed_export_controls: DeemedExportControls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAdministrationRegulations {
    pub commerce_control_list: CommerceControlList,
    pub country_chart: CountryChart,
    pub license_requirements: Vec<AtomicLegalRule>,
    pub license_exceptions: Vec<AtomicLegalRule>,
    pub end_user_controls: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRegulations {
    pub tariff_classifications: TariffClassifications,
    pub customs_duties: CustomsDuties,
    pub non_tariff_barriers: NonTariffBarriers,
    pub import_licensing: ImportLicensing,
    pub product_safety_standards: ProductSafetyStandards,
    pub phytosanitary_requirements: PhytosanitaryRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsEmbargoes {
    pub us_sanctions: USSanctions,
    pub eu_sanctions: EUSanctions,
    pub un_sanctions: UNSanctions,
    pub uk_sanctions: UKSanctions,
    pub national_sanctions: Vec<NationalSanctionsRegime>,
    pub sectoral_sanctions: Vec<SectoralSanctions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USSanctions {
    pub ofac_sanctions_programs: Vec<OFACSanctionsProgram>,
    pub specially_designated_nationals: SpeciallyDesignatedNationals,
    pub sectoral_sanctions_identifications: SectoralSanctionsIdentifications,
    pub foreign_sanctions_evaders: ForeignSanctionsEvaders,
    pub non_sdn_menu_based_sanctions: NonSDNMenuBasedSanctions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRemedies {
    pub antidumping_measures: AntidumpingMeasures,
    pub countervailing_duties: CountervailingDuties,
    pub safeguard_measures: SafeguardMeasures,
    pub trade_adjustment_assistance: TradeAdjustmentAssistance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginRules {
    pub non_preferential_origin: NonPreferentialOrigin,
    pub preferential_origin: PreferentialOrigin,
    pub substantial_transformation: SubstantialTransformation,
    pub value_added_requirements: ValueAddedRequirements,
    pub origin_marking_requirements: OriginMarkingRequirements,
}

impl GlobalInternationalTradeCustomsLibrary {
    pub fn new() -> Self {
        Self {
            customs_regulations: CustomsRegulations::new(),
            trade_agreements: TradeAgreements::new(),
            export_control_regulations: ExportControlRegulations::new(),
            import_regulations: ImportRegulations::new(),
            free_trade_zones: FreeTradeZones::new(),
            trade_remedies: TradeRemedies::new(),
            sanctions_embargoes: SanctionsEmbargoes::new(),
            origin_rules: OriginRules::new(),
            trade_facilitation: TradeFacilitation::new(),
            customs_valuation: CustomsValuation::new(),
        }
    }

    pub fn get_customs_regulations(&self, jurisdiction: &str, trade_type: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        match jurisdiction.to_uppercase().as_str() {
            "US" => {
                rules.extend(self.customs_regulations.us_customs_regulations.get_applicable_rules(trade_type));
            },
            "EU" => {
                rules.extend(self.customs_regulations.eu_customs_regulations.get_applicable_rules(trade_type));
            },
            "UK" => {
                rules.extend(self.customs_regulations.uk_customs_regulations.get_applicable_rules(trade_type));
            },
            _ => {
                for code in &self.customs_regulations.national_customs_codes {
                    if code.jurisdiction == jurisdiction {
                        rules.extend(code.rules.clone());
                    }
                }
            }
        }

        rules.extend(self.customs_regulations.wto_customs_frameworks.get_applicable_rules());

        rules
    }

    pub fn get_export_control_rules(&self, jurisdiction: &str, product_category: &str, destination: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        match jurisdiction.to_uppercase().as_str() {
            "US" => {
                rules.extend(self.export_control_regulations.us_export_controls.get_applicable_rules(product_category, destination));
            },
            "EU" => {
                rules.extend(self.export_control_regulations.eu_dual_use_regulation.get_applicable_rules(product_category, destination));
            },
            _ => {}
        }

        rules.extend(self.export_control_regulations.multilateral_export_control_regimes.get_applicable_rules(product_category));

        rules
    }

    pub fn get_trade_agreement_rules(&self, country1: &str, country2: &str, trade_type: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        // Check bilateral agreements
        for agreement in &self.trade_agreements.bilateral_agreements {
            if (agreement.country1 == country1 && agreement.country2 == country2) ||
               (agreement.country1 == country2 && agreement.country2 == country1) {
                rules.extend(agreement.provisions.clone());
            }
        }

        // Check regional agreements
        for agreement in &self.trade_agreements.regional_trade_agreements {
            if agreement.member_countries.contains(&country1.to_string()) &&
               agreement.member_countries.contains(&country2.to_string()) {
                rules.extend(agreement.provisions.clone());
            }
        }

        // Always include WTO rules
        rules.extend(self.trade_agreements.multilateral_agreements.wto_agreements.get_applicable_rules(trade_type));

        rules
    }

    pub fn get_sanctions_rules(&self, jurisdiction: &str, target_country: &str, product_sector: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        match jurisdiction.to_uppercase().as_str() {
            "US" => {
                rules.extend(self.sanctions_embargoes.us_sanctions.get_applicable_rules(target_country, product_sector));
            },
            "EU" => {
                rules.extend(self.sanctions_embargoes.eu_sanctions.get_applicable_rules(target_country, product_sector));
            },
            "UK" => {
                rules.extend(self.sanctions_embargoes.uk_sanctions.get_applicable_rules(target_country, product_sector));
            },
            _ => {}
        }

        rules.extend(self.sanctions_embargoes.un_sanctions.get_applicable_rules(target_country, product_sector));

        rules
    }

    pub fn get_import_regulations(&self, jurisdiction: &str, product_category: &str, origin_country: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        rules.extend(self.import_regulations.tariff_classifications.get_applicable_rules(jurisdiction, product_category));
        rules.extend(self.import_regulations.customs_duties.get_applicable_rules(jurisdiction, product_category, origin_country));
        rules.extend(self.import_regulations.product_safety_standards.get_applicable_rules(jurisdiction, product_category));

        if product_category.contains("agricultural") || product_category.contains("food") {
            rules.extend(self.import_regulations.phytosanitary_requirements.get_applicable_rules(jurisdiction, origin_country));
        }

        rules
    }

    pub fn get_origin_rules(&self, jurisdiction: &str, product_category: &str, manufacturing_process: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        rules.extend(self.origin_rules.non_preferential_origin.get_applicable_rules(jurisdiction, product_category));
        rules.extend(self.origin_rules.substantial_transformation.get_applicable_rules(manufacturing_process));
        rules.extend(self.origin_rules.origin_marking_requirements.get_applicable_rules(jurisdiction));

        rules
    }

    pub fn get_trade_remedy_rules(&self, jurisdiction: &str, remedy_type: &str, product_category: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        match remedy_type.to_lowercase().as_str() {
            "antidumping" => {
                rules.extend(self.trade_remedies.antidumping_measures.get_applicable_rules(jurisdiction, product_category));
            },
            "countervailing" => {
                rules.extend(self.trade_remedies.countervailing_duties.get_applicable_rules(jurisdiction, product_category));
            },
            "safeguard" => {
                rules.extend(self.trade_remedies.safeguard_measures.get_applicable_rules(jurisdiction, product_category));
            },
            _ => {}
        }

        rules
    }
}

impl CustomsRegulations {
    pub fn new() -> Self {
        Self {
            wto_customs_frameworks: WTOCustomsFrameworks::new(),
            us_customs_regulations: USCustomsRegulations::new(),
            eu_customs_regulations: EUCustomsRegulations::new(),
            uk_customs_regulations: UKCustomsRegulations::new(),
            national_customs_codes: Self::create_national_customs_codes(),
            customs_procedures: CustomsProcedures::new(),
        }
    }

    fn create_national_customs_codes() -> Vec<NationalCustomsCode> {
        vec![
            NationalCustomsCode {
                jurisdiction: "CA".to_string(),
                code_name: "Canada Customs Act".to_string(),
                rules: vec![
                    AtomicLegalRule {
                        id: "CA.CBSA.CUSTOMS.ACT.S.12".to_string(),
                        title: "Declaration requirements for imported goods".to_string(),
                        content: "All goods imported into Canada must be declared to customs at the first point of entry".to_string(),
                        hierarchical_path: "Canada > CBSA > Customs Act > Section 12".to_string(),
                        effective_date: DateTime::parse_from_rfc3339("1986-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
                        jurisdiction: "CA".to_string(),
                        authority: "Canada Border Services Agency".to_string(),
                        regulation_type: RegulationType::NationalLaw,
                        scope: RuleScope::National,
                        sector: Some("All".to_string()),
                        tags: vec!["customs".to_string(), "import".to_string(), "declaration".to_string()],
                        certainty_level: CertaintyLevel::High,
                        interpretations: HashMap::new(),
                        related_rules: vec![],
                        penalties: vec![],
                        implementation_guidance: "Complete customs declaration forms accurately".to_string(),
                        exceptions: vec!["Goods in transit".to_string()],
                        last_updated: Utc::now(),
                    },
                ],
            },
        ]
    }
}

impl USCustomsRegulations {
    pub fn new() -> Self {
        Self {
            customs_modernization_act: CustomsModernizationAct::new(),
            tariff_act_1930: TariffAct1930::new(),
            trade_act_1974: TradeAct1974::new(),
            customs_trade_partnership_against_terrorism: CTPAT::new(),
            automated_commercial_environment: ACE::new(),
            customs_broker_licensing: CustomsBrokerLicensing::new(),
        }
    }

    pub fn get_applicable_rules(&self, trade_type: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        rules.extend(self.tariff_act_1930.get_applicable_rules(trade_type));
        rules.extend(self.customs_modernization_act.get_applicable_rules());

        if trade_type == "import" {
            rules.extend(self.automated_commercial_environment.get_import_rules());
        }

        rules
    }
}

impl TariffAct1930 {
    pub fn new() -> Self {
        Self {
            sections: Self::create_sections(),
        }
    }

    fn create_sections() -> Vec<AtomicLegalRule> {
        vec![
            AtomicLegalRule {
                id: "US.CBP.TARIFF.ACT.1930.S.484".to_string(),
                title: "Entry of merchandise".to_string(),
                content: "Merchandise may be entered for consumption or warehouse at the port of entry within 15 days after arrival".to_string(),
                hierarchical_path: "US > CBP > Tariff Act 1930 > Section 484".to_string(),
                effective_date: DateTime::parse_from_rfc3339("1930-06-17T00:00:00Z").unwrap().with_timezone(&Utc),
                jurisdiction: "US".to_string(),
                authority: "U.S. Customs and Border Protection".to_string(),
                regulation_type: RegulationType::FederalLaw,
                scope: RuleScope::National,
                sector: Some("All".to_string()),
                tags: vec!["customs".to_string(), "entry".to_string(), "import".to_string()],
                certainty_level: CertaintyLevel::High,
                interpretations: HashMap::new(),
                related_rules: vec![],
                penalties: vec![],
                implementation_guidance: "File entry documents within required timeframe".to_string(),
                exceptions: vec!["Emergency situations".to_string()],
                last_updated: Utc::now(),
            },
            AtomicLegalRule {
                id: "US.CBP.TARIFF.ACT.1930.S.592".to_string(),
                title: "Penalties for fraud, gross negligence, and negligence".to_string(),
                content: "Persons who enter, introduce, or attempt to introduce merchandise by means of fraud are subject to penalties".to_string(),
                hierarchical_path: "US > CBP > Tariff Act 1930 > Section 592".to_string(),
                effective_date: DateTime::parse_from_rfc3339("1930-06-17T00:00:00Z").unwrap().with_timezone(&Utc),
                jurisdiction: "US".to_string(),
                authority: "U.S. Customs and Border Protection".to_string(),
                regulation_type: RegulationType::FederalLaw,
                scope: RuleScope::National,
                sector: Some("All".to_string()),
                tags: vec!["customs".to_string(), "penalties".to_string(), "fraud".to_string()],
                certainty_level: CertaintyLevel::High,
                interpretations: HashMap::new(),
                related_rules: vec![],
                penalties: vec![],
                implementation_guidance: "Ensure accuracy and truthfulness in customs declarations".to_string(),
                exceptions: vec![],
                last_updated: Utc::now(),
            },
        ]
    }

    pub fn get_applicable_rules(&self, _trade_type: &str) -> Vec<AtomicLegalRule> {
        self.sections.clone()
    }
}

impl ExportControlRegulations {
    pub fn new() -> Self {
        Self {
            us_export_controls: USExportControls::new(),
            eu_dual_use_regulation: EUDualUseRegulation::new(),
            multilateral_export_control_regimes: MultilateralExportControlRegimes::new(),
            strategic_trade_controls: StrategicTradeControls::new(),
            technology_transfer_controls: TechnologyTransferControls::new(),
        }
    }
}

impl USExportControls {
    pub fn new() -> Self {
        Self {
            export_administration_regulations: ExportAdministrationRegulations::new(),
            international_traffic_arms_regulations: ITAR::new(),
            office_foreign_assets_control: OFAC::new(),
            foreign_direct_product_rule: ForeignDirectProductRule::new(),
            deemed_export_controls: DeemedExportControls::new(),
        }
    }

    pub fn get_applicable_rules(&self, product_category: &str, destination: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        if product_category.contains("defense") || product_category.contains("military") {
            rules.extend(self.international_traffic_arms_regulations.get_applicable_rules(destination));
        } else {
            rules.extend(self.export_administration_regulations.get_applicable_rules(product_category, destination));
        }

        rules.extend(self.office_foreign_assets_control.get_applicable_rules(destination));

        rules
    }
}

impl ExportAdministrationRegulations {
    pub fn new() -> Self {
        Self {
            commerce_control_list: CommerceControlList::new(),
            country_chart: CountryChart::new(),
            license_requirements: Self::create_license_requirements(),
            license_exceptions: Self::create_license_exceptions(),
            end_user_controls: Self::create_end_user_controls(),
        }
    }

    fn create_license_requirements() -> Vec<AtomicLegalRule> {
        vec![
            AtomicLegalRule {
                id: "US.BIS.EAR.736.2.A.1".to_string(),
                title: "General prohibition on exports to embargoed destinations".to_string(),
                content: "You may not export, reexport, or transfer items subject to the EAR to embargoed destinations without authorization".to_string(),
                hierarchical_path: "US > BIS > EAR > 736.2(a)(1)".to_string(),
                effective_date: DateTime::parse_from_rfc3339("2018-08-13T00:00:00Z").unwrap().with_timezone(&Utc),
                jurisdiction: "US".to_string(),
                authority: "Bureau of Industry and Security".to_string(),
                regulation_type: RegulationType::FederalRegulation,
                scope: RuleScope::Extraterritorial,
                sector: Some("All".to_string()),
                tags: vec!["export-control".to_string(), "embargo".to_string(), "licensing".to_string()],
                certainty_level: CertaintyLevel::High,
                interpretations: HashMap::new(),
                related_rules: vec![],
                penalties: vec![],
                implementation_guidance: "Check Country Chart for licensing requirements by destination".to_string(),
                exceptions: vec!["Licensed transactions".to_string(), "License exceptions".to_string()],
                last_updated: Utc::now(),
            },
        ]
    }

    fn create_license_exceptions() -> Vec<AtomicLegalRule> {
        vec![
            AtomicLegalRule {
                id: "US.BIS.EAR.740.17".to_string(),
                title: "License Exception ENC for encryption commodities, software, and technology".to_string(),
                content: "Certain encryption items may be exported under License Exception ENC without individual licenses".to_string(),
                hierarchical_path: "US > BIS > EAR > 740.17".to_string(),
                effective_date: DateTime::parse_from_rfc3339("2016-01-20T00:00:00Z").unwrap().with_timezone(&Utc),
                jurisdiction: "US".to_string(),
                authority: "Bureau of Industry and Security".to_string(),
                regulation_type: RegulationType::FederalRegulation,
                scope: RuleScope::Extraterritorial,
                sector: Some("Technology".to_string()),
                tags: vec!["export-control".to_string(), "encryption".to_string(), "license-exception".to_string()],
                certainty_level: CertaintyLevel::Medium,
                interpretations: HashMap::new(),
                related_rules: vec![],
                penalties: vec![],
                implementation_guidance: "Review encryption review requirements and notification procedures".to_string(),
                exceptions: vec![],
                last_updated: Utc::now(),
            },
        ]
    }

    fn create_end_user_controls() -> Vec<AtomicLegalRule> {
        vec![
            AtomicLegalRule {
                id: "US.BIS.EAR.744.11".to_string(),
                title: "Entity List restrictions".to_string(),
                content: "License requirements apply to exports, reexports, and transfers to entities on the Entity List".to_string(),
                hierarchical_path: "US > BIS > EAR > 744.11".to_string(),
                effective_date: DateTime::parse_from_rfc3339("1997-02-12T00:00:00Z").unwrap().with_timezone(&Utc),
                jurisdiction: "US".to_string(),
                authority: "Bureau of Industry and Security".to_string(),
                regulation_type: RegulationType::FederalRegulation,
                scope: RuleScope::Extraterritorial,
                sector: Some("All".to_string()),
                tags: vec!["export-control".to_string(), "entity-list".to_string(), "end-user".to_string()],
                certainty_level: CertaintyLevel::High,
                interpretations: HashMap::new(),
                related_rules: vec![],
                penalties: vec![],
                implementation_guidance: "Screen against Entity List before export transactions".to_string(),
                exceptions: vec![],
                last_updated: Utc::now(),
            },
        ]
    }

    pub fn get_applicable_rules(&self, product_category: &str, destination: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        rules.extend(self.license_requirements.clone());
        rules.extend(self.end_user_controls.clone());

        if product_category.contains("encryption") || product_category.contains("cryptography") {
            rules.extend(self.license_exceptions.clone());
        }

        rules
    }
}

impl SanctionsEmbargoes {
    pub fn new() -> Self {
        Self {
            us_sanctions: USSanctions::new(),
            eu_sanctions: EUSanctions::new(),
            un_sanctions: UNSanctions::new(),
            uk_sanctions: UKSanctions::new(),
            national_sanctions: Self::create_national_sanctions(),
            sectoral_sanctions: Self::create_sectoral_sanctions(),
        }
    }

    fn create_national_sanctions() -> Vec<NationalSanctionsRegime> {
        vec![
            NationalSanctionsRegime {
                jurisdiction: "CA".to_string(),
                regime_name: "Special Economic Measures Act".to_string(),
                target_countries: vec!["Various".to_string()],
                measures: vec!["Asset freezes".to_string(), "Import/export prohibitions".to_string()],
            },
        ]
    }

    fn create_sectoral_sanctions() -> Vec<SectoralSanctions> {
        vec![
            SectoralSanctions {
                sector: "Financial Services".to_string(),
                description: "Restrictions on access to US financial system".to_string(),
                jurisdictions: vec!["US".to_string()],
                measures: vec!["Prohibition on US persons providing funds".to_string()],
            },
        ]
    }
}

impl USSanctions {
    pub fn new() -> Self {
        Self {
            ofac_sanctions_programs: Self::create_ofac_programs(),
            specially_designated_nationals: SpeciallyDesignatedNationals::new(),
            sectoral_sanctions_identifications: SectoralSanctionsIdentifications::new(),
            foreign_sanctions_evaders: ForeignSanctionsEvaders::new(),
            non_sdn_menu_based_sanctions: NonSDNMenuBasedSanctions::new(),
        }
    }

    fn create_ofac_programs() -> Vec<OFACSanctionsProgram> {
        vec![
            OFACSanctionsProgram {
                program_name: "Iran Sanctions".to_string(),
                target: "Iran".to_string(),
                measures: vec![
                    AtomicLegalRule {
                        id: "US.OFAC.IRAN.31CFR.560.204".to_string(),
                        title: "Prohibition on transactions with Iran".to_string(),
                        content: "Except as authorized, no US person may engage in any transaction with Iran or Iranian persons".to_string(),
                        hierarchical_path: "US > OFAC > Iran Sanctions > 31 CFR 560.204".to_string(),
                        effective_date: DateTime::parse_from_rfc3339("1995-05-06T00:00:00Z").unwrap().with_timezone(&Utc),
                        jurisdiction: "US".to_string(),
                        authority: "Office of Foreign Assets Control".to_string(),
                        regulation_type: RegulationType::FederalRegulation,
                        scope: RuleScope::Extraterritorial,
                        sector: Some("All".to_string()),
                        tags: vec!["sanctions".to_string(), "Iran".to_string(), "prohibition".to_string()],
                        certainty_level: CertaintyLevel::High,
                        interpretations: HashMap::new(),
                        related_rules: vec![],
                        penalties: vec![],
                        implementation_guidance: "Screen all transactions against OFAC sanctions lists".to_string(),
                        exceptions: vec!["Licensed transactions".to_string(), "Humanitarian trade".to_string()],
                        last_updated: Utc::now(),
                    },
                ],
            },
        ]
    }

    pub fn get_applicable_rules(&self, target_country: &str, product_sector: &str) -> Vec<AtomicLegalRule> {
        let mut rules = Vec::new();

        for program in &self.ofac_sanctions_programs {
            if program.target.to_lowercase().contains(&target_country.to_lowercase()) {
                rules.extend(program.measures.clone());
            }
        }

        rules
    }
}

impl TradeAgreements {
    pub fn new() -> Self {
        Self {
            multilateral_agreements: MultilateralAgreements::new(),
            bilateral_agreements: Self::create_bilateral_agreements(),
            regional_trade_agreements: Self::create_regional_agreements(),
            preferential_trade_agreements: Self::create_preferential_agreements(),
        }
    }

    fn create_bilateral_agreements() -> Vec<BilateralAgreement> {
        vec![
            BilateralAgreement {
                name: "United States-Mexico-Canada Agreement".to_string(),
                country1: "US".to_string(),
                country2: "CA".to_string(),
                effective_date: DateTime::parse_from_rfc3339("2020-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
                provisions: vec![
                    AtomicLegalRule {
                        id: "USMCA.ART.2.1".to_string(),
                        title: "Elimination of customs duties".to_string(),
                        content: "Each Party shall eliminate its customs duties on originating goods of the other Parties".to_string(),
                        hierarchical_path: "USMCA > Chapter 2 > Article 2.1".to_string(),
                        effective_date: DateTime::parse_from_rfc3339("2020-07-01T00:00:00Z").unwrap().with_timezone(&Utc),
                        jurisdiction: "USMCA".to_string(),
                        authority: "USMCA Parties".to_string(),
                        regulation_type: RegulationType::TradeAgreement,
                        scope: RuleScope::Regional,
                        sector: Some("All".to_string()),
                        tags: vec!["trade-agreement".to_string(), "customs-duties".to_string(), "USMCA".to_string()],
                        certainty_level: CertaintyLevel::High,
                        interpretations: HashMap::new(),
                        related_rules: vec![],
                        penalties: vec![],
                        implementation_guidance: "Apply USMCA tariff schedules for originating goods".to_string(),
                        exceptions: vec![],
                        last_updated: Utc::now(),
                    },
                ],
            },
        ]
    }

    fn create_regional_agreements() -> Vec<RegionalTradeAgreement> {
        vec![
            RegionalTradeAgreement {
                name: "Comprehensive and Progressive Trans-Pacific Partnership".to_string(),
                member_countries: vec!["AU".to_string(), "CA".to_string(), "CL".to_string(), "JP".to_string(), "MY".to_string(), "MX".to_string(), "NZ".to_string(), "PE".to_string(), "SG".to_string(), "VN".to_string()],
                effective_date: DateTime::parse_from_rfc3339("2018-12-30T00:00:00Z").unwrap().with_timezone(&Utc),
                provisions: vec![
                    AtomicLegalRule {
                        id: "CPTPP.ART.2.4".to_string(),
                        title: "Tariff elimination".to_string(),
                        content: "Each Party shall eliminate its customs duties on originating goods of another Party".to_string(),
                        hierarchical_path: "CPTPP > Chapter 2 > Article 2.4".to_string(),
                        effective_date: DateTime::parse_from_rfc3339("2018-12-30T00:00:00Z").unwrap().with_timezone(&Utc),
                        jurisdiction: "CPTPP".to_string(),
                        authority: "CPTPP Parties".to_string(),
                        regulation_type: RegulationType::TradeAgreement,
                        scope: RuleScope::Regional,
                        sector: Some("All".to_string()),
                        tags: vec!["trade-agreement".to_string(), "tariff-elimination".to_string(), "CPTPP".to_string()],
                        certainty_level: CertaintyLevel::High,
                        interpretations: HashMap::new(),
                        related_rules: vec![],
                        penalties: vec![],
                        implementation_guidance: "Apply CPTPP preferential tariffs for originating goods".to_string(),
                        exceptions: vec![],
                        last_updated: Utc::now(),
                    },
                ],
            },
        ]
    }

    fn create_preferential_agreements() -> Vec<PreferentialTradeAgreement> {
        vec![
            PreferentialTradeAgreement {
                name: "Generalized System of Preferences".to_string(),
                beneficiary_countries: vec!["Various developing countries".to_string()],
                granting_countries: vec!["US".to_string(), "EU".to_string(), "CA".to_string()],
                product_coverage: vec!["Manufactured goods".to_string(), "Agricultural products".to_string()],
                preferences: vec!["Duty-free treatment".to_string(), "Reduced tariffs".to_string()],
            },
        ]
    }
}

impl MultilateralAgreements {
    pub fn new() -> Self {
        Self {
            wto_agreements: WTOAgreements::new(),
            gatt_1994: GATT1994::new(),
            general_agreement_trade_services: GATS::new(),
            agreement_trade_related_aspects_intellectual_property: TRIPS::new(),
            technical_barriers_to_trade: TBTAgreement::new(),
            sanitary_phytosanitary_measures: SPSAgreement::new(),
        }
    }
}

impl WTOAgreements {
    pub fn new() -> Self {
        Self {
            dispute_settlement_understanding: DisputeSettlementUnderstanding::new(),
            trade_policy_review_mechanism: TradePolicyReviewMechanism::new(),
            plurilateral_agreements: Self::create_plurilateral_agreements(),
            trade_facilitation_agreement: TradeFacilitationAgreement::new(),
        }
    }

    fn create_plurilateral_agreements() -> Vec<PlurilateralAgreement> {
        vec![
            PlurilateralAgreement {
                name: "Agreement on Government Procurement".to_string(),
                parties: vec!["Various WTO Members".to_string()],
                coverage: "Government procurement".to_string(),
            },
        ]
    }

    pub fn get_applicable_rules(&self, _trade_type: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

// Placeholder types and implementations for compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WTOCustomsFrameworks {
    pub frameworks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUCustomsRegulations {
    pub union_customs_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UKCustomsRegulations {
    pub customs_handling_import_export_freight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalCustomsCode {
    pub jurisdiction: String,
    pub code_name: String,
    pub rules: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsProcedures {
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsModernizationAct {
    pub provisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffAct1930 {
    pub sections: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAct1974 {
    pub sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTPAT {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACE {
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsBrokerLicensing {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilateralAgreement {
    pub name: String,
    pub country1: String,
    pub country2: String,
    pub effective_date: DateTime<Utc>,
    pub provisions: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTradeAgreement {
    pub name: String,
    pub member_countries: Vec<String>,
    pub effective_date: DateTime<Utc>,
    pub provisions: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferentialTradeAgreement {
    pub name: String,
    pub beneficiary_countries: Vec<String>,
    pub granting_countries: Vec<String>,
    pub product_coverage: Vec<String>,
    pub preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GATT1994 {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GATS {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TRIPS {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TBTAgreement {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPSAgreement {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeSettlementUnderstanding {
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePolicyReviewMechanism {
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlurilateralAgreement {
    pub name: String,
    pub parties: Vec<String>,
    pub coverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFacilitationAgreement {
    pub articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUDualUseRegulation {
    pub regulation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilateralExportControlRegimes {
    pub regimes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicTradeControls {
    pub controls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyTransferControls {
    pub controls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITAR {
    pub regulations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OFAC {
    pub programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignDirectProductRule {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeemedExportControls {
    pub controls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceControlList {
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryChart {
    pub countries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffClassifications {
    pub classifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsDuties {
    pub duties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTariffBarriers {
    pub barriers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportLicensing {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSafetyStandards {
    pub standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhytosanitaryRequirements {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUSanctions {
    pub regulations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UNSanctions {
    pub resolutions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UKSanctions {
    pub regulations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalSanctionsRegime {
    pub jurisdiction: String,
    pub regime_name: String,
    pub target_countries: Vec<String>,
    pub measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectoralSanctions {
    pub sector: String,
    pub description: String,
    pub jurisdictions: Vec<String>,
    pub measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OFACSanctionsProgram {
    pub program_name: String,
    pub target: String,
    pub measures: Vec<AtomicLegalRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciallyDesignatedNationals {
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectoralSanctionsIdentifications {
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignSanctionsEvaders {
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonSDNMenuBasedSanctions {
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntidumpingMeasures {
    pub measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountervailingDuties {
    pub duties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeguardMeasures {
    pub measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAdjustmentAssistance {
    pub programs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonPreferentialOrigin {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferentialOrigin {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstantialTransformation {
    pub criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueAddedRequirements {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginMarkingRequirements {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeTradeZones {
    pub zones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFacilitation {
    pub measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsValuation {
    pub methods: Vec<String>,
}

// Implement all placeholder methods
impl WTOCustomsFrameworks {
    pub fn new() -> Self {
        Self {
            frameworks: vec!["Kyoto Convention".to_string(), "WTO Trade Facilitation Agreement".to_string()],
        }
    }

    pub fn get_applicable_rules(&self) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl EUCustomsRegulations {
    pub fn new() -> Self {
        Self {
            union_customs_code: "Regulation (EU) No 952/2013".to_string(),
        }
    }

    pub fn get_applicable_rules(&self, _trade_type: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl UKCustomsRegulations {
    pub fn new() -> Self {
        Self {
            customs_handling_import_export_freight: "CHIEF".to_string(),
        }
    }

    pub fn get_applicable_rules(&self, _trade_type: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl CustomsProcedures {
    pub fn new() -> Self {
        Self {
            procedures: vec!["Entry procedures".to_string(), "Clearance procedures".to_string()],
        }
    }
}

impl CustomsModernizationAct {
    pub fn new() -> Self {
        Self {
            provisions: vec!["Informed compliance".to_string(), "Shared responsibility".to_string()],
        }
    }

    pub fn get_applicable_rules(&self) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl TradeAct1974 {
    pub fn new() -> Self {
        Self {
            sections: vec!["Section 301".to_string(), "GSP".to_string()],
        }
    }
}

impl CTPAT {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Supply chain security".to_string()],
        }
    }
}

impl ACE {
    pub fn new() -> Self {
        Self {
            modules: vec!["Entry/Entry Summary".to_string(), "Partner Government Agencies".to_string()],
        }
    }

    pub fn get_import_rules(&self) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl CustomsBrokerLicensing {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Customs broker license".to_string()],
        }
    }
}

impl GATT1994 {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article I - Most Favoured Nation".to_string()],
        }
    }
}

impl GATS {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article II - Most Favoured Nation".to_string()],
        }
    }
}

impl TRIPS {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article 3 - National Treatment".to_string()],
        }
    }
}

impl TBTAgreement {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article 2 - Technical Regulations".to_string()],
        }
    }
}

impl SPSAgreement {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article 2 - Basic Rights and Obligations".to_string()],
        }
    }
}

impl DisputeSettlementUnderstanding {
    pub fn new() -> Self {
        Self {
            procedures: vec!["Panel procedures".to_string(), "Appellate Body".to_string()],
        }
    }
}

impl TradePolicyReviewMechanism {
    pub fn new() -> Self {
        Self {
            procedures: vec!["Regular reviews".to_string()],
        }
    }
}

impl TradeFacilitationAgreement {
    pub fn new() -> Self {
        Self {
            articles: vec!["Article 1 - Publication and Availability of Information".to_string()],
        }
    }
}

impl EUDualUseRegulation {
    pub fn new() -> Self {
        Self {
            regulation: "Regulation (EU) 2021/821".to_string(),
        }
    }

    pub fn get_applicable_rules(&self, _product_category: &str, _destination: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl MultilateralExportControlRegimes {
    pub fn new() -> Self {
        Self {
            regimes: vec!["Wassenaar Arrangement".to_string(), "Australia Group".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl StrategicTradeControls {
    pub fn new() -> Self {
        Self {
            controls: vec!["Strategic goods".to_string()],
        }
    }
}

impl TechnologyTransferControls {
    pub fn new() -> Self {
        Self {
            controls: vec!["Technology transfer".to_string()],
        }
    }
}

impl ITAR {
    pub fn new() -> Self {
        Self {
            regulations: vec!["22 CFR 120-130".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _destination: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl OFAC {
    pub fn new() -> Self {
        Self {
            programs: vec!["Sanctions programs".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _destination: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl ForeignDirectProductRule {
    pub fn new() -> Self {
        Self {
            rules: vec!["FDPR".to_string()],
        }
    }
}

impl DeemedExportControls {
    pub fn new() -> Self {
        Self {
            controls: vec!["Deemed exports".to_string()],
        }
    }
}

impl CommerceControlList {
    pub fn new() -> Self {
        Self {
            categories: vec!["Category 0-9".to_string()],
        }
    }
}

impl CountryChart {
    pub fn new() -> Self {
        Self {
            countries: vec!["Country Groups A-E".to_string()],
        }
    }
}

impl ImportRegulations {
    pub fn new() -> Self {
        Self {
            tariff_classifications: TariffClassifications::new(),
            customs_duties: CustomsDuties::new(),
            non_tariff_barriers: NonTariffBarriers::new(),
            import_licensing: ImportLicensing::new(),
            product_safety_standards: ProductSafetyStandards::new(),
            phytosanitary_requirements: PhytosanitaryRequirements::new(),
        }
    }
}

impl TariffClassifications {
    pub fn new() -> Self {
        Self {
            classifications: vec!["Harmonized System".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl CustomsDuties {
    pub fn new() -> Self {
        Self {
            duties: vec!["Ad valorem".to_string(), "Specific".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str, _origin_country: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl NonTariffBarriers {
    pub fn new() -> Self {
        Self {
            barriers: vec!["Quotas".to_string(), "Licenses".to_string()],
        }
    }
}

impl ImportLicensing {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Import licenses".to_string()],
        }
    }
}

impl ProductSafetyStandards {
    pub fn new() -> Self {
        Self {
            standards: vec!["Product standards".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl PhytosanitaryRequirements {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Plant health".to_string(), "Animal health".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _origin_country: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl EUSanctions {
    pub fn new() -> Self {
        Self {
            regulations: vec!["Council Regulations".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _target_country: &str, _product_sector: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl UNSanctions {
    pub fn new() -> Self {
        Self {
            resolutions: vec!["Security Council Resolutions".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _target_country: &str, _product_sector: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl UKSanctions {
    pub fn new() -> Self {
        Self {
            regulations: vec!["The Sanctions and Anti-Money Laundering Act 2018".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _target_country: &str, _product_sector: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl SpeciallyDesignatedNationals {
    pub fn new() -> Self {
        Self {
            list: vec!["SDN List".to_string()],
        }
    }
}

impl SectoralSanctionsIdentifications {
    pub fn new() -> Self {
        Self {
            list: vec!["SSI List".to_string()],
        }
    }
}

impl ForeignSanctionsEvaders {
    pub fn new() -> Self {
        Self {
            list: vec!["FSE List".to_string()],
        }
    }
}

impl NonSDNMenuBasedSanctions {
    pub fn new() -> Self {
        Self {
            list: vec!["NS-MBS List".to_string()],
        }
    }
}

impl TradeRemedies {
    pub fn new() -> Self {
        Self {
            antidumping_measures: AntidumpingMeasures::new(),
            countervailing_duties: CountervailingDuties::new(),
            safeguard_measures: SafeguardMeasures::new(),
            trade_adjustment_assistance: TradeAdjustmentAssistance::new(),
        }
    }
}

impl AntidumpingMeasures {
    pub fn new() -> Self {
        Self {
            measures: vec!["Antidumping duties".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl CountervailingDuties {
    pub fn new() -> Self {
        Self {
            duties: vec!["Countervailing duties".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl SafeguardMeasures {
    pub fn new() -> Self {
        Self {
            measures: vec!["Safeguard measures".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl TradeAdjustmentAssistance {
    pub fn new() -> Self {
        Self {
            programs: vec!["TAA programs".to_string()],
        }
    }
}

impl OriginRules {
    pub fn new() -> Self {
        Self {
            non_preferential_origin: NonPreferentialOrigin::new(),
            preferential_origin: PreferentialOrigin::new(),
            substantial_transformation: SubstantialTransformation::new(),
            value_added_requirements: ValueAddedRequirements::new(),
            origin_marking_requirements: OriginMarkingRequirements::new(),
        }
    }
}

impl NonPreferentialOrigin {
    pub fn new() -> Self {
        Self {
            rules: vec!["Country of origin".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str, _product_category: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl PreferentialOrigin {
    pub fn new() -> Self {
        Self {
            rules: vec!["Preferential origin".to_string()],
        }
    }
}

impl SubstantialTransformation {
    pub fn new() -> Self {
        Self {
            criteria: vec!["Change in tariff classification".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _manufacturing_process: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl ValueAddedRequirements {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Value content".to_string()],
        }
    }
}

impl OriginMarkingRequirements {
    pub fn new() -> Self {
        Self {
            requirements: vec!["Country of origin marking".to_string()],
        }
    }

    pub fn get_applicable_rules(&self, _jurisdiction: &str) -> Vec<AtomicLegalRule> {
        vec![]
    }
}

impl FreeTradeZones {
    pub fn new() -> Self {
        Self {
            zones: vec!["Foreign Trade Zones".to_string()],
        }
    }
}

impl TradeFacilitation {
    pub fn new() -> Self {
        Self {
            measures: vec!["Single window".to_string(), "Risk management".to_string()],
        }
    }
}

impl CustomsValuation {
    pub fn new() -> Self {
        Self {
            methods: vec!["Transaction value".to_string(), "Deductive value".to_string()],
        }
    }
}