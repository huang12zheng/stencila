use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;
use super::superficial_anatomy::SuperficialAnatomy;

/// A medical condition associated with this anatomy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum relatedCondition {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    SuperficialAnatomy(SuperficialAnatomy),
}
