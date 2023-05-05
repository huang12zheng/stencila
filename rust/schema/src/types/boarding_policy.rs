use crate::prelude::*;

use super::airline::Airline;
use super::flight::Flight;

/// The type of boarding policy used by the airline (e.g. zone-based or group-based).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum boardingPolicy {
    Airline(Airline),
    Flight(Flight),
}
