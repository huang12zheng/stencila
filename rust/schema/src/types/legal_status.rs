use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;
use super::medical_entity::MedicalEntity;

/// The drug or supplement's legal status, including any controlled substance schedules that apply.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legalStatus {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
    MedicalEntity(MedicalEntity),
}
