// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::drug::Drug;
use super::grant::Grant;
use super::medical_code::MedicalCode;
use super::medical_condition::MedicalCondition;
use super::medical_device::MedicalDevice;
use super::medical_guideline::MedicalGuideline;
use super::medical_imaging_technique::MedicalImagingTechnique;
use super::medical_sign::MedicalSign;
use super::medical_specialty::MedicalSpecialty;
use super::medical_study::MedicalStudy;
use super::medicine_system::MedicineSystem;
use super::organization::Organization;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::legal_status::legalStatus;
use super::main_entity_of_page::mainEntityOfPage;
use super::normal_range::normalRange;
use super::subject_of::subjectOf;

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
    pub additional_type: Option<URL>,

    /// An alias for the item.
    pub alternate_name: Option<Text>,

    /// A description of the item.
    pub description: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<identifier>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<image>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<mainEntityOfPage>,

    /// The name of the item.
    pub name: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<subjectOf>,

    /// URL of the item.
    pub url: Option<URL>,

    /// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
    pub code: Option<MedicalCode>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<Grant>,

    /// A medical guideline related to this entity.
    pub guideline: Option<MedicalGuideline>,

    /// The drug or supplement's legal status, including any controlled substance schedules that apply.
    pub legal_status: Option<legalStatus>,

    /// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
    pub medicine_system: Option<MedicineSystem>,

    /// If applicable, the organization that officially recognizes this entity as part of its endorsed system of medicine.
    pub recognizing_authority: Option<Organization>,

    /// If applicable, a medical specialty in which this entity is relevant.
    pub relevant_specialty: Option<MedicalSpecialty>,

    /// A medical study or trial related to this entity.
    pub study: Option<MedicalStudy>,

    /// Drugs that affect the test's results.
    pub affected_by: Option<Drug>,

    /// Range of acceptable values for a typical patient, when applicable.
    pub normal_range: Option<normalRange>,

    /// A sign detected by the test.
    pub sign_detected: Option<MedicalSign>,

    /// A condition the test is used to diagnose.
    pub used_to_diagnose: Option<MedicalCondition>,

    /// Device used to perform the test.
    pub uses_device: Option<MedicalDevice>,

    /// Imaging technique used.
    pub imaging_technique: Option<MedicalImagingTechnique>,
}

impl ImagingTest {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
