// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::date_time::DateTime;
use super::program_membership::ProgramMembership;
use super::reservation_status_type::ReservationStatusType;
use super::text::Text;
use super::thing::Thing;
use super::ticket::Ticket;
use super::url::URL;
use super::broker::broker;
use super::checkin_time::checkinTime;
use super::checkout_time::checkoutTime;
use super::identifier::identifier;
use super::image::image;
use super::lodging_unit_type::lodgingUnitType;
use super::main_entity_of_page::mainEntityOfPage;
use super::num_adults::numAdults;
use super::num_children::numChildren;
use super::provider::provider;
use super::subject_of::subjectOf;
use super::total_price::totalPrice;
use super::under_name::underName;

/// * COMMENT: A reservation for lodging at a hotel, motel, inn, etc.<br/><br/>  Note: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. * EXTEND FROM: https://schema.org/Reservation
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct LodgingReservation {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<LodgingReservationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct LodgingReservationOptions {
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

    /// The date and time the reservation was booked.
    pub booking_time: Option<DateTime>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker: Option<broker>,

    /// The date and time the reservation was modified.
    pub modified_time: Option<DateTime>,

    /// The currency of the price, or a price component when attached to <a class="localLink" href="/PriceSpecification">PriceSpecification</a> and its subtypes.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub price_currency: Option<Text>,

    /// Any membership in a frequent flyer, hotel loyalty program, etc. being applied to the reservation.
    pub program_membership_used: Option<ProgramMembership>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// The thing -- flight, event, restaurant, etc. being reserved.
    pub reservation_for: Option<Thing>,

    /// A unique identifier for the reservation.
    pub reservation_id: Option<Text>,

    /// The current status of the reservation.
    pub reservation_status: Option<ReservationStatusType>,

    /// A ticket associated with the reservation.
    pub reserved_ticket: Option<Ticket>,

    /// The total price for the reservation or ticket, including applicable taxes, shipping, etc.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub total_price: Option<totalPrice>,

    /// The person or organization the reservation or ticket is for.
    pub under_name: Option<underName>,

    /// The earliest someone may check into a lodging establishment.
    pub checkin_time: Option<checkinTime>,

    /// The latest someone may check out of a lodging establishment.
    pub checkout_time: Option<checkoutTime>,

    /// A full description of the lodging unit.
    pub lodging_unit_description: Option<Text>,

    /// Textual description of the unit type (including suite vs. room, size of bed, etc.).
    pub lodging_unit_type: Option<lodgingUnitType>,

    /// The number of adults staying in the unit.
    pub num_adults: Option<numAdults>,

    /// The number of children staying in the unit.
    pub num_children: Option<numChildren>,
}

impl LodgingReservation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
