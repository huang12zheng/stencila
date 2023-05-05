// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::boolean::Boolean;
use super::defined_region::DefinedRegion;
use super::monetary_amount::MonetaryAmount;
use super::quantitative_value::QuantitativeValue;
use super::shipping_delivery_time::ShippingDeliveryTime;
use super::text::Text;
use super::url::URL;
use super::depth::depth;
use super::height::height;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;
use super::width::width;

/// * MOD OF: https://pending.schema.org * COMMENT: OfferShippingDetails represents information about shipping destinations.<br/><br/>  Multiple of these entities can be used to represent different shipping rates for different destinations:<br/><br/>  One entity for Alaska/Hawaii. A different one for continental US. A different one for all France.<br/><br/>  Multiple of these entities can be used to represent different shipping costs and delivery times.<br/><br/>  Two entities that are identical but differ in rate and time:<br/><br/>  E.g. Cheaper and slower: $5 in 5-7 days or Fast and expensive: $15 in 1-2 days. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OfferShippingDetails {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<OfferShippingDetailsOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OfferShippingDetailsOptions {
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

    /// The total delay between the receipt of the order and the goods reaching the final customer.
    pub delivery_time: Option<ShippingDeliveryTime>,

    /// The depth of the item.
    pub depth: Option<depth>,

    /// Indicates when shipping to a particular <a class="localLink" href="/shippingDestination">shippingDestination</a> is not available.
    pub does_not_ship: Option<Boolean>,

    /// The height of the item.
    pub height: Option<height>,

    /// indicates (possibly multiple) shipping destinations. These can be defined in several ways, e.g. postalCode ranges.
    pub shipping_destination: Option<DefinedRegion>,

    /// Label to match an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with a <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> (within the context of a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> cross-reference).
    pub shipping_label: Option<Text>,

    /// Indicates the origin of a shipment, i.e. where it should be coming from.
    pub shipping_origin: Option<DefinedRegion>,

    /// The shipping rate is the cost of shipping to the specified destination. Typically, the maxValue and currency values (of the <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>) are most appropriate.
    pub shipping_rate: Option<MonetaryAmount>,

    /// Link to a page containing <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> and <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a> details.
    pub shipping_settings_link: Option<URL>,

    /// Label to match an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with a <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a> (within the context of a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> cross-reference).
    pub transit_time_label: Option<Text>,

    /// The weight of the product or person.
    pub weight: Option<QuantitativeValue>,

    /// The width of the item.
    pub width: Option<width>,
}

impl OfferShippingDetails {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
