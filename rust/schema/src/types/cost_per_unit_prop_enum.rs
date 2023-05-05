use crate::prelude::*;

use super::number::Number;
use super::qualitative_value::QualitativeValue;
use super::text::Text;


/// http://schema.org/costPerUnit
/// The cost per unit of the drug.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CostPerUnitPropEnum {
    Number(Number),
    QualitativeValue(QualitativeValue),
    Text(Text),
}
