use crate::prelude::*;

use super::drug_legal_status::DrugLegalStatus;
use super::medical_enumeration::MedicalEnumeration;
use super::text::Text;

/// The drug or supplement's legal status, including any controlled substance schedules that apply.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legalStatus {
    DrugLegalStatus(DrugLegalStatus),
    MedicalEnumeration(MedicalEnumeration),
    Text(Text),
}
