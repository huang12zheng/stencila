// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::boolean::Boolean;
use super::defined_region::DefinedRegion;
use super::monetary_amount::MonetaryAmount;
use super::text::Text;
use super::url::URL;
use super::free_shipping_threshold::freeShippingThreshold;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;

/// * MOD OF: https://pending.schema.org * COMMENT: A ShippingRateSettings represents re-usable pieces of shipping information. It is designed for publication on an URL that may be referenced via the <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> property of an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a>. Several occurrences can be published, distinguished and matched (i.e. identified/referenced) by their different values for <a class="localLink" href="/shippingLabel">shippingLabel</a>. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ShippingRateSettings {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ShippingRateSettingsOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ShippingRateSettingsOptions {
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

    /// Indicates when shipping to a particular <a class="localLink" href="/shippingDestination">shippingDestination</a> is not available.
    pub does_not_ship: Option<Boolean>,

    /// A monetary value above (or at) which the shipping rate becomes free. Intended to be used via an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> matching this <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a>.
    pub free_shipping_threshold: Option<freeShippingThreshold>,

    /// This can be marked 'true' to indicate that some published <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a> or <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> are intended to apply to all <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> published by the same merchant, when referenced by a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> in those settings. It is not meaningful to use a 'true' value for this property alongside a transitTimeLabel (for <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a>) or shippingLabel (for <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a>), since this property is for use with unlabelled settings.
    pub is_unlabelled_fallback: Option<Boolean>,

    /// indicates (possibly multiple) shipping destinations. These can be defined in several ways, e.g. postalCode ranges.
    pub shipping_destination: Option<DefinedRegion>,

    /// Label to match an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with a <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> (within the context of a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> cross-reference).
    pub shipping_label: Option<Text>,

    /// The shipping rate is the cost of shipping to the specified destination. Typically, the maxValue and currency values (of the <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>) are most appropriate.
    pub shipping_rate: Option<MonetaryAmount>,
}

impl ShippingRateSettings {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
