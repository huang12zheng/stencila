use crate::prelude::*;

use super::medical_contraindication::MedicalContraindication;
use super::text::Text;

/// A contraindication for this therapy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contraindication {
    MedicalContraindication(MedicalContraindication),
    Text(Text),
}
