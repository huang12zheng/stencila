use crate::prelude::*;

use super::medical_procedure::MedicalProcedure;
use super::medical_test::MedicalTest;
use super::medical_therapy::MedicalTherapy;


/// http://schema.org/availableService
/// A medical service available from this provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AvailableServicePropEnum {
    MedicalProcedure(MedicalProcedure),
    MedicalTest(MedicalTest),
    MedicalTherapy(MedicalTherapy),
}
