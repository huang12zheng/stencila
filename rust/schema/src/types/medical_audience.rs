// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::administrative_area::AdministrativeArea;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::integer::Integer;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::medical_condition::MedicalCondition;
use super::number::Number;
use super::quantitative_value::QuantitativeValue;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::suggested_gender_prop_enum::SuggestedGenderPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/MedicalAudience
/// * COMMENT:
/// Target audiences for medical web pages.
/// * EXTEND FROM:
/// https://schema.org/Audience, https://schema.org/PeopleAudience
/// * LOOK ALSO:
/// https://schema.org/Patient
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MedicalAudience {
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

    /// The target group associated with a given audience (e.g. veterans, car owners, musicians, etc.).
    pub audience_type_prop_enum: Option<Text>,

    /// The geographic area associated with the audience.
    pub geographic_area_prop_enum: Option<AdministrativeArea>,

    /// Specifying the health condition(s) of a patient, medical study, or other target audience.
    pub health_condition_prop_enum: Option<MedicalCondition>,

    /// Audiences defined by a person's gender.
    pub required_gender_prop_enum: Option<Text>,

    /// Audiences defined by a person's maximum age.
    pub required_max_age_prop_enum: Option<Integer>,

    /// Audiences defined by a person's minimum age.
    pub required_min_age_prop_enum: Option<Integer>,

    /// The age or age range for the intended audience or person, for example 3-12 months for infants, 1-5 years for toddlers.
    pub suggested_age_prop_enum: Option<QuantitativeValue>,

    /// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
    pub suggested_gender_prop_enum: Option<SuggestedGenderPropEnum>,

    /// Maximum recommended age in years for the audience or user.
    pub suggested_max_age_prop_enum: Option<Number>,

    /// A suggested range of body measurements for the intended audience or person, for example inseam between 32 and 34 inches or height between 170 and 190 cm. Typically found on a size chart for wearable products.
    pub suggested_measurement_prop_enum: Option<QuantitativeValue>,

    /// Minimum recommended age in years for the audience or user.
    pub suggested_min_age_prop_enum: Option<Number>,
}

impl MedicalAudience {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
