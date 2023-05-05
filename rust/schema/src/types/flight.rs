// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::airport::Airport;
use super::boarding_policy_type::BoardingPolicyType;
use super::date_time::DateTime;
use super::text::Text;
use super::trip::Trip;
use super::url::URL;
use super::aircraft::aircraft;
use super::arrival_time::arrivalTime;
use super::departure_time::departureTime;
use super::estimated_flight_duration::estimatedFlightDuration;
use super::flight_distance::flightDistance;
use super::identifier::identifier;
use super::image::image;
use super::itinerary::itinerary;
use super::main_entity_of_page::mainEntityOfPage;
use super::offers::offers;
use super::provider::provider;
use super::seller::seller;
use super::subject_of::subjectOf;

/// * COMMENT: An airline flight. * EXTEND FROM: https://schema.org/Trip
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Flight {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<FlightOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FlightOptions {
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

    /// The expected arrival time.
    pub arrival_time: Option<arrivalTime>,

    /// The expected departure time.
    pub departure_time: Option<departureTime>,

    /// Destination(s) ( <a class="localLink" href="/Place">Place</a> ) that make up a trip. For a trip where destination order is important use <a class="localLink" href="/ItemList">ItemList</a> to specify that order (see examples).
    pub itinerary: Option<itinerary>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers: Option<offers>,

    /// Identifies that this <a class="localLink" href="/Trip">Trip</a> is a subTrip of another Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
    pub part_of_trip: Option<Trip>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// Identifies a <a class="localLink" href="/Trip">Trip</a> that is a subTrip of this Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
    pub sub_trip: Option<Trip>,

    /// The kind of aircraft (e.g., "Boeing 747").
    pub aircraft: Option<aircraft>,

    /// The airport where the flight terminates.
    pub arrival_airport: Option<Airport>,

    /// Identifier of the flight's arrival gate.
    pub arrival_gate: Option<Text>,

    /// Identifier of the flight's arrival terminal.
    pub arrival_terminal: Option<Text>,

    /// The type of boarding policy used by the airline (e.g. zone-based or group-based).
    pub boarding_policy: Option<BoardingPolicyType>,

    /// The airport where the flight originates.
    pub departure_airport: Option<Airport>,

    /// Identifier of the flight's departure gate.
    pub departure_gate: Option<Text>,

    /// Identifier of the flight's departure terminal.
    pub departure_terminal: Option<Text>,

    /// The estimated time the flight will take.
    pub estimated_flight_duration: Option<estimatedFlightDuration>,

    /// The distance of the flight.
    pub flight_distance: Option<flightDistance>,

    /// The unique identifier for a flight including the airline IATA code. For example, if describing United flight 110, where the IATA code for United is 'UA', the flightNumber is 'UA110'.
    pub flight_number: Option<Text>,

    /// Description of the meals that will be provided or available for purchase.
    pub meal_service: Option<Text>,

    /// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
    pub seller: Option<seller>,

    /// The time when a passenger can check into the flight online.
    pub web_checkin_time: Option<DateTime>,
}

impl Flight {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
