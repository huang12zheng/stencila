// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;

/// * COMMENT: Structured values are used when the value of a property has a more complex structure than simply being a textual value or a reference to another thing. * EXTEND FROM: https://schema.org/Intangible * LOOK ALSO: https://schema.org/CDCPMDRecord, https://schema.org/ContactPoint, https://schema.org/DatedMoneySpecification, https://schema.org/DefinedRegion, https://schema.org/DeliveryTimeSettings, https://schema.org/EngineSpecification, https://schema.org/ExchangeRateSpecification, https://schema.org/GeoCoordinates, https://schema.org/GeoShape, https://schema.org/InteractionCounter, https://schema.org/MonetaryAmount, https://schema.org/NutritionInformation, https://schema.org/OfferShippingDetails, https://schema.org/OpeningHoursSpecification, https://schema.org/OwnershipInfo, https://schema.org/PostalCodeRangeSpecification, https://schema.org/PriceSpecification, https://schema.org/PropertyValue, https://schema.org/QuantitativeValue, https://schema.org/QuantitativeValueDistribution, https://schema.org/RepaymentSpecification, https://schema.org/ShippingDeliveryTime, https://schema.org/ShippingRateSettings, https://schema.org/TypeAndQuantityNode, https://schema.org/WarrantyPromise
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct StructuredValue {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<StructuredValueOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct StructuredValueOptions {
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
}

impl StructuredValue {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
