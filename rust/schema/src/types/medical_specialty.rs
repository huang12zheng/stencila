use crate::prelude::*;

use super::hospital::Hospital;
use super::medical_clinic::MedicalClinic;
use super::medical_organization::MedicalOrganization;
use super::physician::Physician;

/// A medical specialty of the provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum medicalSpecialty {
    Hospital(Hospital),
    MedicalClinic(MedicalClinic),
    MedicalOrganization(MedicalOrganization),
    Physician(Physician),
}
