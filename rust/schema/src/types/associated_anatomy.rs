use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::physical_activity::PhysicalActivity;

/// The anatomy of the underlying organ system or structures associated with this entity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum associatedAnatomy {
    MedicalCondition(MedicalCondition),
    PhysicalActivity(PhysicalActivity),
}
