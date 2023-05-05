use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/repetitions
/// Number of times one should repeat the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RepetitionsPropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
