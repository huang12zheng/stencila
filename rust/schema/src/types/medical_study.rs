// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::thing::Thing;
use super::funding::funding;
use super::health_condition::healthCondition;
use super::legal_status::legalStatus;
use super::sponsor::sponsor;
use super::status::status;

/// * COMMENT: A medical study is an umbrella type covering all kinds of research studies relating to human medicine or health, including observational studies and interventional trials and registries, randomized, controlled or not. When the specific type of study is known, use one of the extensions of this type, such as MedicalTrial or MedicalObservationalStudy. Also, note that this type should be used to mark up data that describes the study itself; to tag an article that publishes the results of a study, use MedicalScholarlyArticle. Note: use the code property of MedicalEntity to store study IDs, e.g. clinicaltrials.gov ID. * EXTEND FROM: https://schema.org/MedicalEntity * LOOK ALSO: https://schema.org/MedicalObservationalStudy, https://schema.org/MedicalTrial
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalStudy {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MedicalStudyOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalStudyOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<Thing>,

    /// An alias for the item.
    pub alternate_name: Option<Thing>,

    /// A description of the item.
    pub description: Option<Thing>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Thing>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<Thing>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<Thing>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<Thing>,

    /// The name of the item.
    pub name: Option<Thing>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Thing>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Thing>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<Thing>,

    /// URL of the item.
    pub url: Option<Thing>,

    /// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
    pub code: Option<MedicalEntity>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<funding>,

    /// A medical guideline related to this entity.
    pub guideline: Option<MedicalEntity>,

    /// The drug or supplement's legal status, including any controlled substance schedules that apply.
    pub legal_status: Option<legalStatus>,

    /// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
    pub medicine_system: Option<MedicalEntity>,

    /// If applicable, the organization that officially recognizes this entity as part of its endorsed system of medicine.
    pub recognizing_authority: Option<MedicalEntity>,

    /// If applicable, a medical specialty in which this entity is relevant.
    pub relevant_specialty: Option<MedicalEntity>,

    /// A medical study or trial related to this entity.
    pub study: Option<MedicalEntity>,

    /// Specifying the health condition(s) of a patient, medical study, or other target audience.
    pub health_condition: Option<healthCondition>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: Option<sponsor>,

    /// The status of the study (enumerated).
    pub status: Option<status>,

    /// The location in which the study is taking/took place.
    pub study_location: Option<MedicalStudy>,

    /// A subject of the study, i.e. one of the medical conditions, therapies, devices, drugs, etc. investigated by the study.
    pub study_subject: Option<MedicalStudy>,
}

impl MedicalStudy {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
