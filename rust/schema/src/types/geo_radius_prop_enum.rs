use crate::prelude::*;

use super::distance::Distance;
use super::number::Number;
use super::text::Text;


/// http://schema.org/geoRadius
/// Indicates the approximate radius of a GeoCircle (metres unless indicated otherwise via Distance notation).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoRadiusPropEnum {
    Distance(Distance),
    Number(Number),
    Text(Text),
}
