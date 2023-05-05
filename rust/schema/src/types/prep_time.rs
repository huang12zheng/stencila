use crate::prelude::*;

use super::how_to::HowTo;
use super::how_to_direction::HowToDirection;

/// The length of time it takes to prepare the items to be used in instructions or a direction, in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 duration format</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum prepTime {
    HowTo(HowTo),
    HowToDirection(HowToDirection),
}
