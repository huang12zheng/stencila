use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// The position of an item in a series or sequence of items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum position {
    Integer(Integer),
    Text(Text),
}
