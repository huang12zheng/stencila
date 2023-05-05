use crate::prelude::*;

use super::airline::Airline;
use super::airport::Airport;

/// IATA identifier for an airline or airport.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum iataCode {
    Airline(Airline),
    Airport(Airport),
}
