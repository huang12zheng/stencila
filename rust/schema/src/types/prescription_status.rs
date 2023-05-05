use crate::prelude::*;

use super::drug_prescription_status::DrugPrescriptionStatus;
use super::text::Text;

/// Indicates the status of drug prescription, e.g. local catalogs classifications or whether the drug is available by prescription or over-the-counter, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum prescriptionStatus {
    DrugPrescriptionStatus(DrugPrescriptionStatus),
    Text(Text),
}
