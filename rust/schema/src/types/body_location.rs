use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::medical_procedure::MedicalProcedure;

/// Location in the body of the anatomical structure.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum bodyLocation {
    AnatomicalStructure(AnatomicalStructure),
    MedicalProcedure(MedicalProcedure),
}
