use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/clipNumber
/// Position of the clip within an ordered group of clips.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ClipNumberPropEnum {
    Integer(Integer),
    Text(Text),
}
