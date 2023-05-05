use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;


/// http://schema.org/actionOption
/// A sub property of object. The options subject to this action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ActionOptionPropEnum {
    Text(Text),
    Thing(Thing),
}
