// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::property_value::PropertyValue;
use super::qualitative_value::QualitativeValue;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::superseded_by_prop_enum::SupersededByPropEnum;
use super::text::Text;
use super::url::URL;
use super::value_reference_prop_enum::ValueReferencePropEnum;

/// https://schema.org/AllWheelDriveConfiguration
/// * COMMENT:
/// All-wheel Drive is a transmission layout where the engine drives all four wheels.
/// * EXTEND FROM:
/// https://schema.org/DriveWheelConfigurationValue
/// * ENUMERATION FROM: https://schema.org/DriveWheelConfigurationValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct AllWheelDriveConfiguration {
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

    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by_prop_enum: Option<SupersededByPropEnum>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property_prop_enum: Option<PropertyValue>,

    /// This ordering relation for qualitative values indicates that the subject is equal to the object.
    pub equal_prop_enum: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is greater than the object.
    pub greater_prop_enum: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is greater than or equal to the object.
    pub greater_or_equal_prop_enum: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is lesser than the object.
    pub lesser_prop_enum: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is lesser than or equal to the object.
    pub lesser_or_equal_prop_enum: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is not equal to the object.
    pub non_equal_prop_enum: Option<QualitativeValue>,

    /// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
    pub value_reference_prop_enum: Option<ValueReferencePropEnum>,
}

impl AllWheelDriveConfiguration {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
