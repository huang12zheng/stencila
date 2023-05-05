use crate::prelude::*;

use super::distance::Distance;
use super::text::Text;

/// The distance of the flight.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum flightDistance {
    Distance(Distance),
    Text(Text),
}
