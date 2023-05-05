// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::property_value::PropertyValue;
use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;
use super::superseded_by::supersededBy;
use super::value_reference::valueReference;

/// * COMMENT: A type of bed. This is used for indicating the bed or beds available in an accommodation. * EXTEND FROM: https://schema.org/QualitativeValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct BedType {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<BedTypeOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct BedTypeOptions {
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

    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: Option<supersededBy>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: Option<PropertyValue>,

    /// This ordering relation for qualitative values indicates that the subject is equal to the object.
    pub equal: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is greater than the object.
    pub greater: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is greater than or equal to the object.
    pub greater_or_equal: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is lesser than the object.
    pub lesser: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is lesser than or equal to the object.
    pub lesser_or_equal: Option<QualitativeValue>,

    /// This ordering relation for qualitative values indicates that the subject is not equal to the object.
    pub non_equal: Option<QualitativeValue>,

    /// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
    pub value_reference: Option<valueReference>,
}

impl BedType {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
