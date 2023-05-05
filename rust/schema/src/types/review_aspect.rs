use crate::prelude::*;

use super::guide::Guide;
use super::rating::Rating;
use super::review::Review;

/// This Review or Rating is relevant to this part or facet of the itemReviewed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum reviewAspect {
    Guide(Guide),
    Rating(Rating),
    Review(Review),
}
