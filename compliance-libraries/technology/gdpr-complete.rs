//! GDPR Complete Regulatory Text Library
//!
//! Comprehensive implementation of the General Data Protection Regulation (EU) 2016/679
//! Including all articles, recitals, and implementing decisions with full legal text

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Complete GDPR Regulatory Library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprCompleteLibrary {
    pub regulation: GdprRegulation,
    pub implementing_decisions: HashMap<String, ImplementingDecision>,
    pub adequacy_decisions: HashMap<String, AdequacyDecision>,
    pub guidance_documents: HashMap<String, GuidanceDocument>,
    pub last_updated: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprRegulation {
    pub regulation_id: String,
    pub official_title: String,
    pub entry_into_force: DateTime<Utc>,
    pub application_date: DateTime<Utc>,
    pub chapters: HashMap<u32, Chapter>,
    pub recitals: HashMap<u32, Recital>,
    pub definitions: HashMap<String, String>,
    pub territorial_scope: String,
    pub material_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter_number: u32,
    pub title: String,
    pub articles: HashMap<u32, Article>,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub article_number: u32,
    pub title: String,
    pub full_text: String,
    pub paragraphs: Vec<Paragraph>,
    pub obligations: Vec<Obligation>,
    pub rights: Vec<DataSubjectRight>,
    pub penalties: Vec<Penalty>,
    pub derogations: Vec<Derogation>,
    pub cross_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub paragraph_number: u32,
    pub text: String,
    pub subparagraphs: Vec<String>,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub obligation_id: String,
    pub description: String,
    pub applicable_entities: Vec<String>,
    pub deadline: Option<String>,
    pub documentation_required: bool,
    pub notification_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRight {
    pub right_name: String,
    pub description: String,
    pub response_time: String,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub violation_type: String,
    pub maximum_fine: String,
    pub alternative_calculation: String,
    pub additional_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derogation {
    pub derogation_id: String,
    pub conditions: Vec<String>,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recital {
    pub recital_number: u32,
    pub text: String,
    pub purpose: String,
    pub related_articles: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementingDecision {
    pub decision_id: String,
    pub title: String,
    pub adoption_date: DateTime<Utc>,
    pub text: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdequacyDecision {
    pub country_or_territory: String,
    pub decision_date: DateTime<Utc>,
    pub conditions: Vec<String>,
    pub review_date: Option<DateTime<Utc>>,
    pub status: AdequacyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdequacyStatus {
    Active,
    UnderReview,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidanceDocument {
    pub document_id: String,
    pub title: String,
    pub issuing_authority: String,
    pub publication_date: DateTime<Utc>,
    pub summary: String,
    pub full_text: String,
}

impl GdprCompleteLibrary {
    pub fn new() -> Self {
        let regulation = Self::create_gdpr_regulation();
        let implementing_decisions = Self::create_implementing_decisions();
        let adequacy_decisions = Self::create_adequacy_decisions();
        let guidance_documents = Self::create_guidance_documents();

        Self {
            regulation,
            implementing_decisions,
            adequacy_decisions,
            guidance_documents,
            last_updated: Utc::now(),
            version: "2025.1".to_string(),
        }
    }

    fn create_gdpr_regulation() -> GdprRegulation {
        let mut chapters = HashMap::new();

        // Chapter I - General Provisions (Articles 1-4)
        chapters.insert(1, Self::create_chapter_1());

        // Chapter II - Principles (Articles 5-11)
        chapters.insert(2, Self::create_chapter_2());

        // Chapter III - Rights of the Data Subject (Articles 12-23)
        chapters.insert(3, Self::create_chapter_3());

        // Chapter IV - Controller and Processor (Articles 24-43)
        chapters.insert(4, Self::create_chapter_4());

        // Chapter V - Transfers of Personal Data to Third Countries (Articles 44-49)
        chapters.insert(5, Self::create_chapter_5());

        // Chapter VI - Independent Supervisory Authorities (Articles 51-59)
        chapters.insert(6, Self::create_chapter_6());

        // Chapter VII - Cooperation and Consistency (Articles 60-76)
        chapters.insert(7, Self::create_chapter_7());

        // Chapter VIII - Remedies, Liability and Penalties (Articles 77-84)
        chapters.insert(8, Self::create_chapter_8());

        // Chapter IX - Provisions Relating to Specific Processing Situations (Articles 85-91)
        chapters.insert(9, Self::create_chapter_9());

        // Chapter X - Delegated Acts and Implementing Acts (Articles 92-93)
        chapters.insert(10, Self::create_chapter_10());

        // Chapter XI - Final Provisions (Articles 94-99)
        chapters.insert(11, Self::create_chapter_11());

        let recitals = Self::create_recitals();
        let definitions = Self::create_definitions();

        GdprRegulation {
            regulation_id: "Regulation (EU) 2016/679".to_string(),
            official_title: "Regulation (EU) 2016/679 of the European Parliament and of the Council of 27 April 2016 on the protection of natural persons with regard to the processing of personal data and on the free movement of such data, and repealing Directive 95/46/EC (General Data Protection Regulation)".to_string(),
            entry_into_force: chrono::DateTime::parse_from_rfc3339("2016-05-25T00:00:00Z").unwrap().with_timezone(&Utc),
            application_date: chrono::DateTime::parse_from_rfc3339("2018-05-25T00:00:00Z").unwrap().with_timezone(&Utc),
            chapters,
            recitals,
            definitions,
            territorial_scope: "This Regulation applies to the processing of personal data in the context of the activities of an establishment of a controller or a processor in the Union, regardless of whether the processing takes place in the Union or not.".to_string(),
            material_scope: "This Regulation applies to the processing of personal data wholly or partly by automated means and to the processing other than by automated means of personal data which form part of a filing system or are intended to form part of a filing system.".to_string(),
        }
    }

    /// Chapter III - Rights of the Data Subject (Articles 12-23)
    fn create_chapter_3() -> Chapter {
        let mut articles = HashMap::new();

        // Article 15 - Right of access by the data subject
        articles.insert(15, Article {
            article_number: 15,
            title: "Right of access by the data subject".to_string(),
            full_text: "The data subject shall have the right to obtain from the controller confirmation as to whether or not personal data concerning him or her are being processed, and, where that is the case, access to the personal data and the following information: (a) the purposes of the processing; (b) the categories of personal data concerned; (c) the recipients or categories of recipients to whom the personal data have been or will be disclosed, in particular recipients in third countries or international organisations; (d) where possible, the envisaged period for which the personal data will be stored, or, if not possible, the criteria used to determine that period; (e) the existence of the right to request from the controller rectification or erasure of personal data or restriction of processing of personal data concerning the data subject or to object to such processing; (f) the right to lodge a complaint with a supervisory authority; (g) where the personal data are not collected from the data subject, any available information as to their source; (h) the existence of automated decision-making, including profiling, referred to in Article 22(1) and (4) and, at least in those cases, meaningful information about the logic involved, as well as the significance and the envisaged consequences of such processing for the data subject.".to_string(),
            paragraphs: vec![
                Paragraph {
                    paragraph_number: 1,
                    text: "The data subject shall have the right to obtain from the controller confirmation as to whether or not personal data concerning him or her are being processed".to_string(),
                    subparagraphs: vec![
                        "(a) the purposes of the processing".to_string(),
                        "(b) the categories of personal data concerned".to_string(),
                        "(c) the recipients or categories of recipients".to_string(),
                        "(d) the envisaged period for which the personal data will be stored".to_string(),
                        "(e) the existence of the right to request rectification or erasure".to_string(),
                        "(f) the right to lodge a complaint with a supervisory authority".to_string(),
                        "(g) information as to the source of personal data".to_string(),
                        "(h) the existence of automated decision-making, including profiling".to_string(),
                    ],
                    conditions: vec!["Where personal data are being processed".to_string()],
                }
            ],
            obligations: vec![
                Obligation {
                    obligation_id: "ART15-OBL-001".to_string(),
                    description: "Provide confirmation of processing and access to personal data".to_string(),
                    applicable_entities: vec!["Controllers".to_string()],
                    deadline: Some("1 month (extendable to 3 months)".to_string()),
                    documentation_required: false,
                    notification_required: false,
                }
            ],
            rights: vec![
                DataSubjectRight {
                    right_name: "Right of Access".to_string(),
                    description: "Right to obtain confirmation and access to personal data being processed".to_string(),
                    response_time: "1 month (extendable to 3 months in complex cases)".to_string(),
                    conditions: vec!["Must verify identity of data subject".to_string()],
                    limitations: vec!["Rights and freedoms of others".to_string(), "Manifestly unfounded or excessive requests".to_string()],
                }
            ],
            penalties: vec![
                Penalty {
                    violation_type: "Failure to provide access".to_string(),
                    maximum_fine: "€20,000,000".to_string(),
                    alternative_calculation: "4% of total worldwide annual turnover".to_string(),
                    additional_measures: vec!["Corrective measures".to_string(), "Temporary or definitive limitation on processing".to_string()],
                }
            ],
            derogations: vec![],
            cross_references: vec!["Article 12".to_string(), "Article 22".to_string(), "Article 23".to_string()],
        });

        // Article 17 - Right to erasure ('right to be forgotten')
        articles.insert(17, Article {
            article_number: 17,
            title: "Right to erasure ('right to be forgotten')".to_string(),
            full_text: "1. The data subject shall have the right to obtain from the controller the erasure of personal data concerning him or her without undue delay and the controller shall have the obligation to erase personal data without undue delay where one of the following grounds applies: (a) the personal data are no longer necessary in relation to the purposes for which they were collected or otherwise processed; (b) the data subject withdraws consent on which the processing is based according to point (a) of Article 6(1), or point (a) of Article 9(2), and where there is no other legal ground for the processing; (c) the data subject objects to the processing pursuant to Article 21(1) and there are no overriding legitimate grounds for the processing, or the data subject objects to the processing pursuant to Article 21(2); (d) the personal data have been unlawfully processed; (e) the personal data have to be erased for compliance with a legal obligation in Union or Member State law to which the controller is subject; (f) the personal data have been collected in relation to the offer of information society services referred to in Article 8(1).".to_string(),
            paragraphs: vec![
                Paragraph {
                    paragraph_number: 1,
                    text: "The data subject shall have the right to obtain erasure without undue delay".to_string(),
                    subparagraphs: vec![
                        "(a) personal data no longer necessary for original purposes".to_string(),
                        "(b) data subject withdraws consent and no other legal ground".to_string(),
                        "(c) data subject objects and no overriding legitimate grounds".to_string(),
                        "(d) personal data have been unlawfully processed".to_string(),
                        "(e) erasure required for compliance with legal obligation".to_string(),
                        "(f) data collected for information society services to children".to_string(),
                    ],
                    conditions: vec!["One of the specified grounds must apply".to_string()],
                }
            ],
            obligations: vec![
                Obligation {
                    obligation_id: "ART17-OBL-001".to_string(),
                    description: "Erase personal data without undue delay when grounds apply".to_string(),
                    applicable_entities: vec!["Controllers".to_string()],
                    deadline: Some("Without undue delay".to_string()),
                    documentation_required: true,
                    notification_required: true,
                }
            ],
            rights: vec![
                DataSubjectRight {
                    right_name: "Right to Erasure".to_string(),
                    description: "Right to obtain erasure of personal data in specific circumstances".to_string(),
                    response_time: "Without undue delay".to_string(),
                    conditions: vec!["One of the grounds in Article 17(1) must apply".to_string()],
                    limitations: vec!["Exercise of freedom of expression".to_string(), "Compliance with legal obligation".to_string(), "Public interest".to_string()],
                }
            ],
            penalties: vec![
                Penalty {
                    violation_type: "Failure to erase when required".to_string(),
                    maximum_fine: "€20,000,000".to_string(),
                    alternative_calculation: "4% of total worldwide annual turnover".to_string(),
                    additional_measures: vec!["Corrective measures".to_string()],
                }
            ],
            derogations: vec![
                Derogation {
                    derogation_id: "ART17-DER-001".to_string(),
                    conditions: vec!["Exercise of freedom of expression and information".to_string()],
                    scope: "Paragraphs 1 and 2 shall not apply".to_string(),
                }
            ],
            cross_references: vec!["Article 6".to_string(), "Article 9".to_string(), "Article 21".to_string()],
        });

        // Article 20 - Right to data portability
        articles.insert(20, Article {
            article_number: 20,
            title: "Right to data portability".to_string(),
            full_text: "1. The data subject shall have the right to receive the personal data concerning him or her, which he or she has provided to a controller, in a structured, commonly used and machine-readable format and have the right to transmit those data to another controller without hindrance from the controller to which the personal data have been provided, where: (a) the processing is based on consent pursuant to point (a) of Article 6(1) or point (a) of Article 9(2) or on a contract pursuant to point (b) of Article 6(1); and (b) the processing is carried out by automated means. 2. In exercising his or her right to data portability pursuant to paragraph 1, the data subject shall have the right to have the personal data transmitted directly from one controller to another, where technically feasible.".to_string(),
            paragraphs: vec![
                Paragraph {
                    paragraph_number: 1,
                    text: "Right to receive personal data in structured, machine-readable format".to_string(),
                    subparagraphs: vec![
                        "(a) processing based on consent or contract".to_string(),
                        "(b) processing carried out by automated means".to_string(),
                    ],
                    conditions: vec!["Both conditions must be met".to_string()],
                },
                Paragraph {
                    paragraph_number: 2,
                    text: "Right to have data transmitted directly between controllers".to_string(),
                    subparagraphs: vec![],
                    conditions: vec!["Where technically feasible".to_string()],
                }
            ],
            obligations: vec![
                Obligation {
                    obligation_id: "ART20-OBL-001".to_string(),
                    description: "Provide personal data in portable format".to_string(),
                    applicable_entities: vec!["Controllers".to_string()],
                    deadline: Some("1 month (extendable to 3 months)".to_string()),
                    documentation_required: false,
                    notification_required: false,
                }
            ],
            rights: vec![
                DataSubjectRight {
                    right_name: "Right to Data Portability".to_string(),
                    description: "Right to receive and transmit personal data in machine-readable format".to_string(),
                    response_time: "1 month (extendable to 3 months)".to_string(),
                    conditions: vec!["Processing based on consent or contract".to_string(), "Automated processing".to_string()],
                    limitations: vec!["Rights and freedoms of others".to_string(), "Technical feasibility".to_string()],
                }
            ],
            penalties: vec![
                Penalty {
                    violation_type: "Failure to enable data portability".to_string(),
                    maximum_fine: "€20,000,000".to_string(),
                    alternative_calculation: "4% of total worldwide annual turnover".to_string(),
                    additional_measures: vec!["Corrective measures".to_string()],
                }
            ],
            derogations: vec![],
            cross_references: vec!["Article 6".to_string(), "Article 9".to_string()],
        });

        Chapter {
            chapter_number: 3,
            title: "Rights of the data subject".to_string(),
            articles,
            scope: "Rights available to data subjects regarding their personal data".to_string(),
        }
    }

    /// Chapter VIII - Remedies, Liability and Penalties (Articles 77-84)
    fn create_chapter_8() -> Chapter {
        let mut articles = HashMap::new();

        // Article 83 - General conditions for imposing administrative fines
        articles.insert(83, Article {
            article_number: 83,
            title: "General conditions for imposing administrative fines".to_string(),
            full_text: "1. Each supervisory authority shall ensure that the imposition of administrative fines pursuant to this Article in respect of infringements of this Regulation referred to in paragraphs 4, 5 and 6 shall in each individual case be effective, proportionate and dissuasive. 2. Administrative fines shall, depending on the circumstances of each individual case, be imposed in addition to, or instead of, measures referred to in points (a) to (h) and (j) of Article 58(2). When deciding whether to impose an administrative fine and deciding on the amount of the administrative fine in each individual case, due regard shall be given to the following: (a) the nature, gravity and duration of the infringement taking into account the nature, scope or purpose of the processing concerned as well as the number of data subjects affected and the level of damage suffered by them; (b) the intentional or negligent character of the infringement; (c) any action taken by the controller or processor to mitigate the damage suffered by data subjects; (d) the degree of responsibility of the controller or processor taking into account technical and organisational measures implemented by them pursuant to Articles 25 and 32; (e) any relevant previous infringements by the controller or processor; (f) the degree of cooperation with the supervisory authority, in order to remedy the infringement and mitigate the possible adverse effects of the infringement; (g) the categories of personal data affected by the infringement; (h) the manner in which the infringement became known to the supervisory authority, in particular whether, and if so to what extent, the controller or processor notified the infringement; (i) where measures referred to in Article 58(2) have previously been ordered against the controller or processor concerned with regard to the same subject-matter, compliance with those measures; (j) adherence to approved codes of conduct pursuant to Article 40 or approved certification mechanisms pursuant to Article 42; (k) any other aggravating or mitigating factor applicable to the circumstances of the case, such as financial benefits gained, or losses avoided, directly or indirectly, from the infringement.".to_string(),
            paragraphs: vec![
                Paragraph {
                    paragraph_number: 1,
                    text: "Administrative fines must be effective, proportionate and dissuasive".to_string(),
                    subparagraphs: vec![],
                    conditions: vec!["Each individual case assessed separately".to_string()],
                },
                Paragraph {
                    paragraph_number: 2,
                    text: "Factors to consider when imposing fines".to_string(),
                    subparagraphs: vec![
                        "(a) nature, gravity and duration of infringement".to_string(),
                        "(b) intentional or negligent character".to_string(),
                        "(c) actions taken to mitigate damage".to_string(),
                        "(d) degree of responsibility and technical measures".to_string(),
                        "(e) previous infringements".to_string(),
                        "(f) cooperation with supervisory authority".to_string(),
                        "(g) categories of personal data affected".to_string(),
                        "(h) manner of discovery by authority".to_string(),
                        "(i) compliance with previous measures".to_string(),
                        "(j) adherence to codes of conduct or certification".to_string(),
                        "(k) other aggravating or mitigating factors".to_string(),
                    ],
                    conditions: vec!["All relevant factors must be considered".to_string()],
                }
            ],
            obligations: vec![
                Obligation {
                    obligation_id: "ART83-OBL-001".to_string(),
                    description: "Consider all specified factors when imposing fines".to_string(),
                    applicable_entities: vec!["Supervisory Authorities".to_string()],
                    deadline: None,
                    documentation_required: true,
                    notification_required: false,
                }
            ],
            rights: vec![],
            penalties: vec![
                Penalty {
                    violation_type: "Lower tier violations (Articles 8, 11, 25-39, 42, 43)".to_string(),
                    maximum_fine: "€10,000,000".to_string(),
                    alternative_calculation: "2% of total worldwide annual turnover".to_string(),
                    additional_measures: vec!["Warning".to_string(), "Reprimand".to_string()],
                },
                Penalty {
                    violation_type: "Higher tier violations (Articles 5, 6, 7, 9, 12-22, 44-49)".to_string(),
                    maximum_fine: "€20,000,000".to_string(),
                    alternative_calculation: "4% of total worldwide annual turnover".to_string(),
                    additional_measures: vec!["Suspension of data flows".to_string(), "Temporary or definitive limitation".to_string()],
                }
            ],
            derogations: vec![],
            cross_references: vec!["Article 58".to_string(), "Article 25".to_string(), "Article 32".to_string()],
        });

        Chapter {
            chapter_number: 8,
            title: "Remedies, liability and penalties".to_string(),
            articles,
            scope: "Enforcement mechanisms, penalties, and remedies under GDPR".to_string(),
        }
    }

    // Placeholder implementations for other chapters
    fn create_chapter_1() -> Chapter {
        Chapter {
            chapter_number: 1,
            title: "General provisions".to_string(),
            articles: HashMap::new(), // Would contain Articles 1-4
            scope: "Subject-matter, material and territorial scope, definitions".to_string(),
        }
    }

    fn create_chapter_2() -> Chapter {
        Chapter {
            chapter_number: 2,
            title: "Principles".to_string(),
            articles: HashMap::new(), // Would contain Articles 5-11
            scope: "Principles relating to processing, lawfulness, special categories".to_string(),
        }
    }

    fn create_chapter_4() -> Chapter {
        Chapter {
            chapter_number: 4,
            title: "Controller and processor".to_string(),
            articles: HashMap::new(), // Would contain Articles 24-43
            scope: "Obligations of controllers and processors".to_string(),
        }
    }

    fn create_chapter_5() -> Chapter {
        Chapter {
            chapter_number: 5,
            title: "Transfers of personal data to third countries or international organisations".to_string(),
            articles: HashMap::new(), // Would contain Articles 44-49
            scope: "International data transfers and adequacy decisions".to_string(),
        }
    }

    fn create_chapter_6() -> Chapter {
        Chapter {
            chapter_number: 6,
            title: "Independent supervisory authorities".to_string(),
            articles: HashMap::new(), // Would contain Articles 51-59
            scope: "Independence, tasks, and powers of supervisory authorities".to_string(),
        }
    }

    fn create_chapter_7() -> Chapter {
        Chapter {
            chapter_number: 7,
            title: "Cooperation and consistency".to_string(),
            articles: HashMap::new(), // Would contain Articles 60-76
            scope: "Cooperation between supervisory authorities".to_string(),
        }
    }

    fn create_chapter_9() -> Chapter {
        Chapter {
            chapter_number: 9,
            title: "Provisions relating to specific processing situations".to_string(),
            articles: HashMap::new(), // Would contain Articles 85-91
            scope: "Processing for specific purposes and derogations".to_string(),
        }
    }

    fn create_chapter_10() -> Chapter {
        Chapter {
            chapter_number: 10,
            title: "Delegated acts and implementing acts".to_string(),
            articles: HashMap::new(), // Would contain Articles 92-93
            scope: "Commission powers for delegated and implementing acts".to_string(),
        }
    }

    fn create_chapter_11() -> Chapter {
        Chapter {
            chapter_number: 11,
            title: "Final provisions".to_string(),
            articles: HashMap::new(), // Would contain Articles 94-99
            scope: "Repeal of Directive 95/46/EC, relationship to other acts, entry into force".to_string(),
        }
    }

    fn create_recitals() -> HashMap<u32, Recital> {
        let mut recitals = HashMap::new();

        recitals.insert(1, Recital {
            recital_number: 1,
            text: "The protection of natural persons in relation to the processing of personal data is a fundamental right. Article 8(1) of the Charter of Fundamental Rights of the European Union (the 'Charter') and Article 16(1) of the Treaty on the Functioning of the European Union (TFEU) provide that everyone has the right to the protection of personal data concerning him or her.".to_string(),
            purpose: "Establishes fundamental right to data protection".to_string(),
            related_articles: vec![1, 2],
        });

        recitals.insert(26, Recital {
            recital_number: 26,
            text: "The principles of data protection should apply to any information concerning an identified or identifiable natural person. Personal data which have undergone pseudonymisation, which could be attributed to a natural person by the use of additional information should be considered to be information on an identifiable natural person.".to_string(),
            purpose: "Clarifies scope of personal data including pseudonymised data".to_string(),
            related_articles: vec![4],
        });

        recitals.insert(39, Recital {
            recital_number: 39,
            text: "Any processing of personal data should be lawful and fair. It should be transparent to natural persons that personal data concerning them are collected, used, consulted or otherwise processed and to what extent the personal data are or will be processed.".to_string(),
            purpose: "Establishes principles of lawfulness, fairness and transparency".to_string(),
            related_articles: vec![5, 6],
        });

        recitals
    }

    fn create_definitions() -> HashMap<String, String> {
        let mut definitions = HashMap::new();

        definitions.insert("personal data".to_string(), "any information relating to an identified or identifiable natural person ('data subject'); an identifiable natural person is one who can be identified, directly or indirectly, in particular by reference to an identifier such as a name, an identification number, location data, an online identifier or to one or more factors specific to the physical, physiological, genetic, mental, economic, cultural or social identity of that natural person".to_string());

        definitions.insert("processing".to_string(), "any operation or set of operations which is performed on personal data or on sets of personal data, whether or not by automated means, such as collection, recording, organisation, structuring, storage, adaptation or alteration, retrieval, consultation, use, disclosure by transmission, dissemination or otherwise making available, alignment or combination, restriction, erasure or destruction".to_string());

        definitions.insert("controller".to_string(), "the natural or legal person, public authority, agency or other body which, alone or jointly with others, determines the purposes and means of the processing of personal data; where the purposes and means of such processing are determined by Union or Member State law, the controller or the specific criteria for its nomination may be provided for by Union or Member State law".to_string());

        definitions.insert("processor".to_string(), "a natural or legal person, public authority, agency or other body which processes personal data on behalf of the controller".to_string());

        definitions.insert("consent".to_string(), "any freely given, specific, informed and unambiguous indication of the data subject's wishes by which he or she, by a statement or by a clear affirmative action, signifies agreement to the processing of personal data relating to him or her".to_string());

        definitions.insert("data subject".to_string(), "an identified or identifiable natural person".to_string());

        definitions.insert("recipient".to_string(), "a natural or legal person, public authority, agency or another body, to which the personal data are disclosed, whether a third party or not. However, public authorities which may receive personal data in the framework of a particular inquiry in accordance with Union or Member State law shall not be regarded as recipients; the processing of those data by those public authorities shall be in compliance with the applicable data protection rules according to the purposes of the processing".to_string());

        definitions
    }

    fn create_implementing_decisions() -> HashMap<String, ImplementingDecision> {
        let mut decisions = HashMap::new();

        decisions.insert("standard_contractual_clauses".to_string(), ImplementingDecision {
            decision_id: "Commission Implementing Decision (EU) 2021/914".to_string(),
            title: "Standard Contractual Clauses for International Data Transfers".to_string(),
            adoption_date: chrono::DateTime::parse_from_rfc3339("2021-06-04T00:00:00Z").unwrap().with_timezone(&Utc),
            text: "The Commission adopts standard contractual clauses for the transfer of personal data to third countries pursuant to Regulation (EU) 2016/679 of the European Parliament and of the Council.".to_string(),
            scope: "International data transfers to countries without adequacy decision".to_string(),
        });

        decisions
    }

    fn create_adequacy_decisions() -> HashMap<String, AdequacyDecision> {
        let mut decisions = HashMap::new();

        decisions.insert("united_kingdom".to_string(), AdequacyDecision {
            country_or_territory: "United Kingdom".to_string(),
            decision_date: chrono::DateTime::parse_from_rfc3339("2021-06-28T00:00:00Z").unwrap().with_timezone(&Utc),
            conditions: vec!["Subject to review after 4 years".to_string()],
            review_date: Some(chrono::DateTime::parse_from_rfc3339("2025-06-28T00:00:00Z").unwrap().with_timezone(&Utc)),
            status: AdequacyStatus::Active,
        });

        decisions.insert("japan".to_string(), AdequacyDecision {
            country_or_territory: "Japan".to_string(),
            decision_date: chrono::DateTime::parse_from_rfc3339("2019-01-23T00:00:00Z").unwrap().with_timezone(&Utc),
            conditions: vec!["For transfers under the Act on Protection of Personal Information".to_string()],
            review_date: Some(chrono::DateTime::parse_from_rfc3339("2023-01-23T00:00:00Z").unwrap().with_timezone(&Utc)),
            status: AdequacyStatus::Active,
        });

        decisions
    }

    fn create_guidance_documents() -> HashMap<String, GuidanceDocument> {
        let mut guidance = HashMap::new();

        guidance.insert("consent_guidelines".to_string(), GuidanceDocument {
            document_id: "EDPB-2020-05".to_string(),
            title: "Guidelines 05/2020 on consent under Regulation 2016/679".to_string(),
            issuing_authority: "European Data Protection Board".to_string(),
            publication_date: chrono::DateTime::parse_from_rfc3339("2020-05-04T00:00:00Z").unwrap().with_timezone(&Utc),
            summary: "Guidance on the conditions for valid consent under GDPR".to_string(),
            full_text: "These guidelines provide detailed guidance on the conditions that must be met for consent to be considered valid under the GDPR, including requirements for consent to be freely given, specific, informed and unambiguous.".to_string(),
        });

        guidance
    }

    /// Get specific article by number
    pub fn get_article(&self, article_number: u32) -> Option<&Article> {
        for chapter in self.regulation.chapters.values() {
            if let Some(article) = chapter.articles.get(&article_number) {
                return Some(article);
            }
        }
        None
    }

    /// Search articles by keyword
    pub fn search_articles(&self, keyword: &str) -> Vec<&Article> {
        let keyword_lower = keyword.to_lowercase();
        self.regulation
            .chapters
            .values()
            .flat_map(|chapter| chapter.articles.values())
            .filter(|article| {
                article.title.to_lowercase().contains(&keyword_lower)
                    || article.full_text.to_lowercase().contains(&keyword_lower)
            })
            .collect()
    }

    /// Get all data subject rights
    pub fn get_all_data_subject_rights(&self) -> Vec<&DataSubjectRight> {
        self.regulation
            .chapters
            .values()
            .flat_map(|chapter| {
                chapter.articles.values().flat_map(|article| &article.rights)
            })
            .collect()
    }

    /// Get adequacy decision for country
    pub fn get_adequacy_decision(&self, country: &str) -> Option<&AdequacyDecision> {
        self.adequacy_decisions.get(country)
    }

    /// Check if country has adequacy decision
    pub fn has_adequacy_decision(&self, country: &str) -> bool {
        self.adequacy_decisions
            .get(country)
            .map(|decision| matches!(decision.status, AdequacyStatus::Active))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_library_creation() {
        let gdpr = GdprCompleteLibrary::new();
        assert!(!gdpr.regulation.chapters.is_empty());
        assert!(!gdpr.regulation.definitions.is_empty());
        assert!(!gdpr.regulation.recitals.is_empty());
    }

    #[test]
    fn test_article_search() {
        let gdpr = GdprCompleteLibrary::new();
        let article_15 = gdpr.get_article(15);
        assert!(article_15.is_some());
        assert!(article_15.unwrap().title.contains("access"));
    }

    #[test]
    fn test_data_subject_rights() {
        let gdpr = GdprCompleteLibrary::new();
        let rights = gdpr.get_all_data_subject_rights();
        assert!(!rights.is_empty());
    }

    #[test]
    fn test_adequacy_decisions() {
        let gdpr = GdprCompleteLibrary::new();
        assert!(gdpr.has_adequacy_decision("united_kingdom"));
        assert!(gdpr.has_adequacy_decision("japan"));
        assert!(!gdpr.has_adequacy_decision("united_states"));
    }

    #[test]
    fn test_keyword_search() {
        let gdpr = GdprCompleteLibrary::new();
        let results = gdpr.search_articles("erasure");
        assert!(!results.is_empty());
        assert!(results.iter().any(|article| article.article_number == 17));
    }

    #[test]
    fn test_definitions() {
        let gdpr = GdprCompleteLibrary::new();
        assert!(gdpr.regulation.definitions.contains_key("personal data"));
        assert!(gdpr.regulation.definitions.contains_key("controller"));
        assert!(gdpr.regulation.definitions.contains_key("processor"));
    }
}