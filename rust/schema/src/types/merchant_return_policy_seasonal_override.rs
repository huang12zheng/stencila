// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::end_date::endDate;
use super::merchant_return_days::merchantReturnDays;
use super::return_policy_category::returnPolicyCategory;
use super::start_date::startDate;

/// * MOD OF: https://pending.schema.org * COMMENT: A seasonal override of a return policy, for example used for holidays. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MerchantReturnPolicySeasonalOverride {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MerchantReturnPolicySeasonalOverrideOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MerchantReturnPolicySeasonalOverrideOptions {
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

    /// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub end_date: Option<endDate>,

    /// Specifies either a fixed return date or the number of days (from the delivery date) that a product can be returned. Used when the <a class="localLink" href="/returnPolicyCategory">returnPolicyCategory</a> property is specified as <a class="localLink" href="/MerchantReturnFiniteReturnWindow">MerchantReturnFiniteReturnWindow</a>.
    pub merchant_return_days: Option<merchantReturnDays>,

    /// Specifies an applicable return policy (from an enumeration).
    pub return_policy_category: Option<returnPolicyCategory>,

    /// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub start_date: Option<startDate>,
}

impl MerchantReturnPolicySeasonalOverride {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
