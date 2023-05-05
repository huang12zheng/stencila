use crate::prelude::*;

use super::integer::Integer;
use super::structured_value::StructuredValue;

/// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfCredits {
    Integer(Integer),
    StructuredValue(StructuredValue),
}
