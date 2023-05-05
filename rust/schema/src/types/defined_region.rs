// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::address_country_prop_enum::AddressCountryPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::postal_code_range_specification::PostalCodeRangeSpecification;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/DefinedRegion
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// A DefinedRegion is a geographic area defined by potentially arbitrary (rather than political, administrative or natural geographical) criteria. Properties are provided for defining a region by reference to sets of postal codes.<br/><br/>
/// 
/// Examples: a delivery destination when shopping. Region where regional pricing is configured.<br/><br/>
/// 
/// Requirement 1:
/// Country: US
/// States: "NY", "CA"<br/><br/>
/// 
/// Requirement 2:
/// Country: US
/// PostalCode Set: { [94000-94585], [97000, 97999], [13000, 13599]}
/// { [12345, 12345], [78945, 78945], }
/// Region = state, canton, prefecture, autonomous community...
/// * EXTEND FROM:
/// https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct DefinedRegion {
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

    /// The country. For example, USA. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>.
    pub address_country_prop_enum: Option<AddressCountryPropEnum>,

    /// The region in which the locality is, and which is in the country. For example, California or another appropriate first-level <a href="https://en.wikipedia.org/wiki/List_of_administrative_divisions_by_country">Administrative division</a>.
    pub address_region_prop_enum: Option<Text>,

    /// The postal code. For example, 94043.
    pub postal_code_prop_enum: Option<Text>,

    /// A defined range of postal codes indicated by a common textual prefix. Used for non-numeric systems such as UK.
    pub postal_code_prefix_prop_enum: Option<Text>,

    /// A defined range of postal codes.
    pub postal_code_range_prop_enum: Option<PostalCodeRangeSpecification>,
}

impl DefinedRegion {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
