use crate::prelude::*;

use super::hospital::Hospital;
use super::medical_clinic::MedicalClinic;
use super::physician::Physician;

/// A medical service available from this provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availableService {
    Hospital(Hospital),
    MedicalClinic(MedicalClinic),
    Physician(Physician),
}
