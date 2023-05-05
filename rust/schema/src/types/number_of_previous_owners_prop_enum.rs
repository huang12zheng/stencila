use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/numberOfPreviousOwners
/// The number of owners of the vehicle, including the current one.<br/><br/>  Typical unit code(s): C62
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NumberOfPreviousOwnersPropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
