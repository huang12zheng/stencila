use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;
use super::superficial_anatomy::SuperficialAnatomy;


/// http://schema.org/associatedAnatomy
/// The anatomy of the underlying organ system or structures associated with this entity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AssociatedAnatomyPropEnum {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    SuperficialAnatomy(SuperficialAnatomy),
}
