use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The number of axles.<br/><br/>  Typical unit code(s): C62
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfAxles {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}