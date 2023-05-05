use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The total number of forward gears available for the transmission system of the vehicle.<br/><br/>  Typical unit code(s): C62
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfForwardGears {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
