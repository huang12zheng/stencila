use crate::prelude::*;

use super::how_to::HowTo;
use super::how_to_direction::HowToDirection;

/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum supply {
    HowTo(HowTo),
    HowToDirection(HowToDirection),
}
