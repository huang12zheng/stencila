use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;

/// The page on which the work ends; for example "138" or "xvi".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pageEnd {
    Integer(Integer),
    Text(Text),
}
