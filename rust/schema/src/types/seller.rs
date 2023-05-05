use crate::prelude::*;

use super::buy_action::BuyAction;
use super::demand::Demand;
use super::flight::Flight;
use super::offer::Offer;
use super::order::Order;

/// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seller {
    BuyAction(BuyAction),
    Demand(Demand),
    Flight(Flight),
    Offer(Offer),
    Order(Order),
}
