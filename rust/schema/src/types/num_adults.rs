use crate::prelude::*;

use super::integer::Integer;
use super::quantitative_value::QuantitativeValue;

/// The number of adults staying in the unit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numAdults {
    Integer(Integer),
    QuantitativeValue(QuantitativeValue),
}
