use crate::prelude::*;

use super::how_to_tool::HowToTool;
use super::text::Text;

/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum tool {
    HowToTool(HowToTool),
    Text(Text),
}
