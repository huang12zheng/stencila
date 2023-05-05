use crate::prelude::*;

use super::offer::Offer;
use super::price_specification::PriceSpecification;
use super::reservation::Reservation;
use super::ticket::Ticket;
use super::trade_action::TradeAction;

/// The currency of the price, or a price component when attached to <a class="localLink" href="/PriceSpecification">PriceSpecification</a> and its subtypes.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum priceCurrency {
    Offer(Offer),
    PriceSpecification(PriceSpecification),
    Reservation(Reservation),
    Ticket(Ticket),
    TradeAction(TradeAction),
}
