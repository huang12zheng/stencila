// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::currency::currency;
use super::max_value::maxValue;
use super::min_value::minValue;
use super::valid_from::validFrom;
use super::valid_through::validThrough;
use super::value::value;

/// * COMMENT: A monetary value or range. This type can be used to describe an amount of money such as $50 USD, or a range as in describing a bank account being suitable for a balance between £1,000 and £1,000,000 GBP, or the value of a salary, etc. It is recommended to use <a class="localLink" href="/PriceSpecification">PriceSpecification</a> Types to describe the price of an Offer, Invoice, etc. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MonetaryAmount {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MonetaryAmountOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MonetaryAmountOptions {
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

    /// The currency in which the monetary amount is expressed.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub currency: Option<currency>,

    /// The upper value of some characteristic or property.
    pub max_value: Option<maxValue>,

    /// The lower value of some characteristic or property.
    pub min_value: Option<minValue>,

    /// The date when the item becomes valid.
    pub valid_from: Option<validFrom>,

    /// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
    pub valid_through: Option<validThrough>,

    /// The value of the quantitative value or property value node.<br/><br/>  <ul> <li>For <a class="localLink" href="/QuantitativeValue">QuantitativeValue</a> and <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>, the recommended type for values is 'Number'.</li> <li>For <a class="localLink" href="/PropertyValue">PropertyValue</a>, it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.</li> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub value: Option<value>,
}

impl MonetaryAmount {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
