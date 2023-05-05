use crate::prelude::*;

use super::medical_enumeration::MedicalEnumeration;
use super::text::Text;

/// Range of acceptable values for a typical patient, when applicable.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum normalRange {
    MedicalEnumeration(MedicalEnumeration),
    Text(Text),
}
