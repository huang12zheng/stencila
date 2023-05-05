use crate::prelude::*;

use super::lymphatic_vessel::LymphaticVessel;
use super::vein::Vein;

/// The anatomical or organ system drained by this vessel; generally refers to a specific part of an organ.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum regionDrained {
    LymphaticVessel(LymphaticVessel),
    Vein(Vein),
}
