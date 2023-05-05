use crate::prelude::*;

use super::medical_procedure::MedicalProcedure;
use super::medical_test::MedicalTest;
use super::medical_therapy::MedicalTherapy;

/// A medical service available from this provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availableService {
    MedicalProcedure(MedicalProcedure),
    MedicalTest(MedicalTest),
    MedicalTherapy(MedicalTherapy),
}
