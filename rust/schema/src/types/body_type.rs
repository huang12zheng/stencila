use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;

/// Indicates the design and body style of the vehicle (e.g. station wagon, hatchback, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum bodyType {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
