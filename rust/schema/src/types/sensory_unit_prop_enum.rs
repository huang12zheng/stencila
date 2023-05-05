use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::superficial_anatomy::SuperficialAnatomy;


/// http://schema.org/sensoryUnit
/// The neurological pathway extension that inputs and sends information to the brain or spinal cord.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SensoryUnitPropEnum {
    AnatomicalStructure(AnatomicalStructure),
    SuperficialAnatomy(SuperficialAnatomy),
}
