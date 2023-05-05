use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;


/// http://schema.org/comprisedOf
/// Specifying something physically contained by something else. Typically used here for the underlying anatomical structures, such as organs, that comprise the anatomical system.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ComprisedOfPropEnum {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
}
