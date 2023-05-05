use crate::prelude::*;

use super::medical_device::MedicalDevice;
use super::therapeutic_procedure::TherapeuticProcedure;

/// A possible complication and/or side effect of this therapy. If it is known that an adverse outcome is serious (resulting in death, disability, or permanent damage; requiring hospitalization; or otherwise life-threatening or requiring immediate medical attention), tag it as a seriousAdverseOutcome instead.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum adverseOutcome {
    MedicalDevice(MedicalDevice),
    TherapeuticProcedure(TherapeuticProcedure),
}
