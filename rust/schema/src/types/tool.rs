use crate::prelude::*;

use super::how_to::HowTo;
use super::how_to_direction::HowToDirection;

/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum tool {
    HowTo(HowTo),
    HowToDirection(HowToDirection),
}
