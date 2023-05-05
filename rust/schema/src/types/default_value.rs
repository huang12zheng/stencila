use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;

/// The default value of the input.  For properties that expect a literal, the default is a literal value, for properties that expect an object, it's an ID reference to one of the current values.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum defaultValue {
    Text(Text),
    Thing(Thing),
}
