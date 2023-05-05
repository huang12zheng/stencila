use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::physical_activity::PhysicalActivity;

/// The characteristics of associated patients, such as age, gender, race etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum epidemiology {
    MedicalCondition(MedicalCondition),
    PhysicalActivity(PhysicalActivity),
}
