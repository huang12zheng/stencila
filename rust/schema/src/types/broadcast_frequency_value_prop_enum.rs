use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/broadcastFrequencyValue
/// The frequency in MHz for a particular broadcast.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BroadcastFrequencyValuePropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
