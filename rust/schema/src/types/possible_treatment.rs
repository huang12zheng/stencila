use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::medical_sign_or_symptom::MedicalSignOrSymptom;

/// A possible treatment to address this condition, sign or symptom.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum possibleTreatment {
    MedicalCondition(MedicalCondition),
    MedicalSignOrSymptom(MedicalSignOrSymptom),
}
