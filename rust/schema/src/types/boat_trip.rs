// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::arrival_time_prop_enum::ArrivalTimePropEnum;
use super::boat_terminal::BoatTerminal;
use super::departure_time_prop_enum::DepartureTimePropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::itinerary_prop_enum::ItineraryPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::offers_prop_enum::OffersPropEnum;
use super::provider_prop_enum::ProviderPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::trip::Trip;
use super::url::URL;

/// https://schema.org/BoatTrip
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// A trip on a commercial ferry line.
/// * EXTEND FROM:
/// https://schema.org/Trip
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct BoatTrip {
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

    /// The expected arrival time.
    pub arrival_time_prop_enum: Option<ArrivalTimePropEnum>,

    /// The expected departure time.
    pub departure_time_prop_enum: Option<DepartureTimePropEnum>,

    /// Destination(s) ( <a class="localLink" href="/Place">Place</a> ) that make up a trip. For a trip where destination order is important use <a class="localLink" href="/ItemList">ItemList</a> to specify that order (see examples).
    pub itinerary_prop_enum: Option<ItineraryPropEnum>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers_prop_enum: Option<OffersPropEnum>,

    /// Identifies that this <a class="localLink" href="/Trip">Trip</a> is a subTrip of another Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
    pub part_of_trip_prop_enum: Option<Trip>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider_prop_enum: Option<ProviderPropEnum>,

    /// Identifies a <a class="localLink" href="/Trip">Trip</a> that is a subTrip of this Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
    pub sub_trip_prop_enum: Option<Trip>,

    /// The terminal or port from which the boat arrives.
    pub arrival_boat_terminal_prop_enum: Option<BoatTerminal>,

    /// The terminal or port from which the boat departs.
    pub departure_boat_terminal_prop_enum: Option<BoatTerminal>,
}

impl BoatTrip {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
