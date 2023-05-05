use crate::prelude::*;

use super::medical_enumeration::MedicalEnumeration;
use super::text::Text;


/// http://schema.org/normalRange
/// Range of acceptable values for a typical patient, when applicable.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NormalRangePropEnum {
    MedicalEnumeration(MedicalEnumeration),
    Text(Text),
}
