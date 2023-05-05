use crate::prelude::*;

use super::number::Number;
use super::text::Text;

/// Any discount applied (to an Order).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum discount {
    Number(Number),
    Text(Text),
}
