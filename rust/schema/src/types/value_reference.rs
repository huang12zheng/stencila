use crate::prelude::*;

use super::property_value::PropertyValue;
use super::qualitative_value::QualitativeValue;
use super::quantitative_value::QuantitativeValue;

/// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum valueReference {
    PropertyValue(PropertyValue),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
}
