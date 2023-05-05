use crate::prelude::*;

use super::rating::Rating;
use super::text::Text;

/// Official rating of a piece of content&#x2014;for example, 'MPAA PG-13'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contentRating {
    Rating(Rating),
    Text(Text),
}
