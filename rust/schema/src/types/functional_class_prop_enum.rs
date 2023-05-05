use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::text::Text;


/// http://schema.org/functionalClass
/// The degree of mobility the joint allows.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FunctionalClassPropEnum {
    MedicalEntity(MedicalEntity),
    Text(Text),
}
