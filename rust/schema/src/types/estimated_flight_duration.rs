use crate::prelude::*;

use super::duration::Duration;
use super::text::Text;

/// The estimated time the flight will take.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum estimatedFlightDuration {
    Duration(Duration),
    Text(Text),
}
