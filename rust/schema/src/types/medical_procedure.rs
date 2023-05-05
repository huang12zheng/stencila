// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::grant::Grant;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::legal_status_prop_enum::LegalStatusPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::medical_code::MedicalCode;
use super::medical_guideline::MedicalGuideline;
use super::medical_procedure_type::MedicalProcedureType;
use super::medical_specialty::MedicalSpecialty;
use super::medical_study::MedicalStudy;
use super::medicine_system::MedicineSystem;
use super::organization::Organization;
use super::preparation_prop_enum::PreparationPropEnum;
use super::status_prop_enum::StatusPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/MedicalProcedure
/// * COMMENT:
/// A process of care used in either a diagnostic, therapeutic, preventive or palliative capacity that relies on invasive (surgical), non-invasive, or other techniques.
/// * EXTEND FROM:
/// https://schema.org/MedicalEntity
/// * LOOK ALSO:
/// https://schema.org/DiagnosticProcedure, https://schema.org/PalliativeProcedure, https://schema.org/PhysicalExam, https://schema.org/SurgicalProcedure, https://schema.org/TherapeuticProcedure
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalProcedure {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type_prop_enum: Option<URL>,

    /// An alias for the item.
    pub alternate_name_prop_enum: Option<Text>,

    /// A description of the item.
    pub description_prop_enum: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description_prop_enum: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier_prop_enum: Option<IdentifierPropEnum>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image_prop_enum: Option<ImagePropEnum>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page_prop_enum: Option<MainEntityOfPagePropEnum>,

    /// The name of the item.
    pub name_prop_enum: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action_prop_enum: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as_prop_enum: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of_prop_enum: Option<SubjectOfPropEnum>,

    /// URL of the item.
    pub url_prop_enum: Option<URL>,

    /// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
    pub code_prop_enum: Option<MedicalCode>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding_prop_enum: Option<Grant>,

    /// A medical guideline related to this entity.
    pub guideline_prop_enum: Option<MedicalGuideline>,

    /// The drug or supplement's legal status, including any controlled substance schedules that apply.
    pub legal_status_prop_enum: Option<LegalStatusPropEnum>,

    /// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
    pub medicine_system_prop_enum: Option<MedicineSystem>,

    /// If applicable, the organization that officially recognizes this entity as part of its endorsed system of medicine.
    pub recognizing_authority_prop_enum: Option<Organization>,

    /// If applicable, a medical specialty in which this entity is relevant.
    pub relevant_specialty_prop_enum: Option<MedicalSpecialty>,

    /// A medical study or trial related to this entity.
    pub study_prop_enum: Option<MedicalStudy>,

    /// Location in the body of the anatomical structure.
    pub body_location_prop_enum: Option<Text>,

    /// Typical or recommended followup care after the procedure is performed.
    pub followup_prop_enum: Option<Text>,

    /// How the procedure is performed.
    pub how_performed_prop_enum: Option<Text>,

    /// Typical preparation that a patient must undergo before having the procedure performed.
    pub preparation_prop_enum: Option<PreparationPropEnum>,

    /// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
    pub procedure_type_prop_enum: Option<MedicalProcedureType>,

    /// The status of the study (enumerated).
    pub status_prop_enum: Option<StatusPropEnum>,
}

impl MedicalProcedure {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
