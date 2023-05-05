use crate::prelude::*;

use super::integer::Integer;
use super::structured_value::StructuredValue;


/// http://schema.org/numberOfCredits
/// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NumberOfCreditsPropEnum {
    Integer(Integer),
    StructuredValue(StructuredValue),
}
