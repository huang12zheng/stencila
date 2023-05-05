// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::address_country_prop_enum::AddressCountryPropEnum;
use super::area_served_prop_enum::AreaServedPropEnum;
use super::available_language_prop_enum::AvailableLanguagePropEnum;
use super::contact_point_option::ContactPointOption;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::product_supported_prop_enum::ProductSupportedPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/PostalAddress
/// * COMMENT:
/// The mailing address.
/// * EXTEND FROM:
/// https://schema.org/ContactPoint
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PostalAddress {
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

    /// The geographic area where a service or offered item is provided.
    pub area_served_prop_enum: Option<AreaServedPropEnum>,

    /// A language someone may use with or at the item, service or place. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/inLanguage">inLanguage</a>.
    pub available_language_prop_enum: Option<AvailableLanguagePropEnum>,

    /// An option available on this contact point (e.g. a toll-free number or support for hearing-impaired callers).
    pub contact_option_prop_enum: Option<ContactPointOption>,

    /// A person or organization can have different contact points, for different purposes. For example, a sales contact point, a PR contact point and so on. This property is used to specify the kind of contact point.
    pub contact_type_prop_enum: Option<Text>,

    /// Email address.
    pub email_prop_enum: Option<Text>,

    /// The fax number.
    pub fax_number_prop_enum: Option<Text>,

    /// The hours during which this service or contact is available.
    pub hours_available_prop_enum: Option<OpeningHoursSpecification>,

    /// The product or service this support contact point is related to (such as product support for a particular product line). This can be a specific product or product line (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
    pub product_supported_prop_enum: Option<ProductSupportedPropEnum>,

    /// The telephone number.
    pub telephone_prop_enum: Option<Text>,

    /// The country. For example, USA. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>.
    pub address_country_prop_enum: Option<AddressCountryPropEnum>,

    /// The locality in which the street address is, and which is in the region. For example, Mountain View.
    pub address_locality_prop_enum: Option<Text>,

    /// The region in which the locality is, and which is in the country. For example, California or another appropriate first-level <a href="https://en.wikipedia.org/wiki/List_of_administrative_divisions_by_country">Administrative division</a>.
    pub address_region_prop_enum: Option<Text>,

    /// The post office box number for PO box addresses.
    pub post_office_box_number_prop_enum: Option<Text>,

    /// The postal code. For example, 94043.
    pub postal_code_prop_enum: Option<Text>,

    /// The street address. For example, 1600 Amphitheatre Pkwy.
    pub street_address_prop_enum: Option<Text>,
}

impl PostalAddress {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
