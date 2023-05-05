use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;

/// The priority status assigned to a passenger for security or boarding (e.g. FastTrack or Priority).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum passengerPriorityStatus {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
