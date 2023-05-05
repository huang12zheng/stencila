use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// Identifies the volume of publication or multi-part work; for example, "iii" or "2".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum volumeNumber {
    Integer(Integer),
    Text(Text),
}
