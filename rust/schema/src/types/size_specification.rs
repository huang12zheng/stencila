// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::thing::Thing;
use super::additional_property::additionalProperty;
use super::has_measurement::hasMeasurement;
use super::suggested_age::suggestedAge;
use super::suggested_gender::suggestedGender;
use super::suggested_measurement::suggestedMeasurement;
use super::superseded_by::supersededBy;
use super::value_reference::valueReference;

/// * MOD OF: https://pending.schema.org * COMMENT: Size related properties of a product, typically a size code (<a class="localLink" href="/name">name</a>) and optionally a <a class="localLink" href="/sizeSystem">sizeSystem</a>, <a class="localLink" href="/sizeGroup">sizeGroup</a>, and product measurements (<a class="localLink" href="/hasMeasurement">hasMeasurement</a>). In addition, the intended audience can be defined through <a class="localLink" href="/suggestedAge">suggestedAge</a>, <a class="localLink" href="/suggestedGender">suggestedGender</a>, and suggested body measurements (<a class="localLink" href="/suggestedMeasurement">suggestedMeasurement</a>). * EXTEND FROM: https://schema.org/QualitativeValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct SizeSpecification {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<SizeSpecificationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct SizeSpecificationOptions {
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

    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: Option<supersededBy>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: Option<additionalProperty>,

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

    /// A product measurement, for example the inseam of pants, the wheel size of a bicycle, or the gauge of a screw. Usually an exact measurement, but can also be a range of measurements for adjustable products, for example belts and ski bindings.
    pub has_measurement: Option<hasMeasurement>,

    /// The size group (also known as "size type") for a product's size. Size groups are common in the fashion industry to define size segments and suggested audiences for wearable products. Multiple values can be combined, for example "men's big and tall", "petite maternity" or "regular"
    pub size_group: Option<SizeSpecification>,

    /// The size system used to identify a product's size. Typically either a standard (for example, "GS1" or "ISO-EN13402"), country code (for example "US" or "JP"), or a measuring system (for example "Metric" or "Imperial").
    pub size_system: Option<SizeSpecification>,

    /// The age or age range for the intended audience or person, for example 3-12 months for infants, 1-5 years for toddlers.
    pub suggested_age: Option<suggestedAge>,

    /// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
    pub suggested_gender: Option<suggestedGender>,

    /// A suggested range of body measurements for the intended audience or person, for example inseam between 32 and 34 inches or height between 170 and 190 cm. Typically found on a size chart for wearable products.
    pub suggested_measurement: Option<suggestedMeasurement>,
}

impl SizeSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
