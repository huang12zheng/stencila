// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::availability_ends_prop_enum::AvailabilityEndsPropEnum;
use super::availability_starts_prop_enum::AvailabilityStartsPropEnum;
use super::category_prop_enum::CategoryPropEnum;
use super::eligible_region_prop_enum::EligibleRegionPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::ineligible_region_prop_enum::IneligibleRegionPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::offer::Offer;
use super::requires_subscription_prop_enum::RequiresSubscriptionPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/ActionAccessSpecification
/// * COMMENT:
/// A set of requirements that must be fulfilled in order to perform an Action.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ActionAccessSpecification {
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

    /// The end of the availability of the product or service included in the offer.
    pub availability_ends_prop_enum: Option<AvailabilityEndsPropEnum>,

    /// The beginning of the availability of the product or service included in the offer.
    pub availability_starts_prop_enum: Option<AvailabilityStartsPropEnum>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category_prop_enum: Option<CategoryPropEnum>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.<br/><br/>  See also <a class="localLink" href="/ineligibleRegion">ineligibleRegion</a>.
    pub eligible_region_prop_enum: Option<EligibleRegionPropEnum>,

    /// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
    pub expects_acceptance_of_prop_enum: Option<Offer>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
    pub ineligible_region_prop_enum: Option<IneligibleRegionPropEnum>,

    /// Indicates if use of the media require a subscription  (either paid or free). Allowed values are <code>true</code> or <code>false</code> (note that an earlier version had 'yes', 'no').
    pub requires_subscription_prop_enum: Option<RequiresSubscriptionPropEnum>,
}

impl ActionAccessSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
