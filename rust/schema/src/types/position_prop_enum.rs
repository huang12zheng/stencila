use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/position
/// The position of an item in a series or sequence of items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PositionPropEnum {
    Integer(Integer),
    Text(Text),
}
