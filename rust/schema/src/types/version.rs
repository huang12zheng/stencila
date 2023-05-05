use crate::prelude::*;

use super::number::Number;
use super::text::Text;

/// The version of the CreativeWork embodied by a specified resource.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum version {
    Number(Number),
    Text(Text),
}
