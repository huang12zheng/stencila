use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// Position of the episode within an ordered group of episodes.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum episodeNumber {
    Integer(Integer),
    Text(Text),
}
