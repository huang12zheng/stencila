use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;

/// Indicates that the vehicle meets the respective emission standard.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum meetsEmissionStandard {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
