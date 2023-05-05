use crate::prelude::*;

use super::medical_study::MedicalStudy;
use super::patient::Patient;
use super::people_audience::PeopleAudience;

/// Specifying the health condition(s) of a patient, medical study, or other target audience.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum healthCondition {
    MedicalStudy(MedicalStudy),
    Patient(Patient),
    PeopleAudience(PeopleAudience),
}
