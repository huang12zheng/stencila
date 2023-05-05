use crate::prelude::*;

use super::text::Text;
use super::vehicle::Vehicle;


/// http://schema.org/aircraft
/// The kind of aircraft (e.g., "Boeing 747").
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AircraftPropEnum {
    Text(Text),
    Vehicle(Vehicle),
}
