use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// Position of the season within an ordered group of seasons.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seasonNumber {
    Integer(Integer),
    Text(Text),
}
