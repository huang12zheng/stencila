use crate::prelude::*;

use super::how_to::HowTo;
use super::how_to_supply::HowToSupply;

/// The estimated cost of the supply or supplies consumed when performing instructions.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum estimatedCost {
    HowTo(HowTo),
    HowToSupply(HowToSupply),
}
