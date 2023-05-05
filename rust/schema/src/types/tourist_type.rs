use crate::prelude::*;

use super::tourist_attraction::TouristAttraction;
use super::tourist_destination::TouristDestination;
use super::tourist_trip::TouristTrip;

/// Attraction suitable for type(s) of tourist. E.g. children, visitors from a particular country, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum touristType {
    TouristAttraction(TouristAttraction),
    TouristDestination(TouristDestination),
    TouristTrip(TouristTrip),
}
