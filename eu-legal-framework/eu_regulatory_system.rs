use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUTreaty {
    pub treaty_name: String,
    pub treaty_type: String,
    pub date_signed: String,
    pub date_effective: String,
    pub articles: Vec<EUTreatyArticle>,
    pub protocols: Vec<EUProtocol>,
    pub member_states_signatories: Vec<String>,
    pub legal_basis: String,
    pub amendment_procedures: String,
    pub primacy_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUTreatyArticle {
    pub article_number: String,
    pub article_title: String,
    pub article_text: String,
    pub legal_effect: String,
    pub competence_type: String,
    pub implementation_requirements: Vec<String>,
    pub member_state_obligations: Vec<String>,
    pub eu_institution_powers: Vec<String>,
    pub jurisprudence_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUProtocol {
    pub protocol_number: i32,
    pub protocol_title: String,
    pub protocol_text: String,
    pub attached_treaty: String,
    pub legal_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUDirective {
    pub directive_number: String,
    pub directive_title: String,
    pub directive_text: String,
    pub legal_basis: String,
    pub date_adopted: String,
    pub transposition_deadline: String,
    pub member_states_addressed: Vec<String>,
    pub objectives: Vec<String>,
    pub minimum_requirements: Vec<String>,
    pub implementation_measures: Vec<String>,
    pub compliance_monitoring: String,
    pub infringement_procedures: Vec<String>,
    pub sector: String,
    pub cjeu_interpretations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EURegulation {
    pub regulation_number: String,
    pub regulation_title: String,
    pub regulation_text: String,
    pub legal_basis: String,
    pub date_adopted: String,
    pub date_effective: String,
    pub directly_applicable: bool,
    pub binding_articles: Vec<EURegulationArticle>,
    pub member_state_obligations: Vec<String>,
    pub enforcement_mechanisms: Vec<String>,
    pub sector: String,
    pub supremacy_clause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EURegulationArticle {
    pub article_number: String,
    pub article_title: String,
    pub article_text: String,
    pub direct_effect: bool,
    pub immediate_applicability: bool,
    pub enforcement_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUDecision {
    pub decision_number: String,
    pub decision_title: String,
    pub decision_text: String,
    pub addressees: Vec<String>,
    pub date_adopted: String,
    pub legal_basis: String,
    pub binding_nature: String,
    pub implementation_requirements: Vec<String>,
    pub compliance_deadline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CJEUJudgment {
    pub case_number: String,
    pub case_name: String,
    pub date_judgment: String,
    pub court_formation: String,
    pub legal_questions: Vec<String>,
    pub judgment_text: String,
    pub operative_part: String,
    pub legal_principles: Vec<String>,
    pub precedent_value: String,
    pub member_states_affected: Vec<String>,
    pub eu_law_interpretation: Vec<String>,
    pub preliminary_ruling: bool,
    pub infringement_case: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUInstitution {
    pub institution_name: String,
    pub institution_type: String,
    pub legal_basis: String,
    pub composition: Vec<String>,
    pub powers: Vec<String>,
    pub decision_making_procedures: Vec<String>,
    pub legislative_role: String,
    pub executive_role: String,
    pub judicial_role: String,
    pub accountability_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUCompetence {
    pub competence_area: String,
    pub competence_type: String,
    pub exclusive_competence: bool,
    pub shared_competence: bool,
    pub supporting_competence: bool,
    pub legal_basis: String,
    pub eu_powers: Vec<String>,
    pub member_state_powers: Vec<String>,
    pub subsidiarity_principle: String,
    pub proportionality_principle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUMemberState {
    pub country_name: String,
    pub accession_date: String,
    pub accession_treaty: String,
    pub derogations: Vec<String>,
    pub opt_outs: Vec<String>,
    pub eurozone_member: bool,
    pub schengen_member: bool,
    pub voting_weight: i32,
    pub national_implementation_record: Vec<String>,
    pub infringement_cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EULegalSystem {
    pub founding_treaties: Vec<EUTreaty>,
    pub secondary_legislation: EUSecondaryLegislation,
    pub court_system: EUCourtSystem,
    pub institutional_framework: Vec<EUInstitution>,
    pub competence_framework: Vec<EUCompetence>,
    pub member_states: Vec<EUMemberState>,
    pub legal_principles: Vec<String>,
    pub legislative_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUSecondaryLegislation {
    pub regulations: Vec<EURegulation>,
    pub directives: Vec<EUDirective>,
    pub decisions: Vec<EUDecision>,
    pub recommendations: Vec<String>,
    pub opinions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUCourtSystem {
    pub court_of_justice: CourtOfJustice,
    pub general_court: GeneralCourt,
    pub specialized_courts: Vec<String>,
    pub national_courts_integration: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtOfJustice {
    pub composition: String,
    pub jurisdiction: Vec<String>,
    pub preliminary_rulings: Vec<CJEUJudgment>,
    pub infringement_proceedings: Vec<CJEUJudgment>,
    pub direct_actions: Vec<CJEUJudgment>,
    pub appeals: Vec<CJEUJudgment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralCourt {
    pub composition: String,
    pub jurisdiction: Vec<String>,
    pub first_instance_cases: Vec<CJEUJudgment>,
    pub competition_cases: Vec<CJEUJudgment>,
    pub state_aid_cases: Vec<CJEUJudgment>,
}

impl EULegalSystem {
    pub fn new() -> Self {
        let founding_treaties = vec![
            EUTreaty {
                treaty_name: "Treaty on European Union (TEU)".to_string(),
                treaty_type: "Founding Treaty".to_string(),
                date_signed: "1992-02-07".to_string(),
                date_effective: "1993-11-01".to_string(),
                articles: vec![
                    EUTreatyArticle {
                        article_number: "1".to_string(),
                        article_title: "Establishment of the Union".to_string(),
                        article_text: "By this Treaty, the HIGH CONTRACTING PARTIES establish among themselves a EUROPEAN UNION, hereinafter called 'the Union', on which the Member States confer competences to attain objectives they have in common. This Treaty marks a new stage in the process of creating an ever closer union among the peoples of Europe, in which decisions are taken as openly as possible and as closely as possible to the citizen.".to_string(),
                        legal_effect: "Constitutive".to_string(),
                        competence_type: "Foundational".to_string(),
                        implementation_requirements: vec!["Constitutional ratification by all Member States".to_string()],
                        member_state_obligations: vec!["Sincere cooperation principle".to_string(), "Loyalty clause".to_string()],
                        eu_institution_powers: vec!["Institutional framework establishment".to_string()],
                        jurisprudence_references: vec!["Van Gend en Loos".to_string(), "Costa v ENEL".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "2".to_string(),
                        article_title: "Union Values".to_string(),
                        article_text: "The Union is founded on the values of respect for human dignity, freedom, democracy, equality, the rule of law and respect for human rights, including the rights of persons belonging to minorities. These values are common to the Member States in a society in which pluralism, non-discrimination, tolerance, justice, solidarity and equality between women and men prevail.".to_string(),
                        legal_effect: "Foundational".to_string(),
                        competence_type: "Constitutional".to_string(),
                        implementation_requirements: vec!["Article 7 procedures for values violation".to_string()],
                        member_state_obligations: vec!["Uphold EU values".to_string(), "Democratic governance".to_string()],
                        eu_institution_powers: vec!["Article 7 sanctions procedures".to_string()],
                        jurisprudence_references: vec!["Commission v Poland (Rule of Law)".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "3".to_string(),
                        article_title: "Union Objectives".to_string(),
                        article_text: "1. The Union's aim is to promote peace, its values and the well-being of its peoples. 2. The Union shall offer its citizens an area of freedom, security and justice without internal frontiers, in which the free movement of persons is ensured in conjunction with appropriate measures with respect to external border controls, asylum, immigration and the prevention and combating of crime.".to_string(),
                        legal_effect: "Programmatic".to_string(),
                        competence_type: "Objective-setting".to_string(),
                        implementation_requirements: vec!["AFSJ development".to_string(), "Common policies".to_string()],
                        member_state_obligations: vec!["Policy coordination".to_string(), "Internal market development".to_string()],
                        eu_institution_powers: vec!["Legislative competence in AFSJ".to_string()],
                        jurisprudence_references: vec!["Kadi cases".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "4".to_string(),
                        article_title: "Competence Framework".to_string(),
                        article_text: "1. In accordance with Article 5, competences not conferred upon the Union in the Treaties remain with the Member States. 2. The Union shall respect the equality of Member States before the Treaties as well as their national identities, inherent in their fundamental structures, political and constitutional, inclusive of regional and local self-government.".to_string(),
                        legal_effect: "Competence-delimiting".to_string(),
                        competence_type: "Foundational".to_string(),
                        implementation_requirements: vec!["Competence catalogues".to_string(), "National identity respect".to_string()],
                        member_state_obligations: vec!["Residual competence exercise".to_string()],
                        eu_institution_powers: vec!["Conferred competences only".to_string()],
                        jurisprudence_references: vec!["Lisbon Treaty cases".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "5".to_string(),
                        article_title: "Subsidiarity and Proportionality".to_string(),
                        article_text: "1. The limits of Union competences are governed by the principle of conferral. 2. Under the principle of subsidiarity, in areas which do not fall within its exclusive competence, the Union shall act only if and in so far as the objectives of the proposed action cannot be sufficiently achieved by the Member States. 3. Under the principle of proportionality, the content and form of Union action shall not exceed what is necessary to achieve the objectives of the Treaties.".to_string(),
                        legal_effect: "Competence-limiting".to_string(),
                        competence_type: "Procedural".to_string(),
                        implementation_requirements: vec!["Subsidiarity test".to_string(), "Proportionality assessment".to_string()],
                        member_state_obligations: vec!["National parliaments scrutiny".to_string()],
                        eu_institution_powers: vec!["Limited action principle".to_string()],
                        jurisprudence_references: vec!["Germany v Parliament and Council (Tobacco Advertising)".to_string()],
                    }
                ],
                protocols: vec![
                    EUProtocol {
                        protocol_number: 1,
                        protocol_title: "Role of National Parliaments".to_string(),
                        protocol_text: "National Parliaments shall contribute actively to the good functioning of the Union through monitoring subsidiarity and proportionality principles.".to_string(),
                        attached_treaty: "TEU".to_string(),
                        legal_status: "Integral part".to_string(),
                    },
                    EUProtocol {
                        protocol_number: 2,
                        protocol_title: "Application of Subsidiarity and Proportionality".to_string(),
                        protocol_text: "Yellow card and orange card procedures for national parliaments objections to legislative proposals.".to_string(),
                        attached_treaty: "TEU/TFEU".to_string(),
                        legal_status: "Integral part".to_string(),
                    }
                ],
                member_states_signatories: vec![
                    "Belgium".to_string(), "Denmark".to_string(), "Germany".to_string(), "Greece".to_string(),
                    "Spain".to_string(), "France".to_string(), "Ireland".to_string(), "Italy".to_string(),
                    "Luxembourg".to_string(), "Netherlands".to_string(), "Portugal".to_string(), "United Kingdom".to_string()
                ],
                legal_basis: "Primary Law".to_string(),
                amendment_procedures: "Article 48 TEU - Ordinary and simplified revision procedures".to_string(),
                primacy_level: 1,
            },
            EUTreaty {
                treaty_name: "Treaty on the Functioning of the European Union (TFEU)".to_string(),
                treaty_type: "Founding Treaty".to_string(),
                date_signed: "1957-03-25".to_string(),
                date_effective: "2009-12-01".to_string(),
                articles: vec![
                    EUTreatyArticle {
                        article_number: "1".to_string(),
                        article_title: "Scope of Treaty".to_string(),
                        article_text: "This Treaty organises the functioning of the Union and determines the areas of, delimitation of, and arrangements for exercising its competences.".to_string(),
                        legal_effect: "Organizational".to_string(),
                        competence_type: "Functional".to_string(),
                        implementation_requirements: vec!["Institutional procedures".to_string()],
                        member_state_obligations: vec!["Treaty compliance".to_string()],
                        eu_institution_powers: vec!["Functional competences".to_string()],
                        jurisprudence_references: vec!["ERTA case".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "18".to_string(),
                        article_title: "Non-discrimination".to_string(),
                        article_text: "Within the scope of application of the Treaties, and without prejudice to any special provisions contained therein, any discrimination on grounds of nationality shall be prohibited.".to_string(),
                        legal_effect: "Prohibitive".to_string(),
                        competence_type: "Fundamental principle".to_string(),
                        implementation_requirements: vec!["National law adaptation".to_string()],
                        member_state_obligations: vec!["Non-discrimination enforcement".to_string()],
                        eu_institution_powers: vec!["Infringement procedures".to_string()],
                        jurisprudence_references: vec!["Gravier case".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "26".to_string(),
                        article_title: "Internal Market".to_string(),
                        article_text: "1. The Union shall adopt measures with the aim of establishing or ensuring the functioning of the internal market, in accordance with the relevant provisions of the Treaties. 2. The internal market shall comprise an area without internal frontiers in which the free movement of goods, persons, services and capital is ensured in accordance with the provisions of the Treaties.".to_string(),
                        legal_effect: "Constitutive".to_string(),
                        competence_type: "Shared competence".to_string(),
                        implementation_requirements: vec!["Four freedoms implementation".to_string()],
                        member_state_obligations: vec!["Barrier removal".to_string(), "Mutual recognition".to_string()],
                        eu_institution_powers: vec!["Harmonization measures".to_string()],
                        jurisprudence_references: vec!["Cassis de Dijon".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "34".to_string(),
                        article_title: "Free Movement of Goods".to_string(),
                        article_text: "Quantitative restrictions on imports and all measures having equivalent effect shall be prohibited between Member States.".to_string(),
                        legal_effect: "Prohibitive".to_string(),
                        competence_type: "Negative integration".to_string(),
                        implementation_requirements: vec!["Direct effect".to_string()],
                        member_state_obligations: vec!["MEQR elimination".to_string()],
                        eu_institution_powers: vec!["Enforcement actions".to_string()],
                        jurisprudence_references: vec!["Dassonville formula".to_string(), "Keck judgment".to_string()],
                    },
                    EUTreatyArticle {
                        article_number: "45".to_string(),
                        article_title: "Free Movement of Workers".to_string(),
                        article_text: "1. Freedom of movement for workers shall be secured within the Union. 2. Such freedom of movement shall entail the abolition of any discrimination based on nationality between workers of the Member States as regards employment, remuneration and other conditions of work and employment.".to_string(),
                        legal_effect: "Constitutive".to_string(),
                        competence_type: "Fundamental freedom".to_string(),
                        implementation_requirements: vec!["Worker mobility rights".to_string()],
                        member_state_obligations: vec!["Equal treatment".to_string(), "Social security coordination".to_string()],
                        eu_institution_powers: vec!["Secondary legislation adoption".to_string()],
                        jurisprudence_references: vec!["Bosman case".to_string()],
                    }
                ],
                protocols: vec![
                    EUProtocol {
                        protocol_number: 3,
                        protocol_title: "Statute of the Court of Justice".to_string(),
                        protocol_text: "Organization and procedures of the Court of Justice of the European Union.".to_string(),
                        attached_treaty: "TFEU".to_string(),
                        legal_status: "Integral part".to_string(),
                    }
                ],
                member_states_signatories: vec![
                    "Belgium".to_string(), "France".to_string(), "Germany".to_string(),
                    "Italy".to_string(), "Luxembourg".to_string(), "Netherlands".to_string()
                ],
                legal_basis: "Primary Law".to_string(),
                amendment_procedures: "Article 48 TEU".to_string(),
                primacy_level: 1,
            }
        ];

        let secondary_legislation = EUSecondaryLegislation {
            regulations: vec![
                EURegulation {
                    regulation_number: "2016/679".to_string(),
                    regulation_title: "General Data Protection Regulation (GDPR)".to_string(),
                    regulation_text: "This Regulation lays down rules relating to the protection of natural persons with regard to the processing of personal data and rules relating to the free movement of such data.".to_string(),
                    legal_basis: "Article 16 TFEU".to_string(),
                    date_adopted: "2016-04-27".to_string(),
                    date_effective: "2018-05-25".to_string(),
                    directly_applicable: true,
                    binding_articles: vec![
                        EURegulationArticle {
                            article_number: "1".to_string(),
                            article_title: "Subject matter and objectives".to_string(),
                            article_text: "This Regulation lays down rules relating to the protection of natural persons with regard to the processing of personal data and rules relating to the free movement of such data.".to_string(),
                            direct_effect: true,
                            immediate_applicability: true,
                            enforcement_authority: "National Data Protection Authorities".to_string(),
                        },
                        EURegulationArticle {
                            article_number: "5".to_string(),
                            article_title: "Principles relating to processing of personal data".to_string(),
                            article_text: "Personal data shall be: (a) processed lawfully, fairly and in a transparent manner in relation to the data subject ('lawfulness, fairness and transparency')".to_string(),
                            direct_effect: true,
                            immediate_applicability: true,
                            enforcement_authority: "National Data Protection Authorities".to_string(),
                        }
                    ],
                    member_state_obligations: vec!["Establish supervisory authorities".to_string(), "Ensure enforcement".to_string()],
                    enforcement_mechanisms: vec!["Administrative fines up to 4% of turnover".to_string(), "Corrective measures".to_string()],
                    sector: "Data Protection".to_string(),
                    supremacy_clause: "Directly applicable in all Member States".to_string(),
                },
                EURegulation {
                    regulation_number: "2019/1150".to_string(),
                    regulation_title: "Platform to Business Regulation".to_string(),
                    regulation_text: "This Regulation lays down rules on promoting fairness and transparency for business users of online intermediation services.".to_string(),
                    legal_basis: "Article 114 TFEU".to_string(),
                    date_adopted: "2019-06-20".to_string(),
                    date_effective: "2020-07-12".to_string(),
                    directly_applicable: true,
                    binding_articles: vec![],
                    member_state_obligations: vec!["Designate competent authorities".to_string()],
                    enforcement_mechanisms: vec!["Penalty provisions".to_string()],
                    sector: "Digital Markets".to_string(),
                    supremacy_clause: "Uniform application".to_string(),
                }
            ],
            directives: vec![
                EUDirective {
                    directive_number: "2014/24/EU".to_string(),
                    directive_title: "Public Procurement Directive".to_string(),
                    directive_text: "This Directive establishes rules on the procedures for procurement by contracting authorities with respect to public contracts.".to_string(),
                    legal_basis: "Article 53(1), Article 62 and Article 114 TFEU".to_string(),
                    date_adopted: "2014-02-26".to_string(),
                    transposition_deadline: "2016-04-18".to_string(),
                    member_states_addressed: vec!["All Member States".to_string()],
                    objectives: vec!["Ensure free movement of supplies, services and works".to_string(), "Open public procurement to competition".to_string()],
                    minimum_requirements: vec!["Transparency obligations".to_string(), "Non-discrimination principles".to_string()],
                    implementation_measures: vec!["National transposition laws".to_string(), "Remedies directives".to_string()],
                    compliance_monitoring: "Commission monitoring reports".to_string(),
                    infringement_procedures: vec!["Failure to transpose".to_string(), "Incorrect transposition".to_string()],
                    sector: "Public Procurement".to_string(),
                    cjeu_interpretations: vec!["Teckal doctrine".to_string(), "In-house exemption".to_string()],
                },
                EUDirective {
                    directive_number: "2019/790".to_string(),
                    directive_title: "Copyright in the Digital Single Market Directive".to_string(),
                    directive_text: "This Directive lays down rules on copyright and related rights in the Digital Single Market.".to_string(),
                    legal_basis: "Article 114 TFEU".to_string(),
                    date_adopted: "2019-04-17".to_string(),
                    transposition_deadline: "2021-06-07".to_string(),
                    member_states_addressed: vec!["All Member States".to_string()],
                    objectives: vec!["Modernize copyright law".to_string(), "Balance rightsholder and user interests".to_string()],
                    minimum_requirements: vec!["Exception for text and data mining".to_string(), "Platform liability rules".to_string()],
                    implementation_measures: vec!["Article 17 implementation".to_string(), "Complaint mechanisms".to_string()],
                    compliance_monitoring: "Regular Commission reports".to_string(),
                    infringement_procedures: vec!["Non-transposition procedures".to_string()],
                    sector: "Intellectual Property".to_string(),
                    cjeu_interpretations: vec!["Pending".to_string()],
                }
            ],
            decisions: vec![
                EUDecision {
                    decision_number: "2013/755/EU".to_string(),
                    decision_title: "European Semester Decision".to_string(),
                    decision_text: "This Decision establishes provisions for enhanced surveillance of Member States experiencing or threatened with serious difficulties with respect to their financial stability.".to_string(),
                    addressees: vec!["All Member States".to_string()],
                    date_adopted: "2013-12-09".to_string(),
                    legal_basis: "Article 121(6) TFEU".to_string(),
                    binding_nature: "Fully binding on addressees".to_string(),
                    implementation_requirements: vec!["National reform programmes".to_string(), "Stability programmes".to_string()],
                    compliance_deadline: "Annual cycle".to_string(),
                }
            ],
            recommendations: vec!["Country-specific recommendations".to_string(), "Best practices sharing".to_string()],
            opinions: vec!["European Central Bank opinions".to_string(), "Committee of the Regions opinions".to_string()],
        };

        let court_system = EUCourtSystem {
            court_of_justice: CourtOfJustice {
                composition: "One judge per Member State plus Advocates General".to_string(),
                jurisdiction: vec!["Preliminary rulings".to_string(), "Infringement proceedings".to_string(), "Appeals from General Court".to_string()],
                preliminary_rulings: vec![
                    CJEUJudgment {
                        case_number: "C-26/62".to_string(),
                        case_name: "Van Gend en Loos v Nederlandse Administratie der Belastingen".to_string(),
                        date_judgment: "1963-02-05".to_string(),
                        court_formation: "Full Court".to_string(),
                        legal_questions: vec!["Direct effect of EU law".to_string()],
                        judgment_text: "The Community constitutes a new legal order of international law for the benefit of which the states have limited their sovereign rights".to_string(),
                        operative_part: "EU law has direct effect".to_string(),
                        legal_principles: vec!["Direct effect".to_string(), "New legal order".to_string()],
                        precedent_value: "Foundational".to_string(),
                        member_states_affected: vec!["All Member States".to_string()],
                        eu_law_interpretation: vec!["Revolutionary interpretation".to_string()],
                        preliminary_ruling: true,
                        infringement_case: false,
                    },
                    CJEUJudgment {
                        case_number: "C-6/64".to_string(),
                        case_name: "Costa v ENEL".to_string(),
                        date_judgment: "1964-07-15".to_string(),
                        court_formation: "Full Court".to_string(),
                        legal_questions: vec!["Supremacy of EU law".to_string()],
                        judgment_text: "The law stemming from the treaty, an independent source of law, could not, because of its special and original nature, be overridden by domestic legal provisions".to_string(),
                        operative_part: "EU law has supremacy over national law".to_string(),
                        legal_principles: vec!["Supremacy".to_string(), "Autonomous legal order".to_string()],
                        precedent_value: "Foundational".to_string(),
                        member_states_affected: vec!["All Member States".to_string()],
                        eu_law_interpretation: vec!["Constitutional significance".to_string()],
                        preliminary_ruling: true,
                        infringement_case: false,
                    }
                ],
                infringement_proceedings: vec![],
                direct_actions: vec![],
                appeals: vec![],
            },
            general_court: GeneralCourt {
                composition: "Two judges per Member State".to_string(),
                jurisdiction: vec!["Direct actions by individuals".to_string(), "Competition cases".to_string(), "State aid cases".to_string()],
                first_instance_cases: vec![],
                competition_cases: vec![],
                state_aid_cases: vec![],
            },
            specialized_courts: vec!["European Union Civil Service Tribunal (defunct)".to_string()],
            national_courts_integration: vec!["Preliminary reference procedure".to_string(), "Mutual recognition".to_string()],
        };

        let institutional_framework = vec![
            EUInstitution {
                institution_name: "European Council".to_string(),
                institution_type: "Intergovernmental".to_string(),
                legal_basis: "Article 15 TEU".to_string(),
                composition: vec!["Heads of State or Government".to_string(), "President of the European Council".to_string(), "President of the Commission".to_string()],
                powers: vec!["Define general political directions".to_string(), "Deal with complex/sensitive issues".to_string()],
                decision_making_procedures: vec!["Consensus".to_string(), "Qualified majority (exceptional)".to_string()],
                legislative_role: "None (except Treaty revision)".to_string(),
                executive_role: "Strategic guidance".to_string(),
                judicial_role: "None".to_string(),
                accountability_mechanisms: vec!["National parliaments".to_string(), "European Parliament (limited)".to_string()],
            },
            EUInstitution {
                institution_name: "Council of the European Union".to_string(),
                institution_type: "Intergovernmental".to_string(),
                legal_basis: "Article 16 TEU".to_string(),
                composition: vec!["One minister per Member State".to_string(), "Rotating presidency".to_string()],
                powers: vec!["Legislative function".to_string(), "Executive function".to_string(), "Policy coordination".to_string()],
                decision_making_procedures: vec!["Qualified majority voting".to_string(), "Unanimity (specific areas)".to_string()],
                legislative_role: "Co-legislator with Parliament".to_string(),
                executive_role: "Implementation oversight".to_string(),
                judicial_role: "None".to_string(),
                accountability_mechanisms: vec!["National parliaments".to_string(), "Public debates".to_string()],
            },
            EUInstitution {
                institution_name: "European Parliament".to_string(),
                institution_type: "Supranational".to_string(),
                legal_basis: "Article 14 TEU".to_string(),
                composition: vec!["705 directly elected MEPs".to_string(), "Representation by population".to_string()],
                powers: vec!["Legislative function".to_string(), "Budgetary authority".to_string(), "Democratic oversight".to_string()],
                decision_making_procedures: vec!["Majority voting".to_string(), "Absolute majority (specific cases)".to_string()],
                legislative_role: "Co-legislator with Council".to_string(),
                executive_role: "Commission oversight".to_string(),
                judicial_role: "None".to_string(),
                accountability_mechanisms: vec!["Direct elections".to_string(), "European Citizens".to_string()],
            },
            EUInstitution {
                institution_name: "European Commission".to_string(),
                institution_type: "Supranational".to_string(),
                legal_basis: "Article 17 TEU".to_string(),
                composition: vec!["One Commissioner per Member State".to_string(), "President appointed by European Council".to_string()],
                powers: vec!["Legislative initiative".to_string(), "Executive implementation".to_string(), "Guardian of Treaties".to_string()],
                decision_making_procedures: vec!["College principle".to_string(), "Majority voting".to_string()],
                legislative_role: "Monopoly of initiative".to_string(),
                executive_role: "Implementation and enforcement".to_string(),
                judicial_role: "None".to_string(),
                accountability_mechanisms: vec!["European Parliament".to_string(), "Motion of censure".to_string()],
            }
        ];

        let competence_framework = vec![
            EUCompetence {
                competence_area: "Customs Union".to_string(),
                competence_type: "Exclusive".to_string(),
                exclusive_competence: true,
                shared_competence: false,
                supporting_competence: false,
                legal_basis: "Article 3(1)(a) TFEU".to_string(),
                eu_powers: vec!["Full regulatory power".to_string(), "External representation".to_string()],
                member_state_powers: vec!["None (unless authorized)".to_string()],
                subsidiarity_principle: "Not applicable".to_string(),
                proportionality_principle: "Applicable".to_string(),
            },
            EUCompetence {
                competence_area: "Internal Market".to_string(),
                competence_type: "Shared".to_string(),
                exclusive_competence: false,
                shared_competence: true,
                supporting_competence: false,
                legal_basis: "Article 4(2)(a) TFEU".to_string(),
                eu_powers: vec!["Harmonization measures".to_string(), "Mutual recognition".to_string()],
                member_state_powers: vec!["Residual competence".to_string(), "National implementation".to_string()],
                subsidiarity_principle: "Fully applicable".to_string(),
                proportionality_principle: "Applicable".to_string(),
            },
            EUCompetence {
                competence_area: "Education, Culture, Youth".to_string(),
                competence_type: "Supporting".to_string(),
                exclusive_competence: false,
                shared_competence: false,
                supporting_competence: true,
                legal_basis: "Article 6 TFEU".to_string(),
                eu_powers: vec!["Supporting actions".to_string(), "Coordination".to_string()],
                member_state_powers: vec!["Primary responsibility".to_string(), "National policies".to_string()],
                subsidiarity_principle: "Fully applicable".to_string(),
                proportionality_principle: "Applicable".to_string(),
            }
        ];

        let member_states = vec![
            EUMemberState {
                country_name: "Germany".to_string(),
                accession_date: "1957-03-25".to_string(),
                accession_treaty: "Treaty of Rome".to_string(),
                derogations: vec![],
                opt_outs: vec![],
                eurozone_member: true,
                schengen_member: true,
                voting_weight: 29,
                national_implementation_record: vec!["Generally compliant".to_string()],
                infringement_cases: vec!["Occasional environmental law issues".to_string()],
            },
            EUMemberState {
                country_name: "France".to_string(),
                accession_date: "1957-03-25".to_string(),
                accession_treaty: "Treaty of Rome".to_string(),
                derogations: vec![],
                opt_outs: vec![],
                eurozone_member: true,
                schengen_member: true,
                voting_weight: 29,
                national_implementation_record: vec!["Generally compliant".to_string()],
                infringement_cases: vec!["State aid issues".to_string()],
            },
            EUMemberState {
                country_name: "Poland".to_string(),
                accession_date: "2004-05-01".to_string(),
                accession_treaty: "Treaty of Accession 2003".to_string(),
                derogations: vec![],
                opt_outs: vec![],
                eurozone_member: false,
                schengen_member: true,
                voting_weight: 27,
                national_implementation_record: vec!["Rule of law concerns".to_string()],
                infringement_cases: vec!["Judicial independence".to_string(), "Article 7 procedure".to_string()],
            }
        ];

        EULegalSystem {
            founding_treaties,
            secondary_legislation,
            court_system,
            institutional_framework,
            competence_framework,
            member_states,
            legal_principles: vec![
                "Supremacy of EU law".to_string(),
                "Direct effect".to_string(),
                "State liability".to_string(),
                "Fundamental rights protection".to_string(),
                "Proportionality".to_string(),
                "Legal certainty".to_string(),
                "Non-discrimination".to_string(),
                "Solidarity".to_string(),
                "Sincere cooperation".to_string()
            ],
            legislative_procedures: vec![
                "Ordinary legislative procedure (co-decision)".to_string(),
                "Consent procedure".to_string(),
                "Consultation procedure".to_string(),
                "Special legislative procedures".to_string()
            ],
        }
    }

    pub fn get_all_regulations(&self) -> &Vec<EURegulation> {
        &self.secondary_legislation.regulations
    }

    pub fn get_all_directives(&self) -> &Vec<EUDirective> {
        &self.secondary_legislation.directives
    }

    pub fn get_treaty_by_name(&self, name: &str) -> Option<&EUTreaty> {
        self.founding_treaties.iter().find(|treaty| treaty.treaty_name.contains(name))
    }

    pub fn get_member_state_info(&self, country: &str) -> Option<&EUMemberState> {
        self.member_states.iter().find(|state| state.country_name == country)
    }

    pub fn search_cjeu_cases(&self, query: &str) -> Vec<&CJEUJudgment> {
        let mut results = Vec::new();

        for case in &self.court_system.court_of_justice.preliminary_rulings {
            if case.case_name.to_lowercase().contains(&query.to_lowercase()) ||
               case.legal_principles.iter().any(|p| p.to_lowercase().contains(&query.to_lowercase())) {
                results.push(case);
            }
        }

        results
    }

    pub fn get_competence_in_area(&self, area: &str) -> Option<&EUCompetence> {
        self.competence_framework.iter().find(|comp| comp.competence_area.to_lowercase().contains(&area.to_lowercase()))
    }

    pub fn get_institution_powers(&self, institution: &str) -> Option<&Vec<String>> {
        self.institutional_framework.iter()
            .find(|inst| inst.institution_name.to_lowercase().contains(&institution.to_lowercase()))
            .map(|inst| &inst.powers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eu_legal_system_creation() {
        let eu_system = EULegalSystem::new();
        assert!(!eu_system.founding_treaties.is_empty());
        assert!(!eu_system.secondary_legislation.regulations.is_empty());
        assert!(!eu_system.institutional_framework.is_empty());
    }

    #[test]
    fn test_treaty_search() {
        let eu_system = EULegalSystem::new();
        let teu = eu_system.get_treaty_by_name("European Union");
        assert!(teu.is_some());
        assert_eq!(teu.unwrap().treaty_type, "Founding Treaty");
    }

    #[test]
    fn test_member_state_lookup() {
        let eu_system = EULegalSystem::new();
        let germany = eu_system.get_member_state_info("Germany");
        assert!(germany.is_some());
        assert!(germany.unwrap().eurozone_member);
    }

    #[test]
    fn test_cjeu_case_search() {
        let eu_system = EULegalSystem::new();
        let cases = eu_system.search_cjeu_cases("direct effect");
        assert!(!cases.is_empty());
    }
}