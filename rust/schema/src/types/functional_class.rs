use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::text::Text;

/// The degree of mobility the joint allows.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum functionalClass {
    MedicalEntity(MedicalEntity),
    Text(Text),
}
