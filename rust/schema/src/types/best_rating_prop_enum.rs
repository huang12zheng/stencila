use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/bestRating
/// The highest value allowed in this rating system. If bestRating is omitted, 5 is assumed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BestRatingPropEnum {
    Number(Number),
    Text(Text),
}
