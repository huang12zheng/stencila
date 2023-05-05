use crate::prelude::*;

use super::integer::Integer;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/partySize
/// Number of people the reservation should accommodate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PartySizePropEnum {
    Integer(Integer),
    QuantitativeValue(QuantitativeValue),
}
