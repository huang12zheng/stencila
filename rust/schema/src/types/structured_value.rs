// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/StructuredValue
/// * COMMENT:
/// Structured values are used when the value of a property has a more complex structure than simply being a textual value or a reference to another thing.
/// * EXTEND FROM:
/// https://schema.org/Intangible
/// * LOOK ALSO:
/// https://schema.org/CDCPMDRecord, https://schema.org/ContactPoint, https://schema.org/DatedMoneySpecification, https://schema.org/DefinedRegion, https://schema.org/DeliveryTimeSettings, https://schema.org/EngineSpecification, https://schema.org/ExchangeRateSpecification, https://schema.org/GeoCoordinates, https://schema.org/GeoShape, https://schema.org/InteractionCounter, https://schema.org/MonetaryAmount, https://schema.org/NutritionInformation, https://schema.org/OfferShippingDetails, https://schema.org/OpeningHoursSpecification, https://schema.org/OwnershipInfo, https://schema.org/PostalCodeRangeSpecification, https://schema.org/PriceSpecification, https://schema.org/PropertyValue, https://schema.org/QuantitativeValue, https://schema.org/QuantitativeValueDistribution, https://schema.org/RepaymentSpecification, https://schema.org/ShippingDeliveryTime, https://schema.org/ShippingRateSettings, https://schema.org/TypeAndQuantityNode, https://schema.org/WarrantyPromise
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct StructuredValue {
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
}

impl StructuredValue {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
