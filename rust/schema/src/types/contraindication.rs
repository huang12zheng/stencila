use crate::prelude::*;

use super::medical_device::MedicalDevice;
use super::medical_therapy::MedicalTherapy;

/// A contraindication for this therapy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contraindication {
    MedicalDevice(MedicalDevice),
    MedicalTherapy(MedicalTherapy),
}
