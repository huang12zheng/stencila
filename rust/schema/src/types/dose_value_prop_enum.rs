use crate::prelude::*;

use super::number::Number;
use super::qualitative_value::QualitativeValue;


/// http://schema.org/doseValue
/// The value of the dose, e.g. 500.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DoseValuePropEnum {
    Number(Number),
    QualitativeValue(QualitativeValue),
}
