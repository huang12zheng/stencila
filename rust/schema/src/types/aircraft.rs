use crate::prelude::*;

use super::text::Text;
use super::vehicle::Vehicle;

/// The kind of aircraft (e.g., "Boeing 747").
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum aircraft {
    Text(Text),
    Vehicle(Vehicle),
}
