use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;


/// http://schema.org/lodgingUnitType
/// Textual description of the unit type (including suite vs. room, size of bed, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LodgingUnitTypePropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
