use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The frequency in MHz for a particular broadcast.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum broadcastFrequencyValue {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
