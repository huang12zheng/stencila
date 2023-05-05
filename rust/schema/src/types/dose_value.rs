use crate::prelude::*;

use super::number::Number;
use super::qualitative_value::QualitativeValue;

/// The value of the dose, e.g. 500.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum doseValue {
    Number(Number),
    QualitativeValue(QualitativeValue),
}
