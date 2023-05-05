use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::superficial_anatomy::SuperficialAnatomy;

/// The neurological pathway extension that inputs and sends information to the brain or spinal cord.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sensoryUnit {
    AnatomicalStructure(AnatomicalStructure),
    SuperficialAnatomy(SuperficialAnatomy),
}
