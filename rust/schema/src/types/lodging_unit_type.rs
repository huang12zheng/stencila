use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;

/// Textual description of the unit type (including suite vs. room, size of bed, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum lodgingUnitType {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
