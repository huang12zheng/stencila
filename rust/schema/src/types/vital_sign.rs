// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::medical_entity::MedicalEntity;
use super::medical_sign::MedicalSign;
use super::thing::Thing;
use super::associated_anatomy::associatedAnatomy;
use super::drug::drug;
use super::epidemiology::epidemiology;
use super::funding::funding;
use super::legal_status::legalStatus;
use super::pathophysiology::pathophysiology;
use super::possible_treatment::possibleTreatment;
use super::status::status;

/// * COMMENT: Vital signs are measures of various physiological functions in order to assess the most basic body functions. * EXTEND FROM: https://schema.org/MedicalSign
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct VitalSign {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<VitalSignOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct VitalSignOptions {
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

    /// The anatomy of the underlying organ system or structures associated with this entity.
    pub associated_anatomy: Option<associatedAnatomy>,

    /// One of a set of differential diagnoses for the condition. Specifically, a closely-related or competing diagnosis typically considered later in the cognitive process whereby this medical condition is distinguished from others most likely responsible for a similar collection of signs and symptoms to reach the most parsimonious diagnosis or diagnoses in a patient.
    pub differential_diagnosis: Option<MedicalCondition>,

    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Option<drug>,

    /// The characteristics of associated patients, such as age, gender, race etc.
    pub epidemiology: Option<epidemiology>,

    /// The likely outcome in either the short term or long term of the medical condition.
    pub expected_prognosis: Option<MedicalCondition>,

    /// The expected progression of the condition if it is not treated and allowed to progress naturally.
    pub natural_progression: Option<MedicalCondition>,

    /// Changes in the normal mechanical, physical, and biochemical functions that are associated with this activity or condition.
    pub pathophysiology: Option<pathophysiology>,

    /// A possible unexpected and unfavorable evolution of a medical condition. Complications may include worsening of the signs or symptoms of the disease, extension of the condition to other organ systems, etc.
    pub possible_complication: Option<MedicalCondition>,

    /// A possible treatment to address this condition, sign or symptom.
    pub possible_treatment: Option<possibleTreatment>,

    /// A preventative therapy used to prevent an initial occurrence of the medical condition, such as vaccination.
    pub primary_prevention: Option<MedicalCondition>,

    /// A modifiable or non-modifiable factor that increases the risk of a patient contracting this condition, e.g. age,  coexisting condition.
    pub risk_factor: Option<MedicalCondition>,

    /// A preventative therapy used to prevent reoccurrence of the medical condition after an initial episode of the condition.
    pub secondary_prevention: Option<MedicalCondition>,

    /// A sign or symptom of this condition. Signs are objective or physically observable manifestations of the medical condition while symptoms are the subjective experience of the medical condition.
    pub sign_or_symptom: Option<MedicalCondition>,

    /// The stage of the condition, if applicable.
    pub stage: Option<MedicalCondition>,

    /// The status of the study (enumerated).
    pub status: Option<status>,

    /// A medical test typically performed given this condition.
    pub typical_test: Option<MedicalCondition>,

    /// A physical examination that can identify this sign.
    pub identifying_exam: Option<MedicalSign>,

    /// A diagnostic test that can identify this sign.
    pub identifying_test: Option<MedicalSign>,
}

impl VitalSign {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
