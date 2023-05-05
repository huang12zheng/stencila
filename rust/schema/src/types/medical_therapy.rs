// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::medical_procedure::MedicalProcedure;
use super::thing::Thing;
use super::adverse_outcome::adverseOutcome;
use super::body_location::bodyLocation;
use super::contraindication::contraindication;
use super::dose_schedule::doseSchedule;
use super::drug::drug;
use super::funding::funding;
use super::legal_status::legalStatus;
use super::serious_adverse_outcome::seriousAdverseOutcome;
use super::status::status;

/// * COMMENT: Any medical intervention designed to prevent, treat, and cure human diseases and medical conditions, including both curative and palliative therapies. Medical therapies are typically processes of care relying upon pharmacotherapy, behavioral therapy, supportive therapy (with fluid or nutrition for example), or detoxification (e.g. hemodialysis) aimed at improving or preventing a health condition. * EXTEND FROM: https://schema.org/TherapeuticProcedure * LOOK ALSO: https://schema.org/OccupationalTherapy, https://schema.org/PalliativeProcedure, https://schema.org/PhysicalTherapy, https://schema.org/RadiationTherapy, https://schema.org/RespiratoryTherapy
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalTherapy {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MedicalTherapyOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalTherapyOptions {
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

    /// Location in the body of the anatomical structure.
    pub body_location: Option<bodyLocation>,

    /// Typical or recommended followup care after the procedure is performed.
    pub followup: Option<MedicalProcedure>,

    /// How the procedure is performed.
    pub how_performed: Option<MedicalProcedure>,

    /// Typical preparation that a patient must undergo before having the procedure performed.
    pub preparation: Option<MedicalProcedure>,

    /// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
    pub procedure_type: Option<MedicalProcedure>,

    /// The status of the study (enumerated).
    pub status: Option<status>,

    /// A possible complication and/or side effect of this therapy. If it is known that an adverse outcome is serious (resulting in death, disability, or permanent damage; requiring hospitalization; or otherwise life-threatening or requiring immediate medical attention), tag it as a seriousAdverseOutcome instead.
    pub adverse_outcome: Option<adverseOutcome>,

    /// A dosing schedule for the drug for a given population, either observed, recommended, or maximum dose based on the type used.
    pub dose_schedule: Option<doseSchedule>,

    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Option<drug>,

    /// A contraindication for this therapy.
    pub contraindication: Option<contraindication>,

    /// A therapy that duplicates or overlaps this one.
    pub duplicate_therapy: Option<MedicalTherapy>,

    /// A possible serious complication and/or serious side effect of this therapy. Serious adverse outcomes include those that are life-threatening; result in death, disability, or permanent damage; require hospitalization or prolong existing hospitalization; cause congenital anomalies or birth defects; or jeopardize the patient and may require medical or surgical intervention to prevent one of the outcomes in this definition.
    pub serious_adverse_outcome: Option<seriousAdverseOutcome>,
}

impl MedicalTherapy {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
