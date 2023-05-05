use crate::prelude::*;

use super::distance::Distance;
use super::number::Number;
use super::text::Text;

/// Indicates the approximate radius of a GeoCircle (metres unless indicated otherwise via Distance notation).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum geoRadius {
    Distance(Distance),
    Number(Number),
    Text(Text),
}
