use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// Identifies the issue of publication; for example, "iii" or "2".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum issueNumber {
    Integer(Integer),
    Text(Text),
}
