use crate::prelude::*;

use super::number::Number;
use super::text::Text;

/// The highest value allowed in this rating system. If bestRating is omitted, 5 is assumed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum bestRating {
    Number(Number),
    Text(Text),
}
