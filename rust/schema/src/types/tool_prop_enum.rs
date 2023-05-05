use crate::prelude::*;

use super::how_to_tool::HowToTool;
use super::text::Text;


/// http://schema.org/tool
/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ToolPropEnum {
    HowToTool(HowToTool),
    Text(Text),
}
