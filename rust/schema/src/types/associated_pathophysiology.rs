use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;
use super::superficial_anatomy::SuperficialAnatomy;

/// If applicable, a description of the pathophysiology associated with the anatomical system, including potential abnormal changes in the mechanical, physical, and biochemical functions of the system.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum associatedPathophysiology {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    SuperficialAnatomy(SuperficialAnatomy),
}
