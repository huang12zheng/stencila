use crate::prelude::*;

use super::integer::Integer;
use super::structured_value::StructuredValue;


/// http://schema.org/typicalCreditsPerTerm
/// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TypicalCreditsPerTermPropEnum {
    Integer(Integer),
    StructuredValue(StructuredValue),
}
