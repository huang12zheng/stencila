use crate::prelude::*;

use super::number::Number;
use super::qualitative_value::QualitativeValue;
use super::text::Text;

/// The cost per unit of the drug.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum costPerUnit {
    Number(Number),
    QualitativeValue(QualitativeValue),
    Text(Text),
}
