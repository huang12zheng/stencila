use crate::prelude::*;

use super::reservation::Reservation;
use super::ticket::Ticket;

/// The person or organization the reservation or ticket is for.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum underName {
    Reservation(Reservation),
    Ticket(Ticket),
}
