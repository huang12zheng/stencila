use crate::prelude::*;

use super::drug_prescription_status::DrugPrescriptionStatus;
use super::text::Text;


/// http://schema.org/prescriptionStatus
/// Indicates the status of drug prescription, e.g. local catalogs classifications or whether the drug is available by prescription or over-the-counter, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PrescriptionStatusPropEnum {
    DrugPrescriptionStatus(DrugPrescriptionStatus),
    Text(Text),
}
