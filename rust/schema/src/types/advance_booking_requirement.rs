use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The amount of time that is required between accepting the offer and the actual usage of the resource or service.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum advanceBookingRequirement {
    Demand(Demand),
    Offer(Offer),
}
