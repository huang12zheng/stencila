use crate::prelude::*;

use super::drug_legal_status::DrugLegalStatus;
use super::medical_enumeration::MedicalEnumeration;
use super::text::Text;


/// http://schema.org/legalStatus
/// The drug or supplement's legal status, including any controlled substance schedules that apply.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegalStatusPropEnum {
    DrugLegalStatus(DrugLegalStatus),
    MedicalEnumeration(MedicalEnumeration),
    Text(Text),
}
