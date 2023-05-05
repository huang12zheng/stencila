use crate::prelude::*;

use super::medical_device::MedicalDevice;
use super::medical_therapy::MedicalTherapy;

/// A possible serious complication and/or serious side effect of this therapy. Serious adverse outcomes include those that are life-threatening; result in death, disability, or permanent damage; require hospitalization or prolong existing hospitalization; cause congenital anomalies or birth defects; or jeopardize the patient and may require medical or surgical intervention to prevent one of the outcomes in this definition.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seriousAdverseOutcome {
    MedicalDevice(MedicalDevice),
    MedicalTherapy(MedicalTherapy),
}
