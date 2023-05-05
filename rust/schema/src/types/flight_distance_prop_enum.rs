use crate::prelude::*;

use super::distance::Distance;
use super::text::Text;


/// http://schema.org/flightDistance
/// The distance of the flight.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FlightDistancePropEnum {
    Distance(Distance),
    Text(Text),
}
