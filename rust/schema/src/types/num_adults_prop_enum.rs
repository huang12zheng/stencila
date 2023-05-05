use crate::prelude::*;

use super::integer::Integer;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/numAdults
/// The number of adults staying in the unit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NumAdultsPropEnum {
    Integer(Integer),
    QuantitativeValue(QuantitativeValue),
}
