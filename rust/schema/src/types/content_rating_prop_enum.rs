use crate::prelude::*;

use super::rating::Rating;
use super::text::Text;


/// http://schema.org/contentRating
/// Official rating of a piece of content&#x2014;for example, 'MPAA PG-13'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ContentRatingPropEnum {
    Rating(Rating),
    Text(Text),
}
