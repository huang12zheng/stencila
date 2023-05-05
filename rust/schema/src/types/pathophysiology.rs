use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::physical_activity::PhysicalActivity;

/// Changes in the normal mechanical, physical, and biochemical functions that are associated with this activity or condition.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pathophysiology {
    MedicalCondition(MedicalCondition),
    PhysicalActivity(PhysicalActivity),
}
