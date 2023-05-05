// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::medical_test::MedicalTest;
use super::thing::Thing;
use super::funding::funding;
use super::legal_status::legalStatus;

/// * COMMENT: Any medical imaging modality typically used for diagnostic purposes. * EXTEND FROM: https://schema.org/MedicalTest
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ImagingTest {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ImagingTestOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ImagingTestOptions {
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

    /// Drugs that affect the test's results.
    pub affected_by: Option<MedicalTest>,

    /// Range of acceptable values for a typical patient, when applicable.
    pub normal_range: Option<MedicalTest>,

    /// A sign detected by the test.
    pub sign_detected: Option<MedicalTest>,

    /// A condition the test is used to diagnose.
    pub used_to_diagnose: Option<MedicalTest>,

    /// Device used to perform the test.
    pub uses_device: Option<MedicalTest>,

    /// Imaging technique used.
    pub imaging_technique: Option<ImagingTest>,
}

impl ImagingTest {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
