use crate::prelude::*;

use super::medical_contraindication::MedicalContraindication;
use super::text::Text;


/// http://schema.org/contraindication
/// A contraindication for this therapy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ContraindicationPropEnum {
    MedicalContraindication(MedicalContraindication),
    Text(Text),
}
