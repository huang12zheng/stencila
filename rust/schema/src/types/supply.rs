use crate::prelude::*;

use super::how_to_supply::HowToSupply;
use super::text::Text;

/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum supply {
    HowToSupply(HowToSupply),
    Text(Text),
}
