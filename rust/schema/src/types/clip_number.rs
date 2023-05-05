use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// Position of the clip within an ordered group of clips.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum clipNumber {
    Integer(Integer),
    Text(Text),
}
