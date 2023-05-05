use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// The page on which the work starts; for example "135" or "xiii".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pageStart {
    Integer(Integer),
    Text(Text),
}
