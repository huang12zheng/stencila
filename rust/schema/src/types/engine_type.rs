use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;

/// The type of engine or engines powering the vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum engineType {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
