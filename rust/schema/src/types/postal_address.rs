// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::thing::Thing;
use super::address_country::addressCountry;
use super::address_region::addressRegion;
use super::area_served::areaServed;
use super::available_language::availableLanguage;
use super::email::email;
use super::fax_number::faxNumber;
use super::hours_available::hoursAvailable;
use super::postal_code::postalCode;
use super::telephone::telephone;

/// * COMMENT: The mailing address. * EXTEND FROM: https://schema.org/ContactPoint
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PostalAddress {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<PostalAddressOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PostalAddressOptions {
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

    /// The geographic area where a service or offered item is provided.
    pub area_served: Option<areaServed>,

    /// A language someone may use with or at the item, service or place. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/inLanguage">inLanguage</a>.
    pub available_language: Option<availableLanguage>,

    /// An option available on this contact point (e.g. a toll-free number or support for hearing-impaired callers).
    pub contact_option: Option<ContactPoint>,

    /// A person or organization can have different contact points, for different purposes. For example, a sales contact point, a PR contact point and so on. This property is used to specify the kind of contact point.
    pub contact_type: Option<ContactPoint>,

    /// Email address.
    pub email: Option<email>,

    /// The fax number.
    pub fax_number: Option<faxNumber>,

    /// The hours during which this service or contact is available.
    pub hours_available: Option<hoursAvailable>,

    /// The product or service this support contact point is related to (such as product support for a particular product line). This can be a specific product or product line (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
    pub product_supported: Option<ContactPoint>,

    /// The telephone number.
    pub telephone: Option<telephone>,

    /// The country. For example, USA. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>.
    pub address_country: Option<addressCountry>,

    /// The locality in which the street address is, and which is in the region. For example, Mountain View.
    pub address_locality: Option<PostalAddress>,

    /// The region in which the locality is, and which is in the country. For example, California or another appropriate first-level <a href="https://en.wikipedia.org/wiki/List_of_administrative_divisions_by_country">Administrative division</a>.
    pub address_region: Option<addressRegion>,

    /// The post office box number for PO box addresses.
    pub post_office_box_number: Option<PostalAddress>,

    /// The postal code. For example, 94043.
    pub postal_code: Option<postalCode>,

    /// The street address. For example, 1600 Amphitheatre Pkwy.
    pub street_address: Option<PostalAddress>,
}

impl PostalAddress {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
