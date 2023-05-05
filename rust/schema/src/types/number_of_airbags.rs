use crate::prelude::*;

use super::number::Number;
use super::text::Text;

/// The number or type of airbags in the vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfAirbags {
    Number(Number),
    Text(Text),
}
