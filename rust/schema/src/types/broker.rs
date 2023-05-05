use crate::prelude::*;

use super::invoice::Invoice;
use super::order::Order;
use super::reservation::Reservation;
use super::service::Service;

/// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum broker {
    Invoice(Invoice),
    Order(Order),
    Reservation(Reservation),
    Service(Service),
}
