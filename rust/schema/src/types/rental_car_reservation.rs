// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::reservation::Reservation;
use super::thing::Thing;
use super::broker::broker;
use super::pickup_location::pickupLocation;
use super::pickup_time::pickupTime;
use super::price_currency::priceCurrency;
use super::provider::provider;
use super::total_price::totalPrice;
use super::under_name::underName;

/// * COMMENT: A reservation for a rental car.<br/><br/>  Note: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. * EXTEND FROM: https://schema.org/Reservation
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct RentalCarReservation {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<RentalCarReservationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct RentalCarReservationOptions {
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

    /// The date and time the reservation was booked.
    pub booking_time: Option<Reservation>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker: Option<broker>,

    /// The date and time the reservation was modified.
    pub modified_time: Option<Reservation>,

    /// The currency of the price, or a price component when attached to <a class="localLink" href="/PriceSpecification">PriceSpecification</a> and its subtypes.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub price_currency: Option<priceCurrency>,

    /// Any membership in a frequent flyer, hotel loyalty program, etc. being applied to the reservation.
    pub program_membership_used: Option<Reservation>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// The thing -- flight, event, restaurant, etc. being reserved.
    pub reservation_for: Option<Reservation>,

    /// A unique identifier for the reservation.
    pub reservation_id: Option<Reservation>,

    /// The current status of the reservation.
    pub reservation_status: Option<Reservation>,

    /// A ticket associated with the reservation.
    pub reserved_ticket: Option<Reservation>,

    /// The total price for the reservation or ticket, including applicable taxes, shipping, etc.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub total_price: Option<totalPrice>,

    /// The person or organization the reservation or ticket is for.
    pub under_name: Option<underName>,

    /// Where a rental car can be dropped off.
    pub dropoff_location: Option<RentalCarReservation>,

    /// When a rental car can be dropped off.
    pub dropoff_time: Option<RentalCarReservation>,

    /// Where a taxi will pick up a passenger or a rental car can be picked up.
    pub pickup_location: Option<pickupLocation>,

    /// When a taxi will pick up a passenger or a rental car can be picked up.
    pub pickup_time: Option<pickupTime>,
}

impl RentalCarReservation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
