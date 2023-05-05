// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::availability_ends::availabilityEnds;
use super::availability_starts::availabilityStarts;
use super::category::category;
use super::eligible_region::eligibleRegion;
use super::expects_acceptance_of::expectsAcceptanceOf;
use super::ineligible_region::ineligibleRegion;
use super::requires_subscription::requiresSubscription;

/// * COMMENT: A set of requirements that must be fulfilled in order to perform an Action. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ActionAccessSpecification {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ActionAccessSpecificationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ActionAccessSpecificationOptions {
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

    /// The end of the availability of the product or service included in the offer.
    pub availability_ends: Option<availabilityEnds>,

    /// The beginning of the availability of the product or service included in the offer.
    pub availability_starts: Option<availabilityStarts>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<category>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.<br/><br/>  See also <a class="localLink" href="/ineligibleRegion">ineligibleRegion</a>.
    pub eligible_region: Option<eligibleRegion>,

    /// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
    pub expects_acceptance_of: Option<expectsAcceptanceOf>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
    pub ineligible_region: Option<ineligibleRegion>,

    /// Indicates if use of the media require a subscription  (either paid or free). Allowed values are <code>true</code> or <code>false</code> (note that an earlier version had 'yes', 'no').
    pub requires_subscription: Option<requiresSubscription>,
}

impl ActionAccessSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
