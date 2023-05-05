use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/seatingCapacity
/// The number of persons that can be seated (e.g. in a vehicle), both in terms of the physical space available, and in terms of limitations set by law.<br/><br/>  Typical unit code(s): C62 for persons
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SeatingCapacityPropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
