use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;

/// The type/class of the seat.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seatingType {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
