use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/worstRating
/// The lowest value allowed in this rating system. If worstRating is omitted, 1 is assumed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum WorstRatingPropEnum {
    Number(Number),
    Text(Text),
}
