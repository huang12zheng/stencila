// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::day_of_week::dayOfWeek;
use super::valid_from::validFrom;
use super::valid_through::validThrough;

/// * COMMENT: A structured value providing information about the opening hours of a place or a certain service inside a place.<br/><br/>  The place is <strong>open</strong> if the <a class="localLink" href="/opens">opens</a> property is specified, and <strong>closed</strong> otherwise.<br/><br/>  If the value for the <a class="localLink" href="/closes">closes</a> property is less than the value for the <a class="localLink" href="/opens">opens</a> property then the hour range is assumed to span over the next day. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OpeningHoursSpecification {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<OpeningHoursSpecificationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OpeningHoursSpecificationOptions {
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

    /// The closing hour of the place or service on the given day(s) of the week.
    pub closes: Option<OpeningHoursSpecification>,

    /// The day of the week for which these opening hours are valid.
    pub day_of_week: Option<dayOfWeek>,

    /// The opening hour of the place or service on the given day(s) of the week.
    pub opens: Option<OpeningHoursSpecification>,

    /// The date when the item becomes valid.
    pub valid_from: Option<validFrom>,

    /// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
    pub valid_through: Option<validThrough>,
}

impl OpeningHoursSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
