use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;

/// The modulation (e.g. FM, AM, etc) used by a particular broadcast service.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum broadcastSignalModulation {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
