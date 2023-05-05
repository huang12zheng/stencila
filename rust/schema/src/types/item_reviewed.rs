use crate::prelude::*;

use super::aggregate_rating::AggregateRating;
use super::review::Review;

/// The item that is being reviewed/rated.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum itemReviewed {
    AggregateRating(AggregateRating),
    Review(Review),
}
