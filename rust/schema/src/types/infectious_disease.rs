// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::d_dx_element::DDxElement;
use super::drug::Drug;
use super::grant::Grant;
use super::infectious_agent_class::InfectiousAgentClass;
use super::medical_code::MedicalCode;
use super::medical_condition_stage::MedicalConditionStage;
use super::medical_guideline::MedicalGuideline;
use super::medical_risk_factor::MedicalRiskFactor;
use super::medical_sign_or_symptom::MedicalSignOrSymptom;
use super::medical_specialty::MedicalSpecialty;
use super::medical_study::MedicalStudy;
use super::medical_test::MedicalTest;
use super::medical_therapy::MedicalTherapy;
use super::medicine_system::MedicineSystem;
use super::organization::Organization;
use super::text::Text;
use super::url::URL;
use super::associated_anatomy::associatedAnatomy;
use super::identifier::identifier;
use super::image::image;
use super::legal_status::legalStatus;
use super::main_entity_of_page::mainEntityOfPage;
use super::status::status;
use super::subject_of::subjectOf;

/// * COMMENT: An infectious disease is a clinically evident human disease resulting from the presence of pathogenic microbial agents, like pathogenic viruses, pathogenic bacteria, fungi, protozoa, multicellular parasites, and prions. To be considered an infectious disease, such pathogens are known to be able to cause this disease. * EXTEND FROM: https://schema.org/MedicalCondition
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct InfectiousDisease {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<InfectiousDiseaseOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct InfectiousDiseaseOptions {
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

    /// The anatomy of the underlying organ system or structures associated with this entity.
    pub associated_anatomy: Option<associatedAnatomy>,

    /// One of a set of differential diagnoses for the condition. Specifically, a closely-related or competing diagnosis typically considered later in the cognitive process whereby this medical condition is distinguished from others most likely responsible for a similar collection of signs and symptoms to reach the most parsimonious diagnosis or diagnoses in a patient.
    pub differential_diagnosis: Option<DDxElement>,

    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Option<Drug>,

    /// The characteristics of associated patients, such as age, gender, race etc.
    pub epidemiology: Option<Text>,

    /// The likely outcome in either the short term or long term of the medical condition.
    pub expected_prognosis: Option<Text>,

    /// The expected progression of the condition if it is not treated and allowed to progress naturally.
    pub natural_progression: Option<Text>,

    /// Changes in the normal mechanical, physical, and biochemical functions that are associated with this activity or condition.
    pub pathophysiology: Option<Text>,

    /// A possible unexpected and unfavorable evolution of a medical condition. Complications may include worsening of the signs or symptoms of the disease, extension of the condition to other organ systems, etc.
    pub possible_complication: Option<Text>,

    /// A possible treatment to address this condition, sign or symptom.
    pub possible_treatment: Option<MedicalTherapy>,

    /// A preventative therapy used to prevent an initial occurrence of the medical condition, such as vaccination.
    pub primary_prevention: Option<MedicalTherapy>,

    /// A modifiable or non-modifiable factor that increases the risk of a patient contracting this condition, e.g. age,  coexisting condition.
    pub risk_factor: Option<MedicalRiskFactor>,

    /// A preventative therapy used to prevent reoccurrence of the medical condition after an initial episode of the condition.
    pub secondary_prevention: Option<MedicalTherapy>,

    /// A sign or symptom of this condition. Signs are objective or physically observable manifestations of the medical condition while symptoms are the subjective experience of the medical condition.
    pub sign_or_symptom: Option<MedicalSignOrSymptom>,

    /// The stage of the condition, if applicable.
    pub stage: Option<MedicalConditionStage>,

    /// The status of the study (enumerated).
    pub status: Option<status>,

    /// A medical test typically performed given this condition.
    pub typical_test: Option<MedicalTest>,

    /// The actual infectious agent, such as a specific bacterium.
    pub infectious_agent: Option<Text>,

    /// The class of infectious agent (bacteria, prion, etc.) that causes the disease.
    pub infectious_agent_class: Option<InfectiousAgentClass>,

    /// How the disease spreads, either as a route or vector, for example 'direct contact', 'Aedes aegypti', etc.
    pub transmission_method: Option<Text>,
}

impl InfectiousDisease {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
