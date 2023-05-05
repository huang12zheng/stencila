use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// Number of times one should repeat the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum repetitions {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
