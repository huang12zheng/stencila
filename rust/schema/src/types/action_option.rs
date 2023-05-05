use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;

/// A sub property of object. The options subject to this action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum actionOption {
    Text(Text),
    Thing(Thing),
}
