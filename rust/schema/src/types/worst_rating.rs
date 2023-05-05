use crate::prelude::*;

use super::number::Number;
use super::text::Text;

/// The lowest value allowed in this rating system. If worstRating is omitted, 1 is assumed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum worstRating {
    Number(Number),
    Text(Text),
}
