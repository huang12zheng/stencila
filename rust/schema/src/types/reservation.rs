// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::broker_prop_enum::BrokerPropEnum;
use super::date_time::DateTime;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::program_membership::ProgramMembership;
use super::provider_prop_enum::ProviderPropEnum;
use super::reservation_status_type::ReservationStatusType;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::thing::Thing;
use super::ticket::Ticket;
use super::total_price_prop_enum::TotalPricePropEnum;
use super::url::URL;
use super::under_name_prop_enum::UnderNamePropEnum;

/// https://schema.org/Reservation
/// * COMMENT:
/// Describes a reservation for travel, dining or an event. Some reservations require tickets. <br/><br/>
/// 
/// Note: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, restaurant reservations, flights, or rental cars, use <a class="localLink" href="/Offer">Offer</a>.
/// * EXTEND FROM:
/// https://schema.org/Intangible
/// * LOOK ALSO:
/// https://schema.org/BoatReservation, https://schema.org/BusReservation, https://schema.org/EventReservation, https://schema.org/FlightReservation, https://schema.org/FoodEstablishmentReservation, https://schema.org/LodgingReservation, https://schema.org/RentalCarReservation, https://schema.org/ReservationPackage, https://schema.org/TaxiReservation, https://schema.org/TrainReservation
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Reservation {
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

    /// The date and time the reservation was booked.
    pub booking_time_prop_enum: Option<DateTime>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker_prop_enum: Option<BrokerPropEnum>,

    /// The date and time the reservation was modified.
    pub modified_time_prop_enum: Option<DateTime>,

    /// The currency of the price, or a price component when attached to <a class="localLink" href="/PriceSpecification">PriceSpecification</a> and its subtypes.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub price_currency_prop_enum: Option<Text>,

    /// Any membership in a frequent flyer, hotel loyalty program, etc. being applied to the reservation.
    pub program_membership_used_prop_enum: Option<ProgramMembership>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider_prop_enum: Option<ProviderPropEnum>,

    /// The thing -- flight, event, restaurant, etc. being reserved.
    pub reservation_for_prop_enum: Option<Thing>,

    /// A unique identifier for the reservation.
    pub reservation_id_prop_enum: Option<Text>,

    /// The current status of the reservation.
    pub reservation_status_prop_enum: Option<ReservationStatusType>,

    /// A ticket associated with the reservation.
    pub reserved_ticket_prop_enum: Option<Ticket>,

    /// The total price for the reservation or ticket, including applicable taxes, shipping, etc.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub total_price_prop_enum: Option<TotalPricePropEnum>,

    /// The person or organization the reservation or ticket is for.
    pub under_name_prop_enum: Option<UnderNamePropEnum>,
}

impl Reservation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
