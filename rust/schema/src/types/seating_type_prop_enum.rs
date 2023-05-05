use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;


/// http://schema.org/seatingType
/// The type/class of the seat.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SeatingTypePropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
