use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::offer::Offer;
use super::product::Product;

/// Indicates whether this content is family friendly.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isFamilyFriendly {
    CreativeWork(CreativeWork),
    Offer(Offer),
    Product(Product),
}
