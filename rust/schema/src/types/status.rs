use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::medical_procedure::MedicalProcedure;
use super::medical_study::MedicalStudy;

/// The status of the study (enumerated).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum status {
    MedicalCondition(MedicalCondition),
    MedicalProcedure(MedicalProcedure),
    MedicalStudy(MedicalStudy),
}
