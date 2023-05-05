use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/seasonNumber
/// Position of the season within an ordered group of seasons.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SeasonNumberPropEnum {
    Integer(Integer),
    Text(Text),
}
